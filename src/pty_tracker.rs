//! PTY process CWD tracking via platform-specific APIs
//!
//! This module provides functionality to track the current working directory
//! of PTY child processes. On macOS, this uses libproc to query process info.
//! On Linux, this reads from /proc/{pid}/cwd.

use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Tracks PTY process working directory
pub struct PtyTracker {
    /// PTY child process ID
    pid: u32,
    /// Last known current working directory
    current_dir: PathBuf,
    /// Timestamp of last successful poll
    last_poll: Instant,
    /// Polling interval
    poll_interval: Duration,
}

impl PtyTracker {
    /// Create a new tracker for the given process ID
    ///
    /// Returns None if the process CWD cannot be determined
    pub fn new(pid: u32) -> Option<Self> {
        let current_dir = get_process_cwd(pid)?;
        Some(Self {
            pid,
            current_dir,
            last_poll: Instant::now(),
            poll_interval: Duration::from_millis(500),
        })
    }

    /// Get the process ID being tracked
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Get the last known current working directory
    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    /// Set the polling interval
    pub fn set_interval(&mut self, interval: Duration) {
        self.poll_interval = interval;
    }

    /// Poll for CWD changes if the interval has elapsed
    ///
    /// Returns true if the CWD has changed since the last poll
    pub fn poll(&mut self) -> bool {
        if self.last_poll.elapsed() < self.poll_interval {
            return false;
        }

        self.last_poll = Instant::now();

        if let Some(new_dir) = get_process_cwd(self.pid) {
            if new_dir != self.current_dir {
                self.current_dir = new_dir;
                return true;
            }
        }
        // If we can't get the CWD, keep the last known value

        false
    }
}

/// Get the current working directory of a process by PID (macOS implementation)
///
/// Uses libproc's proc_pidinfo with PROC_PIDVNODEPATHINFO flavor to get the
/// process's current directory (pvi_cdir).
#[cfg(target_os = "macos")]
fn get_process_cwd(pid: u32) -> Option<PathBuf> {
    use std::ffi::CStr;
    use std::mem;

    // proc_vnodepathinfo contains pvi_cdir (current dir) and pvi_rdir (root dir)
    // pvi_cdir is a vnode_info_path which contains vip_path as [c_char; 1024]
    #[repr(C)]
    struct VnodeInfoPath {
        _vi: [u8; 152], // vnode_info struct (we don't need its contents)
        vip_path: [i8; 1024],
    }

    #[repr(C)]
    struct ProcVnodePathInfo {
        pvi_cdir: VnodeInfoPath,
        _pvi_rdir: VnodeInfoPath,
    }

    // PROC_PIDVNODEPATHINFO = 9
    const PROC_PIDVNODEPATHINFO: i32 = 9;

    extern "C" {
        fn proc_pidinfo(
            pid: i32,
            flavor: i32,
            arg: u64,
            buffer: *mut libc::c_void,
            buffersize: i32,
        ) -> i32;
    }

    let mut info: ProcVnodePathInfo = unsafe { mem::zeroed() };
    let info_size = mem::size_of::<ProcVnodePathInfo>() as i32;

    let ret = unsafe {
        proc_pidinfo(
            pid as i32,
            PROC_PIDVNODEPATHINFO,
            0,
            &mut info as *mut _ as *mut libc::c_void,
            info_size,
        )
    };

    if ret <= 0 {
        return None;
    }

    // Convert the path bytes to a string
    let path_bytes = &info.pvi_cdir.vip_path;
    let cstr = unsafe { CStr::from_ptr(path_bytes.as_ptr()) };

    cstr.to_str().ok().map(PathBuf::from)
}

/// Get the current working directory of a process by PID (Linux implementation)
///
/// Reads the /proc/{pid}/cwd symlink to get the process's current directory.
#[cfg(target_os = "linux")]
fn get_process_cwd(pid: u32) -> Option<PathBuf> {
    std::fs::read_link(format!("/proc/{}/cwd", pid)).ok()
}

/// Fallback for unsupported platforms - always returns None
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn get_process_cwd(_pid: u32) -> Option<PathBuf> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_process_cwd() {
        let pid = std::process::id();
        let cwd = get_process_cwd(pid);
        assert!(cwd.is_some(), "Should be able to get CWD of current process");

        let expected = std::env::current_dir().ok();
        assert_eq!(cwd, expected, "CWD should match env::current_dir()");
    }

    #[test]
    fn test_tracker_creation() {
        let pid = std::process::id();
        let tracker = PtyTracker::new(pid);
        assert!(tracker.is_some(), "Should create tracker for current process");

        let tracker = tracker.unwrap();
        assert_eq!(tracker.pid(), pid);
    }

    #[test]
    fn test_tracker_poll_interval() {
        let pid = std::process::id();
        let mut tracker = PtyTracker::new(pid).unwrap();

        // First poll after creation should return false (no change, interval not elapsed)
        assert!(!tracker.poll());

        // Set a very short interval
        tracker.set_interval(Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(5));

        // Now poll should work (interval elapsed), but no change expected
        assert!(!tracker.poll());
    }
}

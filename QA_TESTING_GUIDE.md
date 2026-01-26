# VibeTerm P0-P2 Features - Complete QA Testing Guide

**Version:** v0.6.0  
**Platform:** macOS (Darwin 25.2.0)  
**Date:** 2026-01-26  
**Status:** Ready for Manual Testing

---

## Quick Start

### Option 1: Automated Pre-Flight Checks
```bash
cd /Users/bernocrest/Desktop/dev/projects/vibeterm
./run_qa_tests.sh
```

This script will:
- Verify binary exists and is recent
- Check system resources
- Create test artifacts
- Generate timestamped test report
- Optionally launch VibeTerm with logging

### Option 2: Manual Launch
```bash
cd /Users/bernocrest/Desktop/dev/projects/vibeterm
./target/release/vibeterm
```

For debug logging:
```bash
RUST_LOG=debug ./target/release/vibeterm 2>&1 | tee vibeterm_qa.log
```

---

## Test Documentation

### 1. QA_TEST_REPORT.md
Comprehensive test plan with 10 test cases covering:
- **P0 Tests (4):** Multi-pane contextual sidebar
- **P1 Tests (2):** Scrollback & text selection
- **P2 Tests (4):** Command palette & tab drag-drop

Each test includes:
- Objective
- Step-by-step instructions
- Expected behavior
- Actual results section (to be filled)

### 2. QA_SHORTCUTS.md
Quick reference for keyboard shortcuts organized by category:
- Application controls
- Tab management
- Pane management
- Command palette
- Sidebar
- Terminal operations
- Text selection

Includes troubleshooting tips and platform-specific behaviors.

### 3. run_qa_tests.sh
Automated pre-flight check script that:
- Verifies binary exists and is recent
- Checks dependencies (cargo, rustc)
- Validates test directories
- Checks project markers (.git, Cargo.toml)
- Monitors system resources (RAM, CPU)
- Detects existing VibeTerm processes
- Creates test artifacts
- Generates timestamped test report
- Optionally launches VibeTerm with logging

---

## Test Coverage Matrix

| Priority | Feature | Test Count | Status |
|----------|---------|------------|--------|
| P0 | Multi-Pane Contextual Sidebar | 4 | Ready |
| P1 | Scrollback & Text Selection | 2 | Ready |
| P2 | Command Palette | 3 | Ready |
| P2 | Tab Drag-Drop | 1 | Ready |
| **Total** | | **10** | **Ready** |

---

## Specific Test Cases

### P0: Multi-Pane Contextual Sidebar

**Test 1: CWD Tracking**
- Verify sidebar updates when changing directories
- Test: `cd /tmp` → sidebar shows /tmp contents

**Test 2: Project Root Detection**
- Verify sidebar prefers project root over CWD
- Test: Navigate to subdirectory → sidebar still shows root

**Test 3: Pane Indicators**
- Verify sidebar shows pane mini-tabs
- Test: Split pane → click indicators → focus switches

**Test 4: Async Directory Loading**
- Verify large directories don't freeze UI
- Test: `cd /usr/bin` → UI stays responsive

### P1: Scrollback & Text Selection

**Test 5: Scrollback Buffer**
- Verify can scroll terminal history
- Test: Generate long output → scroll up → scroll down

**Test 6: Text Selection**
- Verify click-drag, double-click, triple-click selection
- Test: Select text → copy → paste elsewhere

### P2: Command Palette

**Test 7: Command Palette Opening**
- Verify palette opens and closes
- Test: Cmd+P → palette appears → ESC → closes

**Test 8: Fuzzy Search**
- Verify fuzzy search filters commands
- Test: Type "split" → relevant commands shown

**Test 9: Command Execution**
- Verify commands execute correctly
- Test: Execute "New Tab", "Split", "Toggle Sidebar"

### P2: Tab Drag-Drop

**Test 10: Tab Reordering**
- Verify tabs can be reordered via drag-drop
- Test: Drag tab → ghost preview → drop → order changes

---

## Expected Results

### Performance Targets
- **Startup time:** < 2 seconds
- **Terminal responsiveness:** Excellent (no lag on typing)
- **Sidebar loading:** < 2 seconds for 1000+ files
- **UI frame rate:** Smooth (60 FPS target)

### Stability Targets
- **Crashes:** 0
- **Memory leaks:** None
- **CPU usage (idle):** < 5%
- **CPU usage (active):** < 30%

### Visual Targets
- **Theme consistency:** All UI elements use theme colors
- **Text rendering:** Sharp, no blurriness
- **Colors/contrast:** Readable in all lighting conditions
- **Animations:** Smooth, no jank

---

## Known Limitations

1. **CWD Tracking:** Requires macOS libproc or Linux procfs
2. **File Count:** Sidebar caps at ~1000 files for performance
3. **Project Detection:** Looks for .git, Cargo.toml, package.json, go.mod
4. **Platform:** Currently tested on macOS only

---

## Test Execution Workflow

### Before Testing
1. Kill any existing VibeTerm processes:
   ```bash
   killall vibeterm
   ```

2. Open Activity Monitor for performance tracking

3. Run pre-flight checks:
   ```bash
   ./run_qa_tests.sh
   ```

4. Review keyboard shortcuts:
   ```bash
   cat QA_SHORTCUTS.md
   ```

### During Testing
1. Launch VibeTerm (via script or manually)

2. Work through QA_TEST_REPORT.md test cases sequentially

3. Document results in test report:
   - ✅ PASS - Feature works as expected
   - ⚠️ PARTIAL - Works with minor issues
   - ❌ FAIL - Feature broken
   - 📝 NOTES - Observations

4. Take screenshots of any issues

5. Record error messages verbatim

6. Monitor Activity Monitor for memory/CPU anomalies

### After Testing
1. Complete test report summary section:
   - Total tests passed/partial/failed
   - Overall status (Ready/Needs Fixes/Major Issues)
   - Recommendation

2. Review logs:
   ```bash
   cat qa_logs/vibeterm_*.log
   ```

3. Check for crash reports:
   ```bash
   open ~/Library/Logs/DiagnosticReports/
   ```

4. Archive test results:
   ```bash
   tar -czf qa_results_$(date +%Y%m%d).tar.gz qa_logs/
   ```

---

## Bug Reporting Template

If issues are found, report using this format:

```
**Test Case:** Test N: [Name]
**Status:** ❌ FAIL
**Severity:** [Critical/High/Medium/Low]

**Steps to Reproduce:**
1. ...
2. ...
3. ...

**Expected Behavior:**
...

**Actual Behavior:**
...

**Evidence:**
- Screenshot: [path/to/screenshot.png]
- Log excerpt: [relevant log lines]
- System info: [macOS version, hardware]

**Workaround:**
...
```

---

## Success Criteria

Testing is complete when:
- ✅ All 10 test cases executed
- ✅ Results documented in test report
- ✅ Performance targets met
- ✅ No critical bugs found
- ✅ Test report signed off

Release readiness:
- **Ready for Release:** 10/10 PASS, 0 critical bugs
- **Needs Fixes:** 8+/10 PASS, minor bugs documented
- **Major Issues:** <8/10 PASS or critical bugs present

---

## Additional Resources

### Source Code References
- Main app: `/Users/bernocrest/Desktop/dev/projects/vibeterm/src/app.rs`
- UI components: `/Users/bernocrest/Desktop/dev/projects/vibeterm/src/ui/`
- Config: `/Users/bernocrest/Desktop/dev/projects/vibeterm/src/config.rs`

### Documentation
- Completion summary: `COMPLETION_SUMMARY.md`
- Keyboard shortcuts: `SHORTCUTS.md`
- Changelog: `CHANGELOG.md`

### Logs Location
- Test logs: `qa_logs/`
- Application logs: `vibeterm.log` (if RUST_LOG=info)
- Crash reports: `~/Library/Logs/DiagnosticReports/`

---

## Questions or Issues?

1. Review COMPLETION_SUMMARY.md for feature specifications
2. Check source code for implementation details
3. Test with RUST_LOG=debug for verbose diagnostics
4. Review QA_SHORTCUTS.md for correct keyboard shortcuts

---

**Testing Team:** Ready to begin  
**Estimated Time:** 30-45 minutes for complete test suite  
**Last Updated:** 2026-01-26


# VibeTerm P0-P2 Manual QA Test Report

**Version:** v0.6.0  
**Platform:** macOS (Darwin 25.2.0)  
**Date:** 2026-01-26  
**Binary:** `/Users/bernocrest/Desktop/dev/projects/vibeterm/target/release/vibeterm`

---

## Test Execution Instructions

Launch VibeTerm:
```bash
cd /Users/bernocrest/Desktop/dev/projects/vibeterm
./target/release/vibeterm
```

For verbose logging (optional):
```bash
RUST_LOG=debug ./target/release/vibeterm
```

---

## P0 Tests: Multi-Pane Contextual Sidebar

### Test 1: CWD Tracking
**Objective:** Verify sidebar tracks current working directory

**Steps:**
1. Launch VibeTerm
2. Note initial sidebar contents (should show project root if .git detected)
3. In terminal, run: `cd /tmp`
4. Wait 2 seconds
5. Observe sidebar contents

**Expected:**
- Sidebar updates to show `/tmp` directory contents
- Files like `.X11-unix/`, `.ICE-unix/` visible
- Loading indicator appears briefly during scan

**Actual:**
- Status: [ ] ✅ PASS / [ ] ⚠️ PARTIAL / [ ] ❌ FAIL
- Notes: _______________________________________

---

### Test 2: Project Root Detection
**Objective:** Verify sidebar prefers project root over CWD

**Steps:**
1. Navigate to this project: `cd /Users/bernocrest/Desktop/dev/projects/vibeterm`
2. Wait 2 seconds, observe sidebar
3. Navigate to subdirectory: `cd src`
4. Wait 2 seconds, observe sidebar

**Expected:**
- Step 2: Sidebar shows project root (vibeterm/) with Cargo.toml, src/, target/ visible
- Step 4: Sidebar STILL shows project root (doesn't switch to src/)
- Sidebar title shows "vibeterm" (project name)

**Actual:**
- Status: [ ] ✅ PASS / [ ] ⚠️ PARTIAL / [ ] ❌ FAIL
- Notes: _______________________________________

---

### Test 3: Pane Indicators
**Objective:** Verify sidebar shows pane mini-tabs and tracks focus

**Steps:**
1. Split horizontally: Press `Cmd+D` (or menu: Shell → Split Pane Horizontally)
2. Observe sidebar header (should show 2 pane indicators)
3. Click on pane 1 indicator in sidebar
4. Note which terminal pane gains focus (should be top pane)
5. Click on pane 2 indicator in sidebar
6. Note which terminal pane gains focus (should be bottom pane)
7. Click directly on bottom terminal pane
8. Observe sidebar indicator (should highlight pane 2)

**Expected:**
- Sidebar header shows mini-tabs: `[1] [2]` or similar visual indicators
- Clicking indicator switches focus to that pane
- Active pane indicator highlighted with distinct color/border
- Sidebar contents update to match active pane's CWD

**Actual:**
- Status: [ ] ✅ PASS / [ ] ⚠️ PARTIAL / [ ] ❌ FAIL
- Notes: _______________________________________

---

### Test 4: Async Directory Loading
**Objective:** Verify large directories load without UI freeze

**Steps:**
1. Navigate to large directory: `cd /usr/bin`
2. Immediately try to interact with terminal (type `ls`)
3. Observe sidebar loading indicator
4. Wait for sidebar to populate
5. Scroll sidebar list, count approximate file entries

**Expected:**
- Terminal remains responsive during sidebar load
- Loading indicator (spinner/text) appears in sidebar
- Sidebar populates within 2-3 seconds
- File count capped at ~1000 entries max
- No application freeze or hang

**Actual:**
- Status: [ ] ✅ PASS / [ ] ⚠️ PARTIAL / [ ] ❌ FAIL
- Notes: _______________________________________
- File count shown: _______

---

## P1 Tests: Scrollback & Text Selection

### Test 5: Scrollback Buffer
**Objective:** Verify terminal scrollback history works

**Steps:**
1. Generate long output: `ls -la /usr/bin` (or `find /usr -type f | head -100`)
2. Observe output scrolls past top of terminal
3. Use mouse wheel to scroll UP in terminal area
4. Scroll to view oldest output at top
5. Scroll DOWN to bottom
6. Generate new output: `echo "test"`

**Expected:**
- Can scroll up to view previous output
- Scrollbar appears when content exceeds viewport
- Auto-follows (sticks to bottom) when scrolled to bottom
- New output visible immediately when at bottom
- Smooth scrolling animation

**Actual:**
- Status: [ ] ✅ PASS / [ ] ⚠️ PARTIAL / [ ] ❌ FAIL
- Notes: _______________________________________

---

### Test 6: Text Selection
**Objective:** Verify text selection and clipboard integration

**Steps:**
1. Generate some output: `echo "Hello VibeTerm Testing 12345"`
2. Click-drag across "VibeTerm" word
3. Observe selection highlighting
4. Press `Cmd+C` to copy
5. Open TextEdit or Notes app, paste with `Cmd+V`
6. Back in VibeTerm, double-click the word "Testing"
7. Observe selection
8. Press `Cmd+C` and paste elsewhere
9. Triple-click anywhere on the "Hello..." line
10. Observe selection

**Expected:**
- Click-drag: Text highlights with distinct background color
- Copy works: Clipboard contains selected text
- Double-click: Entire word selected (word boundaries respected)
- Triple-click: Entire line selected (including newline)
- Selection color matches theme (contrast visible)

**Actual:**
- Status: [ ] ✅ PASS / [ ] ⚠️ PARTIAL / [ ] ❌ FAIL
- Single selection: _______________________________________
- Double-click: _______________________________________
- Triple-click: _______________________________________

---

## P2 Tests: Command Palette

### Test 7: Command Palette Opening
**Objective:** Verify command palette can be opened/closed

**Steps:**
1. Press `Cmd+P` (or `Cmd+Shift+P` depending on implementation)
2. Observe command palette appearance
3. Press `ESC` key
4. Verify palette disappears
5. Click elsewhere in terminal (palette should stay closed)
6. Press `Cmd+P` again

**Expected:**
- Palette appears centered at top 25% of window
- Semi-transparent dark overlay behind palette
- Input field focused (cursor blinking)
- Empty input shows placeholder "Type command..."
- ESC closes palette immediately
- Clicking outside palette closes it

**Actual:**
- Status: [ ] ✅ PASS / [ ] ⚠️ PARTIAL / [ ] ❌ FAIL
- Keyboard shortcut used: _______
- Notes: _______________________________________

---

### Test 8: Fuzzy Search
**Objective:** Verify fuzzy search filters commands

**Steps:**
1. Open command palette (`Cmd+P`)
2. Type: `split`
3. Observe filtered commands
4. Clear input (Cmd+A, Delete)
5. Type: `tab`
6. Observe filtered commands
7. Type: `xyz123` (nonsense)
8. Observe results

**Expected:**
- `split` shows: "Split Pane Horizontally", "Split Pane Vertically"
- `tab` shows: "New Tab", "Close Tab", "Next Tab", "Previous Tab"
- Matching characters highlighted in results
- `xyz123` shows: "No commands found" or empty list
- Fuzzy matching works: `sph` matches "Split Pane Horizontally"

**Actual:**
- Status: [ ] ✅ PASS / [ ] ⚠️ PARTIAL / [ ] ❌ FAIL
- "split" results: _______________________________________
- "tab" results: _______________________________________
- Fuzzy works: [ ] YES / [ ] NO

---

### Test 9: Command Execution
**Objective:** Verify commands execute correctly

**Steps:**
1. Open palette, type `new tab`, press Enter
2. Verify result
3. Open palette, type `split h`, select "Split Pane Horizontally", press Enter
4. Verify result
5. Open palette, type `toggle`, select "Toggle Sidebar", press Enter
6. Verify sidebar hides
7. Open palette again, execute "Toggle Sidebar" again
8. Verify sidebar reappears

**Expected:**
- "New Tab": New workspace tab created in tab bar
- "Split Horizontally": Active pane splits into 2 horizontal panes
- "Toggle Sidebar": Sidebar hides/shows without error
- Palette closes automatically after command execution
- No visual glitches or crashes

**Actual:**
- Status: [ ] ✅ PASS / [ ] ⚠️ PARTIAL / [ ] ❌ FAIL
- New Tab: _______________________________________
- Split: _______________________________________
- Toggle Sidebar: _______________________________________

---

## P2 Tests: Tab Drag-Drop

### Test 10: Tab Reordering
**Objective:** Verify workspace tabs can be reordered via drag-drop

**Steps:**
1. Create 3 tabs total (press `Cmd+T` twice)
2. Note tab order (Tab1, Tab2, Tab3)
3. Click-hold on Tab1's header in tab bar
4. Drag slowly to the right (past Tab2, Tab3)
5. Observe ghost preview following cursor
6. Observe drop indicator (line/arrow showing insertion point)
7. Release mouse button
8. Verify tab order changed
9. Create new drag: Click-hold Tab3
10. Drag left, then press `ESC` mid-drag
11. Verify drag cancelled (tab returns to original position)

**Expected:**
- Ghost preview: Semi-transparent tab header follows cursor
- Drop indicator: Vertical line or arrow shows where tab will insert
- Drop completes: Tab order changes correctly (e.g., Tab2, Tab3, Tab1)
- ESC cancellation: Tab snaps back to original position
- No crashes or visual artifacts

**Actual:**
- Status: [ ] ✅ PASS / [ ] ⚠️ PARTIAL / [ ] ❌ FAIL
- Ghost preview visible: [ ] YES / [ ] NO
- Drop indicator visible: [ ] YES / [ ] NO
- Reordering works: [ ] YES / [ ] NO
- ESC cancellation works: [ ] YES / [ ] NO
- Notes: _______________________________________

---

## Additional Observations

### Performance
- Application startup time: _______ seconds
- Terminal responsiveness: [ ] Excellent / [ ] Good / [ ] Sluggish / [ ] Unresponsive
- Sidebar loading speed: [ ] Fast (<1s) / [ ] Moderate (1-2s) / [ ] Slow (>2s)
- UI frame rate: [ ] Smooth / [ ] Occasional drops / [ ] Consistently choppy

### Stability
- Crashes encountered: [ ] 0 / [ ] 1-2 / [ ] 3+
- Error messages: _______________________________________
- Memory usage (Activity Monitor): _______ MB
- CPU usage (idle): _______ %
- CPU usage (active): _______ %

### Visual Polish
- Theme consistency: [ ] Good / [ ] Minor issues / [ ] Major issues
- Text rendering: [ ] Sharp / [ ] Blurry / [ ] Artifacts
- Colors/contrast: [ ] Readable / [ ] Too dark / [ ] Too bright
- Animations: [ ] Smooth / [ ] Janky / [ ] None visible

---

## Critical Issues Found

List any blocking/critical issues here:

1. _______________________________________
2. _______________________________________
3. _______________________________________

---

## Summary

**Total Tests:** 10  
**Passed:** ___  
**Partial:** ___  
**Failed:** ___  

**Overall Status:** [ ] READY FOR RELEASE / [ ] NEEDS FIXES / [ ] MAJOR ISSUES

**Recommendation:** _______________________________________

---

## Tester Information

- **Tester Name:** _______________________________________
- **Test Duration:** _______ minutes
- **Additional Notes:** _______________________________________


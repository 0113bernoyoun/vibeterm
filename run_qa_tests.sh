#!/bin/bash
# VibeTerm Automated QA Test Runner
# This script performs pre-launch checks and generates test artifacts

set -e

PROJECT_ROOT="/Users/bernocrest/Desktop/dev/projects/vibeterm"
BINARY="$PROJECT_ROOT/target/release/vibeterm"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_DIR="$PROJECT_ROOT/qa_logs"

echo "=========================================="
echo "VibeTerm QA Test Runner v1.0"
echo "Date: $(date)"
echo "=========================================="
echo ""

# Create log directory
mkdir -p "$LOG_DIR"

# 1. Verify binary exists and is recent
echo "[1/7] Checking binary..."
if [[ ! -f "$BINARY" ]]; then
    echo "❌ FAIL: Binary not found at $BINARY"
    echo "Run: cargo build --release"
    exit 1
fi

BINARY_AGE=$(($(date +%s) - $(stat -f %m "$BINARY")))
if [[ $BINARY_AGE -gt 3600 ]]; then
    echo "⚠️  WARNING: Binary is older than 1 hour (${BINARY_AGE}s)"
    echo "Consider rebuilding: cargo build --release"
else
    echo "✅ PASS: Binary found and recent"
fi

BINARY_SIZE=$(stat -f %z "$BINARY")
echo "   Binary size: $(numfmt --to=iec $BINARY_SIZE 2>/dev/null || echo "${BINARY_SIZE} bytes")"

# 2. Check dependencies
echo ""
echo "[2/7] Checking dependencies..."
if command -v cargo &>/dev/null; then
    echo "✅ Cargo: $(cargo --version)"
else
    echo "❌ Cargo not found"
fi

if command -v rustc &>/dev/null; then
    echo "✅ Rustc: $(rustc --version)"
else
    echo "❌ Rustc not found"
fi

# 3. Verify test directories exist
echo ""
echo "[3/7] Verifying test directories..."
TEST_DIRS=(
    "/tmp"
    "/usr/bin"
    "$PROJECT_ROOT"
    "$HOME"
)

for dir in "${TEST_DIRS[@]}"; do
    if [[ -d "$dir" ]]; then
        FILE_COUNT=$(ls -1 "$dir" 2>/dev/null | wc -l | tr -d ' ')
        echo "✅ $dir ($FILE_COUNT files)"
    else
        echo "❌ $dir (not found)"
    fi
done

# 4. Check for project markers
echo ""
echo "[4/7] Checking project markers..."
if [[ -f "$PROJECT_ROOT/.git/config" ]]; then
    echo "✅ Git repository detected"
    GIT_BRANCH=$(cd "$PROJECT_ROOT" && git branch --show-current 2>/dev/null || echo "unknown")
    echo "   Branch: $GIT_BRANCH"
else
    echo "⚠️  No .git directory found"
fi

if [[ -f "$PROJECT_ROOT/Cargo.toml" ]]; then
    echo "✅ Cargo.toml found (Rust project)"
    VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d'"' -f2)
    echo "   Version: $VERSION"
else
    echo "❌ Cargo.toml not found"
fi

# 5. Check system resources
echo ""
echo "[5/7] Checking system resources..."
MEM_TOTAL=$(sysctl -n hw.memsize | awk '{print $0/1024/1024/1024 " GB"}')
echo "✅ Total RAM: $MEM_TOTAL"

CPU_COUNT=$(sysctl -n hw.ncpu)
echo "✅ CPU cores: $CPU_COUNT"

# Check if any VibeTerm processes running
RUNNING_COUNT=$(ps aux | grep -c "[v]ibeterm" || true)
if [[ $RUNNING_COUNT -gt 0 ]]; then
    echo "⚠️  WARNING: $RUNNING_COUNT VibeTerm process(es) already running"
    echo "   PIDs: $(ps aux | grep "[v]ibeterm" | awk '{print $2}' | tr '\n' ' ')"
else
    echo "✅ No existing VibeTerm processes"
fi

# 6. Generate test artifacts
echo ""
echo "[6/7] Generating test artifacts..."

# Create sample test files
mkdir -p "$LOG_DIR/test_files"
echo "This is a test file for sidebar testing" > "$LOG_DIR/test_files/sample.txt"
echo "#!/bin/bash" > "$LOG_DIR/test_files/script.sh"
chmod +x "$LOG_DIR/test_files/script.sh"

# Create test directories
mkdir -p "$LOG_DIR/test_dirs/dir1/subdir"
mkdir -p "$LOG_DIR/test_dirs/dir2"
mkdir -p "$LOG_DIR/test_dirs/dir3"

echo "✅ Test artifacts created in $LOG_DIR"

# 7. Create pre-filled test report
echo ""
echo "[7/7] Creating test report template..."
cp "$PROJECT_ROOT/QA_TEST_REPORT.md" "$LOG_DIR/test_report_${TIMESTAMP}.md"
echo "✅ Test report: $LOG_DIR/test_report_${TIMESTAMP}.md"

# Summary
echo ""
echo "=========================================="
echo "Pre-flight checks complete!"
echo "=========================================="
echo ""
echo "Next steps:"
echo "1. Open Activity Monitor to track performance"
echo "2. Launch VibeTerm:"
echo "   cd $PROJECT_ROOT"
echo "   ./target/release/vibeterm"
echo ""
echo "3. Follow test steps in:"
echo "   $PROJECT_ROOT/QA_TEST_REPORT.md"
echo ""
echo "4. Document results in:"
echo "   $LOG_DIR/test_report_${TIMESTAMP}.md"
echo ""
echo "Logs directory: $LOG_DIR"
echo ""

# Optional: Launch VibeTerm automatically
read -p "Launch VibeTerm now? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Launching VibeTerm..."
    cd "$PROJECT_ROOT"
    RUST_LOG=info "$BINARY" 2>&1 | tee "$LOG_DIR/vibeterm_${TIMESTAMP}.log" &
    VIBETERM_PID=$!
    echo "VibeTerm launched (PID: $VIBETERM_PID)"
    echo "Log: $LOG_DIR/vibeterm_${TIMESTAMP}.log"
    echo ""
    echo "To stop: kill $VIBETERM_PID"
else
    echo "Skipping launch. Run manually when ready."
fi

echo ""
echo "QA testing ready! 🚀"

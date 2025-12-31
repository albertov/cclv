#!/usr/bin/env bash
# Verification script for tracing implementation
# This acts as our "test" for the configuration task

set -e

echo "=== Tracing Verification ==="
echo

# Check 1: No println! in src/
echo "Check 1: No println! in src/"
if grep -r "println!" src/ 2>/dev/null; then
    echo "FAIL: Found println! in src/"
    exit 1
else
    echo "PASS: No println! found"
fi
echo

# Check 2: Tracing dependencies in Cargo.toml
echo "Check 2: Tracing dependencies in Cargo.toml"
if ! grep -q "^tracing = " Cargo.toml; then
    echo "FAIL: tracing dependency missing"
    exit 1
fi
if ! grep -q "^tracing-subscriber = " Cargo.toml; then
    echo "FAIL: tracing-subscriber dependency missing"
    exit 1
fi
echo "PASS: Tracing dependencies present"
echo

# Check 3: Tracing initialized in main.rs
echo "Check 3: Tracing initialized in main.rs"
if ! grep -q "tracing_subscriber" src/main.rs; then
    echo "FAIL: tracing_subscriber not used in main.rs"
    exit 1
fi
echo "PASS: Tracing subscriber initialized"
echo

# Check 4: At least one tracing macro used
echo "Check 4: At least one tracing macro used (debug!, trace!, info!, etc.)"
if ! grep -rE "(debug!|trace!|info!|warn!|error!)" src/ 2>/dev/null; then
    echo "FAIL: No tracing macros found in src/"
    exit 1
fi
echo "PASS: Tracing macros in use"
echo

# Check 5: Builds with zero warnings
echo "Check 5: Cargo build succeeds with zero warnings"
if ! cargo build 2>&1 | tee /tmp/build.log | grep -q "Finished"; then
    echo "FAIL: Build failed"
    cat /tmp/build.log
    exit 1
fi
if grep -i "warning" /tmp/build.log; then
    echo "FAIL: Build has warnings"
    exit 1
fi
echo "PASS: Clean build"
echo

echo "=== ALL CHECKS PASSED ==="

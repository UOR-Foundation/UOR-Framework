#!/bin/bash
# UOR Benchmark Conformance Validator
#
# This script runs UOR benchmarks and validates results against conformance targets.
#
# Usage:
#   ./scripts/validate_benchmarks.sh [cpu_ghz]
#
# Arguments:
#   cpu_ghz  - CPU frequency estimate in GHz (default: 3.5)
#
# Conformance Targets:
#   - Single wavefront: < 5 cycles
#   - 64-wavefront sequence: < 200 cycles
#   - Throughput: >= 512 bits/cycle

set -e

CPU_GHZ=${1:-3.5}
TARGET_SINGLE_CYCLES=5
TARGET_SEQUENCE_CYCLES=200
TARGET_BITS_PER_CYCLE=512
VIRTUALIZED_THRESHOLD_NS=15
UOR_STATE_BITS=4992

echo "=============================================="
echo "UOR Benchmark Conformance Validator"
echo "=============================================="
echo ""
echo "Configuration:"
echo "  CPU frequency estimate: ${CPU_GHZ} GHz"
echo "  Single wavefront target: < ${TARGET_SINGLE_CYCLES} cycles"
echo "  64-wavefront sequence target: < ${TARGET_SEQUENCE_CYCLES} cycles"
echo "  Throughput target: >= ${TARGET_BITS_PER_CYCLE} bits/cycle"
echo ""

# Check if we're on x86_64
ARCH=$(uname -m)
if [[ "$ARCH" != "x86_64" ]]; then
    echo "WARNING: Not running on x86_64 architecture. Benchmarks require x86_64."
    exit 1
fi

# Run benchmarks
echo "Running benchmarks..."
echo ""
RUSTFLAGS="-C target-feature=+avx2,+sha,+aes" cargo bench -p uor -- --noplot 2>&1 | tee /tmp/uor_bench.txt

echo ""
echo "=============================================="
echo "Conformance Validation Results"
echo "=============================================="
echo ""

# Track pass/fail counts
PASS_COUNT=0
WARN_COUNT=0
FAIL_COUNT=0

# Function to validate operation
validate_operation() {
    local op_name="$1"
    local search_pattern="$2"

    # Extract median time from Criterion output
    # Format: "name    time:   [X.XX ns X.XX ns X.XX ns]"
    NS=$(grep -E "${search_pattern}" /tmp/uor_bench.txt | grep -oP 'time:\s+\[\d+\.\d+ ns \K\d+\.\d+' 2>/dev/null | head -1 || echo "")

    if [ -n "$NS" ]; then
        CYCLES=$(echo "scale=2; $NS * $CPU_GHZ" | bc)
        CYCLES_INT=$(printf "%.0f" "$CYCLES")

        if [ "$CYCLES_INT" -le "$TARGET_SINGLE_CYCLES" ]; then
            echo "  PASS  $op_name: ${NS} ns = ~${CYCLES_INT} cycles"
            ((PASS_COUNT++))
        elif (( $(echo "$NS < $VIRTUALIZED_THRESHOLD_NS" | bc -l) )); then
            echo "  WARN  $op_name: ${NS} ns = ~${CYCLES_INT} cycles (virtualized?)"
            ((WARN_COUNT++))
        else
            echo "  FAIL  $op_name: ${NS} ns = ~${CYCLES_INT} cycles"
            ((FAIL_COUNT++))
        fi
    fi
}

echo "Single Wavefront Operations:"
echo ""

# Validate conformance group operations
for op in xor and or not add sub rotr_7 rotr_13 rotr_22 rotl_7 shr_3 shl_10 shuffle permute sha256_round aes_round; do
    validate_operation "$op" "conformance/${op}[[:space:]]+time"
done

echo ""
echo "Port Efficiency:"
echo ""

for port in single_port two_ports all_ports max_complexity; do
    validate_operation "$port" "port_efficiency/${port}[[:space:]]+time"
done

echo ""
echo "=============================================="
echo "Summary"
echo "=============================================="
echo ""
echo "  Passed:   $PASS_COUNT"
echo "  Warnings: $WARN_COUNT"
echo "  Failed:   $FAIL_COUNT"
echo ""

# Calculate throughput from XOR benchmark
XOR_NS=$(grep -E "conformance/xor[[:space:]]+time" /tmp/uor_bench.txt | grep -oP 'time:\s+\[\d+\.\d+ ns \K\d+\.\d+' 2>/dev/null | head -1 || echo "")
if [ -n "$XOR_NS" ]; then
    CYCLES=$(echo "scale=4; $XOR_NS * $CPU_GHZ" | bc)
    if (( $(echo "$CYCLES > 0" | bc -l) )); then
        BPC=$(echo "scale=0; $UOR_STATE_BITS / $CYCLES" | bc)
        echo "Estimated throughput: $BPC bits/cycle"
        if [ "$BPC" -ge "$TARGET_BITS_PER_CYCLE" ]; then
            echo "  PASS  Throughput target met (>= $TARGET_BITS_PER_CYCLE bits/cycle)"
        else
            echo "  WARN  Throughput below target (< $TARGET_BITS_PER_CYCLE bits/cycle)"
        fi
    fi
fi

echo ""
echo "Validation complete."

# Exit with error if any failures
if [ "$FAIL_COUNT" -gt 0 ]; then
    exit 1
fi

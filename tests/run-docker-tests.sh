#!/bin/bash

set -e

echo "🐳 Running Docker Installation Tests (Parallel Mode)"

# Create temporary directory for results
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Function to run test for a specific OS in background
run_test() {
    local os=$1
    local result_file="$TEMP_DIR/$os.result"
    local log_file="$TEMP_DIR/$os.log"
    
    {
        echo "🔨 Building $os image..."
        if docker build -t projektwoche-test-$os -f tests/Dockerfile.$os . &> "$log_file.build"; then
            echo "✅ $os image built successfully"
            
            echo "🚀 Running $os test..."
            if docker run --rm projektwoche-test-$os &> "$log_file.run"; then
                echo "✅ $os test passed"
                echo "PASSED" > "$result_file"
            else
                echo "❌ $os test failed"
                echo "FAILED" > "$result_file"
            fi
        else
            echo "❌ $os image build failed"
            echo "BUILD_FAILED" > "$result_file"
        fi
    } &
    
    echo $! > "$TEMP_DIR/$os.pid"
}

# Parse command line arguments
OSES=()
RUN_ALL=true

while [[ $# -gt 0 ]]; do
    case $1 in
        ubuntu|windows)
            OSES+=("$1")
            RUN_ALL=false
            shift
            ;;
        --all|-a)
            RUN_ALL=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [ubuntu|windows] [--all|-a] [--help|-h]"
            echo "  ubuntu, windows: Run tests for specific operating systems"
            echo "  --all, -a: Run tests for all operating systems (default)"
            echo "  --help, -h: Show this help message"
            echo ""
            echo "Tests run in parallel for faster execution."
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Set default operating systems if running all
if [ "$RUN_ALL" = true ]; then
    OSES=("ubuntu" "windows")
fi

if [ ${#OSES[@]} -eq 0 ]; then
    echo "No operating systems specified to test."
    exit 1
fi

# Start all tests in parallel
echo "🚀 Starting ${#OSES[@]} test(s) in parallel..."
for os in "${OSES[@]}"; do
    run_test "$os"
done

# Wait for all background jobs to complete
echo "⏳ Waiting for all tests to complete..."
for os in "${OSES[@]}"; do
    if [ -f "$TEMP_DIR/$os.pid" ]; then
        pid=$(cat "$TEMP_DIR/$os.pid")
        wait $pid
    fi
done

# Collect results
failed_tests=()
passed_tests=()
build_failed_tests=()

for os in "${OSES[@]}"; do
    if [ -f "$TEMP_DIR/$os.result" ]; then
        result=$(cat "$TEMP_DIR/$os.result")
        case $result in
            PASSED)
                passed_tests+=("$os")
                ;;
            FAILED)
                failed_tests+=("$os")
                ;;
            BUILD_FAILED)
                build_failed_tests+=("$os")
                ;;
        esac
    else
        echo "⚠️  No result found for $os"
        failed_tests+=("$os")
    fi
done

# Show logs for failed tests
if [ ${#failed_tests[@]} -gt 0 ] || [ ${#build_failed_tests[@]} -gt 0 ]; then
    echo ""
    echo "📋 Error Details:"
    
    for os in "${build_failed_tests[@]}"; do
        echo "❌ $os (Build Failed):"
        if [ -f "$TEMP_DIR/$os.log.build" ]; then
            tail -10 "$TEMP_DIR/$os.log.build" | sed 's/^/   /'
        fi
        echo ""
    done
    
    for os in "${failed_tests[@]}"; do
        echo "❌ $os (Test Failed):"
        if [ -f "$TEMP_DIR/$os.log.run" ]; then
            tail -10 "$TEMP_DIR/$os.log.run" | sed 's/^/   /'
        fi
        echo ""
    done
fi

# Summary
echo "🎯 Test Summary:"
echo "✅ Passed: ${#passed_tests[@]} (${passed_tests[*]})"
echo "❌ Failed: ${#failed_tests[@]} (${failed_tests[*]})"
echo "🔨 Build Failed: ${#build_failed_tests[@]} (${build_failed_tests[*]})"

total_failed=$((${#failed_tests[@]} + ${#build_failed_tests[@]}))

if [ $total_failed -eq 0 ]; then
    echo "🎉 All tests passed!"
    exit 0
else
    echo "💥 $total_failed test(s) failed!"
    exit 1
fi
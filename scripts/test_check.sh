#!/bin/sh

set -u

script_dir=$(CDPATH= cd "$(dirname "$0")" && pwd)
repo_root=$(CDPATH= cd "$script_dir/.." && pwd)
cd "$repo_root" || exit 1

failures=0
output=
status=0

run_check() {
    output=$(sh check.sh "$@" 2>&1)
    status=$?
}

assert_status() {
    expected=$1
    if [ "$status" -ne "$expected" ]; then
        printf '%s\n' "FAIL: expected status $expected, got $status"
        printf '%s\n' "$output"
        failures=$((failures + 1))
    fi
}

assert_contains() {
    expected=$1
    case $output in
        *"$expected"*) ;;
        *)
            printf '%s\n' "FAIL: output does not contain: $expected"
            printf '%s\n' "$output"
            failures=$((failures + 1))
            ;;
    esac
}

run_check solutions/00_hello/01_print_line
assert_status 0
assert_contains "PASS: solutions/00_hello/01_print_line"

run_check solutions/00_hello
assert_status 0
assert_contains "4/4 passed"

run_check exercises/00_hello/01_print_line
if [ "$status" -eq 0 ]; then
    printf '%s\n' "FAIL: starter package unexpectedly passed"
    failures=$((failures + 1))
fi
assert_contains "FAIL: exercises/00_hello/01_print_line"

run_check exercises/00_hello/01_print_line --run-all
if [ "$status" -eq 0 ]; then
    printf '%s\n' "FAIL: run-all unexpectedly passed"
    failures=$((failures + 1))
fi
assert_contains "0/1 passed"

run_check exercises/00_hello --run-all
if [ "$status" -eq 0 ]; then
    printf '%s\n' "FAIL: multi-package run-all unexpectedly passed"
    failures=$((failures + 1))
fi
assert_contains "0/4 passed"
assert_contains "FAIL: exercises/00_hello/04_types"

run_check does-not-exist
assert_status 2
assert_contains "target directory does not exist"

if [ "$failures" -ne 0 ]; then
    printf '%s\n' "$failures shell assertion(s) failed"
    exit 1
fi

printf '%s\n' "all shell assertions passed"

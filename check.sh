#!/bin/sh

set -eu

script_dir=$(CDPATH= cd -P "$(dirname "$0")" && pwd)
repo_root=$script_dir
run_all=0
target_set=0
target=

for arg in "$@"; do
    case $arg in
        --run-all)
            if [ "$run_all" -eq 1 ]; then
                printf '%s\n' "duplicate option: --run-all" >&2
                exit 2
            fi
            run_all=1
            ;;
        -*)
            printf '%s\n' "unknown option: $arg" >&2
            exit 2
            ;;
        *)
            if [ "$target_set" -eq 1 ]; then
                printf '%s\n' "multiple targets are not supported" >&2
                exit 2
            fi
            target=$arg
            target_set=1
            ;;
    esac
done

if [ "$target_set" -eq 0 ]; then
    target=exercises
fi

cd "$repo_root" || exit 1

if [ ! -d "$target" ]; then
    printf '%s\n' "target directory does not exist: $target" >&2
    exit 2
fi

if [ -f "$target/Cargo.toml" ]; then
    manifests=$target/Cargo.toml
else
    manifests=$(find "$target" -type f -name Cargo.toml -print | sort)
    if [ -z "$manifests" ]; then
        printf '%s\n' "no Cargo.toml packages found under target: $target" >&2
        exit 2
    fi
fi

display_path() {
    package_dir=$1
    repo_prefix=$repo_root/
    case $package_dir in
        "$repo_prefix"*) printf '%s\n' "${package_dir#"$repo_prefix"}" ;;
        *) printf '%s\n' "$package_dir" ;;
    esac
}

passed=0
failed=0
total=0
old_ifs=$IFS
IFS='
'
set -f

for manifest in $manifests; do
    total=$((total + 1))
    package_dir=$(CDPATH= cd -P "$(dirname "$manifest")" && pwd)
    display=$(display_path "$package_dir")

    if cargo test --quiet --manifest-path "$package_dir/Cargo.toml"; then
        printf '%s\n' "PASS: $display"
        passed=$((passed + 1))
    else
        printf '%s\n' "FAIL: $display"
        failed=$((failed + 1))
        if [ "$run_all" -eq 0 ]; then
            exit 1
        fi
    fi
done

set +f
IFS=$old_ifs

if [ "$run_all" -eq 1 ] || [ "$total" -gt 1 ]; then
    printf '%s\n' "$passed/$total passed"
    if [ "$failed" -eq 0 ]; then
        printf '%s\n' "All $target pass"
    fi
fi

if [ "$failed" -ne 0 ]; then
    exit 1
fi

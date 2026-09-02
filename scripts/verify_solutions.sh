#!/bin/sh

set -eu

script_dir=$(CDPATH= cd -P "$(dirname "$0")" && pwd)
repo_root=$(CDPATH= cd -P "$script_dir/.." && pwd)
cd "$repo_root" || exit 1

printf '%s\n' "Verifying all solution packages."
sh check.sh solutions --run-all

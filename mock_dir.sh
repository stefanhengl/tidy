#!/usr/bin/env bash
set -euo pipefail

rm -rf ./example
mkdir -p ./example/dir1/dir2

# dir1
touch ./example/dir1/20190229_foo_bar.txt
touch ./example/dir1/20190330_foo_bar.txt
touch ./example/dir1/20200107_foo_bar.txt
touch ./example/dir1/another_format_May_20202.txt

# dir2
touch ./example/dir1/dir2/20210304_bas_qux.txt

#!/bin/sh
set -e

bash scripts/format.sh
bash scripts/lint.sh
bash scripts/test.sh

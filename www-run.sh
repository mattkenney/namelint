#!/usr/bin/env bash
#
# script to run on localhost
#

set -o errexit
set -o pipefail
set -o nounset

jekyll serve \
    --source docs \
    --trace \
    --watch


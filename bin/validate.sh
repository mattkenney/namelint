#!/usr/bin/env bash
#
# validate the all types of yaml files according to their respective JSON schema
#

set -o errexit
set -o pipefail
set -o nounset

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

"${SCRIPT_DIR}"/validate-configs.sh
"${SCRIPT_DIR}"/validate-rules.sh

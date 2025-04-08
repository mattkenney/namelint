#!/usr/bin/env bash
#
# validate the yaml files according to a JSON schema
#

set -o errexit
set -o pipefail
set -o nounset

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_DIR="$( cd "${SCRIPT_DIR}/.." && pwd )"

SCHEMA_YAML="${REPO_DIR}/docs/namelint-config-schema.yaml"
YAML_DIR="${REPO_DIR}/config"

"${SCRIPT_DIR}/_validate.sh" "${SCHEMA_YAML}" "${YAML_DIR}"

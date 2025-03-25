#!/usr/bin/env bash
#
# validate the rules library
#

set -o errexit
set -o pipefail
set -o nounset

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_DIR="$( cd "${SCRIPT_DIR}/.." && pwd )"

SCHEMA_YAML="${REPO_DIR}/docs/namelint-schema.yaml"

if [ ! -f "${SCHEMA_YAML}" ]; then
	echo "ERROR: schema file not found: ${SCHEMA_YAML}"
	exit 1
fi

# check that ajv is installed
if ! command -v ajv > /dev/null; then
	echo "ERROR: ajv is not installed: try 'npm install -g ajv-cli'"
	exit 1
fi

echo "INFO: validation starting at $(date -u +'%Y-%m-%dT%H:%M:%SZ')"

for RULE_FILE in "${REPO_DIR}/rules/"*.yaml; do
	echo "INFO: validating ${RULE_FILE}"
	ajv validate \
		--errors=text \
		--spec=draft2020 \
		-s "${SCHEMA_YAML}" \
		-d "${RULE_FILE}"
done

echo "INFO: validation complete at $(date -u +'%Y-%m-%dT%H:%M:%SZ')"

#!/usr/bin/env bash
#
# validate the yaml files according to a JSON schema
#

set -o errexit
set -o pipefail
set -o nounset

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_DIR="$( cd "${SCRIPT_DIR}/.." && pwd )"

SCHEMA_YAML=${1:-BAD}
YAML_DIR=${2:-BAD}
if [ "${SCHEMA_YAML}" = "BAD" ]; then
	echo "ERROR: schema file not specified"
	echo "Usage: $0 <schema.yaml> <target_dir>"
	exit 1
fi

if [ "${YAML_DIR}" = "BAD" ]; then
	echo "ERROR: target directory not specified"
	echo "Usage: $0 <schema.yaml> <target_dir>"
	exit 1
fi

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

for YAML_FILE in "${YAML_DIR}"/*.yaml; do
	echo "INFO: validating ${YAML_FILE}"
	ajv validate \
		--errors=text \
		--spec=draft2020 \
		-s "${SCHEMA_YAML}" \
		-d "${YAML_FILE}"
done

echo "INFO: validation complete at $(date -u +'%Y-%m-%dT%H:%M:%SZ')"

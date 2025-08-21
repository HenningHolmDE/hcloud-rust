#!/bin/bash
# Set environment variables used by various scripts

# Exit immediately if a command exits with a non-zero status,
# treat unset variables as an error when substituting,
# pipelines return the exit status of the last command to exit with a non-zero status, or zero if no command exited with a non-zero status.
set -euo pipefail

# directory for temporary downloads
DOWNLOAD_DIR=generator_files
mkdir -p ${DOWNLOAD_DIR}

# OpenAPI Generator version and URL
GENERATOR_VERSION=7.7.0
GENERATOR_URL=https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/${GENERATOR_VERSION}/openapi-generator-cli-${GENERATOR_VERSION}.jar
GENERATOR_JAR=${DOWNLOAD_DIR}/openapi-generator-cli-${GENERATOR_VERSION}.jar

# hcloud-openapi version and URL
HCLOUD_OPENAPI_VERSION=e6a5a46

HCLOUD_OPENAPI_REPO_URL=https://github.com/MaximilianKoestler/hcloud-openapi.git
# select the download URL based on the version format (release, branch or commit)
if [[ "${HCLOUD_OPENAPI_VERSION}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  # assume it's a version tag, download from releases
  HCLOUD_OPENAPI_URL=https://github.com/MaximilianKoestler/hcloud-openapi/releases/download/${HCLOUD_OPENAPI_VERSION}/hcloud.json
else
  # assume it's a branch name or commit hash, download from raw sources
  HCLOUD_OPENAPI_URL=https://raw.githubusercontent.com/MaximilianKoestler/hcloud-openapi/${HCLOUD_OPENAPI_VERSION}/openapi/hcloud.json
fi

# only define HCLOUD_OPENAPI_JSON if not already set
if [[ -z "${HCLOUD_OPENAPI_JSON:-}" ]]; then
  HCLOUD_OPENAPI_JSON=${DOWNLOAD_DIR}/hcloud_${HCLOUD_OPENAPI_VERSION}.json
fi

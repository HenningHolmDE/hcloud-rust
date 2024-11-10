#!/bin/bash
# Set environment variables used by various scripts

# directory for temporary downloads
DOWNLOAD_DIR=generator_files
mkdir -p ${DOWNLOAD_DIR}

# OpenAPI Generator version and URL
GENERATOR_VERSION=7.7.0
GENERATOR_URL=https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/${GENERATOR_VERSION}/openapi-generator-cli-${GENERATOR_VERSION}.jar
GENERATOR_JAR=${DOWNLOAD_DIR}/openapi-generator-cli-${GENERATOR_VERSION}.jar

# hcloud-openapi version and URL
HCLOUD_OPENAPI_VERSION=0.22.0
HCLOUD_OPENAPI_URL=https://github.com/MaximilianKoestler/hcloud-openapi/releases/download/v${HCLOUD_OPENAPI_VERSION}/hcloud.json
HCLOUD_OPENAPI_JSON=${DOWNLOAD_DIR}/hcloud_${HCLOUD_OPENAPI_VERSION}.json

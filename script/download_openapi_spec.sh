#!/bin/bash
# Download OpenAPI spec for the Hetzner Cloud API

. "$(dirname "$0")/config.sh"

echo "Downloading version ${HCLOUD_OPENAPI_VERSION} of the OpenAPI spec for the Hetzner Cloud API..."
curl -L "${HCLOUD_OPENAPI_URL}" -o "${HCLOUD_OPENAPI_JSON}"

#!/bin/bash
# Download OpenAPI spec for the Hetzner Cloud API

# hcloud-openapi version
VERSION=0.0.1

URL=https://github.com/MaximilianKoestler/hcloud-openapi/releases/download/v${VERSION}/hcloud.json
DOWNLOADDIR=generator_files
SPECFILE=$DOWNLOADDIR/hcloud.json

mkdir -p $SPECDIR
echo "Downloading version $VERSION of the OpenAPI spec for the Hetzner Cloud API..."
curl -L "$URL" -o "$SPECFILE"

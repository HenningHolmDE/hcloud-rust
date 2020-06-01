#!/bin/bash
# Generate API code from OpenAPI specification.

# OpenAPI Generator version
VERSION=4.3.1

DOWNLOADDIR=generator_files
URL=https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/$VERSION/openapi-generator-cli-$VERSION.jar
JARFILE=$DOWNLOADDIR/openapi-generator-cli-$VERSION.jar
SPECFILE=$DOWNLOADDIR/hcloud.json

# make sure Java runtime is available
if type -p java >/dev/null 2>&1; then
    _JAVA=java
elif [[ -n "$JAVA_HOME" ]] && [[ -x "$JAVA_HOME/bin/java" ]];  then
    _JAVA="$JAVA_HOME/bin/java"
else
    echo "Java not found in PATH or JAVA_HOME."
    exit 1
fi
echo "Using JAVA from $_JAVA..."

# download OpenAPI generator if JARFILE doesn't exist
if [[ ! -f "$JARFILE" ]]; then
    echo "Downloading version $VERSION of the OpenAPI Generator..."
    curl -L "$URL" -o "$JARFILE"
fi

# download OpenAPI spec if SPECFILE doesn't exists
test -f $SPECFILE || $(dirname "$0")/download_openapi_spec.sh

# delete old generated code first (as filenames may change)
echo "Cleaning up generated code..."
test -d src/apis && rm -r src/apis
test -d src/models && rm -r src/models

echo "Running OpenAPI generator..."
"$_JAVA" -jar -DapiDocs=false -DmodelDocs=false "$JARFILE" generate \
    -i $SPECFILE \
    -g rust \
    -c openapi-generator.yaml \
    -t templates

#!/bin/bash
# Extract templates from OpenAPI Generator for customization

. "$(dirname "$0")/config.sh"

# make sure Java runtime is available
if type -p java >/dev/null 2>&1; then
    _JAVA=java
elif [[ -n "$JAVA_HOME" ]] && [[ -x "$JAVA_HOME/bin/java" ]];  then
    _JAVA="$JAVA_HOME/bin/java"
else
    echo "Java not found in PATH or JAVA_HOME."
    exit 1
fi
echo "Using Java from $(which "$_JAVA")..."

# download OpenAPI generator if JARFILE doesn't exist
if [[ ! -f "${GENERATOR_JAR}" ]]; then
    echo "Downloading version $VERSION of the OpenAPI Generator..."
    curl -L "${GENERATOR_URL}" -o "${GENERATOR_JAR}"
fi

# remove output directory first
rm -rf templates_orig

echo "Running OpenAPI Generator template extraction..."
"$_JAVA" -jar "${GENERATOR_JAR}" author template \
    -g rust \
    -o templates_orig

# restore CRLF line endings on Windows
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    unix2dos -k -q templates/**/*
fi

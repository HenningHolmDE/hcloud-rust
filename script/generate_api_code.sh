#!/bin/bash
# Generate API code from OpenAPI specification.

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

# download OpenAPI generator if GENERATOR_JAR doesn't exist
if [[ ! -f "${GENERATOR_JAR}" ]]; then
    echo "Downloading version ${GENERATOR_VERSION} of the OpenAPI Generator..."
    curl -L "${GENERATOR_URL}" -o "${GENERATOR_JAR}"
fi

# download OpenAPI spec if HCLOUD_OPENAPI_JSON doesn't exists
test -f ${HCLOUD_OPENAPI_JSON} || $(dirname "$0")/download_openapi_spec.sh

# delete old generated code first (as filenames may change)
echo "Cleaning up generated code..."
test -d src/apis && rm -r src/apis
test -d src/models && rm -r src/models

echo "Running OpenAPI generator..."
"$_JAVA" -jar -DapiDocs=false -DmodelDocs=false "${GENERATOR_JAR}" generate \
    -i ${HCLOUD_OPENAPI_JSON} \
    -g rust \
    -c openapi-generator.yaml \
    -t templates

# replace references to non-existing OneOf types with serde_json::Value
sed -ri".bak" "s/crate::models::OneOf[[:alnum:]]+/serde_json::Value/g" src/models/*.rs

# Remove *Optional and *Nullable models, as they result from JSON spec limitations
# (nullable being part of object) and are not necessary in Rust.
for names in action,Action image,Image iso,Iso placement_group,PlacementGroup; do
    IFS=","
    set -- $names
    if [ -e src/models/$1_optional.rs ]; then
        rm src/models/$1_optional.rs
        sed -i".bak" "s/$2Optional/$2/g" src/models/*.rs
        sed -i".bak" "/$1_optional/d" src/models/mod.rs
    elif [ -e src/models/$1_nullable.rs ]; then
        rm src/models/$1_nullable.rs
        sed -i".bak" "s/$2Nullable/$2/g" src/models/*.rs
        sed -i".bak" "/$1_nullable/d" src/models/mod.rs
    fi
done

# The extension parameter (`sed -i EXTENSION`) is required on macOS while it is
# optional for the GNU version of `sed`. Additionally, while it would be ok to
# leave the parameter blank for `sed` on macOS to skip backups, an empty
# extension is not valid for the the GNU version.
# Thus, backups in .bak files are created in the steps above and deleted here.
rm src/models/*.bak

echo "Formatting generated code..."
cargo fmt

# restore CRLF line endings on Windows
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    unix2dos -k -q src/apis/*.rs
    unix2dos -k -q src/models/*.rs
fi

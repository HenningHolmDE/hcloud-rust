#!/bin/bash
# Remove all comments in Rust files for diffing non-doc changes
find src -name '*.rs' -exec sed -Ei".bak" "/^[ ]*\/\/\//d" {} \;

rm -rf src/*.bak
rm -rf src/apis/*.bak
rm -rf src/models/*.bak

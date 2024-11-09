#!/bin/bash
# Diff local templates against extracted OpenAPI Generator templates

$(dirname "$0")/extract_generator_templates.sh

git diff -w --no-index --diff-filter=M --output templates/templates.diff templates_orig templates

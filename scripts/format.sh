#!/usr/bin/env sh
set -e

SCRIPTS="$HOME/streamline-cli/scripts"
TEMPLATE_REPO="$HOME/streamline-cli/template-repo"

awk -f "$SCRIPTS/awk/module-format.awk" "$1" \
    | awk -f "$SCRIPTS/awk/format-types.awk" \
    | awk -f "$SCRIPTS/awk/graphql/function_calls.awk" \
    > "$TEMPLATE_REPO/streamline.rhai"

# append the code generation functions to the end of the file
echo "\
if in_repl() {\
generate_yaml(\"$TEMPLATE_REPO/streamline.yaml\");\
generate_rust(\"$TEMPLATE_REPO/src/streamline.rs\");\
}" \
    >> "$TEMPLATE_REPO/streamline.rhai"

echo "Formatted streamline file!"

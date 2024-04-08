#!/usr/bin/env sh
set -e

SCRIPTS="$HOME/streamline-cli/scripts"
TEMPLATE_REPO="$HOME/streamline-cli/template-repo"

gawk -f "$SCRIPTS/awk/module-format.awk" "$1" \
    | gawk -f "$SCRIPTS/awk/format-types.awk" \
    | gawk -f "$SCRIPTS/awk/graphql/function_calls.awk" \
    > "$TEMPLATE_REPO/streamline.rhai"

GENERATE_YAML="generate_yaml(\"$TEMPLATE_REPO/streamline.yaml\");"

# If a start block is passed in as the second arg, include it as the second arg to the function
if [ -n "$2" ]; then
    GENERATE_YAML="generate_yaml(\"$TEMPLATE_REPO/streamline.yaml\", $2);"
fi

GENERATE_RUST="generate_rust(\"$TEMPLATE_REPO/src/streamline.rs\");"

# append the code generation functions to the end of the file
echo "\
if in_repl() {\
 $GENERATE_YAML \
 $GENERATE_RUST \
}" \
    >> "$TEMPLATE_REPO/streamline.rhai"

echo "Formatted streamline file!"

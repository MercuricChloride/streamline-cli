#!/usr/bin/env bash
set -e

TEMPLATE_REPO="$HOME/streamline-cli/template-repo"
YAML="$TEMPLATE_REPO/streamline.yaml"

echo "Updating network field..."

UPDATE_YAML=$(awk "/network: mainnet/ {print \"network: $1\"; next} // {print \$0}" \
"$YAML")

echo "$UPDATE_YAML" > "$YAML"

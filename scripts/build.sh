#!/usr/bin/env sh
set -e

TEMPLATE_REPO="$HOME/streamline-cli/template-repo"
ABIS="$HOME/streamline-cli/abis"

echo "Copying the abis to template repo"
cp "$ABIS"/* "$TEMPLATE_REPO/abis"
echo "Copied abis to template repo"

(cd "$TEMPLATE_REPO" \
    && make pack \
    && cp ./output.spkg /tmp/output.spkg)

echo "Built the template repo and packaged the spkg into /tmp/output.spkg"

cp /tmp/output.spkg ./output.spkg

echo "BUILD COMPLETE!"

#!/usr/bin/env bash
set -e

if [ -z "$1" ]; then
    "You need to pass a path into the build script!"
    exit 1
fi

echo "Starting build..."

echo "Formatting the streamline file..."
format.sh "$1"
echo "Formatted successfully!"

# Copy the tangled streamline file into the template repository
echo "Copying streamline file to template repo"
cp build/formatted.rhai $TEMPLATE_REPO_PATH/streamline.rhai
echo "Copied streamline file to template repo path!"

echo "Copying the abis to template repo"
cp abis/* $TEMPLATE_REPO_PATH/abis
echo "Copied streamline file to template repo path"

echo "Running the streamline file to generate the code"
rhai-run build/formatted.rhai
echo "Ran streamline file and generated code"

CURRENT_PATH=$(pwd)
#echo "Set Current Path to: " $CURRENT_PATH

(cd $TEMPLATE_REPO_PATH \
    && make pack\
    && cp ./output.spkg /tmp/output.spkg #$CURRENT_PATH/build/output.spkg\
)
echo "Built the template repo and packaged the spkg into ./output.spkg"

echo "BUILD COMPLETE!"

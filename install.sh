#!/usr/bin/env sh

# abort on errors
set -e

# set the latest release
LATEST_RELEASE="https://github.com/MercuricChloride/streamline-cli/releases/latest"
GITHUB_LINK="https://github.com/MercuricChloride/streamline-cli.git"

# Check for rustup
if ! command -v rustup > /dev/null 2>&1; then
    echo "Rust isn't installed on your system. Please go to: https://rustup.rs/ to install rust!"
    exit 1
fi

# Check for homebrew
if ! command -v brew > /dev/null 2>&1; then
    echo "Homebrew isn't installed on your system. Please go to: https://brew.sh/ to install the homebrew package manager!"
    exit 1
fi

# Install the dependencies
brew install bufbuild/buf/buf protobuf gawk streamingfast/tap/substreams

# Install the wasm target for rust
rustup target add wasm32-unknown-unknown

# Make the abis directory
mkdir -p "$HOME/streamline-cli/abis"

# Clone the template repo down into the template-repo dir
(cd "$HOME/streamline-cli" && \
    git clone "https://github.com/MercuricChloride/streamline-template-repository.git" template-repo)

# Pull the tarball from the latest github release
curl -L -o "/tmp/scripts.tar.gz" "$LATEST_RELEASE/download/scripts.tar.gz"

# Unpack the tarball into the streamline-cli dir
tar -xzf "/tmp/scripts.tar.gz" "$HOME/streamline-cli"

# Install the streamline-cli
cargo install --git $GITHUB_LINK

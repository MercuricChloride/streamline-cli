# Streamline-CLI
`streamline-cli` is a CLI tool designed to manage all things related to development with streamline, a DSL for writing substreams, which are a tool for indexing blockchain data.

This cli allows for easy management and building of substream packages (spkgs) for blazingly fast blockchain data analytics.

# Installation
To install Streamline-CLI, you need to have the following installed on your system:
- `cargo`
- `protoc`
- `substreams`
- `awk`

Once you have those installed, you can simply run this command to install the cli to your PATH.

``` sh
git clone https://github.com/MercuricChloride/streamline-cli.git && cd streamline-cli && cargo install --path .
```

After installing, run the `install` command to setup the rest of the dependencies and paths

``` sh
streamline-cli install
```

This will create a new directory in your home directory with all of the dependencies. You shouldn't need to ever touch this manually. But to remove it, just remove the directory located at: `~/streamline-cli`

# Usage
## Basic Commands

### Install Dependencies

``` sh
streamline-cli install
```

### Add ABI JSON to Global Registry

This will make an abi available within streamline, so you can extract events, or make rpc calls.

_NOTE: THE NAME OF THE JSON FILE IS THE NAME OF THE MODULE IN THE STREAMLINE RUNTIME_
``` sh
streamline-cli add <path-to-abi-json>
```

### List all abis available to the streamline runtime
``` sh
streamline-cli list
```

### Compiling a Streamline File

``` sh
streamline-cli build <path-to-streamline-file> [block-number]
```

### Cleaning the build location

Sometimes changes will be made to the streamline packages, that you will want updated in your spkg. The easiest way to do this is to just remove the lockfile before installing. We have a simple command for this exact purpose.

``` sh
streamline-cli clean
```


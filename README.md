# Motoro CLI

A command-line interface for interacting with [this](https://github.com/broughtj/Motoro.jl) project.


## Installation

Ensure that [Rust](https://rust-lang.org/) and [Julia](https://julialang.org/) are installed.

Then run this command to install:
```bash
cargo install --git "https://github.com/Josh-Liddell/motoro-cli.git"
```

## Usage

Once installed, you can use Motoro CLI to perform various operations.

```bash
# run interactive session
motoro

# view options
motoro --help
```

## Update

You may need to periodically update this, to do so run this command:

```bash
cargo install --git "https://github.com/Josh-Liddell/motoro-cli.git" --force
```

## Uninstall

```bash
cargo uninstall motoro-cli
```

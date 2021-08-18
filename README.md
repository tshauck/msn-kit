# MSn Kit

> A library (in Rust) and CLI (`mm`) for working with Mass Spectrometry data.

<!-- vim-markdown-toc GFM -->

* [Quick Start for CLI](#quick-start-for-cli)
* [Installation](#installation)
* [Usage](#usage)

<!-- vim-markdown-toc -->

## Quick Start for CLI

After downloading the executable, use `--help` to see the commands and options.

```console
$ mm --help
msn-kit 1.0
Trent Hauck <trent@trenthauck.com>
CLI for dealing with MGF files.

USAGE:
    mm [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o <output-format>        The output file to write to [default: mgf] [possible values: json,
                              mgf]

SUBCOMMANDS:
    head               Similar to head(1) in that it can output the top n records from an MGF
                       file
    help               Prints this message or the help of the given subcommand(s)
    metadata-filter    Select spetra based on the key value pairs in the metadata
```

Yours may look different depending on the version that was installed vs the
last time this README was updated 😄.

## Installation

Releases are made to GitHub's release page:
https://github.com/tshauck/msn-kit/releases.

From there, download the version you'd like to use for the appropriate platform.

Currently there are builds for:

* x86 Darwin
* x86 Linux
* x86 Windows
* ARM Linux

## Usage

# MSn Kit

> A library (in Rust) and CLI (`mm`) for working with Mass Spectrometry data.

<!-- vim-markdown-toc GFM -->

* [Quick Start for CLI](#quick-start-for-cli)
* [Installation](#installation)
* [Usage](#usage)
* [Status Badges](#status-badges)

<!-- vim-markdown-toc -->

## Quick Start for CLI

After downloading the executable, use `--help` to see the commands and options.

```console
$ mm --help
msn-kit 0.2.3
Trent Hauck <trent@trenthauck.com>
CLI for dealing with MGF files.

USAGE:
    mm [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o <output-format>        The output file to write to [default: mgf]

SUBCOMMANDS:
    head               Similar to head(1) in that it can output the top n records from an MGF
                       file
    help               Prints this message or the help of the given subcommand(s)
    metadata-filter    Select spectra based on the key value pairs in the metadata
    stats              Compute stats for inputs
```

Yours may look different depending on the version that was installed vs the
last time this README was updated 😄. You'll also likely want to place `mm` is a
location that is on your `PATH`.

## Installation

Releases are made to GitHub's release page:
https://github.com/tshauck/msn-kit/releases/latest.

From there, download the version you'd like to use for the appropriate platform.

Currently there are builds for:

* x86 Darwin
* x86 Linux
* x86 Windows
* ARM Linux

## Usage

A few things to note about the CLI's organization.

* The CLI is organized as a single entrypoint at `mm`, then subcommands under
  that, e.g. `mm stats`, `mm head`, `mm metadata-filter`, etc. They all take a
  `-h` flag for help.
* Many commands can take a `-o` global parameter (i.e. passed directly to `mm`)
  to specify the output format. `mgf` is an option which outputs what's
  expected. `json` is also an option which will output json records, one per
  line.

## Status Badges

| Process      | Status |
| ----------- | ----------- |
| Github Release Build | ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/tshauck/msn-kit/Build%20Release?style=for-the-badge) |
| Github Tests | ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/tshauck/msn-kit/Run%20Tests?style=for-the-badge) |

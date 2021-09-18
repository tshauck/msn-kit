// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use std::fs::File;
use std::io::{stdin, stdout};
use std::path::PathBuf;

mod cmds;

use clap::Clap;
use msn_kit::io;

#[derive(Clap)]
#[clap(
    name = "msn-kit",
    about = "CLI for dealing with MGF files.",
    version = "0.2.4",
    author = "Trent Hauck <trent@trenthauck.com>"
)]
struct Opts {
    #[clap(short, about = "The output file to write to", default_value = "mgf")]
    output_format: io::Format,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about = "Similar to head(1) in that it can output the top n records from an MGF file")]
    Head(Head),

    #[clap(about = "Select spectra based on the key value pairs in the metadata")]
    MetadataFilter(FilterByKeyValue),

    #[clap(about = "Compute stats for inputs")]
    Stats(Stats),

    #[clap(about = "Cat an MzML file.")]
    MzMLCat(MzMLCat),
}

#[derive(Clap)]
struct Stats {
    #[clap(parse(from_os_str), about = "The input path or stdin")]
    input: Option<PathBuf>,
}

#[derive(Clap)]
struct Head {
    #[clap(short, about = "How many records to print", default_value = "5")]
    number: i32,

    #[clap(parse(from_os_str), about = "The input path or stdin")]
    input: Option<PathBuf>,
}

#[derive(Clap)]
struct MzMLCat {
    #[clap(parse(from_os_str), about = "The input path or stdin")]
    input: Option<PathBuf>,
}

#[derive(Clap)]
struct FilterByKeyValue {
    #[clap(short, about = "The key to check, values missing the key are omitted")]
    key: String,

    #[clap(short, about = "The value for key, only equal values are kept")]
    value: Option<String>,

    #[clap(parse(from_os_str), about = "The input path or stdin")]
    input: Option<PathBuf>,
}

/// Main entrypoint for the CLI.
fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let output_enum = io::Format::from(opts.output_format);

    let writer = &mut io::mgf_parser::MGFWriter::new(stdout(), output_enum);

    match opts.subcmd {
        SubCommand::MzMLCat(t) => match t.input {
            None => cmds::mzml_cat::cat(stdin(), stdout()),
            Some(p) => {
                let f = File::open(p).unwrap();
                cmds::mzml_cat::cat(f, stdout())
            }
        },
        SubCommand::Stats(t) => match t.input {
            None => cmds::stats::stats(stdin(), stdout()),
            Some(p) => {
                let f = File::open(p).unwrap();
                cmds::stats::stats(f, stdout())
            }
        },
        SubCommand::Head(t) => match t.input {
            None => cmds::head::head(stdin(), writer, t.number),
            Some(p) => {
                let f = File::open(p).unwrap();
                cmds::head::head(f, writer, t.number)
            }
        },
        SubCommand::MetadataFilter(t) => match t.input {
            None => cmds::metadata_filter::metadata_filter(stdin(), writer, t.key, t.value),
            Some(p) => {
                let f = File::open(p).unwrap();
                cmds::metadata_filter::metadata_filter(f, writer, t.key, t.value)
            }
        },
    }
}

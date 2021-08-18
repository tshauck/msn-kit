// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::path::PathBuf;

use clap::Clap;
use msn_kit::io;
use msn_kit::spectrum;

#[derive(Clap)]
#[clap(
    name = "msn-kit",
    about = "CLI for dealing with MGF files.",
    version = "1.0",
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

    #[clap(about = "Select spetra based on the key value pairs in the metadata")]
    MetadataFilter(FilterByKeyValue),

    #[clap(about = "Compute stats for inputs")]
    Stats(Stats),
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

/// Take the first `number` records from input and write them to output.
///
/// # Arguments
///
/// * `input` - The input reader object.
/// * `output` - The output writer object.
/// * `number` - How many records to keep.
///
fn head<R: Read, W: Write>(
    input: R,
    mgf_writer: &mut io::mgf_parser::MGFWriter<W>,
    number: i32,
) -> std::io::Result<()> {
    let mgf_parser = io::mgf_parser::MGFReader::new(input);

    let head_number = number - 1;

    let spectra = mgf_parser.spectra();
    for (i, s) in spectra.enumerate() {
        if i as i32 > head_number {
            break;
        }

        let writer_s = s?;
        mgf_writer.write(writer_s)?;
    }

    Ok(())
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

/// Write data from input to output, while filtering for key by value.
///
/// # Arguments
///
/// * `input` - A BufReader to read new lines.
/// * `output` - A Writer to write output MGF records.
/// * `key` - The key within metadata to find.
/// * `value` - The value to filter with, match against key.
///
fn metadata_filter<R: Read, W: Write>(
    input: R,
    mgf_writer: &mut io::mgf_parser::MGFWriter<W>,
    key: String,
    value: Option<String>,
) -> std::io::Result<()> {
    let mgf_parser = io::mgf_parser::MGFReader::new(input);

    let spectra = mgf_parser.spectra();
    for spectrum in spectra {
        let writer_s = spectrum?;

        match writer_s.metadata.get(&key) {
            Some(key_value) => {
                if let Some(ref found_value) = value {
                    if key_value == found_value {
                        mgf_writer.write(writer_s)?;
                    }
                } else {
                    mgf_writer.write(writer_s)?;
                }
            }
            None => {
                continue;
            }
        }
    }

    Ok(())
}

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnedSummaryStatistics {
    n_spectra: i32,
    n_peaks: i32,
    metadata_field_counts: HashMap<String, i32>,
    peaks_per_spectra: f32,
    n_no_peaks: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SummaryStatistics {
    // Number of spectra counted.
    n_spectra: i32,

    // Total peaks
    n_peaks: i32,

    // Metadata Field counts
    metadata_field_counts: HashMap<String, i32>,

    // Number of spectra w/o peaks.
    n_no_peaks: i32,
}

impl SummaryStatistics {
    fn new() -> Self {
        return SummaryStatistics {
            n_spectra: 0,
            n_peaks: 0,
            metadata_field_counts: HashMap::<String, i32>::new(),
            n_no_peaks: 0,
        };
    }

    fn add_spectrum(&mut self, s: spectrum::Spectrum) {
        self.n_spectra += 1;

        let n_peaks = s.mz.len() as i32;
        if n_peaks == 0 {
            self.n_no_peaks += 1;
        }

        self.n_peaks += n_peaks;

        let keys = s.metadata.keys();

        for key in keys {
            let cnt = self
                .metadata_field_counts
                .entry(key.to_string())
                .or_insert(0);
            *cnt += 1;
        }
    }

    fn final_stats(&self) -> ReturnedSummaryStatistics {
        let mut peaks_per_spectra = 0.0;
        if self.n_spectra != 0 {
            peaks_per_spectra = (self.n_peaks as f32) / (self.n_spectra as f32);
        }

        ReturnedSummaryStatistics {
            n_spectra: self.n_spectra,
            n_peaks: self.n_peaks,
            metadata_field_counts: self.metadata_field_counts.clone(),
            peaks_per_spectra,
            n_no_peaks: self.n_no_peaks,
        }
    }
}

fn stats<R: Read, W: Write>(input: R, writer: W) -> std::io::Result<()> {
    let mgf_parser = io::mgf_parser::MGFReader::new(input);

    let mut stats = SummaryStatistics::new();
    let buf_writer = &mut BufWriter::new(writer);

    let spectra = mgf_parser.spectra();
    for spectrum in spectra {
        let s = spectrum?;
        stats.add_spectrum(s);
    }

    let final_stats = stats.final_stats();
    let j = serde_json::to_string(&final_stats)?;
    Ok(write!(buf_writer, "{}", j)?)
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let output_enum = io::Format::from(opts.output_format);

    let writer = &mut io::mgf_parser::MGFWriter::new(stdout(), output_enum);

    match opts.subcmd {
        SubCommand::Stats(t) => match t.input {
            None => stats(stdin(), stdout()),
            Some(p) => {
                let f = File::open(p).unwrap();
                stats(f, stdout())
            }
        },
        SubCommand::Head(t) => match t.input {
            None => head(stdin(), writer, t.number),
            Some(p) => {
                let f = File::open(p).unwrap();
                head(f, writer, t.number)
            }
        },
        SubCommand::MetadataFilter(t) => match t.input {
            None => metadata_filter(stdin(), writer, t.key, t.value),
            Some(p) => {
                let f = File::open(p).unwrap();
                metadata_filter(f, writer, t.key, t.value)
            }
        },
    }
}

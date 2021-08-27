// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use std::{
    collections::HashMap,
    io::{BufWriter, Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::{io, spectrum};

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

pub fn stats<R: Read, W: Write>(input: R, writer: W) -> std::io::Result<()> {
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

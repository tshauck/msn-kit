// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use std::io::{Read, Write};

use msn_kit::io;

/// Write data from input to output, while filtering for key by value.
///
/// # Arguments
///
/// * `input` - A BufReader to read new lines.
/// * `output` - A Writer to write output MGF records.
/// * `key` - The key within metadata to find.
/// * `value` - The value to filter with, match against key.
///
pub fn metadata_filter<R: Read, W: Write>(
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

// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use std::io::{Read, Write};

use msn_kit::io;

/// Take the first `number` records from input and write them to output.
///
/// # Arguments
///
/// * `input` - The input reader object.
/// * `output` - The output writer object.
/// * `number` - How many records to keep.
pub fn head<R: Read, W: Write>(
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

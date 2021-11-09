// (c) Copyright 2021 Trent Hauck
// All Rights Reserved
use std;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use msn_kit::io::mzml_parser::MzMLReader;
use serde::Serialize;

#[derive(Debug)]
struct SpectrumWriter<W: Write> {
    writer: W,
}

impl<W: Write> SpectrumWriter<W> {
    /// Writes an input FASTA to the underlying writer.
    fn write_spectrum<S: Serialize>(&mut self, r: S) -> std::io::Result<()> {
        serde_json::to_writer(&mut self.writer, &r)?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }
}

pub fn cat<R: Read, W: Write>(input: R, output: W) -> std::io::Result<()> {
    let buf_reader = BufReader::new(input);

    let mut reader = MzMLReader::from_reader(buf_reader);
    let mut writer = SpectrumWriter { writer: output };

    loop {
        let spectrum = reader.read_spectrum();

        match spectrum {
            Ok(a) => {
                if let Err(e) = writer.write_spectrum(&a) {
                    match e.kind() {
                        std::io::ErrorKind::BrokenPipe => break,
                        _ => return Err(e),
                    }
                }
            }
            Err(_) => break,
        }
    }

    Ok(())
}

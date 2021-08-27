// (c) Copyright 2021 Trent Hauck
// All Rights Reserved
use std::io::Write;

use crate::io::Format;
use crate::spectrum::Spectrum;

#[derive(Debug)]
pub struct MGFReader<A> {
    reader: A,
}

impl<R> MGFReader<std::io::BufReader<R>>
where
    R: std::io::Read,
{
    /// Create a new MGFReader from an object that impelemnts Read.
    ///
    /// # Arguments
    ///
    /// * `reader` - An object that implements the Read trait
    ///
    pub fn new(reader: R) -> Self {
        Self {
            reader: std::io::BufReader::new(reader),
        }
    }
}

impl<R> MGFReader<R>
where
    R: std::io::BufRead,
{
    pub fn spectra(self) -> Records<R> {
        Records { reader: self }
    }

    /// Read from the underlying reader into spectrum.
    ///
    /// # Arguments
    ///
    /// * `s` - A spectrum object that will hold the new spectrum data.
    ///
    pub fn read(&mut self, s: &mut Spectrum) -> std::io::Result<()> {
        let mut line = String::new();

        s.metadata.clear();
        s.mz.clear();
        s.intensities.clear();

        self.reader.read_line(&mut line)?;

        if line.is_empty() {
            return Ok(());
        };

        if line != "BEGIN IONS\n" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Expected 'BEGIN IONS' to start, got {}", line),
            ));
        }

        line.clear();

        loop {
            self.reader.read_line(&mut line)?;

            if line == "END IONS\n" {
                break;
            }

            if line.contains('=') {
                if let Some((k, v)) = line.trim().split_once("=") {
                    s.metadata.insert(String::from(k), String::from(v));
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Could parse key value metadata.",
                    ));
                }
                line.clear();
            } else if line.contains('\t') {
                if let Some((raw_mz, raw_intensity)) = line.trim().split_once("\t") {
                    let new_mz: f64 = raw_mz.parse().unwrap();
                    let new_intensity: f64 = raw_intensity.parse().unwrap();

                    s.mz.push(new_mz);
                    s.intensities.push(new_intensity);
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Vectors"));
                }
                line.clear();
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error parsing data: {}", line),
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct MGFWriter<W: Write> {
    writer: std::io::BufWriter<W>,
    output_format: Format,
}

impl<W: Write> MGFWriter<W> {
    /// Create a new MGFWriter object.
    ///
    /// # Arguments
    ///
    /// * `writer` - An object that can be written two.
    ///
    pub fn new(writer: W, output_format: Format) -> Self {
        MGFWriter {
            writer: std::io::BufWriter::new(writer),
            output_format,
        }
    }

    /// Write spectrum to the underlying buffer in the format.
    ///
    /// # Arguments
    ///
    /// * `spectrum` - The spectrum to write.
    ///
    pub fn write(&mut self, spectrum: Spectrum) -> std::io::Result<()> {
        match self.output_format {
            Format::Mgf => self.write_mgf(spectrum),
            Format::Json => self.write_json(spectrum),
        }
    }

    /// Write spectrum to the underlying buffer in mgf format.
    ///
    /// # Arguments
    ///
    /// * `spectrum` - The spectrum to write.
    ///
    pub fn write_json(&mut self, spectrum: Spectrum) -> std::io::Result<()> {
        let result = serde_json::to_writer(&mut self.writer, &spectrum);
        self.writer.write_all(b"\n")?;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error parsing (replace me with propagated error.",
            )),
        }
    }

    /// Write spectrum to the underlying buffer in mgf format.
    ///
    /// # Arguments
    ///
    /// * `spectrum` - The spectrum to write.
    ///
    pub fn write_mgf(&mut self, spectrum: Spectrum) -> std::io::Result<()> {
        self.writer.write_all(b"BEGIN IONS\n")?;

        for (k, v) in spectrum.metadata.iter() {
            let metadata = format!("{}={}\n", k, v);
            self.writer.write_all(metadata.as_bytes())?;
        }

        let mz_iter = spectrum.mz.iter();
        let intensities_iter = spectrum.intensities.iter();

        for (m, i) in mz_iter.zip(intensities_iter) {
            let line = format!("{}\t{}\n", m, i);
            self.writer.write_all(line.as_bytes())?;
        }

        self.writer.write_all(b"END IONS\n")?;

        Ok(())
    }
}

pub struct Records<R>
where
    R: std::io::BufRead,
{
    reader: MGFReader<R>,
}

impl<R> Iterator for Records<R>
where
    R: std::io::BufRead,
{
    type Item = std::io::Result<Spectrum>;

    /// Implements the next method for the Records iterator.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        let mut record = Spectrum::empty();

        let resp = self.reader.read(&mut record);
        match resp {
            Ok(()) if record.is_empty() => None,
            Ok(()) => Some(Ok(record)),
            Err(_) => Some(Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error parsing (replace me with propagated error.",
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MGF_FILE_SIMPLE: &[u8] = b"BEGIN IONS
PEPMASS=898.727
SCANS=1
13.00	1.0
14.00	1.0
END IONS
BEGIN IONS
PEPMASS=898.727
SCANS=1
END IONS
";

    const SPECTRUM_SIMPLE: &str = r#"
    [
        {
            "metadata": {"PEPMASS": "898.727", "SCANS": "1"},
            "mz": [13.0, 14.0],
            "intensities": [1.0, 1.0]
        },
        {
            "metadata": {"PEPMASS": "898.727", "SCANS": "1"},
            "mz": [],
            "intensities": []
        }
    ]
    "#;

    #[test]
    fn test_reader() {
        let test_s: Vec<Spectrum> = serde_json::from_str(SPECTRUM_SIMPLE).unwrap();

        let reader = MGFReader::new(MGF_FILE_SIMPLE);
        let spectra = reader.spectra();

        let filter_s: Vec<Spectrum> = spectra.map(|s| s.unwrap()).collect();

        assert_eq!(test_s, filter_s);
    }
}

// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use quick_xml;
use quick_xml::events::Event;

use std::io::BufRead;
use std::io::Cursor;

use crate::io::mzml_parser::types;

pub struct MzMLReader<R: BufRead> {
    reader: quick_xml::Reader<R>,
}

impl<R> MzMLReader<R>
where
    R: BufRead,
{
    // new creates a new MzMLReader from an quick_xml::Reader
    pub fn new(reader: quick_xml::Reader<R>) -> Self {
        Self { reader }
    }

    pub fn from_reader(buf_reader: R) -> Self {
        let mut xml_reader = quick_xml::Reader::from_reader(buf_reader);
        xml_reader.trim_text(false);

        Self::new(xml_reader)
    }

    pub fn read_spectrum(&mut self) -> std::io::Result<types::Spectrum> {
        let mut buf = Vec::new();

        loop {
            match self.reader.read_event(&mut buf) {
                Ok(Event::Start(e)) if e.name() != b"spectrum" => {
                    continue;
                }
                Ok(Event::Start(e)) => {
                    let end = b"spectrum";

                    let mut buf3 = Vec::new();
                    let mut writer = quick_xml::Writer::new(Cursor::new(&mut buf3));

                    writer.write_event(Event::Start(e)).unwrap();

                    loop {
                        match self.reader.read_event(&mut buf) {
                            Ok(Event::Start(e)) => {
                                writer.write_event(Event::Start(e)).unwrap();
                            }
                            Ok(Event::Empty(e)) => {
                                writer.write_event(Event::Empty(e)).unwrap();
                            }
                            Ok(Event::Text(e)) => {
                                writer.write_event(Event::Text(e)).unwrap();
                            }
                            Ok(Event::End(e)) => {
                                if e.name() == end {
                                    writer.write_event(Event::End(e)).unwrap();
                                    break;
                                } else {
                                    writer.write_event(Event::End(e)).unwrap();
                                }
                            }
                            Ok(Event::Eof) => {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    "Unexpected Eof Event",
                                ))
                            }
                            Err(_) => panic!("fuck"),
                            Ok(e) => panic!("event: {:?}", e),
                        }
                    }

                    let c = Cursor::new(buf3);
                    let spectrum: types::Spectrum = quick_xml::de::from_reader(c).unwrap();
                    return Ok(spectrum);
                }
                Ok(Event::Eof) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Unexpected Eof Event",
                    ))
                }
                Err(e) => println!("{:?}", e),
                _ => {
                    buf.clear();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    #[test]
    fn reader_test() -> Result<(), String> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/test.mzml.xml");

        let file = File::open(d).expect("Couldn't open test file.");
        let buf_reader = BufReader::new(file);

        let mut xml_reader = quick_xml::Reader::from_reader(buf_reader);
        xml_reader.trim_text(false);

        let mut mzml_reader = MzMLReader::new(xml_reader);

        let spectrum = mzml_reader.read_spectrum();
        assert!(spectrum.is_ok());

        let spectrum = spectrum.unwrap();

        let data = spectrum.binary_data_array_list.binary_data_array[0]
            .binary_array_to_vector()
            .unwrap();

        let expected = vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0,
        ];

        assert_eq!(expected, data);

        Ok(())
    }
}

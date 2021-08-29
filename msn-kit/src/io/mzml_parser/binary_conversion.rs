// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use flate2::read::ZlibDecoder;
use std::io::prelude::*;

use crate::io::mzml_parser::types::{Binary, CompressionType, DataType};

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

/// Convert the binary content into a float of the appropriate type.
pub fn decode_binary_array(b: &Binary, ct: &CompressionType, dt: &DataType) -> Vec<f64> {
    let decoded = base64::decode(&b.content).expect("Unable to decode binary.");

    match (ct, dt) {
        (CompressionType::NoCompression, DataType::Float32Bit) => {
            binary_string_to_array_f32(decoded)
        }
        (CompressionType::NoCompression, DataType::Float64Bit) => {
            binary_string_to_array_f64(decoded)
        }
        (CompressionType::ZlibCompression, DataType::Float64Bit) => {
            let mut decoded_bytes = Vec::<u8>::new();

            let rdr = Cursor::new(decoded);

            let mut d = ZlibDecoder::new(rdr);
            d.read_to_end(&mut decoded_bytes).unwrap();

            binary_string_to_array_f64(decoded_bytes)
        }
        (CompressionType::ZlibCompression, DataType::Float32Bit) => {
            let mut decoded_bytes = Vec::<u8>::new();

            let rdr = Cursor::new(decoded);

            let mut d = ZlibDecoder::new(rdr);
            d.read_to_end(&mut decoded_bytes).unwrap();

            binary_string_to_array_f32(decoded_bytes)
        }
    }
}

pub fn binary_string_to_array_f32(decoded: Vec<u8>) -> Vec<f64> {
    let mut rdr = Cursor::new(decoded);

    let mut peaks = Vec::<f64>::new();
    while let Ok(fl) = rdr.read_f32::<LittleEndian>() {
        peaks.push(f64::from(fl));
    }

    peaks
}

pub fn binary_string_to_array_f64(decoded: Vec<u8>) -> Vec<f64> {
    let mut rdr = Cursor::new(decoded);

    let mut peaks = Vec::<f64>::new();
    while let Ok(fl) = rdr.read_f64::<LittleEndian>() {
        peaks.push(fl);
    }

    peaks
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::io::mzml_parser::types::Binary;
    use crate::io::mzml_parser::types::DataType;

    #[test]
    fn parse_mzml_test() {
        struct TestData {
            binary: Binary,
            compression_type: CompressionType,
            data_type: DataType,
            expected_array: Vec<f64>,
        }

        impl TestData {
            pub fn new(
                binary: Binary,
                compression_type: CompressionType,
                data_type: DataType,
                expected_array: Vec<f64>,
            ) -> Self {
                Self {
                    binary,
                    compression_type,
                    data_type,
                    expected_array,
                }
            }
        }

        let tests = vec![
            TestData::new(Binary::new(String::from("AAAAAAAALkAAAAAAAAAsQAAAAAAAACpAAAAAAAAAKEAAAAAAAAAmQAAAAAAAACRAAAAAAAAAIkAAAAAAAAAgQAAAAAAAABxAAAAAAAAAGEAAAAAAAAAUQAAAAAAAABBAAAAAAAAACEAAAAAAAAAAQAAAAAAAAPA/")), CompressionType::NoCompression, DataType::Float64Bit, vec![15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]),
            TestData::new(Binary::new(String::from("eJxjYEABDhBKAEpLQGkFKK0CpTWgtA6UNoDSRg4AZlQDYw==")), CompressionType::ZlibCompression, DataType::Float64Bit, vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0])
        ];

        for test in tests.iter() {
            let array = decode_binary_array(&test.binary, &test.compression_type, &test.data_type);
            assert_eq!(array, test.expected_array);
        }
    }
}

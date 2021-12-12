// (c) Copyright 2021 Trent Hauck
// All Rights Reserved
//! MSn Kit is a Rust package that facilitates reading and writing common Mass Spectrometry file
//! formats (currently MGF and partially mzml).
//!
//! This package also supports a CLI that implement some common unix commands for these unruly
//! formats. For example, to `head` an MGF file (that has logical recods span multiple newlines).
//! To download the executable for your platform, see <https://github.com/tshauck/msn-kit/releases/latest>.
//!
//! For example, to iterate through the contents of an MGF file.
//!
//! ```ignore
//! // input is a file that implements `Read`
//! let mgf_parser = io::mgf_parser::MGFReader::new(input);
//!
//! let spectra = mgf_parser.spectra();
//! for (i, read_spectra) in spectra.enumerate() {
//!     let s = read_spectra?;
//!     println!("{:?}", s);
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tshauck/msn-kit/main/msn-kit/docs/msn_logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/tshauck/msn-kit/main/msn-kit/docs/msn_logo.svg"
)]

pub mod io;
pub mod similarity;
pub mod spectrum;

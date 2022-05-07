// (c) Copyright 2021 Trent Hauck
// All Rights Reserved
//! Module containing input and output related functionality.

pub mod mgf_parser;
pub mod mzml_parser;

use std::str::FromStr;

/// Types of formats that can be read or written.
///
/// # Examples
///
/// Create `Format::Mgf` from a string.
///
/// ```
/// use std::str::FromStr;
/// use msn_kit::io::Format;
///
/// let f = Format::from_str("mgf").unwrap();
/// assert_eq!(f, Format::Mgf);
/// ```
#[derive(Debug, PartialEq)]
pub enum Format {
    /// json newline format
    Json,

    /// Mascot Generic Format
    Mgf,

    /// mzML format specified here: <https://www.psidev.info/mzML>
    MzML,
}

/// Creates a `Format` type, from a string.
impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "mgf" => Ok(Self::Mgf),
            "mzml" => Ok(Self::MzML),
            _ => Err("Cannot parse input format."),
        }
    }
}

mod tests {
    use std::str::FromStr;
    use crate::io::Format;

    #[test]
    fn from_str() {
        let inputs = vec!["json", "mgf", "mzml"];
        let expected = vec![Format::Json, Format::Mgf, Format::MzML];

        let actual: Vec<Format> = inputs
            .into_iter()
            .map(|i| Format::from_str(i).unwrap())
            .collect();
        assert_eq!(expected, actual);
    }
}

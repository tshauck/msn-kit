// (c) Copyright 2021 Trent Hauck
// All Rights Reserved
//! A Spectrum is a set of peaks and associated metadata.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// # Examples
///
/// The simplest spectrum with a single peak.
///
/// ```
/// use std::collections::HashMap;
///
/// let s = msn_kit::spectrum::Spectrum::new(HashMap::<String, String>::new(), vec![1.0], vec![1.0]);
/// assert_eq!(s.mz, vec![1.0]);
/// assert_eq!(s.intensities, vec![1.0]);
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Spectrum {
    pub metadata: HashMap<String, String>,
    pub mz: Vec<f64>,
    pub intensities: Vec<f64>,
}

impl fmt::Display for Spectrum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json_string = serde_json::to_string(&self).unwrap();
        write!(f, "{}", json_string)
    }
}

impl Spectrum {
    /// Create a new Spectrum.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The map of keys to values of the metadata.
    /// * `mz` - The mass to charge ratio vector.
    /// * `intensities` - The intensities vector.
    ///
    pub fn new(metadata: HashMap<String, String>, mz: Vec<f64>, intensities: Vec<f64>) -> Self {
        Self {
            metadata,
            mz,
            intensities,
        }
    }

    /// Returns an empty Spectrum.
    ///
    pub fn empty() -> Self {
        Self {
            metadata: HashMap::<String, String>::new(),
            mz: Vec::<f64>::new(),
            intensities: Vec::<f64>::new(),
        }
    }

    /// Returns true if the Spectrum is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.mz.is_empty() && self.intensities.is_empty() && self.metadata.is_empty()
    }

    /// Adds a key/value pair to the metadata.
    pub fn add_metadata_field(&mut self, s: String, v: String) -> &mut Self {
        self.metadata.insert(s, v);
        self
    }

    /// Adds a vector of mzs.
    pub fn add_mzs(&mut self, mz: Vec<f64>) -> &mut Self {
        self.mz = mz;
        self
    }

    /// Adds a vector of intensities.
    pub fn add_intensities(&mut self, intensities: Vec<f64>) -> &mut Self {
        self.intensities = intensities;
        self
    }
}

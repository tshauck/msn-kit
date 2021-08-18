// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Spectrum {
    pub metadata: HashMap<String, String>,
    pub mz: Vec<f64>,
    pub intensities: Vec<f64>,
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

    /// Returns an empty is Spectrum.
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
}

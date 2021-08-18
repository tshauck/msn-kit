// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

pub mod mgf_parser;
use std::str::FromStr;

#[derive(Debug)]
pub enum Format {
    Json,
    Mgf,
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "mgf" => Ok(Self::Mgf),
            _ => Err("Cannot parse input format."),
        }
    }
}

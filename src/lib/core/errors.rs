use std::fmt;
use std::io;

pub enum BoltErrors {
    IoError(io::Error),
    MissingFiles(String),
    InvalidAction(String),
}

impl fmt::Display for BoltErrors {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use BoltErrors::*;

        match self {
            IoError(ref e) => e.fmt(fmtr),
            MissingFiles(ref e) => fmtr.write_str(e),
            InvalidAction(ref e) => fmtr.write_str(e),
        }
    }
}

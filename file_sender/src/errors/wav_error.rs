use std::fmt;

#[derive(Debug, Clone)]
pub struct WavError {
    pub details: String,
}

impl fmt::Display for WavError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't manipulate wav file: {}", self.details)
    }
}

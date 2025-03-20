use std::fmt;

#[derive(Debug, Clone)]
pub struct ConnectionError {
    pub details: String,
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't connect to TCP socket: {}", self.details)
    }
}

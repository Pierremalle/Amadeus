use std::fmt;

#[derive(Debug, Clone)]
pub struct SocketError {
    pub details: String,
}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't create the socket: {}", self.details)
    }
}

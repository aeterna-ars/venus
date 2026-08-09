use std::fmt;

#[derive(Debug)]
pub struct IoError(pub(crate) String);

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IO error: {}", self.0)
    }
}

#[derive(Debug, erret_macro::Error)]
pub enum TcpError {
    #[error("OS error: {}")]
    Std(String),

    #[error("TCP conn timed out: {}")]
    Timeout(&'static str),

    #[error("TCP conn closed")]
    ConnectionClosed,
}

#[derive(Debug, erret_macro::Error)]
pub enum UdpError {
    #[error("OS error: {}")]
    Std(String),

    #[error("UDP conn timed out: {}")]
    Timeout(&'static str),
}
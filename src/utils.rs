use std::{error::Error, fmt};

#[derive(Debug)]
pub enum WebScrapperError {
    HTTP(u16,String),
    Generic(String),
}

// Implement Display for pretty-printing
impl fmt::Display for WebScrapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebScrapperError::HTTP(status_code,msg) => write!(f, "HTTP Status: {} Body: {}", status_code,msg),
            WebScrapperError::Generic(msg) => write!(f, "Generic Error: {}",msg),
        }
    }
}

impl Error for WebScrapperError{}

impl WebScrapperError {
    pub fn from_err<T:ToString>(err:T)->Self{
        Self::Generic(err.to_string())
    }
}
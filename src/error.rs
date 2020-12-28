use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CheckError {
    reason: String,
}

impl CheckError {
    pub fn new(msg: &str) -> CheckError {
        CheckError {
            reason: msg.to_string(),
        }
    }
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for CheckError {
    fn description(&self) -> &str {
        &self.reason
    }
}

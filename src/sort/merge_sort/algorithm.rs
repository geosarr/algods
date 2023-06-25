#[cfg(test)]
mod unit_test;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Enumerating the different types of merge sort algorithm
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MergeSortAlgorithm {
    /// The recursive algorithm
    Recursive,
    /// The iterative algorithm
    BottomUp,
}

const ALGORITHMS: &str = "[Recursive, BottonUp]";

impl Default for MergeSortAlgorithm {
    fn default() -> Self {
        Self::Recursive
    }
}

// Formatting an Algorithm type (for printing purpose for e.g.)
impl fmt::Display for MergeSortAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MergeSortAlgorithm::Recursive => write!(f, "Recursive"),
            MergeSortAlgorithm::BottomUp => write!(f, "BottomUp"),
        }
    }
}

// Parsing a string slice (&str) to Algorithm
impl FromStr for MergeSortAlgorithm {
    type Err = ParseMergeSortAlgorithmError;

    fn from_str(s: &str) -> std::result::Result<Self, ParseMergeSortAlgorithmError> {
        match s {
            "Recursive" => Ok(MergeSortAlgorithm::Recursive),
            "BottomUp" => Ok(MergeSortAlgorithm::BottomUp),
            _ => Err(ParseMergeSortAlgorithmError),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseMergeSortAlgorithmError;

impl fmt::Display for ParseMergeSortAlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc_init0: &str = "algorithm spelling incorrect, only available are";
        let desc_init1 = format!("{desc_init0} {ALGORITHMS}");
        let description = desc_init1.as_str();
        f.write_str(description)
    }
}

impl Error for ParseMergeSortAlgorithmError {}

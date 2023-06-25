#[cfg(test)]
mod unit_test;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Enumeration of the different types of union-find algorithm
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnionFindAlgorithm {
    QuickFind,
    QuickUnion,
    WeightedQuickUnion,
    WeightedQuickUnionPathComp,
}
const ALGORITHMS: &str = "[QuickFind, QuickUnion, WeightedQuickUnion, WeightedQuickUnionPathComp]";

impl Default for UnionFindAlgorithm {
    fn default() -> Self {
        Self::QuickFind
    }
}

// Formatting an UnionFindAlgorithm type (for printing purpose for e.g.)
impl fmt::Display for UnionFindAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnionFindAlgorithm::QuickFind => write!(f, "QuickFind"),
            UnionFindAlgorithm::QuickUnion => write!(f, "QuickUnion"),
            UnionFindAlgorithm::WeightedQuickUnion => write!(f, "WeightedQuickUnion"),
            UnionFindAlgorithm::WeightedQuickUnionPathComp => {
                write!(f, "WeightedQuickUnionPathComp")
            }
        }
    }
}

// Parsing a string slice (&str) to UnionFindAlgorithm
impl FromStr for UnionFindAlgorithm {
    type Err = ParseUnionFindAlgorithmError;

    fn from_str(s: &str) -> std::result::Result<Self, ParseUnionFindAlgorithmError> {
        match s {
            "QuickFind" => Ok(UnionFindAlgorithm::QuickFind),
            "QuickUnion" => Ok(UnionFindAlgorithm::QuickUnion),
            "WeightedQuickUnion" => Ok(UnionFindAlgorithm::WeightedQuickUnion),
            "WeightedQuickUnionPathComp" => Ok(UnionFindAlgorithm::WeightedQuickUnionPathComp),
            _ => Err(ParseUnionFindAlgorithmError),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseUnionFindAlgorithmError;

impl fmt::Display for ParseUnionFindAlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc_init0: &str = "algorithm spelling incorrect, only available are";
        let desc_init1 = format!("{desc_init0} {ALGORITHMS}");
        let description = desc_init1.as_str();
        f.write_str(description)
    }
}

impl Error for ParseUnionFindAlgorithmError {}

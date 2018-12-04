
use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;
use std::error;
use std::fmt;

#[derive(Debug)]
/// Represents a claim about a cut of fabric
pub struct FabricClaim {

    /// The id of the claim
    pub id: i32,

    ///
    /// The distance in inches from the left side of the fabric
    pub left: i32,

    /// THe distance in inches from the top of the fabric
    pub top: i32,

    /// The width in inches of the cut
    pub width: i32,

    /// The height in inches of the cut
    pub height: i32
}

impl FromStr for FabricClaim {
    type Err = ParseFabricClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("^#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)$").unwrap();
        }

        for capture in RE.captures_iter(s) {
            let id: i32 = capture[1].parse()?;
            let left: i32 = capture[2].parse()?;
            let top: i32 = capture[3].parse()?;
            let width: i32 = capture[4].parse()?;
            let height: i32 = capture[5].parse()?;

            return Ok(FabricClaim {
                id,
                left,
                top,
                width,
                height
            })
        }

        Err(ParseFabricClaimError::InvalidFormat(String::from(s)))
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseFabricClaimError {
    InvalidFormat(String),
    UnparsableToken,
}

impl From<ParseIntError> for ParseFabricClaimError {
    fn from(_: ParseIntError) -> Self {
        ParseFabricClaimError::UnparsableToken
    }
}

impl fmt::Display for ParseFabricClaimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseFabricClaimError::InvalidFormat(s) => write!(f, "Invalid format: '{}'", s),
            ParseFabricClaimError::UnparsableToken => f.write_str("Part of the string could not be converted"),
        }
    }
}

impl error::Error for ParseFabricClaimError {
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_success() {
        let claim: FabricClaim = "#1 @ 100,366: 24x27".parse().unwrap();

        assert_eq!(1, claim.id);
        assert_eq!(100, claim.left);
        assert_eq!(366, claim.top);
        assert_eq!(24, claim.width);
        assert_eq!(27, claim.height);
    }

    #[test]
    fn parse_unparsable() {
        let claim = "dsjghdklgjdfk".parse::<FabricClaim>();

        assert!(claim.is_err());
    }
}
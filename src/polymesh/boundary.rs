use crate::base::{parser_base::*, FileElement};
use crate::base::{FileParser, FoamStructure};
use nom::{character::complete::char, IResult};

/// The BoundaryData structure holds the data of a polyMesh/boundary file.
#[derive(Debug, PartialEq, Clone)]
pub struct BoundaryData(pub FoamStructure);

impl std::ops::Deref for BoundaryData {
    type Target = FoamStructure;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BoundaryData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FileParser for BoundaryData {
    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/boundary")
    }
}

impl FileElement for BoundaryData {
    fn parse(input: &str) -> IResult<&str, BoundaryData> {
        // number of boundaries
        let (input, n) = next(usize_val)(input)?;
        // opening parenthesis
        let (input, _) = next(char('('))(input)?;
        // parse boundaries
        let (input, boundaries) = FoamStructure::parse(input)?;
        assert_eq!(
            boundaries.len(),
            n,
            "Number of boundaries does not match the number of boundary entries."
        );
        // closing parenthesis
        let (input, _) = next(char(')'))(input)?;
        Ok((input, BoundaryData(boundaries)))
    }
}

impl std::fmt::Display for BoundaryData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.0.len())?;
        writeln!(f, "(")?;
        write!(f, "{}", self.0)?;
        writeln!(f, ")")
    }
}

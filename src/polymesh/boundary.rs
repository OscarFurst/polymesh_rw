use crate::base::{parser_base::*, FileElement};
use crate::base::{FileParser, FoamStructure};
use nom::{character::complete::char, IResult};

#[derive(Debug, PartialEq, Clone)]
pub struct BoundaryData {
    pub n: usize,
    pub boundaries: FoamStructure,
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
        Ok((input, BoundaryData { n, boundaries }))
    }
}

impl std::fmt::Display for BoundaryData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.n)?;
        writeln!(f, "(")?;
        write!(f, "{}", self.boundaries)?;
        writeln!(f, ")")
    }
}

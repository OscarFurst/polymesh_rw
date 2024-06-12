use crate::base::{parser_base::*, FileElement};
use crate::base::{FileParser, FoamStructure};
use indexmap::map::IndexMap;
use nom::{character::complete::char, multi::count, IResult};

#[derive(Debug, PartialEq, Clone)]
pub struct BoundaryData {
    pub n: usize,
    pub boundaries: IndexMap<String, FoamStructure>,
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
        let (input, boundary_vector) = count(FoamStructure::parse, n)(input)?;
        let boundaries = boundary_vector
            .into_iter()
            .map(|boundary| (boundary.name.clone(), boundary))
            .collect();
        // closing parenthesis
        let (input, _) = next(char(')'))(input)?;
        Ok((input, BoundaryData { n, boundaries }))
    }
}

impl std::fmt::Display for BoundaryData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.n)?;
        writeln!(f, "(")?;
        for boundary in self.boundaries.values() {
            write!(f, "{}", boundary)?;
        }
        writeln!(f, ")")
    }
}

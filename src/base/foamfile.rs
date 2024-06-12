use nom::{bytes::complete::tag, combinator::map, sequence::preceded, IResult};

use super::{parser_base::next, FileElement, FoamStructure, FoamValue};

/// The FoamFile structure holds the FoamFile object that is part of the header of a file.
/// It is effectively a HashMap with some extra I/O functionalities.
#[derive(Debug, PartialEq, Clone)]
pub struct FoamFile(pub FoamStructure);

impl FoamFile {
    /// Tries to assemble a relative file path from the location and object fields if they are present.
    pub fn relative_file_path(&self) -> Option<std::path::PathBuf> {
        if let Some(FoamValue::String(location)) = self.get("location") {
            if let Some(FoamValue::String(object)) = self.get("object") {
                return Some(std::path::PathBuf::from(&location).join(object));
            }
        }
        None
    }
}

impl std::ops::Deref for FoamFile {
    type Target = FoamStructure;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FoamFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FileElement for FoamFile {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(next(tag("FoamFile")), FoamValue::parse_structure),
            FoamFile,
        )(input)
    }
}

impl std::fmt::Display for FoamFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "FoamFile")?;
        writeln!(f, "{{")?;
        write!(f, "{}", self.0)?;
        writeln!(f, "}}")
    }
}

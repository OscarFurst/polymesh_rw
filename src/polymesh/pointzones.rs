use super::Zone;
use crate::base::parser_base::*;
use crate::base::writer_base::*;
use crate::base::FileElement;
use nom::{character::complete::char, IResult};

/// The PointZone structure containts the data of a single pointZone.
#[derive(Debug, PartialEq, Clone)]
pub struct PointZone {
    // starts with a "type" which I have only seen as "pointZone", so I'm not storing it for now
    pub name: String,
    pub n: usize,
    pub points: Vec<usize>,
}

impl std::ops::Deref for PointZone {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

impl std::ops::DerefMut for PointZone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.points
    }
}

impl Zone for PointZone {
    fn name(&self) -> &str {
        &self.name
    }

    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/pointZones")
    }
}

impl FileElement for PointZone {
    fn parse(input: &str) -> IResult<&str, PointZone> {
        // starts with name
        let (input, name) = next(string_val)(input)?;
        let name = name.to_string();
        // opening curly brace
        let (input, _) = next(char('{'))(input)?;
        // "    type pointZone;"
        let (input, _) = known_key_value_semicolon("type", "pointZone")(input)?;
        // "pointLabels      List<label> "
        let (input, _) = known_key_value("pointLabels", "List<label>")(input)?;
        // list of points
        let (input, points) = single_i_data(input)?;
        let n = points.len();
        // closing semicolon
        let (input, _) = next(semicolon)(input)?;
        // closing curly brace
        let (input, _) = next(char('}'))(input)?;
        Ok((input, PointZone { name, n, points }))
    }
}

impl std::fmt::Display for PointZone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "{{")?;
        writeln!(f, "    type pointZone;")?;
        writeln!(f, "pointLabels      List<label>  ")?;
        write_single_data(&self.points, f)?;
        writeln!(f, ";")?;
        writeln!(f, "}}\n")?;
        Ok(())
    }
}

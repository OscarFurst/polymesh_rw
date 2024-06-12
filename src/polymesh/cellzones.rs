use super::facezones::Zone;
use crate::base::parser_base::*;
use crate::base::writer_base::*;
use crate::base::FileElement;
use nom::{character::complete::char, IResult};

/// The CellZone structure containts the data of a single cellZone.
#[derive(Debug, PartialEq, Clone)]
pub struct CellZone {
    // starts with a "type" which I have only seen as "cellZone", so I'm not storing it for now
    pub name: String,
    pub n: usize,
    pub cells: Vec<usize>,
}

impl Zone for CellZone {
    fn name(&self) -> &str {
        &self.name
    }

    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/cellZones")
    }
}

impl FileElement for CellZone {
    fn parse(input: &str) -> IResult<&str, CellZone> {
        // starts with name
        let (input, name) = next(string_val)(input)?;
        let name = name.to_string();
        // opening curly brace
        let (input, _) = next(char('{'))(input)?;
        // "    type cellZone;"
        let (input, _) = next(known_key_value_semicolon("type", "cellZone"))(input)?;
        // "cellLabels      List<label> "
        let (input, _) = next(known_key_value("cellLabels", "List<label>"))(input)?;
        // list of cells
        let (input, cells) = single_i_data(input)?;
        let n = cells.len();
        // closing semicolon
        let (input, _) = next(semicolon)(input)?;
        // closing curly brace
        let (input, _) = next(char('}'))(input)?;
        Ok((input, CellZone { name, n, cells }))
    }
}

impl std::fmt::Display for CellZone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "{{")?;
        writeln!(f, "    type cellZone;")?;
        writeln!(f, "cellLabels      List<label>  ")?;
        write_single_data(&self.cells, f)?;
        writeln!(f, ";")?;
        writeln!(f, "}}\n")?;
        Ok(())
    }
}

use crate::file_parser::FileParser;
use crate::parser_base::*;
use crate::writer_base::write_single_data;
use nom::{character::complete::char, multi::count, IResult};
use std::collections::HashMap;
use std::io::prelude::*;

/// Container for the polyMesh cellZones data.
#[derive(Debug, PartialEq, Clone)]
pub struct CellZoneData {
    pub n: usize,
    pub cellzones: HashMap<String, CellZone>,
}

/// Container for the data of a single cellZone.
#[derive(Debug, PartialEq, Clone)]
pub struct CellZone {
    // starts with a "type" which I have only seen as "cellZone", so I'm not storing it for now
    pub name: String,
    pub n: usize,
    pub cells: Vec<usize>,
}

impl CellZone {
    fn write_data(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(file, "{}", self.name)?;
        writeln!(file, "{{")?;
        writeln!(file, "    type cellZone;")?;
        writeln!(file, "cellLabels      List<label>  ")?;
        write_single_data(&self.cells, file)?;
        writeln!(file, ";")?;
        writeln!(file, "}}\n")?;
        Ok(())
    }
}

fn parse_cell_zone(input: &str) -> IResult<&str, CellZone> {
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
    Ok((
        input,
        CellZone {
            name,
            n,
            cells: cells,
        },
    ))
}

impl FileParser for CellZoneData {
    /// Assumes the remaining input contains the point data.
    fn parse_data(input: &str) -> IResult<&str, CellZoneData> {
        // number of cell zones
        let (input, n) = next(usize_val)(input)?;
        // opening parenthesis
        let (input, _) = next(char('('))(input)?;
        // parse cell zones
        let (input, cellzone_vector) = count(parse_cell_zone, n)(input)?;
        let cellzones = cellzone_vector
            .into_iter()
            .map(|cellzone| (cellzone.name.clone(), cellzone))
            .collect();
        // closing parenthesis
        let (input, _) = next(char(')'))(input)?;
        Ok((input, CellZoneData { n, cellzones }))
    }

    fn file_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/cellZones")
    }

    fn write_data(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(file, "{}", self.n)?;
        writeln!(file, "(")?;
        for (_, cellzone) in &self.cellzones {
            cellzone.write_data(file)?;
        }
        writeln!(file, ")")?;
        Ok(())
    }
}

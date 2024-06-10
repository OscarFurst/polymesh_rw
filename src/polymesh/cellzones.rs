use crate::file_parser::FileParser;
use crate::parser_base::*;
use nom::{character::complete::char, multi::count, IResult};

#[derive(Debug, PartialEq, Clone)]
pub struct CellZoneData {
    pub n: usize,
    pub cellzones: Vec<CellZone>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CellZone {
    // starts with a "type" which I have only seen as "cellZone", so I'm not storing it for now
    pub name: String,
    pub n: usize,
    pub cells: Vec<usize>,
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
        let (input, cellzones) = count(parse_cell_zone, n)(input)?;
        // closing parenthesis
        let (input, _) = next(char(')'))(input)?;
        Ok((input, CellZoneData { n, cellzones }))
    }
}

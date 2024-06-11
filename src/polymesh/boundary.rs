use crate::file_parser::FileParser;
use crate::foam_structure;
use crate::parser_base::*;
use indexmap::map::IndexMap;
use nom::{character::complete::char, multi::count, IResult};
use std::fmt;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct BoundaryData {
    pub n: usize,
    pub boundaries: IndexMap<String, Boundary>,
}

type Boundary = foam_structure::FoamStructure;

impl FileParser for BoundaryData {
    /// Assumes the remaining input contains the data.
    fn parse_data(input: &str) -> IResult<&str, BoundaryData> {
        // number of boundaries
        let (input, n) = next(usize_val)(input)?;
        // opening parenthesis
        let (input, _) = next(char('('))(input)?;
        // parse boundaries
        let (input, boundary_vector) = count(Boundary::parse, n)(input)?;
        let mut boundaries = IndexMap::new();
        for boundary in &boundary_vector {
            boundaries.insert(boundary.name.clone(), boundary.clone());
        }
        // closing parenthesis
        let (input, _) = next(char(')'))(input)?;
        Ok((input, BoundaryData { n, boundaries }))
    }

    fn file_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/boundary")
    }

    fn write_data(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(file, "{}", self.n)?;
        writeln!(file, "(")?;
        for boundary in self.boundaries.values() {
            boundary.write(file)?;
        }
        writeln!(file, ")")?;
        Ok(())
    }
}

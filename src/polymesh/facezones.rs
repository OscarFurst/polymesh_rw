use crate::file_parser::FileParser;
use crate::parser_base::*;
use crate::writer_base::{bool_as_num, write_single_data};
use indexmap::map::IndexMap;
use nom::{
    bytes::complete::tag, character::complete::char, multi::count, sequence::delimited, IResult,
};
use std::io::prelude::*;

/// Container for the polyMesh faceZones data.
#[derive(Debug, PartialEq, Clone)]
pub struct FaceZoneData {
    pub n: usize,
    pub facezones: IndexMap<String, FaceZone>,
}

/// Container for the data of a single faceZone.
#[derive(Debug, PartialEq, Clone)]
pub struct FaceZone {
    // starts with a "type" which I have only seen as "faceZone", so I'm not storing it for now
    pub name: String,
    pub n: usize,
    pub faces: Vec<usize>,
    // no idea what flipmap is does and what forms it can take, only seen one example so far
    // maybe even useless : https://gitlab.kitware.com/vtk/vtk/-/issues/17103
    pub flipmap: bool,
}

impl FaceZone {
    fn write_data(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(file, "{}", self.name)?;
        writeln!(file, "{{")?;
        writeln!(file, "    type faceZone;")?;
        writeln!(file, "faceLabels      List<label>  ")?;
        write_single_data(&self.faces, file)?;
        writeln!(file, ";")?;
        self.write_flipmap(file)?;
        writeln!(file, "}}\n")?;
        Ok(())
    }

    fn write_flipmap(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        write!(file, "flipMap         List<bool> ")?;
        write!(file, "{}", self.n)?;
        write!(file, "{{")?;
        write!(file, "{}", bool_as_num(self.flipmap))?;
        writeln!(file, "}};")?;
        Ok(())
    }
}

fn parse_flipmap(input: &str, n: usize) -> IResult<&str, bool> {
    delimited(
        tag(n.to_string().as_str()),
        delimited(char('{'), bool, char('}')),
        semicolon,
    )(input)
}

fn parse_face_zone(input: &str) -> IResult<&str, FaceZone> {
    // starts with name
    let (input, name) = next(string_val)(input)?;
    // opening curly brace
    let (input, _) = next(char('{'))(input)?;
    // "    type faceZone;"
    let (input, _) = next(known_key_value_semicolon("type", "faceZone"))(input)?;
    // "faceLabels      List<label> "
    let (input, _) = next(known_key_value("faceLabels", "List<label>"))(input)?;
    // list of faces
    let (input, faces) = single_i_data(input)?;
    let n = faces.len();
    // closing semicolon
    let (input, _) = next(semicolon)(input)?;
    // "flipMap         List<bool>"
    let (input, _) = next(known_key_value("flipMap", "List<bool>"))(input)?;
    let (input, _) = discard_empty(input)?;
    // on the same line: <number of faces>{<bool>};
    let (input, flipmap) = parse_flipmap(input, n)?;
    // closing curly brace
    let (input, _) = next(char('}'))(input)?;
    Ok((
        input,
        FaceZone {
            name,
            n,
            faces,
            flipmap,
        },
    ))
}

impl FileParser for FaceZoneData {
    /// Assumes the remaining input contains the facezone data.
    fn parse_data(input: &str) -> IResult<&str, FaceZoneData> {
        // number of face zones
        let (input, n) = next(usize_val)(input)?;
        // opening parenthesis
        let (input, _) = next(char('('))(input)?;
        // parse face zones
        let (input, facezone_vector) = count(parse_face_zone, n)(input)?;
        let facezones = facezone_vector
            .into_iter()
            .map(|facezone| (facezone.name.clone(), facezone))
            .collect();
        // closing parenthesis
        let (input, _) = next(char(')'))(input)?;
        Ok((input, FaceZoneData { n, facezones }))
    }

    fn default_file_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/faceZones")
    }

    fn write_data(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(file, "{}", self.n)?;
        writeln!(file, "(")?;
        for (_, facezone) in &self.facezones {
            facezone.write_data(file)?;
        }
        writeln!(file, ")")?;
        Ok(())
    }
}

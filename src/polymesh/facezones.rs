use super::Zone;
use crate::base::parser_base::*;
use crate::base::writer_base::*;
use crate::base::FileElement;
use nom::{bytes::complete::tag, character::complete::char, sequence::delimited, IResult};

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

impl std::ops::Deref for FaceZone {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.faces
    }
}

impl std::ops::DerefMut for FaceZone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.faces
    }
}

impl Zone for FaceZone {
    fn name(&self) -> &str {
        &self.name
    }

    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/faceZones")
    }
}

impl FileElement for FaceZone {
    fn parse(input: &str) -> IResult<&str, FaceZone> {
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
        let (input, flipmap) = Self::parse_flipmap(input, n)?;
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
}

impl std::fmt::Display for FaceZone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "{{")?;
        writeln!(f, "    type faceZone;")?;
        writeln!(f, "faceLabels      List<label>  ")?;
        write_single_data(&self.faces, f)?;
        writeln!(f, ";")?;
        self.write_flipmap(f)?;
        writeln!(f, "}}\n")?;
        Ok(())
    }
}

impl FaceZone {
    fn write_flipmap(&self, file: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(file, "flipMap         List<bool> ")?;
        write!(file, "{}", self.n)?;
        write!(file, "{{")?;
        write!(file, "{}", bool_as_num(self.flipmap))?;
        writeln!(file, "}};")?;
        Ok(())
    }

    fn parse_flipmap(input: &str, n: usize) -> IResult<&str, bool> {
        delimited(
            tag(n.to_string().as_str()),
            delimited(char('{'), bool, char('}')),
            semicolon,
        )(input)
    }
}

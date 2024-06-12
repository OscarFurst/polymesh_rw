use crate::parser_base::*;
use crate::writer_base::*;
use indexmap::map::IndexMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::count;
use nom::multi::fold_many0;
use nom::number::complete::double;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;
use std::io::prelude::*;

/// A structure that holds key-value pairs.
/// Correspons to structures commonly found in OpenFOAM files,
/// such as the following part of a boundary file:
/// ```text
/// down
/// {
///     type            patch;
///     physicalType    wall;
///     nFaces          0;
///     startFace       3890;
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct FoamStructure {
    pub name: String,
    pub content: IndexMap<String, FoamValue>,
}

impl FoamStructure {
    /// Parse a FoamStructure from the given input.
    pub fn parse(input: &str) -> IResult<&str, FoamStructure> {
        let (input, name) = next(string_val)(input)?;
        let (input, structure) = FoamStructure::parse_content(input)?;
        Ok((
            input,
            FoamStructure {
                name,
                content: structure.content,
            },
        ))
    }

    pub fn parse_content(input: &str) -> IResult<&str, FoamStructure> {
        let (input, content) = delimited(
            next(char('{')),
            fold_many0(
                FoamStructure::parse_pair,
                IndexMap::new,
                |mut map, (k, v)| {
                    map.insert(k, v);
                    map
                },
            ),
            next(char('}')),
        )(input)?;
        Ok((
            input,
            FoamStructure {
                name: "".to_string(),
                content,
            },
        ))
    }

    /// Parse a single key-value pair from the given input.
    fn parse_pair(input: &str) -> IResult<&str, (String, FoamValue)> {
        map(pair(next(string_val), lws(FoamValue::parse)), |(s, v)| {
            if let FoamValue::Structure(mut structure) = v {
                structure.name = s.clone();
                return (s, FoamValue::Structure(structure));
            } else {
                return (s, v);
            }
        })(input)
    }

    // TODO: clean up write and write_recursive

    /// Write the structure to the given file.
    pub fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(file, "{}", self.name)?;
        writeln!(file, "{{")?;
        for (key, value) in &self.content {
            write!(file, "    {: <15} ", key)?;
            value.write(file)?;
        }
        writeln!(file, "}}")?;
        Ok(())
    }

    /// Write the structure to the given file, but without the name.
    /// This is used for recursive writing of structures.
    fn write_recursive(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(file, "\n    {{")?;
        for (key, value) in &self.content {
            write!(file, "    {: <15} ", key)?;
            value.write(file)?;
        }
        writeln!(file, "    }}")?;
        Ok(())
    }

    pub fn relative_file_path(&self) -> Option<std::path::PathBuf> {
        if let Some(FoamValue::String(location)) = self.content.get("location") {
            if let Some(FoamValue::String(object)) = self.content.get("object") {
                return Some(
                    std::path::PathBuf::from(&location.to_string()).join(&object.to_string()),
                );
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FoamValue {
    String(String),
    Integer(usize),
    Float(f64),
    Field(FoamField),
    // TODO: Lists should be more generic.
    List(Vec<String>),
    Structure(FoamStructure),
}

impl FoamValue {
    /// Parse a FoamValue from the given input. A FoamValue can span multiple lines and
    /// contain a FoamStructure itself.
    fn parse(input: &str) -> IResult<&str, FoamValue> {
        // Check if if its a positive integer.
        if let Ok((input, value)) = map_res(
            terminated(ws(take_till(|c| c == ';')), semicolon),
            str::parse::<usize>,
        )(input)
        {
            return Ok((input, FoamValue::Integer(value)));
        }
        // Check if its a float.
        // TODO: Negative integers will be parsed as floats. Is this a problem?
        // TODO: Some things that can be parsed as floats might need to be parsed as Strings, e.g. versions!
        if let Ok((input, value)) = map_res(
            terminated(ws(take_till(|c| c == ';')), semicolon),
            str::parse::<f64>,
        )(input)
        {
            return Ok((input, FoamValue::Float(value)));
        }
        // Check if it is a field.
        if let Ok((input, value)) = terminated(FoamField::parse, semicolon)(input) {
            return Ok((input, FoamValue::Field(value)));
        }
        // Check if it is a list.
        if let Ok((input, value)) = terminated(parse_list, semicolon)(input) {
            return Ok((input, FoamValue::List(value)));
        }
        // Check if it is a structure.
        if let Ok((input, value)) = FoamStructure::parse_content(input) {
            return Ok((input, FoamValue::Structure(value)));
        }
        // If none of the above, it is a string or something that is not implemented yet.
        map(terminated(take_till(|c| c == ';'), semicolon), |s| {
            FoamValue::String(s.to_string())
        })(input)
    }

    /// Write the value to the given file.
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            FoamValue::String(value) => writeln!(file, "{};", value)?,
            FoamValue::Integer(value) => writeln!(file, "{};", value)?,
            FoamValue::Float(value) => writeln!(file, "{};", value)?,
            FoamValue::Field(value) => {
                value.write(file)?;
                writeln!(file, ";")?;
            }
            FoamValue::List(values) => {
                write!(file, "List<word> ")?;
                write_single_data(values, file)?;
                writeln!(file, ";")?
            }
            FoamValue::Structure(value) => value.write_recursive(file)?,
        }
        Ok(())
    }
}

// TODO: Lists are found in a lot of places, so they should be
// more generic and used in FoamField for examples.
/// Parses a List<word>.
fn parse_list(input: &str) -> IResult<&str, Vec<String>> {
    preceded(tag("List<word>"), parse_list_content)(input)
}

fn parse_list_content(input: &str) -> IResult<&str, Vec<String>> {
    let (input, n) = lws(usize_val)(input)?;
    block_parentheses(count(lws(string_val), n))(input)
}

/// An enumerator that holds the different types of physical fields
/// that can be found in OpenFOAM files.
// TODO: nonuniform fields are declared as "List<_>" and could maybe be more generic.
#[derive(Debug, PartialEq, Clone)]
pub enum FoamField {
    UniformScalar(f64),
    UniformVector(Vec<f64>),
    Scalar(Vec<f64>),
    Vector(Vec<Vec<f64>>),
}

impl FoamField {
    /// Parse a FoamField from the given input.
    pub fn parse(input: &str) -> IResult<&str, FoamField> {
        // starts with some information about the field
        let (input, field_type) = next(string_val)(input)?;
        match field_type.as_str() {
            "uniform" => parse_uniform(input),
            "nonuniform" => parse_nonuniform(input),
            _ => Err(nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Tag,
            })),
        }
    }

    /// Write the field to the given file.
    pub fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            FoamField::UniformScalar(value) => writeln!(file, "uniform {};", value)?,
            FoamField::UniformVector(value) => {
                write!(file, "uniform (")?;
                write_vector_content(&value, file)?;
                writeln!(file, ");")?
            }
            FoamField::Scalar(ref values) => {
                writeln!(file, "nonuniform List<scalar>")?;
                write_single_data(values, file)?;
                writeln!(file, ";")?
            }
            FoamField::Vector(ref values) => {
                writeln!(file, "nonuniform List<vector>")?;
                write_fixed_witdh_data(values, file)?;
                writeln!(file, ";")?
            }
        }
        Ok(())
    }
}

fn parse_uniform(input: &str) -> IResult<&str, FoamField> {
    terminated(
        alt((
            map(lws(double), |v| FoamField::UniformScalar(v)),
            map(lws(inline_parentheses(double_values)), |v| {
                FoamField::UniformVector(v)
            }),
        )),
        semicolon,
    )(input)
}

fn parse_nonuniform(input: &str) -> IResult<&str, FoamField> {
    // now comes "List<scalar>" or "List<vector>"
    terminated(
        preceded(
            delimited(lws(tag("List<")), string_val, char('>')),
            alt((scalar_field, vector_field)),
        ),
        semicolon,
    )(input)
}

fn scalar_field(input: &str) -> IResult<&str, FoamField> {
    map(double_scalar_field, FoamField::Scalar)(input)
}

fn vector_field(input: &str) -> IResult<&str, FoamField> {
    map(double_vector_field, FoamField::Vector)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_boundary_parsing() {
        let input = "
        down
        {
            type            symmetryPlane;
            inGroups        List<word> 1(symmetryPlane);
            nFaces          60;
            startFace       3890;
        }";
        let expected = FoamStructure {
            name: "down".to_string(),
            content: {
                let mut map = IndexMap::new();
                map.insert(
                    "type".to_string(),
                    FoamValue::String("symmetryPlane".to_string()),
                );
                map.insert(
                    "inGroups".to_string(),
                    FoamValue::List(vec!["symmetryPlane".to_string()]),
                );
                map.insert("nFaces".to_string(), FoamValue::Integer(60));
                map.insert("startFace".to_string(), FoamValue::Integer(3890));
                map
            },
        };
        let result = FoamStructure::parse(input).expect("Failed to parse structure.");
        assert_eq!(result.1, expected);
    }

    #[test]
    fn test_recursive_parsing() {
        let input = "
        boundaryField
        {
            down
            {
                type            symmetryPlane;
            }
            
            right
            {
                type            fixedValue;
            }
        }";
        let inner_down = FoamStructure {
            name: "down".to_string(),
            content: {
                let mut map = IndexMap::new();
                map.insert(
                    "type".to_string(),
                    FoamValue::String("symmetryPlane".to_string()),
                );
                map
            },
        };
        let inner_right = FoamStructure {
            name: "right".to_string(),
            content: {
                let mut map = IndexMap::new();
                map.insert(
                    "type".to_string(),
                    FoamValue::String("fixedValue".to_string()),
                );
                map
            },
        };
        let expected = FoamStructure {
            name: "boundaryField".to_string(),
            content: {
                let mut map = IndexMap::new();
                map.insert("down".to_string(), FoamValue::Structure(inner_down));
                map.insert("right".to_string(), FoamValue::Structure(inner_right));
                map
            },
        };
        let result = FoamStructure::parse(input).expect("Failed to parse structure.");
        assert_eq!(result.1, expected);
    }

    #[test]
    fn test_recursive_with_string() {
        // The "constant (1 0 0)" field was problematic at some point, and a new FoamValue will be
        // needed in the future to deal with this kind of entry.
        let input = "
boundaryField
{
    down
    {
        type            symmetryPlane;
    }

    right
    {
        type            zeroGradient;
    }

    up
    {
        type            symmetryPlane;
    }

    left
    {
        type            uniformFixedValue;
        uniformValue    constant (1 0 0);
    }

    cylinder
    {
        type            symmetry;
    }

    defaultFaces
    {
        type            empty;
    }
}";
        let result = FoamStructure::parse(input).expect("Failed to parse structure.");
        println!("{:?}", result.1);
    }
}

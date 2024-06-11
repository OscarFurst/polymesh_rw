use crate::file_parser::FileParser;
use crate::foam_structure::FoamField;
use crate::parser_base::*;
use indexmap::map::IndexMap;
use nom::multi::fold_many0;
use nom::sequence::pair;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    multi::count,
    sequence::{delimited, preceded},
    IResult,
};
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct BoundaryField {
    pub name: String,
    pub boundary_type: String,
    pub value: Option<FoamField>,
    pub parameters: IndexMap<String, String>,
}

fn write_boundary_fields(
    fieldlist: &Option<IndexMap<String, BoundaryField>>,
    file: &mut std::fs::File,
) -> std::io::Result<()> {
    if let Some(fieldlist) = fieldlist {
        writeln!(file, "boundaryField")?;
        writeln!(file, "{{")?;
        for (name, boundaryfield) in fieldlist {
            writeln!(file, "{}", name)?;
            writeln!(file, "{{")?;
            writeln!(file, "type            {};", boundaryfield.boundary_type)?;
            if let Some(value) = &boundaryfield.value {
                write!(file, "value           ")?;
                value.write(file)?;
            }
            for (key, value) in &boundaryfield.parameters {
                writeln!(file, "{} {};", key, value)?;
            }
            writeln!(file, "}}")?;
        }
        writeln!(file, "}}")?;
    }
    Ok(())
}

fn parse_boundary_fields(input: &str) -> IResult<&str, IndexMap<String, BoundaryField>> {
    preceded(
        next(tag("boundaryField")),
        delimited(
            next(char('{')),
            fold_many0(parse_boundary_field, IndexMap::new, |mut map, v| {
                map.insert(v.name.clone(), v);
                map
            }),
            next(char('}')),
        ),
    )(input)
}

fn parse_boundary_field(input: &str) -> IResult<&str, BoundaryField> {
    let (input, name) = next(string_val)(input)?;
    let (input, _) = next(char('{'))(input)?;
    // TODO: we assume to find things in a certain order, but this is not guaranteed...
    let (input, boundary_type) = next(key_string_semicolon("type"))(input)?;
    let (input, value) = opt(delimited(
        next(key_string("value")),
        FoamField::parse,
        opt(next(char(';'))),
    ))(input)?;
    let (input, parameters) = parse_parameters(input)?;
    let (input, _) = next(char('}'))(input)?;
    Ok((
        input,
        BoundaryField {
            name,
            boundary_type,
            value,
            parameters,
        },
    ))
}

/// Parses arbitrary key-value pairs and stores them in an IndexMap. Both keys and values are strings.
fn parse_parameters(input: &str) -> IResult<&str, IndexMap<String, String>> {
    fold_many0(
        pair(next(string_val), next(string_val)),
        IndexMap::new,
        |mut map: IndexMap<String, String>, (k, v)| {
            map.insert(k, v);
            map
        },
    )(input)
}

type Dimensions = [i32; 7];

fn dimensions_string(dimensions: &Dimensions) -> String {
    format!(
        "dimensions      [{} {} {} {} {} {} {}];",
        dimensions[0],
        dimensions[1],
        dimensions[2],
        dimensions[3],
        dimensions[4],
        dimensions[5],
        dimensions[6]
    )
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(
        recognize(preceded(opt(char('-')), digit1)),
        str::parse::<i32>,
    )(input)
}

fn dimension(input: &str) -> IResult<&str, Dimensions> {
    map_res(
        delimited(char('['), count(lws(parse_i32), 7), tag("];")),
        Vec::try_into,
    )(input)
}

fn dimension_tag(input: &str) -> IResult<&str, Dimensions> {
    preceded(next(tag("dimensions")), next(dimension))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResultData {
    pub n: usize,
    pub dimensions: Dimensions,
    pub result: FoamField,
    pub boundary_field: Option<IndexMap<String, BoundaryField>>,
}

impl FileParser for ResultData {
    /// Assumes the remaining input contains the data.
    /// Data is either a scalar field or a vector field.
    fn parse_data(input: &str) -> IResult<&str, ResultData> {
        // Parse the dimensions.
        let (input, dimensions) = dimension_tag(input)?;
        // Parse the field data.
        let (input, result) = preceded(next(tag("internalField")), FoamField::parse)(input)?;
        let n = match &result {
            FoamField::Scalar(values) => values.len(),
            FoamField::Vector(values) => values.len(),
            _ => 1,
        };
        // Parse the boundary field which is sometimes present (in initial conditions for example).
        let (input, boundary_field) = opt(parse_boundary_fields)(input)?;
        // Return the new data structure
        Ok((
            input,
            ResultData {
                n,
                dimensions,
                result,
                boundary_field,
            },
        ))
    }

    // TODO: this is not correct but needs a larger change to work
    fn file_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/points")
    }

    fn write_data(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        println!("{}\n", dimensions_string(&self.dimensions));
        write!(file, "internalField   ")?;
        self.result.write(file)?;
        write_boundary_fields(&self.boundary_field, file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scalar() {
        let input = "
dimensions      [0 2 -2 0 0 0 0];

internalField   nonuniform List<scalar> 
4
(
685.183
685.183
685.184
685.184
)";
        let expected_value = ResultData {
            n: 4,
            dimensions: [0, 2, -2, 0, 0, 0, 0],
            result: FoamField::Scalar(vec![685.183, 685.183, 685.184, 685.184]),
            boundary_field: None,
        };
        let (_, actual_value) = ResultData::parse_data(input).unwrap();
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    fn test_parse_vector() {
        let input = "
dimensions      [0 1 -1 0 0 0 0];

internalField   nonuniform List<vector> 
4
(
(-8.52809e-05 0.00019428 0.00777701)
(-8.36566e-05 0.00019361 0.00802691)
(-8.15522e-05 0.000192606 0.00828979)
(-7.90789e-05 0.000191318 0.00856647)
)";
        let expected_value = ResultData {
            n: 4,
            dimensions: [0, 1, -1, 0, 0, 0, 0],
            result: FoamField::Vector(vec![
                vec![-8.52809e-05, 0.00019428, 0.00777701],
                vec![-8.36566e-05, 0.00019361, 0.00802691],
                vec![-8.15522e-05, 0.000192606, 0.00828979],
                vec![-7.90789e-05, 0.000191318, 0.00856647],
            ]),
            boundary_field: None,
        };
        let (_, actual_value) = ResultData::parse_data(input).unwrap();
        assert_eq!(expected_value, actual_value);
    }
}

use super::foam_value::FoamValue;
use super::parser_base::*;
use super::FileElement;
use indexmap::map::IndexMap;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair};
use nom::IResult;

/// A structure that holds key-value pairs.
/// Correspons to structures commonly found in OpenFOAM files, such as the following part of a boundary file:
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

impl FileElement for FoamStructure {
    /// Parse a FoamStructure from the given input.
    fn parse(input: &str) -> IResult<&str, FoamStructure> {
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
}

impl std::fmt::Display for FoamStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "{{")?;
        for (key, value) in &self.content {
            write!(f, "    {: <15} ", key)?;
            write!(f, "{}", value)?;
        }
        writeln!(f, "}}")
    }
}

impl FoamStructure {
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
                structure.name.clone_from(&s);
                (s, FoamValue::Structure(structure))
            } else {
                (s, v)
            }
        })(input)
    }

    // TODO: clean up write and write_recursive

    /// Write the structure to the given file, but without the name.
    /// This is used for recursive writing of structures.
    pub fn write_recursive(&self, file: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(file, "\n    {{")?;
        for (key, value) in &self.content {
            write!(file, "    {: <15} ", key)?;
            write!(file, "{}", value)?;
        }
        writeln!(file, "    }}")?;
        Ok(())
    }

    pub fn relative_file_path(&self) -> Option<std::path::PathBuf> {
        if let Some(FoamValue::String(location)) = self.content.get("location") {
            if let Some(FoamValue::String(object)) = self.content.get("object") {
                return Some(std::path::PathBuf::from(&location).join(object));
            }
        }
        None
    }
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

use super::foam_value::FoamValue;
use super::parser_base::*;
use super::FileElement;
use indexmap::map::IndexMap;
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::sequence::pair;
use nom::IResult;

/// A structure that holds key-value pairs.
/// It is effectively a HashMap with some extra I/O functionalities.
/// Correspons to structures commonly found in OpenFOAM files, such as the following part of a uniform/time file:
/// ```text
/// beginTime       0;
/// value           1282;
/// name            "1282";
/// ```
/// They can also be nested, which is then indicated by brackets, as in the following part of a boundary file:
/// ```text
/// down
/// {
///     type            patch;
///     physicalType    wall;
///     nFaces          0;
///     startFace       3890;
/// }
/// In this latter example, the FoamStructure would contain a key "down" with a value that is another FoamStructure.
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct FoamStructure(pub IndexMap<String, FoamValue>);

impl FileElement for FoamStructure {
    fn parse(input: &str) -> IResult<&str, FoamStructure> {
        map(Self::parse_map, FoamStructure)(input)
    }
}

impl std::ops::Deref for FoamStructure {
    type Target = IndexMap<String, FoamValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FoamStructure {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for FoamStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (key, value) in &self.0 {
            write!(f, "{: <15} ", key)?;
            writeln!(f, "{}", value)?;
        }
        Ok(())
    }
}

impl FoamStructure {
    /// Parse a single key-value pair from the given input.
    fn parse_pair(input: &str) -> IResult<&str, (String, FoamValue)> {
        pair(next(string_val), lws(FoamValue::parse))(input)
    }

    /// Parse multiple key-value pair from the given input and store them as IndexMap.
    pub fn parse_map(input: &str) -> IResult<&str, IndexMap<String, FoamValue>> {
        fold_many1(
            FoamStructure::parse_pair,
            IndexMap::new,
            |mut map, (k, v)| {
                map.insert(k, v);
                map
            },
        )(input)
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
        let inner_map = {
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
        };
        let outer_map = {
            let mut map = IndexMap::new();
            map.insert(
                "down".to_string(),
                FoamValue::Structure(FoamStructure(inner_map)),
            );
            map
        };
        let expected = FoamStructure(outer_map);
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
        let inner_down = FoamStructure({
            let mut map = IndexMap::new();
            map.insert(
                "type".to_string(),
                FoamValue::String("symmetryPlane".to_string()),
            );
            map
        });
        let inner_right = FoamStructure({
            let mut map = IndexMap::new();
            map.insert(
                "type".to_string(),
                FoamValue::String("fixedValue".to_string()),
            );
            map
        });
        let second_layer = FoamStructure({
            let mut map = IndexMap::new();
            map.insert("down".to_string(), FoamValue::Structure(inner_down));
            map.insert("right".to_string(), FoamValue::Structure(inner_right));
            map
        });
        let expected = FoamStructure({
            let mut map = IndexMap::new();
            map.insert(
                "boundaryField".to_string(),
                FoamValue::Structure(second_layer),
            );
            map
        });
        let result = FoamStructure::parse(input).expect("Failed to parse structure.");
        assert_eq!(result.1, expected);
    }

    #[test]
    fn test_recursive_with_string() {
        // The "constant (1 0 0)" field was problematic at some point, and a new FoamValue will be
        // needed in the future to deal with this kind of entry.
        let input = "
    down
    {
        type            symmetryPlane;
        inGroups        List<word> 1(symmetryPlane);
        nFaces          60;
        startFace       3890;
    }
    right
    {
        type            patch;
        nFaces          30;
        startFace       3950;
    }
    up
    {
        type            symmetryPlane;
        inGroups        List<word> 1(symmetryPlane);
        nFaces          60;
        startFace       3980;
    }
    left
    {
        type            patch;
        nFaces          30;
        startFace       4040;
    }
    cylinder
    {
        type            symmetry;
        inGroups        List<word> 1(symmetry);
        nFaces          40;
        startFace       4070;
    }
    defaultFaces
    {
        type            empty;
        inGroups        List<word> 1(empty);
        nFaces          4000;
        startFace       4110;
    }";
        let result = FoamStructure::parse(input).expect("Failed to parse structure.");
        println!("{}", result.1);
    }
}

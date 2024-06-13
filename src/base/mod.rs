/// The base module contains the basic building blocks for parsing and writing OpenFOAM files.
mod file_content;
mod file_parser;
mod foam_field;
mod foam_structure;
mod foam_value;
mod foamfile;
pub(crate) mod parser_base;
pub(crate) mod writer_base;

// Re-export the data structures.
pub use file_content::FileContent;
pub use file_parser::FileElement;
pub use file_parser::FileParser;
pub use foam_field::FoamField;
pub use foam_structure::FoamStructure;
pub use foam_value::FoamValue;
pub use foamfile::FoamFile;

#[cfg(test)]
mod tests {
    use crate::base::foam_value::FoamValue;
    use indexmap::map::IndexMap;

    use super::*;

    #[test]
    fn test_parse_foamfile() {
        let input = r#"
FoamFile
{
    format      ascii;
    class       vectorField;
    location    "constant/polyMesh";
    object      points;
}"#;
        let expected_inner = FoamStructure({
            let mut fields = IndexMap::new();
            fields.insert("format".to_string(), FoamValue::String("ascii".to_string()));
            fields.insert(
                "class".to_string(),
                FoamValue::String("vectorField".to_string()),
            );
            fields.insert(
                "location".to_string(),
                FoamValue::String(r#""constant/polyMesh""#.to_string()),
            );
            fields.insert(
                "object".to_string(),
                FoamValue::String("points".to_string()),
            );
            fields
        });
        let expected_data = FoamStructure({
            let mut fields = IndexMap::new();
            fields.insert("FoamFile".to_string(), FoamValue::Structure(expected_inner));
            fields
        });
        let expected = Ok(("", expected_data));
        let actual = FoamStructure::parse(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_header() {
        let input = r#"
/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Version:  10
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/
FoamFile
{
    version     0.0;
    format      ascii;
    class       polyBoundaryMesh;
    note        "nPoints:215  nCells:592  nFaces:1388  nInternalFaces:980";
    location    "constant/polyMesh";
    object      boundary;
}"#;
        let expected_inner = FoamStructure({
            let mut fields = IndexMap::new();
            // TODO: The version is parsed as a float, but it should be a string!
            fields.insert("version".to_string(), FoamValue::Float(0.0));
            fields.insert("format".to_string(), FoamValue::String("ascii".to_string()));
            fields.insert(
                "class".to_string(),
                FoamValue::String("polyBoundaryMesh".to_string()),
            );
            fields.insert(
                "note".to_string(),
                FoamValue::String(
                    r#""nPoints:215  nCells:592  nFaces:1388  nInternalFaces:980""#.to_string(),
                ),
            );
            fields.insert(
                "location".to_string(),
                FoamValue::String(r#""constant/polyMesh""#.to_string()),
            );
            fields.insert(
                "object".to_string(),
                FoamValue::String("boundary".to_string()),
            );
            fields
        });
        let expected = FoamStructure({
            let mut fields = IndexMap::new();
            fields.insert("FoamFile".to_string(), FoamValue::Structure(expected_inner));
            fields
        });
        let (_, actual) = FoamStructure::parse(input).unwrap();
        assert_eq!(expected, actual);
    }
}

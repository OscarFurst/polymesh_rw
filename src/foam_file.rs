use crate::foam_structure::FoamStructure;

pub type FoamFileData = FoamStructure;

#[cfg(test)]
mod tests {
    use crate::foam_structure::FoamValue;
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
        let expected_data = FoamFileData {
            name: "FoamFile".to_string(),
            content: {
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
            },
        };

        let expected = Ok(("", expected_data));
        let actual = FoamFileData::parse(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_header() {
        let input = r#"/*--------------------------------*- C++ -*----------------------------------*\
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
        let expected_data = FoamFileData {
            name: "FoamFile".to_string(),
            content: {
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
            },
        };

        let expected = Ok(("", expected_data));
        let actual = FoamFileData::parse(input);
        assert_eq!(expected, actual);
    }
}

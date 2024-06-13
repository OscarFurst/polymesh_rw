use crate::base::{parser_base::*, FileElement, FileParser};
use indexmap::map::IndexMap;
use nom::{character::complete::char, multi::count, IResult};

/// A trait for the different types of zones, e.g. cellZones, faceZones and pointZones.
pub trait Zone: FileElement {
    fn name(&self) -> &str;
    fn default_file_path() -> std::path::PathBuf;
}

/// Container for the polyMesh Zones data, e.g. cellZones, faceZones and pointZones.
#[derive(Debug, PartialEq, Clone)]
pub struct ZoneData<T: Zone> {
    pub n: usize,
    pub zones: IndexMap<String, T>,
}

impl<T: Zone> FileParser for ZoneData<T> {
    fn default_file_path() -> std::path::PathBuf {
        T::default_file_path()
    }
}

impl<T: Zone> FileElement for ZoneData<T> {
    fn parse(input: &str) -> IResult<&str, ZoneData<T>> {
        // number of face zones
        let (input, n) = next(usize_val)(input)?;
        // opening parenthesis
        let (input, _) = next(char('('))(input)?;
        // parse face zones
        let (input, facezone_vector) = count(T::parse, n)(input)?;
        let zones = facezone_vector
            .into_iter()
            .map(|facezone| (facezone.name().to_string(), facezone))
            .collect();
        // closing parenthesis
        let (input, _) = next(char(')'))(input)?;
        Ok((input, ZoneData { n, zones }))
    }
}

impl<T: Zone> std::fmt::Display for ZoneData<T> {
    fn fmt(&self, file: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(file, "{}", self.n)?;
        writeln!(file, "(")?;
        for zone in self.zones.values() {
            writeln!(file, "{}", zone)?;
        }
        writeln!(file, ")")?;
        Ok(())
    }
}

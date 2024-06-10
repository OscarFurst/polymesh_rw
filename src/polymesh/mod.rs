use crate::file_parser::FileParser;
use crate::foam_file::FoamFileData;
use std::error::Error;
use std::path;

pub mod boundary;
pub mod cellzones;
pub mod faces;
pub mod facezones;
pub mod neighbour;
pub mod owner;
pub mod points;
pub mod pointzones;

pub struct PolyMesh {
    pub points: (Option<FoamFileData>, points::PointData),
    pub faces: (Option<FoamFileData>, faces::FaceData),
    pub owner: (Option<FoamFileData>, owner::OwnerData),
    pub neighbour: (Option<FoamFileData>, neighbour::NeighbourData),
    pub boundary: (Option<FoamFileData>, boundary::BoundaryData),
    pub facezones: (Option<FoamFileData>, facezones::FaceZoneData),
    pub cellzones: (Option<FoamFileData>, cellzones::CellZoneData),
    pub pointzones: (Option<FoamFileData>, pointzones::PointZoneData),
}

impl PolyMesh {
    pub fn parse(dir_path: &path::Path) -> Result<PolyMesh, Box<dyn Error>> {
        let points = points::PointData::parse(&dir_path.join("points"))?;
        let faces = faces::FaceData::parse(&dir_path.join("faces"))?;
        let owner = owner::OwnerData::parse(&dir_path.join("owner"))?;
        let neighbour = neighbour::NeighbourData::parse(&dir_path.join("neighbour"))?;
        let boundary = boundary::BoundaryData::parse(&dir_path.join("boundary"))?;
        let facezones = facezones::FaceZoneData::parse(&dir_path.join("faceZones"))?;
        let cellzones = cellzones::CellZoneData::parse(&dir_path.join("cellZones"))?;
        let pointzones = pointzones::PointZoneData::parse(&dir_path.join("pointZones"))?;
        Ok(PolyMesh {
            points,
            faces,
            owner,
            neighbour,
            boundary,
            facezones,
            cellzones,
            pointzones,
        })
    }
}

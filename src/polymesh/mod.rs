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
pub mod sets;

pub struct FileContent<T: FileParser> {
    pub meta: Option<FoamFileData>,
    pub data: T,
}
impl<T: FileParser> FileContent<T> {
    /// Write the file to the given case directory.
    // TODO: Do I want to enforse the case directory structure, or provide more flexibility?
    fn write(&self, path: &path::Path) {
        //
    }
}

pub struct PolyMesh {
    pub points: FileContent<points::PointData>,
    pub faces: FileContent<faces::FaceData>,
    pub owner: FileContent<owner::OwnerData>,
    pub neighbour: FileContent<neighbour::NeighbourData>,
    pub boundary: FileContent<boundary::BoundaryData>,
    pub facezones: FileContent<facezones::FaceZoneData>,
    pub cellzones: FileContent<cellzones::CellZoneData>,
    pub pointzones: FileContent<pointzones::PointZoneData>,
    pub sets: sets::Sets,
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
        let sets = sets::Sets::parse(&dir_path.join("sets"));
        Ok(PolyMesh {
            points,
            faces,
            owner,
            neighbour,
            boundary,
            facezones,
            cellzones,
            pointzones,
            sets,
        })
    }

    pub fn parse_and_assert(dir_path: &path::Path) -> PolyMesh {
        let points = points::PointData::parse_and_assert(&dir_path.join("points"));
        let faces = faces::FaceData::parse_and_assert(&dir_path.join("faces"));
        let owner = owner::OwnerData::parse_and_assert(&dir_path.join("owner"));
        let neighbour = neighbour::NeighbourData::parse_and_assert(&dir_path.join("neighbour"));
        let boundary = boundary::BoundaryData::parse_and_assert(&dir_path.join("boundary"));
        let facezones = facezones::FaceZoneData::parse_and_assert(&dir_path.join("faceZones"));
        let cellzones = cellzones::CellZoneData::parse_and_assert(&dir_path.join("cellZones"));
        let pointzones = pointzones::PointZoneData::parse_and_assert(&dir_path.join("pointZones"));
        let sets = sets::Sets::parse(&dir_path.join("sets"));
        PolyMesh {
            points,
            faces,
            owner,
            neighbour,
            boundary,
            facezones,
            cellzones,
            pointzones,
            sets,
        }
    }
}

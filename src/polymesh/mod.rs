use crate::base::FileContent;
use std::path;

mod boundary;
mod cellzones;
mod faces;
mod facezones;
mod neighbour;
mod owner;
mod points;
mod pointzones;
mod result;
mod sets;
mod timedir;
mod uniform;
mod zones;

// re-exports
pub use boundary::BoundaryData;
pub use cellzones::CellZone;
pub use faces::FaceData;
pub use facezones::FaceZone;
pub use neighbour::NeighbourData;
pub use owner::OwnerData;
pub use points::PointData;
pub use pointzones::PointZone;
pub use result::ResultData;
pub use sets::Set;
pub use sets::Sets;
pub use timedir::TimeDir;
pub use uniform::UniformData;
pub use zones::Zone;
pub use zones::ZoneData;

/// The PolyMesh structure holds all the data of a polyMesh directory.
#[derive(Debug, PartialEq, Clone)]
pub struct PolyMesh {
    pub points: FileContent<points::PointData>,
    pub faces: FileContent<faces::FaceData>,
    pub owner: FileContent<owner::OwnerData>,
    pub neighbour: FileContent<neighbour::NeighbourData>,
    pub boundary: FileContent<boundary::BoundaryData>,
    pub facezones: Option<FileContent<ZoneData<FaceZone>>>,
    pub cellzones: Option<FileContent<ZoneData<CellZone>>>,
    pub pointzones: Option<FileContent<ZoneData<PointZone>>>,
    pub sets: Option<Sets>,
}

impl PolyMesh {
    pub fn parse(dir_path: &path::Path) -> std::io::Result<PolyMesh> {
        let points = FileContent::<PointData>::parse_file(&dir_path.join("points"))?;
        let faces = FileContent::<FaceData>::parse_file(&dir_path.join("faces"))?;
        let owner = FileContent::<OwnerData>::parse_file(&dir_path.join("owner"))?;
        let neighbour = FileContent::<NeighbourData>::parse_file(&dir_path.join("neighbour"))?;
        let boundary = FileContent::<BoundaryData>::parse_file(&dir_path.join("boundary"))?;
        let facezones =
            FileContent::<ZoneData<FaceZone>>::parse_file(&dir_path.join("faceZones")).ok();
        let cellzones =
            FileContent::<ZoneData<CellZone>>::parse_file(&dir_path.join("cellZones")).ok();
        let pointzones =
            FileContent::<ZoneData<PointZone>>::parse_file(&dir_path.join("pointZones")).ok();
        let sets = sets::Sets::parse_files(&dir_path.join("sets")).ok();
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

    pub fn write(&self, path: &path::Path) -> std::io::Result<()> {
        self.points.write_file(path)?;
        self.faces.write_file(path)?;
        self.owner.write_file(path)?;
        self.neighbour.write_file(path)?;
        self.boundary.write_file(path)?;
        if let Some(facezones) = &self.facezones {
            facezones.write_file(path)?;
        }
        if let Some(cellzones) = &self.cellzones {
            cellzones.write_file(path)?;
        }
        if let Some(pointzones) = &self.pointzones {
            pointzones.write_file(path)?;
        }
        if let Some(sets) = &self.sets {
            sets.write(path)?;
        }
        Ok(())
    }
}

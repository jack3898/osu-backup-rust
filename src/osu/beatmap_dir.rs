use std::{fs::ReadDir, io, path::PathBuf};

pub struct BeatmapDir {
    pub id: String,
    pub title: String,
    pub path: PathBuf,
}

impl BeatmapDir {
    pub fn dir_name(&self) -> String {
        format!("{} {}", self.id, self.title)
    }

    pub fn read_dir(&self) -> Result<ReadDir, io::Error> {
        self.path.read_dir()
    }
}

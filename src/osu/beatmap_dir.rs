use std::path::PathBuf;

pub struct BeatmapDir {
    pub id: String,
    pub title: String,
    pub path: PathBuf,
}

impl BeatmapDir {
    pub fn dir_name(&self) -> String {
        format!("{} {}", self.id, self.title)
    }

    pub fn dir_path(&self) -> PathBuf {
        let mut cloned_path = self.path.clone();
        cloned_path.push(self.dir_name());

        cloned_path
    }
}

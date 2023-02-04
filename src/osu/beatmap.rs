use super::beatmap_dir::BeatmapDir;

pub struct Beatmap<'a> {
    pub dir: &'a BeatmapDir,
    pub online_url: String,
    pub osu_direct_url: String,
    pub mp3s: Vec<Mp3File>,
    pub jpgs: Vec<JpgFile>,
}

impl Beatmap<'_> {
    pub fn load_images() -> Vec<JpgFile> {
        vec![]
    }

    pub fn load_audio() -> Vec<Mp3File> {
        vec![]
    }
}

pub struct Mp3File;

pub struct JpgFile;

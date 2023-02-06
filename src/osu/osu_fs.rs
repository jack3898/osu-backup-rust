use super::{beatmap::Beatmap, beatmap_dir::BeatmapDir};
use std::{env, fs::ReadDir, io, path::Path};

#[derive(Copy, Clone)]
pub struct OsuFs<'a> {
    pub path: &'a Path,
}

impl OsuFs<'_> {
    /**
     * Get the default osu directory using the LocalAppData env var built into Windows.
     */
    pub fn get_default_osu_dir() -> String {
        let mut osu_dir = env::var("LocalAppData").unwrap();

        osu_dir.push_str("\\osu!");

        osu_dir
    }

    pub fn read_songs_dir(&self) -> Result<ReadDir, io::Error> {
        self.path.join("Songs").read_dir()
    }

    pub fn get_beatmap_dirs(&self) -> Result<Vec<BeatmapDir>, io::Error> {
        let dir_contents = self.read_songs_dir()?;

        let beatmap_dirs: Vec<BeatmapDir> = dir_contents
            .filter_map(|dir_entry_result| {
                let dir_entry = dir_entry_result.ok()?;

                let os_file_name = dir_entry.file_name();
                let file_name = os_file_name.to_str()?;
                let (id, title) = file_name.split_once(' ')?;

                let dir = BeatmapDir {
                    id: String::from(id),
                    title: String::from(title),
                    path: dir_entry.path(),
                };

                Some(dir)
            })
            .collect();

        Ok(beatmap_dirs)
    }

    pub fn expand_beatmap_details<'a>(self, beatmap: &'a BeatmapDir) -> Option<Beatmap<'a>> {
        let beatmap_url = format!("https://osu.ppy.sh/beatmapsets/{}", beatmap.id);
        let osu_direct_url = format!("osu://b/{}", beatmap.id);

        let song = Beatmap {
            dir: &beatmap,
            online_url: beatmap_url,
            osu_direct_url,
        };

        Some(song)
    }
}

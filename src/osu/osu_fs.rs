use super::{beatmap::Beatmap, beatmap_dir::BeatmapDir};
use crate::{unwrap_option_or, unwrap_result_or, unwrap_result_or_return_err};
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
        let dir_contents_result = self.read_songs_dir();
        let dir_contents = unwrap_result_or_return_err!(dir_contents_result);

        let beatmap_dirs: Vec<BeatmapDir> = dir_contents
            .filter_map(|dir_entry_result| {
                let dir_entry = unwrap_result_or!(dir_entry_result, { return None });

                let os_file_name = dir_entry.file_name();
                let file_name_str = os_file_name.to_str();
                let file_name = unwrap_option_or!(file_name_str, { return None });
                let file_name_split = file_name.split_once(' ');

                let (id, title) = unwrap_option_or!(file_name_split, { return None });

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
        let song_path = self.path.join("Songs").join(beatmap.dir_name());
        let open_song_dir = song_path.read_dir();

        let song_dir = unwrap_result_or!(open_song_dir, {
            println!("There was a problem opening the song directory.");

            return None;
        });

        println!("{}", song_dir.count());

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

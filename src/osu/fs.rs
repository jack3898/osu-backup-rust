use super::{beatmap::Beatmap, beatmap_dir::BeatmapDir};
use crate::{unwrap_option_or, unwrap_result_or, unwrap_result_or_return_err};
use std::{io, path::Path};

#[derive(Copy, Clone)]
pub struct OsuFs<'a> {
    pub path: &'a Path,
}

impl OsuFs<'_> {
    pub fn get_beatmap_dirs(&self) -> Result<Vec<BeatmapDir>, io::Error> {
        let dir_contents_result = self.path.join("Songs").read_dir();
        let dir_contents = unwrap_result_or_return_err!(dir_contents_result);
        let mut beatmap_dirs: Vec<BeatmapDir> = vec![];

        for dir_entry_result in dir_contents {
            let dir_entry = unwrap_result_or!(dir_entry_result, {
                println!("There was a problem opening the a beatmap directory.");

                continue;
            });

            let os_file_name = dir_entry.file_name();

            let file_name_str = os_file_name.to_str();
            let file_name = unwrap_option_or!(file_name_str, { continue });
            let file_name_split = file_name.split_once(' ');

            let (id, title) = unwrap_option_or!(file_name_split, { continue });

            let beatmap_dir = BeatmapDir {
                id: String::from(id),
                title: String::from(title),
                path: dir_entry.path(),
            };

            beatmap_dirs.push(beatmap_dir);
        }

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
            mp3s: vec![],
            jpgs: vec![],
        };

        Some(song)
    }
}

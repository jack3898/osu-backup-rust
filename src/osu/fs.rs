use super::{beatmap::Beatmap, beatmap_dir::BeatmapDir};
use std::{io, path::Path};

#[derive(Copy, Clone)]
pub struct OsuFs<'a> {
    pub path: &'a Path,
}

impl OsuFs<'_> {
    pub fn get_beatmap_dirs(&self) -> Result<Vec<BeatmapDir>, io::Error> {
        let dir_contents_result = self.path.join("Songs").read_dir();

        let dir_contents = match dir_contents_result {
            Result::Ok(contents) => contents,
            Result::Err(error) => {
                println!("There was an error reading the songs directory.");

                return Err(error);
            }
        };

        let mut beatmap_dirs: Vec<BeatmapDir> = vec![];

        for dir_entry_result in dir_contents {
            let dir_entry = match dir_entry_result {
                Result::Ok(entry) => entry,
                Result::Err(_) => {
                    println!("There was an issue opening a song directory.");

                    continue;
                }
            };

            let os_file_name = dir_entry.file_name();
            let file_name = match os_file_name.to_str() {
                Option::Some(name) => name,
                Option::None => {
                    println!("There was an error retrieving a folder name.");

                    continue;
                }
            };

            let (id, title) = match file_name.split_once(' ') {
                Option::Some(str_tuple) => str_tuple,
                Option::None => continue,
            };

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

        let song_dir = match open_song_dir {
            Result::Ok(dir) => dir,
            Result::Err(_) => {
                println!("Could not open song directory.");

                return None;
            }
        };

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

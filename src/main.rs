mod input;
mod osu;

use crate::osu::fs::OsuFs;
use std::path::Path;

fn main() {
    let osu_fs = OsuFs {
        // Will remove absolute path later!
        path: Path::new("C:\\Users\\Jack\\AppData\\Local\\osu!"),
    };

    let beatmap_dirs = osu_fs.get_beatmap_dirs().unwrap();

    for beatmap_dir in beatmap_dirs.iter() {
        let beatmap = osu_fs.expand_beatmap_details(beatmap_dir).unwrap();

        println!("{:?}", beatmap.dir.dir_path());
    }
}

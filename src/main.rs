mod input;
mod macros;
mod osu;

use crate::osu::osu_fs::OsuFs;
use input::cli::Cli;
use std::path::Path;

fn main() {
    let mut cli = Cli::new();
    let default_osu_dir = OsuFs::get_default_osu_dir();
    let osu_dir = cli.get_args().get_directory().unwrap_or(&default_osu_dir);

    let osu_fs = OsuFs {
        path: Path::new(osu_dir.as_str()),
    };

    let beatmap_dirs = osu_fs.get_beatmap_dirs().unwrap();

    for beatmap_dir in beatmap_dirs.iter() {
        let beatmap = osu_fs.expand_beatmap_details(beatmap_dir).unwrap();

        println!("{:?}", beatmap.dir.dir_path());
    }
}

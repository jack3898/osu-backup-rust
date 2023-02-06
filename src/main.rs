mod input;
mod osu;

use crate::osu::osu_fs::OsuFs;
use input::cli::Cli;
use std::{path::Path, thread};

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

        let images = beatmap.get_images();

        let image_threads = images.into_iter().map(|image| {
            println!("Processing a new image...");
            let name = image.file_name;
            let filter = image::imageops::FilterType::CatmullRom;

            let handle = thread::spawn(move || {
                image
                    .image_result
                    .resize(1280, 720, filter)
                    .blur(5.0)
                    .into_rgb8()
                    .save_with_format(
                        format!("C:\\Users\\Jack\\Downloads\\{}", name),
                        image::ImageFormat::Jpeg,
                    )
                    .expect("2");
            });

            handle
        });

        for image_processor_handle in image_threads {
            image_processor_handle.join().unwrap();
        }
    }
}

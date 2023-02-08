mod input;
mod osu;

extern crate futures;

use crate::osu::osu_fs::OsuFs;
use futures::future::join_all;
use image::ImageError;
use input::cli::Cli;
use std::path::Path;
use tokio::runtime::Handle;

#[tokio::main]
async fn main() {
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
        let filter = image::imageops::FilterType::CatmullRom;
        let format = image::ImageFormat::Jpeg;
        let handle = Handle::current();

        let image_threads: Vec<tokio::task::JoinHandle<Result<(), ImageError>>> = images
            .into_iter()
            .filter_map(|image| {
                println!("Processing a new image...");
                let name = image.file_name;
                let location = format!("C:\\Users\\Jack\\Downloads\\{}", name);

                let join_handle = handle.spawn(async move {
                    image
                        .image_result
                        .resize(1280, 720, filter)
                        .into_rgb8()
                        .save_with_format(location, format)
                });

                Some(join_handle)
            })
            .collect();

        join_all(image_threads).await;
    }
}

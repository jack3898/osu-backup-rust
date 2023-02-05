use super::beatmap_dir::BeatmapDir;
use crate::{unwrap_option_or, unwrap_result_or};
use image::{DynamicImage, ImageResult};
use std::fs::DirEntry;

pub struct Beatmap<'a> {
    pub dir: &'a BeatmapDir,
    pub online_url: String,
    pub osu_direct_url: String,
}

impl Beatmap<'_> {
    pub fn get_images(&self) -> Vec<JpgFile> {
        let contents = self
            .dir
            .read_dir()
            .expect("Unable to open the contents of the song directory.");

        let jpgs: Vec<JpgFile> = contents
            .filter_map(|item_result| {
                let item_option: Option<DirEntry> = match item_result {
                    Ok(item) => Some(item),
                    Err(_) => None,
                };

                let item = item_option.unwrap();

                if str::ends_with(item.file_name().to_str().unwrap(), ".jpg") {
                    let image_result = image::io::Reader::open(item.path());
                    let image_reader = unwrap_result_or!(image_result, { return None });

                    let save_path = item.path().to_str().unwrap().to_owned();

                    let os_file_name = item.file_name();
                    let file_name_option = os_file_name.to_str();
                    let file_name = unwrap_option_or!(file_name_option, { return None });

                    let image_file = JpgFile {
                        image_result: image_reader.decode(),
                        song_dir: save_path,
                        file_name: file_name.to_owned().repeat(2),
                    };

                    return Some(image_file);
                };

                None::<JpgFile>
            })
            .collect();

        jpgs
    }
}

pub struct JpgFile {
    pub image_result: ImageResult<DynamicImage>,
    pub song_dir: String,
    pub file_name: String,
}

impl JpgFile {
    pub fn get_full_path(&self) -> String {
        format!("{}{}", self.song_dir, self.file_name)
    }
}

use super::beatmap_dir::BeatmapDir;
use image::DynamicImage;

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
            .filter_map(|dir_entry_result| -> Option<JpgFile> {
                let dir_entry = dir_entry_result.ok()?;
                let image_read = image::io::Reader::open(dir_entry.path()).ok()?;
                let image_formatted = image_read.format()?;

                if !image::ImageFormat::can_read(&image_formatted) {
                    return None;
                }

                let song_dir = dir_entry.path().to_str()?.to_owned();
                let file_name = dir_entry.file_name().to_str()?.to_owned();
                let image_result = image_read.decode().ok()?;

                let image_file = JpgFile {
                    song_dir,
                    file_name,
                    image_result,
                };

                Some(image_file)
            })
            .collect();

        jpgs
    }
}

pub struct JpgFile {
    pub image_result: DynamicImage,
    pub song_dir: String,
    pub file_name: String,
}

impl JpgFile {
    pub fn get_full_path(&self) -> String {
        format!("{}{}", self.song_dir, self.file_name)
    }
}

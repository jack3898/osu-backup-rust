use crate::unwrap_option_or;

pub struct Args {
    pub directory: Option<String>,
}

impl Args {
    pub fn new<'a>() -> Args {
        Args { directory: None }
    }

    pub fn add_directory(&mut self, path: Option<&String>) -> &mut Self {
        self.directory = path.cloned();

        self
    }

    pub fn get_directory(&self) -> Option<&String> {
        let this_dir = &self.directory;
        let dir = unwrap_option_or!(this_dir, { return None });

        Some(dir)
    }
}

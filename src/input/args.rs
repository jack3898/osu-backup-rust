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
        self.directory.as_ref()
    }
}

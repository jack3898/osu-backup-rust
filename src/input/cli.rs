use super::args::Args;
use std::collections::HashMap;

pub struct Cli {
    args: Vec<String>,
}

impl Cli {
    pub fn new(arg_collection: Vec<String>) -> Cli {
        Cli {
            args: arg_collection,
        }
    }

    fn key_value_pairs(&self) -> HashMap<String, String> {
        let parsed_args: HashMap<String, String> = HashMap::new();

        let key_value_pairs = &self.args.iter().fold(parsed_args, |mut acc, cur| {
            let key_value: Vec<&str> = cur.split("=").collect();

            if key_value.len() == 2 {
                let alpha_key = key_value[0].replace("--", "");

                acc.insert(alpha_key, key_value[1].to_owned());
            }

            acc
        });

        key_value_pairs.clone()
    }

    pub fn get_args(&self) -> Args {
        let mut args = Args::new();
        let arg_hashmap = self.key_value_pairs();

        args.add_directory(arg_hashmap.get("directory"));

        args
    }
}

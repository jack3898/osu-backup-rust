use super::args::Args;
use std::collections::HashMap;
use std::env;

pub struct Cli {
    env_args: Vec<String>,
    args: Args,
}

impl Cli {
    pub fn new<'a>() -> Cli {
        Cli {
            env_args: env::args().collect(),
            args: Args::new(),
        }
    }

    fn key_value_pairs(&self) -> HashMap<String, String> {
        let parsed_args: HashMap<String, String> = HashMap::new();

        let key_value_pairs = &self.env_args.iter().fold(parsed_args, |mut acc, cur| {
            let key_value: Vec<&str> = cur.split("=").collect();

            if key_value.len() == 2 {
                let alpha_key = key_value[0].replace("--", "");

                acc.insert(alpha_key, key_value[1].to_owned());
            }

            acc
        });

        key_value_pairs.clone()
    }

    pub fn get_args<'a>(&'a mut self) -> &Args {
        let arg_hashmap = self.key_value_pairs();

        self.args.add_directory(arg_hashmap.get("directory"));

        &self.args
    }
}

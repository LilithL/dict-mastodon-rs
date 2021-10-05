extern crate yaml_rust;
extern crate elefren;
extern crate rand;
use crate::masto_bot::private::MastoBotPrivate;
use elefren::{helpers::cli, prelude::*, entities::status::Status};
use rand::{prelude::IteratorRandom};
use yaml_rust::{YamlLoader, Yaml};
use std::{default, fs, io::{BufRead, BufReader}};

pub(super) mod private {
    use super::*;

    #[doc(hidden)]
    // Internal utility function.
    pub(super) trait MastoBotPrivate {
        // Read the configuration file and parse it with yaml_rust.
        fn read_conf(path: Option<&str>) -> Yaml;
    }

}

pub trait MastoBot {
    /// Create a new mastodon Bot object.
    /// ```rust
    /// use masto_bot::{Bot, MastoBot};
    /// 
    /// let bot = Bot::new(matches.value_of("config"));
    fn new(path: Option<&str>) -> Bot;

    /// Register to a mastodon instance.
    fn register(&self, url: &str) -> Result<(), elefren::Error>;

    /// Connect the mastodon bot object with the given configuration parameters.
    /// 
    /// ```rust
    /// use mastodon_bot::Bot;
    /// 
    /// let bot = Bot::new("./config.yaml");
    /// bot.connect();
    /// ```
    fn connect(&self);

    /// Get a word, either from wordnik or our local dictionary.
    fn gen_sentence(&self) -> String;

    /// Post a toot on the configured account.
    fn post(&self, msg: String) -> Result<Status, elefren::Error>;
}

pub struct Bot{
    conf: Yaml,
    mastodon: Option<elefren::Mastodon>
}

impl private::MastoBotPrivate for Bot {
    fn read_conf(path: Option<&str>) -> Yaml {
        if path.is_none() {
            panic!("Path not defined !")
        }

        let conf_str = fs::read_to_string(path.unwrap()).expect("An error happened while opening the file");
        let conf_yaml = YamlLoader::load_from_str(&conf_str);
        match conf_yaml {
            Ok(cfg) => return cfg.first().unwrap().clone(),
            Err(e) => panic!("An error happened while decoding the file:\n{:?}", e)
        }
    }
}

impl MastoBot for Bot {

    fn new(path: Option<&str>) -> Bot {
        Bot {
            conf: Bot::read_conf(path),
            mastodon: None
        }
    }
    
    fn register(&self, url: &str) -> Result<(), elefren::Error> {
        let registration = Registration::new(url)
            .client_name("Mastobot")
            .build().expect("An error occured while connecting");
        let url = registration.authorize_url().unwrap();
        println!("Please connect to {} and validate the connection.", url);

        Ok(())
    }

    fn gen_sentence(&self) -> String {
        let local_dict = match fs::File::open(self.conf["local_dictionary"].as_str().unwrap()) {
            Ok(dict) => dict,
            Err(e) => panic!("An error occured while opening the local dictionary:\n{:?}", e)
        };
        
        let mut rng = rand::thread_rng();
        let local_dict = BufReader::new(local_dict);
        let local_dict = local_dict.lines().map(|elem| elem.expect("Couldn't read line"));
        let word = local_dict.choose(&mut rng)
            .expect("No line found");
        if word.contains("**") {
            return word.replace("**", "");
        } else {
            return word+" "+self.conf["appended_word"].as_str().unwrap();
        }
    }

    fn post(&self, msg: String) -> Result<Status, elefren::Error> {
        todo!();
    }

    fn connect(&self) {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn bot_setup(path: &str) -> Bot {
        Bot::new(Some(&format!("{}{}", env!("CARGO_MANIFEST_DIR"), path)))
    }

    #[test]
    #[should_panic(expected = "Path not defined !")]
    fn test_read_conf_empty() {
        Bot::read_conf(None);
    }

    #[test]
    #[should_panic(expected = "An error happened while opening the file")]
    fn test_read_conf_not_found() {
        Bot::read_conf(Some("/somewhere/non/existant"));
    }

    #[test]
    #[should_panic(expected = "An error happened while decoding the file:\nScanError { mark: Marker { index: 137, line: 8, col: 0 }, info: \"while parsing a block mapping, did not find expected key\" }")]
    fn test_read_conf_wrong_syntax() {
        let path = format!("{}/tests/assets/wrong_config.yaml", env!("CARGO_MANIFEST_DIR"));
        println!("{}", path);
        Bot::read_conf(Some(&path));    
    }

    #[test]
    fn test_read_conf() {
        let path = format!("{}/tests/assets/config.yaml", env!("CARGO_MANIFEST_DIR"));
        let conf = Bot::read_conf(Some(&path));
        assert_eq!(
            format!("{:?}", conf),
            "Hash({String(\"mastodon\"): Hash({String(\"client_id\"): String(\"SuperSecretToken\"), String(\"client_secret\"): String(\"SuperSecretToken\"), String(\"token\"): String(\"SuperSecretToken\"), String(\"base\"): String(\"https://example.com\"), String(\"redirect\"): String(\"https://example.com/redirect\")}), String(\"wordnik\"): Hash({String(\"api_token\"): String(\"SuperSecretToken\")}), String(\"local_dictionary\"): String(\"./tests/assets/dictionary.txt\"), String(\"appended_word\"): String(\"myWord\")})"
        );
    }

    #[test]
    fn test_conf_example() {
        let conf_path = format!("{}/assets/config.yaml.example", env!("CARGO_MANIFEST_DIR"));
        Bot::new(Some(&conf_path));
    }


    #[test]
    fn test_gen_sentence() {
        let bot = bot_setup("/tests/assets/config.yaml");
        assert!(bot.gen_sentence().to_lowercase().contains("myword"))
    }

    #[test]
    fn test_gen_sentence_2() {
        let bot = bot_setup("/tests/assets/config_2.yaml");
        assert!(bot.gen_sentence().to_lowercase().contains("myword2"))
    }
}
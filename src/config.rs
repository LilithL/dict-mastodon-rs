use crate::error::Error;
use elefren::Data;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub mastodon: Data,
    pub wordnik: WordnikConf,
    pub local_dictionary: String,
    pub appended_word: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WordnikConf {
    pub api_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mastodon: Data {
                client_id: std::borrow::Cow::from("SuperSecretToken"),
                client_secret: std::borrow::Cow::from("SuperSecretToken"),
                token: std::borrow::Cow::from("SuperSecretToken"),
                base: std::borrow::Cow::from("https://example.com"),
                redirect: std::borrow::Cow::from("https://example.com/redirect"),
            },
            wordnik: Default::default(),
            local_dictionary: Default::default(),
            appended_word: Default::default(),
        }
    }
}

pub fn read_conf(path: &str) -> Result<Config, Error> {
    let parsed = serde_yaml::from_str::<Config>(fs::read_to_string(path)?.as_str())?;
    Ok(parsed)
}

impl Config {
    pub fn write_conf(&self, path: &str) -> Result<(), Error> {
        let mut file = fs::File::create(path)?;
        file.write(serde_yaml::to_string(self)?.as_ref())?;
        Ok(())
    }
}

impl Default for WordnikConf {
    fn default() -> Self {
        Self { api_token: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_conf_1() {
        let conf_proof: Config = Config {
            mastodon: Data {
                client_id: std::borrow::Cow::from("SuperSecretToken"),
                client_secret: std::borrow::Cow::from("SuperSecretToken"),
                token: std::borrow::Cow::from("SuperSecretToken"),
                base: std::borrow::Cow::from("https://example.com"),
                redirect: std::borrow::Cow::from("https://example.com/redirect"),
            },
            wordnik: WordnikConf { api_token: None },
            local_dictionary: String::from("./tests/assets/dictionary.txt"),
            appended_word: String::from("myWord"),
        };

        let config: Config = read_conf(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/assets/config.yaml"
        ))
        .unwrap();
        assert!(dbg!(config).eq(&conf_proof));
    }

    #[test]
    fn read_conf_2() {
        let conf_proof: Config = Config {
            mastodon: Data {
                client_id: std::borrow::Cow::from("SuperSecretToken"),
                client_secret: std::borrow::Cow::from("SuperSecretToken"),
                token: std::borrow::Cow::from("SuperSecretToken"),
                base: std::borrow::Cow::from("https://example.com"),
                redirect: std::borrow::Cow::from("https://example.com/redirect"),
            },
            wordnik: WordnikConf {
                api_token: Some(String::from("SuperSecretToken")),
            },
            local_dictionary: String::from("./tests/assets/dictionary.txt"),
            appended_word: String::from("myWord2"),
        };

        let config: Config = read_conf(concat!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/assets/config_2.yaml"
        )))
        .unwrap();
        assert!(dbg!(config).eq(&conf_proof));
    }

    #[test]
    #[should_panic]
    fn assert_wrong_config() {
        read_conf(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/assets/wrong_config.yaml"
        ))
        .unwrap();
    }

    #[test]
    fn test_write_conf() -> Result<(), Error> {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets/test_write.yaml");
        let conf = Config::default();
        conf.write_conf(path)?;

        let conf_read = read_conf(path)?;
        assert!(conf_read.eq(&conf));
        fs::remove_file(path)?;
        Ok(())
    }
}

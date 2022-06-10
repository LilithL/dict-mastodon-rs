extern crate elefren;
extern crate rand;
mod cmd;
mod config;

use std::{
    fs,
    io::{self, BufRead},
    process::exit,
};

use clap::ArgMatches;
use config::Config;
use rand::{prelude::IteratorRandom, Rng};

// This value indicates the chance of usage for the local dictionary (in %).
const DICT_CHANCE: u8 = 15;

fn main() {
    let matches: ArgMatches = cmd::get_matches();
    let conf_path = matches.value_of("CONFIG").unwrap();
    let conf = match config::read_conf(conf_path) {
        Ok(conf) => conf,
        Err(err) => {
            println!("{}", err);
            Config::default()
        }
    };

    match matches.subcommand() {
        Some(("toot", matches)) => {
            let word = gen_word(&conf).unwrap();
            if matches.is_present("post") {
                todo!("Post generated word on mastodon.");
            } else {
                print!("{}", word)
            }
        }
        Some(("loop", matches)) => {
            todo!("Implement loop posting functions");
        }
        Some(("register", matches)) => {
            // Create registration and use the CLI helper from elefren to authenticate.
            let registration = elefren::Registration::new(matches.value_of("URL").unwrap())
                .client_name(matches.value_of("APP_NAME").unwrap())
                .build()
                .expect("Error when registering app to mastodon instance.");
            let mastodon = elefren::helpers::cli::authenticate(registration)
                .expect("Error while authenticating.");

            // Save authentication data into configuration file
            let mut new_conf = conf.clone();
            new_conf.mastodon = mastodon.data;
            new_conf
                .write_conf(conf_path)
                .expect("Error while saving new configuration to file.")
        }
        _ => exit(0),
    }
}

fn gen_word(conf: &Config) -> Result<String, io::Error> {
    let mut rng = rand::thread_rng();

    // Enter condition when wordnik api is not None and the random number we got
    // chooses to use the wordnik provider instead of local file
    if conf.wordnik.api_token.is_some() && rng.gen_range(0..=100) > DICT_CHANCE {
        todo!("Wordnik call to get a word and return result.")
    }

    // Choose random word, append space + appended word to it and return the result
    let dict = get_dict_iter(&conf)?;
    let mut word = dict.choose(&mut rng).unwrap()?;
    if word.contains("**") {
        return Ok(word.replace("**", ""));
    }
    word.push(' ');
    word.push_str(conf.appended_word.as_str());
    Ok(word)
}

fn get_dict_iter(conf: &Config) -> Result<io::Lines<io::BufReader<fs::File>>, io::Error> {
    let dict_file = fs::File::open(conf.local_dictionary.as_str())?;
    Ok(io::BufReader::new(dict_file).lines())
}

extern crate elefren;
extern crate rand;
mod cmd;
mod config;

use std::{
    fs,
    io::{self, BufRead, Write},
    process::exit,
    thread::sleep,
    time::Duration,
};

use clap::ArgMatches;
use config::Config;
use elefren::{scopes::Scopes, Language, MastodonClient, StatusBuilder};
use rand::{prelude::IteratorRandom, Rng};
use wordnik::Client;

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
                let mstdn = elefren::Mastodon::from(conf.mastodon.clone());
                mstdn
                    .new_status(
                        StatusBuilder::new()
                            .status(word.as_str())
                            .language(Language::Eng)
                            .build()
                            .unwrap(),
                    )
                    .expect("Could not post status");
            }
            print!("{}", &word)
        }
        Some(("loop", matches)) => {
            let mstdn = elefren::Mastodon::from(conf.mastodon.clone());
            loop {
                // Post generated status
                let word = gen_word(&conf).unwrap();
                mstdn
                    .new_status(
                        StatusBuilder::new()
                            .status(word.as_str())
                            .language(Language::Eng)
                            .build()
                            .unwrap(),
                    )
                    .expect("Could not post status");
                println!("Posted : {}", word);
                io::stdout().flush().unwrap();

                // Sleep for DELAY seconds
                sleep(Duration::from_secs(
                    matches
                        .value_of("DELAY")
                        .unwrap()
                        .parse::<u64>()
                        .expect("Error while parsing delay value"),
                ));
            }
        }
        Some(("register", matches)) => {
            // Create registration and use the CLI helper from elefren to authenticate.
            let registration = elefren::Registration::new(matches.value_of("URL").unwrap())
                .client_name(matches.value_of("APP_NAME").unwrap())
                .scopes(
                    Scopes::write(elefren::scopes::Write::Statuses)
                        | Scopes::read(elefren::scopes::Read::Statuses),
                )
                .build()
                .expect("Error when registering app to mastodon instance");
            let mastodon = elefren::helpers::cli::authenticate(registration)
                .expect("Error while authenticating");

            // Save authentication data into configuration file
            let mut new_conf = conf.clone();
            new_conf.mastodon = mastodon.data;
            new_conf
                .write_conf(conf_path)
                .expect("Error while saving new configuration to file")
        }
        _ => exit(0),
    }
}

fn gen_word(conf: &Config) -> Result<String, io::Error> {
    let mut rng = rand::thread_rng();

    // Enter condition when wordnik api is not None and the random number we got
    // chooses to use the wordnik provider instead of local file
    if conf.wordnik.api_token.is_some() && rng.gen_range(0..=100) > DICT_CHANCE {
        let client = Client::new(conf.wordnik.api_token.as_ref().unwrap());
        todo!("Get random word from wordnik api.")
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

extern crate elefren;
extern crate rand;
mod cmd;
mod config;

use std::{fs, process::exit};

use clap::ArgMatches;
use config::Config;

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
            todo!("Implement toot functions");
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

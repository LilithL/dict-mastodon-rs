extern crate yaml_rust;
extern crate elefren;
extern crate rand;
mod cmd;
mod masto_bot;
use masto_bot::{Bot, MastoBot};
use std::process::exit;

fn main() {
    let matches = cmd::get_matches();
    let mut bot = Bot::new(matches.value_of("config"));

    match matches.subcommand() {
        ("toot", Some(sub_match)) => {
            println!("{}", bot.gen_sentence());
            if sub_match.is_present("post") {
                bot.post(bot.gen_sentence()).expect("An error occured while posting");
            }
        },
        ("getcred", Some(sub_match)) => {
            bot.register(sub_match.value_of("url").expect("No url parameter entered.")).expect("An error occured while registering the app");
        }
        _ => exit(0),
    }
}

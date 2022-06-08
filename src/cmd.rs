extern crate clap;
use clap::{arg, ArgMatches, Command};

pub fn get_matches() -> ArgMatches {
    return Command::new(env!("CARGO_PKG_NAME"))
        .arg_required_else_help(true)
        .version(concat!("v", env!("CARGO_PKG_VERSION")))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(arg!([CONFIG] "The path to the configuration file.")
            .default_value("./config.yaml"))

        .subcommand(Command::new("toot")
                    .about("Generate a toot and print it in the console (or post it to mastodon if option -p is provided).")
                    .arg(arg!(-p --post "Post the generated toot.")))

        .subcommand(Command::new("loop")
                    .about("Generate and toot until the program is terminated. The delay between each toot can be set with the timeout option.")
                    .arg(arg!(--delay [DELAY] "The time between each toots (in seconds).")
                        .default_value("1800")))
        .subcommand(Command::new("register")
                    .about("An helper to connect to an account and retrieve API credentials.")
                    .arg(arg!(<APP_NAME> "The name of the application to register."))
                    .arg(arg!([URL] "The instance url on which the app will be registered.")
                        .default_value("https://botsin.space")))
        .get_matches();
}

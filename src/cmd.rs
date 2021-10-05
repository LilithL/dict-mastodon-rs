extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches, AppSettings};

pub fn get_matches() -> ArgMatches<'static> {
    return App::new("lesbBot")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Lilith")
        .about("The core program of lesbianBot. Rust edition !")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .default_value("./config.yaml")
            .help("The path to the configuration file."))

        .subcommand(SubCommand::with_name("toot")
                    .about("Generate a toot and print it in the console.")
                    .arg(Arg::with_name("post")
                        .short("p")
                        .long("post")
                        .help("Post the generated toot.")))

        .subcommand(SubCommand::with_name("loop")
                    .about("Generate and toot until the program is terminated. The delay between each toot can be set with the timeout option.")
                    .arg(Arg::with_name("delay")
                        .value_name("DELAY")
                        .default_value("1800")
                        .help("The time between each toots (in seconds).")))

        .subcommand(SubCommand::with_name("getcred")
                    .about("An helper to connect to an account and retrieve API credentials."))
                    .arg(Arg::with_name("url")
                        .long("url")
                        .takes_value(true)
                        .default_value("https://botsin.space")
                        .help("The instance url on which the app will be registered."))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();
}
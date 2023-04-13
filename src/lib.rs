use std::{fmt::Error};
use clap::{Command, Arg, ArgAction, ArgMatches, value_parser};

pub struct AppConfig {
    pub command: String,
    pub sub_match: ArgMatches,
}

/**
 * Builds out the main app commands
 */
pub fn build_new_app() -> Result<AppConfig, Error> {
    let config = Command::new("devy")
        .version("0.1.0")
        .author("dvnc0")
        .about("Some dev tools")
        .subcommand_required(true)
        .subcommand(Command::new("base64")
            .about("Base64 encoding and decoding.")
            .arg(Arg::new("string")
                .value_name("STRING")
                .help("The string to encode or decode")
                .required(true)
            )
            .arg(Arg::new("decode")
                .long("decode")
                .short('d')
                .help("base64 decode the given value")
                .action(ArgAction::SetTrue)
            )
            .arg(Arg::new("encode")
                .long("encode")
                .short('e')
                .help("base64 encode the given value")
                .action(ArgAction::SetTrue)
            )
        ).subcommand(Command::new("password")
            .about("Generate a password, default is 32 characters with all character types")
            .arg(Arg::new("lower_letters")
                .long("lower")
                .short('l')
                .help("Include lowercase letters")
                .default_value("false")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue)
            )
            .arg(Arg::new("upper_letters")
                .long("upper")
                .short('u')
                .help("Include uppercase letters")
                .default_value("false")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue)
            )
            .arg(Arg::new("numbers")
                .long("numbers")
                .short('n')
                .help("Include numbers")
                .default_value("false")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue)
            )
            .arg(Arg::new("symbols")
                .long("special")
                .short('s')
                .help("Include special characters")
                .default_value("false")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue)
            )
            .arg(Arg::new("use_all")
                .short('A')
                .default_value("true")
                .default_value_ifs(
                    [
                        ("lower_letters", "true", Some("false")),
                        ("upper_letters", "true", Some("false")),
                        ("numbers", "true", Some("false")),
                        ("symbols", "true", Some("false")),
                    ]
                )
                .exclusive(true)
                .action(ArgAction::SetTrue)
                .hide(true)
            )
            .arg(Arg::new("length")
                .long("length")
                .value_name("LENGTH")
                .default_value("32")
                .value_parser(value_parser!(u32))
                .help("The length of the password")
            )
        )
        .subcommand(Command::new("api")
            .about("Make API request(s) from a yaml file")
            .arg(Arg::new("file")
                .long("file")
                .value_name("FILE")
                .required(true)
                .help("Path to Yaml file to use for sending requests")
            )
        ).get_matches();
    
    let app_config = get_app_config(config);

    Ok(app_config)

}

/**
 * Gets the request config
 */
fn get_app_config(config: ArgMatches) -> AppConfig {
    let app = &config.subcommand().unwrap();
    
    let app_config = AppConfig {
        command: app.0.to_string(),
        sub_match: app.1.to_owned(),
    };

    app_config
}
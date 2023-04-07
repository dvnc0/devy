use std::fmt::Error;
use clap::{Command, Arg, ArgAction, ArgMatches};

pub mod build;
// Not crazy about this, may change
#[derive(Debug)]
pub struct AppConfig {
    pub command: String,
    pub options: Vec<(String, bool)>,
    pub values: Vec<(String, String)>
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
        ).get_matches();
    
    let app_config = get_app_config(config);

    Ok(app_config)

}

/**
 * Gets the request config
 */
fn get_app_config(config: ArgMatches) -> AppConfig {
    let app_config = match config.subcommand() {
        Some(("base64", sub_match)) => {
            build::build_base64_app(sub_match)
        },
        _ => unreachable!("No commands found")
    };

    app_config
}
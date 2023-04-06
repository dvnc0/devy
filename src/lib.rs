use core::panic;
use std::fmt::Error;
use clap::{Command, Arg, ArgAction, ArgMatches};

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
            let decode = sub_match.get_flag("decode");
            let encode = sub_match.get_flag("encode");

            if !decode && !encode {
                panic!("base64 requires either the -d or -e flag")
            }

            let option = if decode {
                vec![("decode".to_string(), true)]
            } else {
                vec![("encode".to_string(), true)]
            };

            let base_string = sub_match.get_one::<String>("string").unwrap().to_string();
            let value = vec![("string".to_string(), base_string)];

            AppConfig {
                command: "base64".to_string(),
                options: option,
                values: value
            }
        },
        _ => unreachable!("No commands found")
    };

    app_config
}
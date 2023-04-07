use clap::ArgMatches;
use crate::AppConfig;

/**
 * build the base64 app config
 */
pub fn build_base64_app(sub_match: &ArgMatches) -> AppConfig {
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
}
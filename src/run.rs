use std::fmt::Error;
use base64::{Engine as _, engine::{general_purpose}};
use devy::{AppConfig};

/**
 * base64 encode or decode
 */
pub fn process_base64(app_config: AppConfig) -> Result<String, Error> {
    let options = &app_config.options[0];
    let (_value_id, value_string) = &app_config.values[0];

    let outcome = match options {
        (key, _value) => {
            let action = key.as_str();

            let result = match action {
                "encode" => general_purpose::STANDARD.encode(value_string.as_bytes()),
                "decode" => String::from_utf8(general_purpose::STANDARD.decode(value_string.as_bytes()).unwrap()).unwrap(),
                _ => unreachable!("Could not determine action")
            };

            result
        }
    };

    Ok(outcome)
}
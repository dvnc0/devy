use std::fmt::Error;
use base64::{Engine as _, engine::{general_purpose}};
use clap::ArgMatches;

pub struct Base64 {
    pub string: String,
    pub encode: bool,
    pub decode: bool,
}

/**
 * Implement basic methods for Base64
 */
impl Base64 {
    fn encode_string(&self, string: String) -> String {
        general_purpose::STANDARD.encode(string.as_bytes())
    }

    fn decode_string(&self, string: String) -> String {
        String::from_utf8(general_purpose::STANDARD.decode(string.as_bytes()).unwrap()).unwrap()
    }

    pub fn run(&self) -> Result<String, Error> {
        let outcome:String;
        if self.encode {
            outcome = self.encode_string(self.string.to_string());
        } else if self.decode {
            outcome = self.decode_string(self.string.to_string());
        } else {
            unreachable!("No action")
        }

        Ok(outcome.to_string())
    }
}

/**
 * Execute the Base64 app
 */
pub fn execute(sub_match: ArgMatches) -> Result<String, Error> {
    let decode = sub_match.get_flag("decode");
    let encode = sub_match.get_flag("encode");
    let base_string = sub_match.get_one::<String>("string").unwrap().to_string();

    if !decode && !encode {
        panic!("base64 requires either the -d or -e flag")
    }

    let config = Base64 {
        string: base_string,
        encode: encode,
        decode: decode,
    };

    config.run()
}


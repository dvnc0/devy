use std::vec;
use clap::{Error, ArgMatches};
use rand::Rng;

pub struct Password {
    pub lower_letters: bool,
    pub upper_letters: bool,
    pub numbers: bool,
    pub symbols: bool,
    pub use_all: bool,
    pub length: u32,
}

impl Password {
    pub fn run(&self) -> Vec<Vec<char>> {
        let options = &self.build_options();
        options.to_owned()
    }

    fn build_options(&self) -> Vec<Vec<char>> {
        let numbers = vec!['0','1','2','3','4','5','6','7','8','9'];
        let letters:Vec<char> = (b'a'..=b'z').map(|c| c as char).collect();
        let letters_upper: Vec<char> = (b'A'..=b'Z').map(|c| c as char).collect();
        let symbols:Vec<char> = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=', '[', ']', '{', '}', ';', ':', '\'', '\"', ',', '.', '/', '?'];

        if self.use_all {
            vec![numbers, letters, letters_upper, symbols]
        } else {
            let mut options_out = vec![];
            if self.numbers {
                options_out.push(numbers)
            } 
            
            if self.lower_letters {
                options_out.push(letters)
            } 
            
            if self.upper_letters {
                options_out.push(letters_upper)
            } 
            
            if self.symbols {
                options_out.push(symbols)
            }

            options_out
        }
    }
}

pub fn execute(sub_match: ArgMatches) -> Result<String, Error> {
    let config = Password {
        lower_letters: sub_match.get_flag("lower_letters"),
        upper_letters: sub_match.get_flag("upper_letters"),
        numbers: sub_match.get_flag("numbers"),
        symbols: sub_match.get_flag("symbols"),
        use_all: sub_match.get_flag("use_all"),
        length: sub_match.get_one::<u32>("length").unwrap().to_owned(),
    };

    let options = config.run();
    let mut out:Vec<String> = vec![];

    for _ in 0..config.length {
        let option_key = rand::thread_rng().gen_range(0..options.len());
        let key = rand::thread_rng().gen_range(0..options[option_key].len());
        let value = &options[option_key][key].to_string();

        out.push(value.to_string())
    }

    Ok(out.join("").to_string())
}
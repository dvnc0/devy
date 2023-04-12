use std::collections::HashMap;
use std::{fs::File, fmt::Error};
use std::io::prelude::*;
use clap::ArgMatches;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Request {
    pub title: String,
    pub data: RequestDetails
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct RequestDetails {
    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
}

struct ApiCalls {
    requests: Vec<Request>,
    file_path: String
}

impl ApiCalls {

    pub fn run(&self) -> Result<String, Error> {
        dbg!(&self.requests);
        dbg!(&self.requests.len());
        println!("Running API Requests from {}", self.file_path.to_string());
        Ok("ok".to_string())
    }
}

/**
 * build the request structs from the given yaml file
 */
fn build_requests(file_path: String) -> Vec<Request> {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Errr");

    let requests: Vec<Request> = serde_yaml::from_str(&contents).expect("err");
    
    requests

}

/**
 * execute the API runner
 */
pub fn execute(sub_match: ArgMatches) -> Result<String, Error> {
    let file_path = sub_match.get_one::<String>("file").unwrap().to_string();

    let requests = ApiCalls {
        requests: build_requests(file_path.clone()),
        file_path
    };

    requests.run()
}
use std::collections::HashMap;
use std::{fs::File, fmt::Error};
use std::io::prelude::*;
use clap::ArgMatches;
use serde::{Serialize, Deserialize};
use curl::easy::{Easy, List};

pub mod print;

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

    /**
     * run the api call loop
     */
    pub fn run(&self) -> Result<String, Error> {
        println!("Running API Requests from {}", self.file_path.to_string());
        for request in &self.requests {
            print::info(request.title.to_string());
            dbg!(request);
            let request_body: &RequestDetails = &request.data;
            self.route_request(request_body);
        }
        Ok("ok".to_string())
    }

    /**
     * route the request to the correct handler or panic
     */
    fn route_request(&self, request_body: &RequestDetails) {
        let method = request_body.method.as_str();
        match method {
            "GET" => self.send_get_request(request_body),
            _ => panic!("Method not found!") 
        };
    }

    fn send_get_request(&self, request_body: &RequestDetails) {
        let mut header_list = List::new();

        if let Some(header_map) = &request_body.headers {
            for (key, value) in header_map {
                header_list.append(format!("{}: {}", key, value).as_str()).unwrap();
            }
        }
        
        let mut curl = Easy::new();

        curl.url(request_body.url.as_str()).unwrap();
        curl.http_headers(header_list).unwrap();
        curl.write_function(|data| {
            println!("{}", String::from_utf8_lossy(data));
            Ok(data.len())
        }).unwrap();
        curl.perform().unwrap_or_else(|e| {
            print::error(e.to_string());
        });
        
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
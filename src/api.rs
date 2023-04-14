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
    pub post_body: Option<String>,
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
            let request_body: &RequestDetails = &request.data;
            self.route_request(request_body);
        }
        Ok("Requests completed.".to_string())
    }

    /**
     * route the request to the correct handler or panic
     */
    fn route_request(&self, request_body: &RequestDetails) {
        let method = request_body.method.as_str();
        match method {
            "GET" => self.send_get_request(request_body),
            "POST" => self.send_post_request(request_body),
            _ => panic!("Method not found!") 
        };
    }

    /**
     * sends a get request and attaches any passed headers
     */
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
            print::message("Success! Printing result below:".to_string());
            println!("{}", String::from_utf8_lossy(data));
            Ok(data.len())
        }).unwrap();
        curl.perform().unwrap_or_else(|e| {
            print::error(e.to_string());
        });

        println!("Response Code: {:?}\n\n", curl.response_code());
        
    }

    /**
     * send a post request
     */
    fn send_post_request(&self, request_body: &RequestDetails) {
        let mut header_list = List::new();

        if let Some(header_map) = &request_body.headers {
            for (key, value) in header_map {
                header_list.append(format!("{}: {}", key, value).as_str()).unwrap();
            }
        }
        
        let mut curl = Easy::new();

        curl.url(request_body.url.as_str()).unwrap();

        let post_data = &request_body.post_body.as_ref().unwrap();

        curl.post_field_size(post_data.len() as u64).unwrap();
        curl.post_fields_copy((post_data).as_bytes()).unwrap();

        curl.http_headers(header_list).unwrap();
        curl.write_function(|data| {
            print::message("Success! Printing result below:".to_string());
            println!("{}", String::from_utf8_lossy(data));
            Ok(data.len())
        }).unwrap();
        curl.perform().unwrap_or_else(|e| {
            print::error(e.to_string());
        });

        println!("Response Code: {:?}\n\n", curl.response_code());
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
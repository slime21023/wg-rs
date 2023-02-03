use clap::{Command, arg, value_parser, ArgAction};
use ureq;

#[derive(Debug)]
struct Header(String, String);

fn parse_header(header: &str) -> Header {
    let mut values = header.split(":").collect::<Vec<&str>>();
    if values.len() == 1 {
        values.push("");
    }

    Header(String::from(values[0]), String::from(values[1]))
}

fn get_response(url: &str, headers: Vec<Header>) -> Result<ureq::Response, ureq::Error> {
    let mut req = ureq::get(url);
    for h in headers {
        let Header(key, value) = h; 
        req = req.set(key.as_str(), value.as_str());
    }
    
    req.call()
}

fn handle_request(url: &str, headers: Vec<&String>) {
    let mut h:Vec<Header> = Vec::<Header>::new();
    for header in headers {
        h.push(parse_header(header));
    }

    if let Ok(resp) = get_response(url, h) {
        let output =  resp.into_string().unwrap_or_default();
        println!("{}", output);
    }
}

fn main() {
    let matches = Command::new("wg")
        .arg(
            arg!([url] "The request url")
                .required(true)
                .value_parser(value_parser!(String))
        )
        .arg(
            arg!(-H --header ... "The request header")
                .required(false)
                .value_parser(value_parser!(String))
                .action(ArgAction::Append)
        )
        .about("The simple web get program")
        .get_matches();
    
    let url_value = matches
        .get_one::<String>("url")
        .unwrap();
        
    
    let headers = matches
        .get_many::<String>("header")
        .unwrap_or_default()
        .collect::<Vec<_>>();
    
    handle_request(url_value, headers);

}

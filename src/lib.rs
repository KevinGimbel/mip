use std::net::{TcpStream};
use std::io::{Write, Read};

use regex::Regex;

const ENDPOINTS: [&'static str; 1] = ["httpbin.org:80"];

fn get_ip() -> String {
    let index: usize = 0;
    let host = ENDPOINTS[index];

    // Open socket connection
    let mut socket = TcpStream::connect(host).unwrap();

    // Format HTTP request
    // Important: Needs "\r\n\r\n" at the end! "\r\n" creates a new line, the second "\r\n" creates
    // blank line to mark the headers as "finished".
    let _r = socket.write_fmt(format_args!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", "/ip", host.to_owned()));

    // Build new empty string. HTTP Response is written into this string.
    let mut res = String::from("");
    // Write response into string
    let _resp = socket.read_to_string(&mut res).unwrap();

    // Capture IPv4 from response
    let re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
    let r = re.captures(res.as_str()).unwrap().get(0).map_or("", |m| m.as_str());

    // Return IP
    return r.to_string();
}

pub struct IP {}
impl IP {
    pub fn is() -> String {
        return get_ip();
    }
}

mod tests {
    #[test]
    fn test_match_some_ip() {
        use self::super::*;
        let ip = IP::is();
        let re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();

        assert_eq!(re.is_match(ip.as_str()), true);
    }
}
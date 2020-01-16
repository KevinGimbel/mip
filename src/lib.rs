use std::net::{TcpStream,Ipv4Addr,AddrParseError};
use std::io::{Write, Read};
use std::str::FromStr;
use std::option::Option;

#[derive(Copy, Clone)]
struct Endpoint<'a> {
    host: &'a str,
    path: Option<&'a str>,
    port: i32,
}

const ENDPOINTS: [Endpoint; 1] = [
    Endpoint{
        host: "httpbin.org",
        path: Some("/ip"),
        port: 80
    }
];

/// extract an IP from a string of text
fn parse_ip(input: &str) -> Result<Ipv4Addr, AddrParseError> {

    let ip_parts: Vec<&str> = input.split(|c| c == ' ' || c == '.' || c == '\n' || c == ':').collect();
    let mut ip_vec: Vec<i32> = vec![];

    for p in ip_parts {
        // Skip empty parts
        if p == "" {
            continue;
        }

        // Some clean up. It's just enough for working with httpbin.org
        let mut p_clean = str::replace(p, "\"", "");
        p_clean = str::replace(p_clean.as_str(), ",", "");

        if let Some(_x) = p.find(|c: char| c == '{' || c == '}') {
            p_clean = str::replace(p_clean.as_str(), "{", "");
            p_clean = str::replace(p_clean.as_str(), "}", "");
        }
        let mut r = i32::from_str(p_clean.as_str());

        match r {
            Ok(num) => { r = Ok(num) },
            Err(_e) => { r = Ok(256) }
        }

        let n = r.unwrap();

        if n < 256 {
            ip_vec.push(n);
        }
    }

    return Ipv4Addr::from_str(format!("{}.{}.{}.{}", ip_vec[0],ip_vec[1],ip_vec[2],ip_vec[3]).as_str());
}

fn get_ip() -> String {
    let index: usize = 0;
    let endpoint = ENDPOINTS[index];
    let host = format!("{}:{}", endpoint.host, endpoint.port);

    // Open socket connection
    let mut socket = TcpStream::connect(host).unwrap();

    // Format HTTP request
    // Important: Needs "\r\n\r\n" at the end! "\r\n" creates a new line, the second "\r\n" creates
    // blank line to mark the headers as "finished".
    let _r = socket.write_fmt(format_args!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", endpoint.path.unwrap_or("/"), endpoint.host));

    // Build new empty string. HTTP Response is written into this string.
    let mut res = String::from("");
    // Write response into string
    let _resp = socket.read_to_string(&mut res).unwrap();

    let r = parse_ip(res.as_str());
    // Return IP
    return r.unwrap().to_string();
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
        use regex::Regex;
        use self::super::*;

        let ip = IP::is();
        let re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();

        assert_eq!(re.is_match(ip.as_str()), true);
    }

    #[test]
    fn parse_ip_single_line() {
        use self::super::*;

        let ip = parse_ip("Your IP Address is 12.34.56.78").unwrap().to_string();
        assert_eq!(ip.as_str(), "12.34.56.78");
    }

    #[test]
    fn parse_ip_multi_line() {
        use self::super::*;
        let ip = parse_ip(r#"
        Hello and welcome!

        Your IP Address appears to be: 34.11.250.255,

        thank you for using this service!

        "#).unwrap().to_string();
        assert_eq!(ip.as_str(), "34.11.250.255");
    }

    #[test]
    fn parse_ip_multiple_ips() {
        use self::super::*;

        let ip = parse_ip("12.56.34.78, 12.34.56.78").unwrap().to_string();
        assert_eq!(ip.as_str(), "12.56.34.78");
    }
}
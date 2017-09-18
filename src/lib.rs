#[macro_use]
extern crate nom;

use std::str::from_utf8;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    GET,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    method: Method,
    path: String,
}

named!(parse_get<&[u8], Method>,  map!(tag!("GET"), |_| Method::GET));
named!(parse_space<&[u8], &[u8]>, take_until!(" "));
named!(parse_path<&[u8], &str>,   ws!(map_res!(take_until_and_consume!("\r\n"), from_utf8)));

named!(
    parse_request<&[u8], Request>, do_parse!(
        method: parse_get   >>
                parse_space >>
        path:   parse_path  >>
        (
            Request {
                method: method,
                path: path.into()
            }
        )
    )
);

pub fn parse_http09_request(i: &[u8]) -> IResult<&[u8], Request> {
    parse_request(i)
}

#[cfg(test)]
mod tests {
    use super::{Request, Method, parse_request};
    use nom::{IResult, ErrorKind, Needed};
    const EMPTY_SLICE: &'static [u8] = &[];

    #[test]
    fn http09_get_success_root() {
        assert_eq!(parse_request(&b"GET /\r\n"[..]),
                   IResult::Done(EMPTY_SLICE,
                                 Request {
                                     method: Method::GET,
                                     path: "/".to_string(),
                                 }));
    }

    #[test]
    fn http09_get_success_foo_bar() {
        assert_eq!(parse_request(&b"GET /foo/bar\r\n"[..]),
                   IResult::Done(EMPTY_SLICE,
                                 Request {
                                     method: Method::GET,
                                     path: "/foo/bar".to_string(),
                                 }));
    }

    #[test]
    fn http09_get_with_space_at_head() {
        assert_eq!(parse_request(&b" GET /foo/bar\r\n"[..]),
                   IResult::Error(ErrorKind::Tag));
    }

    #[test]
    fn http09_get_without_space_after_get() {
        assert_eq!(parse_request(&b"GET/foo/bar\r\n"[..]),
                   IResult::Error(ErrorKind::TakeUntil));
    }

    #[test]
    fn http09_post_failure() {
        assert_eq!(parse_request(&b"POST /foo/bar\r\n"[..]),
                   IResult::Error(ErrorKind::Tag));
    }

    #[test]
    fn http09_get_without_carrige_return() {
        assert_eq!(parse_request(&b"GET /"[..]),
                   IResult::Incomplete(Needed::Size(6)));
    }
}
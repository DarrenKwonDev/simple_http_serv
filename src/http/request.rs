use super::method::{Method, MethodError};
use std::str;
use std::{
    convert::TryFrom,
    fmt::{Debug, Display, Result as FmtResult},
    str::Utf8Error,
};
use super::QueryString;

//  구조체 안에 저장하는 모든 참조에 대해 수명을 명시적으로 지정해야 합니다.

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // Get /search?name=abc&sort=1 HTTP/1.1\r\n...
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?; // make buf to &str

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // parse method는 내부적으로 FromStr::from_str 를 호출함
        let method = method.parse::<Method>()?;

        // Option 타입에 값을 넣는 패턴
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "invalid request",
            ParseError::InvalidEncoding => "invalid encoding",
            ParseError::InvalidProtocol => "invalid protocol",
            ParseError::InvalidMethod => "invalid method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::str::Utf8Error;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path()
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&self) -> &Option<&QueryString> {
        &self.query_string.as_ref()
    }
}

// impl Request {
//     fn from_byte_array(value: &[u8]) -> Result<Self, String> {} 
// }

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {

        let request = str::from_utf8(buf)?;
        
        // match get_next_word(request) {
        //     Some((method, request)) => {},
        //     None => return Err(ParseError::InvalidRequest),
        // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        // match path.find("?") {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // let q = path.find("?");
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })

        // let string = String::from("asd");
        // string.encrypt();
        // buf.encrypt();
        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }

        
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
//    let mut iter = request.chars();
//    loop {
//     let item = iter.next();
//     match item {
//         Some(c) => {},
//         None => break,
//     }
//    }

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
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}


impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) 
    -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) 
    -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }

// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }
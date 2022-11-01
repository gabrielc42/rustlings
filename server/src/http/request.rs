use super::method::Method;
use std::convert::TryFrom;
use std::str::Utf8Error;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;


pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

// impl Request {
//     fn from_byte_array(value: &[u8]) -> Result<Self, String> {} 
// }

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        let request = str::from_utf8(buf)?;

        unimplemented!()

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
    if c == ' ' {
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
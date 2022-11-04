use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;
// use std::convert::TryInto;
use crate::http::{Request, Response, StatusCode};

pub struct Server {
    addr: String,
}

// fn arr(a: &[u8]) {

// }

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr, //if name of struct variable is same as fn param, can just pass this. other wise addr: addr
        }
    }

    pub fn run(self)
    // -> (i32, &str, std::net::TcpListener) returning multiple values
    {
        //self is like 'this' of java
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // let a = [1,2,3,4,5];
                    // arr(&a[1..3]);

                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) //or ... as &[u8]
                            // let res: &Result<Request, > = &buffer[..].try_intro();
                            {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h4> It works... </h4>".to_string()),
                                    );
                                    
                                },
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response! >:( : {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }

            // match "abcd" {
            //     "abcd" => println!(),
            //     "a" | "b" => {}
            //     _ => {}
            // }

            // let res = listener.accept();

            // if res.is_err() {
            //     continue;
            // }

            // let (stream, addr) = res.unwrap();
        }

        // (5, "a", listener); returning multiple values
    }
}

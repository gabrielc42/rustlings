use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr, //if name of struct variable is same as fn param, can just pass this. other wise addr: addr
        }
    }

    pub fn run(self) {
        //self is like 'this' of java
        println!("Listening on {}", self.addr);
        
        let listener = TcpListener::bind(&self.addr);
    }
}
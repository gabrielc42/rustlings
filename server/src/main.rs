fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr //if name of struct variable is same as fn param, can just pass this. other wise addr: addr
        }
    }

    fn run(self) { //self is like 'this' of java
        println!("Listening on {}", self.addr);
    }
}
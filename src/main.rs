fn main() {
    let server = Server::new("127.0.0.1:8070".to_string());
    //listen connection ✨
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        print!("listening on {}", self.addr)
    }
}

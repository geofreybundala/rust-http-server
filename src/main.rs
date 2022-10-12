use server::Server;
use http::request::Request;

fn main() {
    let server = Server::new("127.0.0.1:8070".to_string());
    //listen connection ✨
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }

        pub fn run(self) {
            print!("listening on {}", self.addr)
        }
    }
}

mod http {
    pub mod request {
        use super::method::Method;
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: Method,
        }
    }

    mod method {
        pub enum Method {
            GET,
            PUT,
            POST,
            DELETE,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}




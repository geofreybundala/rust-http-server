use http::request::Request;
use server::Server;

mod server;

mod http;
fn main() {
    let server = Server::new("127.0.0.1:8070".to_string());
    //listen connection ✨
    server.run();
}








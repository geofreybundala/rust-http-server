use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request: {}",String::from_utf8_lossy(&buffer));
                        },
                        Err(e) => print!("failed to read connection {}", e)
                    }
                },
                Err(e) => println!("Failed to establish connection{}",e)
                
            }
        }
    }
}
use crate::http::Request;
use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}

// impl에는 pub 불필요
impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        // run method get ownership
        println!("listen on : {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("received a req: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    
                                },
                                Err(e) => println!("fail to parse a req {}", e),
                            }
                        },
                        Err(e) => {
                            println!("fail to read from connection: {}", e);
                        },
                    }
                },
                Err(e) => {
                    println!("fail to establish a conn : {}", e);
                },
            }
        }
    }
}
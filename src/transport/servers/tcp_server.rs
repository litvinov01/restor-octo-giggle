use std::{io::BufReader, net::{TcpListener, TcpStream}};

use crate::transport::servers::server_interface::Server;

trait TcpServer {
    fn handle_client(&self, stream: TcpStream);

    fn listen(&self, host: String) -> std::io::Result<()>;
}

impl TcpServer for dyn Server {
    fn handle_client(&self, stream: TcpStream) {
        let buf_reader = BufReader::new(&stream);
    }

    fn listen(&self, host: String) -> std::io::Result<()> {
        let listener = match TcpListener::bind(host) {
            Err(e) => panic!("host is invalid"),
            Ok(l) => l,
        };

        for stream in listener.incoming() {
            match stream {
                Ok(s) => self.handle_client(s),
                Err(e) => eprintln!("Connection failed: {}", e),
            }        
        }

        Ok(())
    }
}
    
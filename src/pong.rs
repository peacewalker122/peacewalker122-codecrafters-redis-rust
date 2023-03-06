use std::io::{Read, Write};
use std::net::{TcpStream};

pub(crate) fn handle_ping(mut stream: TcpStream) {
    let mut buffer = [0; 10];
    stream.read(&mut buffer).unwrap();

    let request_string = String::from_utf8_lossy(&buffer[..]);

    println!("request_string: {:?}", request_string);

        let response = b"+PONG\r\n";
        stream.write(response).unwrap();


    stream.flush().unwrap();
}

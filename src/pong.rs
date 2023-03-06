use std::io::{Read, Write};
use std::net::{TcpStream};

pub(crate) fn handle_ping(mut stream: TcpStream) {
    let mut buffer = [0; 10];
    stream.read(&mut buffer).unwrap();

    let request_string = String::from_utf8_lossy(&buffer[..]);

    println!("request_string: {:?}", request_string);

    if request_string == ""{
        let response = b"NIL VALuE\r";
        stream.write(response).unwrap();
    }
    else if request_string == "PING\r\n" {
        let response = b"PONG\r";
        stream.write(response).unwrap();
    }
    else {
        stream.write(request_string.as_bytes()).unwrap();
    }

    stream.flush().unwrap();
}

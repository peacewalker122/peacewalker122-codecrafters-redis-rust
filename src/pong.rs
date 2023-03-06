use std::ffi::OsString;
use std::io::{Read, read_to_string, Write};
use std::net::{TcpStream};

pub(crate) fn handle_ping(mut stream: TcpStream) {
    let mut buffer = [0; 10];
    stream.read(&mut buffer).unwrap();

    let request_String = String::from_utf8_lossy(&buffer[..]);

    println!("request_String: {:?}", request_String);

    if request_String == ""{
        let response = b"NIL VALuE\r";
        stream.write(response).unwrap();
    }
    else if request_String == "PING\r\n" {
        let response = b"PONG\r";
        stream.write(response).unwrap();
    }
    else {
        stream.write(request_String.as_bytes()).unwrap();
    }

    stream.flush().unwrap();
}

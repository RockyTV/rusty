use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::str;

static mut SERVER: &'static str = "";

fn main() {
    let mut stream = TcpStream::connect("irc.esper.net:6667").unwrap();
    
    loop {
        let mut buffer: [u8; 128] = [0; 128];
        stream.read(&mut buffer[..]);
        parse_response(&mut buffer[..]);
    }
}

fn parse_response(buffer: &mut [u8]) {
    // sample message:
    // :availo.esper.net 401 test_nick :No such nick/channel
    let mut message = match str::from_utf8(buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    if message != "" {
        // Strip newline from the message
        message = &message.replace("\n", "");
        let parts: Vec<&str> = message.split_whitespace().collect();
        
        let server_prefix: &str = parts[0];
        unsafe {
            if SERVER == "" {
                SERVER = server_prefix;
            }
        }
    }
}

fn send_raw_message(stream: &mut TcpStream, message: &str) -> Result<(), io::Error> {
    let mut actual_message = String::new();
    actual_message.push_str(message);
    actual_message.push_str("\r\n");
    try!(stream.write(&actual_message.as_bytes()));
    Ok(())
}

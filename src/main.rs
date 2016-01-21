use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::str;
use std::str::FromStr;

struct PingData {
    server: String
}

struct IrcMessage {
    raw_message: String,
    prefix: String,
    command: String,
    params: Vec<String>
}

impl IrcMessage {
    fn new(raw_msg: &str) -> IrcMessage {
        let mut raw: &str = raw_msg;
        let mut new_msg = IrcMessage {
            raw_message: String::new(),
            prefix: String::new(),
            command: String::new(),
            params: Vec::with_capacity(15)
        };

        new_msg.raw_message = String::from_str(raw).unwrap();
        if raw.starts_with(":") {
            let first_whitespace: u32 = match raw.find(' ') {
                Some(x) => x as u32,
                None => 0u32
            };
            new_msg.prefix = String::from_str(raw.substr(1, first_whitespace - 1)).unwrap();
            raw = raw.substr(first_whitespace + 1, raw.len() as u32 - (first_whitespace+1));
        }
        
        if raw.contains(' ') {
            let space_index = match raw.find(' ') {
                Some(x) => x as u32,
                None => 0u32,
            };
            new_msg.command = String::from(raw.substr(0, space_index));
            raw = raw.substr(space_index + 1, raw.len() as u32 - (space_index + 1));

            // Parse parameters
            let mut parameters: Vec<String> = Vec::new();
            while raw != "" {
                if raw.starts_with(":") { 
                    parameters.push(String::from(raw.substr(1, raw.len() as u32 - 1)));
                    break;
                }

                if !raw.contains(' ') {
                    parameters.push(String::from(raw));
                    raw = "";
                    break;
                }
                let space_index = match raw.find(' ') {
                    Some(x) => x as u32,
                    None => 0u32
                };
                parameters.push(String::from(raw.substr(0, space_index)));
                raw = raw.substr(space_index + 1, raw.len() as u32 - (space_index + 1));
            }
            new_msg.params = parameters;
        }

        return new_msg;
    }
}

trait Substring {
    fn substr(&self, start_index: u32, length: u32) -> &str;
}

impl Substring for str {
    fn substr(&self, start_index: u32, length: u32) -> &str {
        return &self[start_index as usize .. start_index as usize + length as usize];
    }
}

fn main() {
    let mut stream = TcpStream::connect("irc.freenode.net:6667").unwrap();
    let mut buffer: [u8; 512] = [0; 512];
    send_raw_message(&mut stream, "USER rustybot 0 * :rusty IRC bot");
    send_raw_message(&mut stream, "NICK rustybot_1337");

    loop {
        let mut ping_data = PingData {
            server: String::new(),
        };
        let bytes_read: usize = match stream.read(&mut buffer[..]) {
            Ok(x) => x,
            Err(e) => panic!("Couldn't read bytes from stream!"),
        };
        parse_response(&mut buffer[..], &mut ping_data, &mut stream);
    }
}

fn parse_response(buffer: &mut [u8], ping_data: &mut PingData, stream: &mut TcpStream) {
    // sample message:
    // :availo.esper.net 401 test_nick :No such nick/channel
    let mut msg = match str::from_utf8(buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    if msg != "" {
        let mut msg = msg.trim();
        let message = IrcMessage::new(msg);
        let buffer: [u8; 512] = [0; 512];
        println!(">> {}", message.raw_message);

        if message.command == "PING" { 
            let mut ping_msg = String::from("PONG :");
            ping_msg.push_str(&message.params[0]);
            send_raw_message(stream, &ping_msg); 
        }
    }
}
        

fn send_raw_message(stream: &mut TcpStream, message: &str) -> Result<(), io::Error> {
    let mut actual_message = String::new();
    actual_message.push_str(message);
    actual_message.push_str("\r\n");
    println!("<< {}", message);
    try!(stream.write(&actual_message.as_bytes()));
    Ok(())
}

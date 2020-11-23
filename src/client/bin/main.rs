use std::net::TcpStream;
use std::io::{Write, Read};
use http_parser::{HttpParser, HttpParserType};
use httptun::parser;
use base64::decode;
use html5ever::tendril::fmt::Slice;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut aparser = http_parser::HttpParser::new(HttpParserType::Both);
    let mut cb = parser::HttpCallback::default();
    let get =b"GET / HTTP/1.1\r\n";
    stream.write(get);
    let mut buffer: [u8; 4092] = [0;4092];
    stream.read(&mut buffer);
   // println!("{}",String::from_utf8_lossy(&buffer));
    aparser.execute(&mut cb,&buffer);
    let data = cb.data();
    println!("{} Length: {}",String::from_utf8_lossy(base64::decode(data).unwrap().as_slice()),data.len());
}

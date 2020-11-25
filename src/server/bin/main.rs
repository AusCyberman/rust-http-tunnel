extern crate base64;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::fs::File;
use std::io::Read;
use httptun::parser::http::{HtmlData, HttpCallback, HttpMessage};
use httptun::{HTML_DATA, HTTP_SERVER_SIZE, HTTP_CLIENT_SIZE};
use http_parser::{HttpMethod, HttpParser, HttpParserType};
use httptun::transmission::{get_file_as_byte, Packet};
use std::sync::{Arc, Mutex};


fn handle_connection(mut stream: TcpStream,buffer: Arc<Mutex<Vec<u8>>>){
    let mut clientBuffer = [0; HTTP_CLIENT_SIZE];
    let filedat = get_file_as_byte(&String::from("/home/auscyber/main.hs"));
    let newpacket = Packet::new(&filedat);
    let sendData = base64::encode(Vec::from(newpacket));


    let mut html_file = std::fs::File::open("index.html").unwrap();
    let mut html_dat: [u8;HTML_DATA] = [0;HTML_DATA];
    html_file.read(&mut html_dat);




    let contents = format!("<html><head><!--[{}]-->{}",sendData,String::from_utf8_lossy(&html_dat));
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.read(&mut clientBuffer).unwrap();
    //println!("{}",String::from_utf8_lossy(&buffer)) ;
    //println!("{}",filedat);
    let mut parser = HttpParser::new(HttpParserType::Request);
    let mut cb = HttpCallback::default();
    HtmlData::NoData(Vec::new());
    parser.execute(&mut cb,&clientBuffer);
    let client_request = HttpMessage::parse(cb);
   // println!("{}",contents);
    if let HttpMessage::ClientRequest(HttpMethod::Get,_) = client_request {

    if response.len() > HTTP_SERVER_SIZE {
        panic!("INVALID RESPONSE")
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    }else{
        println!("Not get request")
    }

}





fn main() {
    let buffer = Arc::new(Mutex::new(Vec::new()));

    let http_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in http_listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream,)
    }
}


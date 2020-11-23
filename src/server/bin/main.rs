extern crate base64;
use httptun::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::fs::File;
use std::io::Read;
use httptun::parser::HttpCallback;
use http_parser::{HttpParser, HttpParserType};





fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    let filedat = base64::encode(get_file_as_byte(&String::from("/home/auscyber/main.hs")));
    let contents = format!("<html>\n<head><!--[{}]--></head></html>",filedat);
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",contents.len(),contents);
    stream.read(&mut buffer).unwrap();
    println!("{}",String::from_utf8_lossy(&buffer)) ;
    let mut parser = HttpParser::new(HttpParserType::Both);
    let mut cb = HttpCallback::default();
    parser.execute(&mut cb,&buffer);

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get){


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    }else{

    }

}


fn get_file_as_byte(filename: &String) -> Vec<u8> {
    let mut f = match File::open(&filename){
        Ok(x) => x,
        Err(x) => panic!("File not found {}",x)
    };
    let metadata = f.metadata().unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow lmao");


    buffer

}


fn main() {
     
    

    let http_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in http_listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream)
    }
}


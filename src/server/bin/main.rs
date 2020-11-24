extern crate base64;
use httptun::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::fs::File;
use std::io::Read;
use httptun::parser::http::HtmlData;
use http_parser::{HttpParser, HttpParserType};





fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    let filedat = base64::encode(get_file_as_byte(&String::from("/home/auscyber/main.hs")));

    let mut html_file = std::fs::File::open("index.html").unwrap();
    let mut html_dat: [u8;2048] = [0;2048];
    html_file.read(&mut html_dat);
    let contents = format!("<html>\n<head><!--[{}]-->{}",filedat,String::from_utf8_lossy(&html_dat));
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",contents.len(),contents);
    stream.read(&mut buffer).unwrap();
    //println!("{}",String::from_utf8_lossy(&buffer)) ;
    //println!("{}",filedat);
    let mut parser = HttpParser::new(HttpParserType::Both);
    let mut cb = HtmlData::NoData(Vec::new());
    parser.execute(&mut cb,&buffer);
   // println!("{}",contents);
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


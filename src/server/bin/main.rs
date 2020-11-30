extern crate base64;
use http_parser::{HttpMethod, HttpParser, HttpParserCallback, HttpParserType};
use httptun::http::{HttpCallback, HttpMessage};
use httptun::transmission::{get_file_as_byte, Packet};
use httptun::{DATA_PACKET_SIZE, DATA_SIZE, HTML_DATA, HTTP_CLIENT_SIZE, HTTP_SERVER_SIZE};
use std::{error::Error, fs::File};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::ops::Add;
use std::slice::Chunks;
use std::str;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::{
    collections::{BTreeMap, VecDeque},
    io::prelude::*,
};

fn handle_connection(mut stream: TcpStream, chunks: Arc<Mutex<VecDeque<Vec<u8>>>>) -> Result<(),Box<dyn std::error::Error>>{
    //Simple struct declarations to parse data
    let mut clientBuffer: [u8; HTTP_CLIENT_SIZE] = [0; HTTP_CLIENT_SIZE];
    let mut parser: HttpParser = HttpParser::new(HttpParserType::Request);
    let mut callback: HttpCallback = HttpCallback::default();

    stream.read(&mut clientBuffer);
    parser.execute(&mut callback, &clientBuffer);
    println!("{}", String::from_utf8_lossy(&clientBuffer));
    let message = HttpMessage::parse(callback);

    if let HttpMessage::ClientRequest(HttpMethod::Post, Some(x)) = message {
        println!("is get request");
        println!("ack_num {}", x.ack_num);
        let mut packet = match chunks.lock().unwrap().get(x.ack_num as usize) {
            Some(y) => {
                println!("Len: {}", y.len());
                HttpMessage::ServerResponse(200, Some(Packet::from(y.clone())))
            }
            None => {
                println!("could not find packet");
                HttpMessage::ServerResponse(404, Some(Packet::from(Vec::new())))
            }
        };
        let string = packet.create_http_packet(x.ack_num, 0).unwrap();
        println!("length: {}, max length: {}", string.len(), HTTP_SERVER_SIZE);
        stream.write(string.as_slice());
    } else {
        let mut packet = HttpMessage::ServerResponse(404, Some(Packet::from(Vec::new())));

        stream.write(packet.create_http_packet(0, 0).unwrap().as_slice())?;
    }
    Ok(())
}

fn main() -> Result<(),Box<dyn Error>>{
    let filedat: Vec<u8> = get_file_as_byte(&String::from("/home/auscyber/chair.png"));
    let packets: Arc<Mutex<VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(VecDeque::new()));
    for chunk in filedat.chunks(DATA_SIZE).into_iter() {
        
        packets.lock().unwrap().push_front(chunk.to_vec());
    }
    let http_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in http_listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established");
        let packets = packets.clone();
        std::thread::spawn(|| handle_connection(stream, packets).map_err(|e| println!("Connection Failed: {}",e)));
    }
    Ok(())
}

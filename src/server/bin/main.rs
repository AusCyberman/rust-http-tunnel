extern crate base64;
use std::{collections::{BTreeMap, VecDeque}, io::prelude::*};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::fs::File;
use std::io::Read;
use httptun::http::{HttpCallback, HttpMessage};
use httptun::{HTML_DATA, HTTP_SERVER_SIZE, HTTP_CLIENT_SIZE, DATA_PACKET_SIZE, DATA_SIZE};
use http_parser::{HttpMethod, HttpParser, HttpParserCallback, HttpParserType};
use httptun::transmission::{get_file_as_byte, Packet};
use std::sync::{Arc, Mutex};
use std::slice::Chunks;
use std::sync::mpsc::Receiver;
use std::ops::Add;


fn handle_connection(mut stream: TcpStream, chunks: &VecDeque<Vec<u8>>){
    //Simple struct declarations to parse data
    let mut clientBuffer: [u8; HTTP_CLIENT_SIZE] = [0; HTTP_CLIENT_SIZE];
    let mut parser: HttpParser = HttpParser::new(HttpParserType::Request);
    let mut callback: HttpCallback = HttpCallback::default();
    

    stream.read(&mut clientBuffer);
    parser.execute(&mut callback, &clientBuffer);
     
    let message = HttpMessage::parse(callback);

    if let HttpMessage::ClientRequest(HttpMethod::Post,Some(x)) = message{
        println!("is get request");
        println!("ack_num {}",x.ack_num);
         let mut packet = match chunks.get(x.ack_num as usize){
            Some(y) => {
                println!("Len: {}",y.len());
                HttpMessage::ServerResponse(200,Some(Packet::new(y.as_slice())))
            },
            None => {
                println!("could not find packet");
                HttpMessage::ServerResponse(404,Some(Packet::from(Vec::new())))
            }
        };
        let string = packet.create_http_packet(x.ack_num, 0).unwrap();
         println!("length: {}, max length: {}",string.len(),HTTP_SERVER_SIZE);
        stream.write(string.as_slice());
       
    }else{
    let mut packet = HttpMessage::ServerResponse(404,Some(Packet::from(Vec::new())));

        stream.write(packet.create_http_packet(0, 0).unwrap().as_slice());
     
    }




}





fn main() {
    let filedat: Vec<u8> = get_file_as_byte(&String::from("/home/auscyber/chair.png"));
    let mut packets: VecDeque<Vec<u8>> = VecDeque::new();
    for chunk in filedat.chunks(DATA_SIZE).into_iter(){
            println!("index {}, size: {}",packets.len(),chunk.len());
            packets.push_front(chunk.to_vec());
    }
    let http_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in http_listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream,&packets);
    }
}


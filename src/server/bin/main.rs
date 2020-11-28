extern crate base64;
use std::{collections::BTreeMap, io::prelude::*};
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


fn handle_connection(mut stream: TcpStream, chunks: &BTreeMap<usize,&[u8]>){
    //Simple struct declarations to parse data
    let mut clientBuffer: [u8; HTTP_CLIENT_SIZE] = [0; HTTP_CLIENT_SIZE];
    let mut parser: HttpParser = HttpParser::new(HttpParserType::Request);
    let mut callback: HttpCallback = HttpCallback::default();
    

    stream.read(&mut clientBuffer);
    parser.execute(&mut callback, &clientBuffer);
     
    let message = HttpMessage::parse(callback);

    if let HttpMessage::ClientRequest(HttpMethod::Post,Some(x)) = message{
        println!("is get request");
        let data = match chunks.get(&(x.ack_num as usize)){
            Some(x) => x.to_vec(),
            None => Vec::new()
        };
       let mut packet = HttpMessage::ServerResponse(200,Some(Packet::from(data)));
        stream.write(packet.create_http_packet(x.ack_num, 0).unwrap().as_slice());
        
    }else{
        let mut packet = HttpMessage::ServerResponse(200,Some(Packet::from(chunks.get(&(0 as usize)).unwrap().to_vec())));
        stream.write(packet.create_http_packet(0, 0).unwrap().as_slice());
    }




}





fn main() {
    let filedat: Vec<u8> = get_file_as_byte(&String::from("/home/auscyber/main.hs"));
    let packets: BTreeMap<usize,&[u8]> = filedat.chunks(DATA_PACKET_SIZE).enumerate().collect::<BTreeMap<usize,&[u8]>>();

    let http_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in http_listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream,&packets);
    }
}


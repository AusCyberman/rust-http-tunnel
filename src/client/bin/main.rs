use std::net::TcpStream;
use std::io::{Write, Read};
use base64::decode;
use std::io::Error;
use httptun::http::{HttpMessage, HttpCallback};
use httptun::{HTTP_CLIENT_SIZE, HTTP_SERVER_SIZE};
use http_parser::{HttpParser, HttpParserType};
use std::convert::{TryFrom, TryInto};
use httptun::transmission::Packet;






fn main() -> Result<(),Error>{

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
       
    //Data Parsing interface
    let mut aparser = HttpParser::new(HttpParserType::Response);
    let mut cb = HttpCallback::default();

    let mut counter = 0;
    let sendPack = Packet::from(Vec::new());
    let mut message = HttpMessage::ClientRequest(http_parser::HttpMethod::Post,Some(sendPack));
    stream.write(message.create_http_packet(0, 0).unwrap().as_slice());
    let mut buffer: [u8; HTTP_SERVER_SIZE] = [0; HTTP_SERVER_SIZE];
    stream.read(&mut buffer);
    aparser.execute(&mut cb,&mut buffer);

    let mut servPack = HttpMessage::parse(cb);
    if let HttpMessage::ServerResponse(code,Some(data)) = servPack{
         
        println!("seq_num: {} data: {}",data.seq_num,String::from_utf8(data.data).unwrap());
        
    };




    Ok(())

}

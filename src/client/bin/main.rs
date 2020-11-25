use std::net::TcpStream;
use std::io::{Write, Read};
use base64::decode;
use std::io::Error;
use httptun::parser::http::{HtmlData, HttpMessage, HttpCallback};
use httptun::{HTTP_CLIENT_SIZE, HTTP_SERVER_SIZE};
use http_parser::{HttpParser, HttpParserType};
use std::convert::{TryFrom, TryInto};
use httptun::transmission::Packet;

fn main() -> Result<(),Error>{
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut aparser = HttpParser::new(HttpParserType::Response);
    let mut cb = HttpCallback::default();
    let get = b"GET / HTTP/1.0\r\n\
                     Content-Length: 0\r\n\r\n";;


    stream.write(get);
    let mut buffer: [u8; HTTP_SERVER_SIZE] = [0; HTTP_SERVER_SIZE];
    println!("{}",HTTP_SERVER_SIZE);
    stream.read(&mut buffer);
  // println!("{}",String::from_utf8_lossy(&buffer));
    aparser.execute(&mut cb,&mut buffer);
    let resp = HttpMessage::parse(cb);

    let data = match resp {
        HttpMessage::ServerResponse(x,html_data) => html_data,
        _ => HtmlData::NoData(Vec::new())
    };

    match data {
        HtmlData::CompleteData(x) => {

            //println!("{}",String::from_utf8(base64::decode( x.as_slice()).unwrap()).unwrap())
            let x = base64::decode(x).unwrap();
            let y: Packet = Packet::try_from(&x).unwrap_or_else(|_| panic!("lmao"));

            println!("seq_num {}\nseq_length {}\nack_num {}",y.seq_num,y.seq_length,y.ack_num);

            let dat = y.data;
            if dat.len() > y.seq_length as usize{
                println!("Invalid size");
                return Ok(());
            }
            //print!("{}",String::from_utf8_lossy(dat));
            let mut f = std::fs::File::create("main.hs").unwrap();
            f.write(dat).unwrap();
            f.flush().unwrap();
        },
        HtmlData::ValidData(x,y) => println!("Incomplete"),
        _ => println!("rip")
    }
    Ok(())
}

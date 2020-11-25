use std::net::TcpStream;
use std::io::{Write, Read};
use base64::decode;
use std::io::Error;
use httptun::parser::http::{HtmlData, HttpMessage, HttpCallback};
use httptun::{HTTP_CLIENT_SIZE, HTTP_SERVER_SIZE};
use http_parser::{HttpParser, HttpParserType};

fn main() -> Result<(),Error>{
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut aparser = HttpParser::new(HttpParserType::Response);
    let mut cb = HttpCallback::default();
    let get = b"GET / HTTP/1.0\r\n\
                     Content-Length: 0\r\n\r\n";;


    stream.write(get);
    let mut buffer: [u8; HTTP_SERVER_SIZE] = [0; HTTP_SERVER_SIZE];
    stream.read(&mut buffer);
  // println!("{}",String::from_utf8_lossy(&buffer));
    aparser.execute(&mut cb,&mut buffer);
    let resp = HttpMessage::parse(cb);

    let data = match resp {
        HttpMessage::ServerResponse(x,HtmlData) => HtmlData,
        _ => HtmlData::NoData(Vec::new())
    };

    match data {
        HtmlData::CompleteData(x) => {

            //println!("{}",String::from_utf8(base64::decode( x.as_slice()).unwrap()).unwrap())
        },

        _ => println!("rip")
    }
    //println!("{}",String::from_utf8(base64::decode( data.as_slice()).unwrap()).unwrap());
    //let mut handle = std::fs::File::create("bob.hs").unwrap();
    //handle.write(base64::decode(data.as_slice()).unwrap().as_slice());
    Ok(())
    //println!("{} Length: {}",String::from_utf8_lossy(base64::decode(data).unwrap().as_slice()),data.len());
}

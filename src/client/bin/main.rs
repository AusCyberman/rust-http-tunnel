use std::net::TcpStream;
use std::io::{Write, Read};
use http_parser::{HttpParser, HttpParserType};
use base64::decode;
use std::io::Error;
use httptun::parser::http::HtmlData;


fn main() -> Result<(),Error>{
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut aparser = http_parser::HttpParser::new(HttpParserType::Both);
    let mut cb = HtmlData::NoData(Vec::new());
    let get =b"GET / HTTP/1.1\r\n";
    stream.write(get);
    let mut buffer: [u8; 1000000] = [0; 1000000];
    stream.read(&mut buffer);
   // println!("{}",String::from_utf8_lossy(&buffer));
    aparser.execute(&mut cb,&buffer);
    let data = match cb{
        HtmlData::CompleteData(x) => x,
        HtmlData::ValidData(_,_) => {
            println!("Incomplete Data");
            Vec::new()
        }
        _ => Vec::new()
    };
    //println!("{}",String::from_utf8(base64::decode( data.as_slice()).unwrap()).unwrap());
    let mut handle = std::fs::File::create("bob.hs").unwrap();
    handle.write(base64::decode(data.as_slice()).unwrap().as_slice());
    Ok(())
    //println!("{} Length: {}",String::from_utf8_lossy(base64::decode(data).unwrap().as_slice()),data.len());
}

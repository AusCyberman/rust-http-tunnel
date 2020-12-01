use std::net::TcpStream;
use std::io::{Read,Write};
use std::fs::File;
fn main(){
    let mut stream = TcpStream::connect("SKWebProxy1:3128").unwrap();
    let get = b"GET / HTTP/1.0\r\nHost: www.google.com.au\r\nConnection: keep-alive\r\nAccept: text/html\r\nUser-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:83.0) Gecko/20100101 Firefox/83.0\r\n\r\n";
    println!("{}",String::from_utf8_lossy(get));

    stream.write(get);
    let mut buffer: [u8; 2048] = [0;2048];
    stream.read(&mut buffer);
    //println!("{}",String::from_utf8_lossy(&buffer));
    let mut file = File::create("google.html").unwrap();
    file.write(&buffer);
    println!("{}",String::from_utf8_lossy(&buffer));
}
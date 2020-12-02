use anyhow::Result;
use http_parser::{HttpParser, HttpParserType};
use httptun::transmission::Packet;
use httptun::{
    http::{HttpCallback, HttpMessage},
    transmission::get_file_as_byte,
    DATA_SIZE,
};
use httptun::{HTTP_CLIENT_SIZE, HTTP_SERVER_SIZE};
use std::error::Error;
use std::io::{Read, Write};
use std::{collections::VecDeque, net::TcpStream, println};

fn end_of_data(buffer: VecDeque<Packet>) -> Result<()> {
    let mut file = std::fs::File::create("bob.png")?;
    for packet in buffer {
        file.write(packet.data.as_slice())?;
    }
    Ok(())
}

fn main() -> Result<()> {
    //Data Parsing interface
    let input = get_file_as_byte(&String::from("main.hs"));
    let outp_data_buffer: VecDeque<&[u8]> = input.chunks(DATA_SIZE).collect::<VecDeque<&[u8]>>();
    let mut inp_data_buffer: VecDeque<Packet> = VecDeque::new();
    let mut counter: u32 = 0;
    let mut seq_num: u32 = 0;
    let mut localStream = TcpStream::connect("localhost:22");
    loop {
        let mut aparser = HttpParser::new(HttpParserType::Response);
        let mut stream = match TcpStream::connect("127.0.0.1:7878") {
            Ok(x) => x,
            Err(e) => panic!("Cannot Connect to server: {}", e),
        };
        let mut send_pack = match outp_data_buffer.get(seq_num as usize) {
            Some(x) => {
                let mut pack = Packet::from(x.to_vec());
                pack.seq_num = seq_num;
                pack
            }
            None => break,
        };
        //Set required packet to next packet in sequence
        send_pack.ack_num = counter;
        println!("Next Pack: {}", counter);
        let (send_ack, send_seq) = (send_pack.ack_num, send_pack.seq_num);

        //make post request
        let mut message =
            HttpMessage::ClientRequest(http_parser::HttpMethod::Post, Some(send_pack.clone()));
        stream.write(
            message
                .create_http_packet(send_seq, send_ack)
                .unwrap()
                .as_slice(),
        )?;
        let mut server_buffer: [u8; HTTP_SERVER_SIZE] = [0; HTTP_SERVER_SIZE];
        stream.read(&mut server_buffer).unwrap();
        //callback for http data
        let mut cb = HttpCallback::default();
        aparser.execute(&mut cb, &mut server_buffer);

        let serv_message = HttpMessage::parse(cb);
        if let HttpMessage::ServerResponse(code, Some(packet)) = serv_message {
            println!("code :{}", code);
            if code == 404 {
                println!("is 404");
                break;
            } else {
                println!("Got packet {}, send_ack: {}", packet.data.len(), send_ack);
                if packet.seq_num != send_ack {
                    continue;
                }
                counter = counter + 1;
                seq_num = packet.ack_num;
                inp_data_buffer.push_front(packet);
            }
        };
    }
    end_of_data(inp_data_buffer)?;

    Ok(())
}

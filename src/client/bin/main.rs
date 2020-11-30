use std::{ collections::VecDeque, net::TcpStream, println};
use std::io::{Write, Read};
use std::sync::{ Arc,Mutex};
use httptun::{DATA_SIZE, http::{HttpMessage, HttpCallback}, transmission::get_file_as_byte};
use httptun::{HTTP_CLIENT_SIZE, HTTP_SERVER_SIZE};
use http_parser::{HttpParser, HttpParserType};
use httptun::transmission::Packet;
use std::error::Error;



fn end_of_data(buffer: VecDeque<Packet>) -> Result<(),Box<dyn Error>>{
            let mut file = std::fs::File::create("bob.png")?;
       for packet in buffer{
            file.write(packet.data.as_slice())?;
       }
       Ok(())

}


fn main() -> Result<(),Box<dyn Error>>{

       
    //Data Parsing interface
    let input = get_file_as_byte(&String::from("main.hs"));
    let outp_data_buffer: VecDeque<&[u8]> = input.chunks(DATA_SIZE).collect::<VecDeque<&[u8]>>();
    let mut inp_data_buffer: VecDeque<Packet> = VecDeque::new();
    let mut counter: u32 = 0;
    let mut seq_num: u32 = 0;


    loop{

    let mut aparser = HttpParser::new(HttpParserType::Response);
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        let mut send_pack = match outp_data_buffer.get(seq_num as usize){
            Some(x) => {let mut pack = Packet::from(x.to_vec());
                    pack.seq_num = seq_num;
                    pack
            },
            None => break
        };
        //Set required packet to next packet in sequence
        send_pack.ack_num = counter;
        println!("Next Pack: {}",counter) ;
        let (send_ack,send_seq) = (send_pack.ack_num,send_pack.seq_num);


        //make post request
        let mut message = HttpMessage::ClientRequest(http_parser::HttpMethod::Post,Some(send_pack.clone()));
        if let Err(x) = stream.write(message.create_http_packet(send_seq, send_ack).unwrap().as_slice()){
            return Err(x.into());
        }
        let mut buffer: [u8; HTTP_SERVER_SIZE] = [0; HTTP_SERVER_SIZE];
        stream.read(&mut buffer).unwrap();
        //callback for http data
        let mut cb = HttpCallback::default();
        aparser.execute(&mut cb,&mut buffer);
 
        let serv_message = HttpMessage::parse(cb);
        if let HttpMessage::ServerResponse(code,Some(packet)) = serv_message{     
            println!("code :{}",code);
            if let 404 = code{
                println!("is 206");

                break;
            }else{
                println!("Got packet {}, send_ack: {}",packet.data.len(),send_ack);
                if packet.seq_num != send_ack{
                    continue;
                }
                counter=counter+1;
                seq_num = packet.ack_num;
                inp_data_buffer.push_front(packet);
            }
        };

        }
    end_of_data(inp_data_buffer)?;



    Ok(())

}

extern crate http_parser;
//Parsing
pub const DATA_PACKET_SIZE: usize = DATA_SIZE+12;
pub const HEADER_SIZE: usize = 1000;
pub const HTML_DATA: usize = 1000;
pub const METADATA_SIZE: usize = 12;
pub const DATA_SIZE: usize = 50000;
pub const HTTP_SERVER_SIZE: usize = DATA_PACKET_SIZE+HTML_DATA+HEADER_SIZE;
pub const HTTP_CLIENT_SIZE: usize = DATA_PACKET_SIZE+HEADER_SIZE;




    ///Http Translation
    pub mod http{
        use http_parser::{ParseAction,HttpMethod,HttpParserCallback,HttpParser,CallbackResult};
        use crate::transmission::Packet;
        use std::convert::TryFrom;
        pub struct HttpCallback{
            pub http_method: Option<HttpMethod>,
            pub status_code: Option<u16>,
            pub data: Option<Vec<u8>>
        }
        /// Includes data for packets
        pub enum HttpMessage{
            ///Server Response packet includes status_code and HtmlData
            ServerResponse(u16,Option<Packet>),
            ClientRequest(HttpMethod,Option<Packet>),
            EmptyMessage
        }

        type HtmlData = (Option<Vec<u8>>,Option<Vec<u8>>);
        fn parse_html(htmldat: HtmlData,dat: &u8) -> HtmlData{
        
                let start_char = b"<!--[";
                let end_char = b"]-->";
            
               match htmldat {
                   (Some(mut x),Some(mut y)) => {
                       return if start_char.contains(dat) || end_char.contains(dat) {
                           y.push(*dat);

                           if y.len() > 0 && &end_char[..y.len()] != y.as_slice() {
                               return (Some(x), Some(Vec::new()));
                           }
                           if y.as_slice() == &end_char[..] {
                               return (Some(x),None);
                           } else if y.as_slice() == &start_char[..] {
                               return (None,None);
                           }
                           //println!("end {}",String::from_utf8_lossy(&[*dat]));
                            (Some(x), Some(y) )
                       } else {
                           x.push(*dat);

                           (Some(x), Some(Vec::new()))
                       };
                },
                   (None,Some(mut y))=>{
                       y.push(*dat);
                       if y.len() > 0 && &start_char[..y.len()] != y.as_slice()  {
                           return (None,Some(Vec::new()));
                       }
                       if y.as_slice() == &start_char[..] {
                           return (Some(Vec::new()),Some(Vec::new()));
                       }
                       //println!("{}",String::from_utf8_lossy(&[*dat]));
                      return (None, Some(y));
                   },
                   y => y

            }
        }
    impl HttpCallback{
        pub fn default() -> HttpCallback{
            HttpCallback{
                http_method: None,
                status_code:None,
                data:None
            }
        }
    }
    //Parse the http input and put it into the struct HttpCallback
    impl HttpParserCallback for HttpCallback{
        fn on_body(&mut self,parser: &mut HttpParser,data: &[u8]) -> CallbackResult{
            self.data = Some(Vec::from(data));

            Ok(ParseAction::None)
        }
        fn on_status(&mut self,parser: &mut HttpParser, status: &[u8]) -> CallbackResult{
            Ok(ParseAction::None)
        }
        fn on_message_complete(&mut self, parser: &mut HttpParser) -> CallbackResult{
           // println!("{}",parser.status_code.unwrap());

            if let Some(x) = parser.method{
                println!("METHOD: {}",parser.method.unwrap().to_string());
                self.http_method = Some(x);
            }else if let Some(x) = parser.status_code{
                self.status_code = Some(x);
                println!("{}",parser.status_code.unwrap());
            }
            println!("Message Begin");


            Ok(ParseAction::None)
        }

    }
    impl HttpMessage{
        pub fn parse(callback: HttpCallback) -> HttpMessage{

            if let Some(method) = (&callback).http_method{
                    if let  all@HttpMethod::Post | all@HttpMethod::Get = method {
                        return HttpMessage::ClientRequest(all,None);
                    } else {
                        return HttpMessage::ClientRequest(callback.http_method.unwrap(), None)
                    }
                    }else if let Some(status_code) = callback.status_code{
                        if let (Some(x),None) = callback.data.unwrap().iter().fold((None,Some(Vec::new())),parse_html){


                            println!("{}",status_code);
                            return HttpMessage::ServerResponse(status_code,Some(Packet::parse_vec(base64::decode(x).unwrap()).unwrap()));

                        }else{
                            return HttpMessage::EmptyMessage;

                        }

                    }
                    println!("None");

            HttpMessage::EmptyMessage
                
        }
pub fn create_http_packet(&mut self,ack_num: u32,seq_num:u32) -> Option<Vec<u8>>{
            match self {
                //If input packet is a ServerResponse Packet, parse it and return the Vec
                //containing valid data
                HttpMessage::ServerResponse(resp,data) =>{
                        if let Some(dat) = data{

                        let contents = format!("<html><head><!--[{}]-->{}",base64::encode(dat.create_packet(seq_num,ack_num)),"</head></html>");
                        let response = format!(
                                "HTTP/1.1 {} OK\r\nContent-Length: {2}\r\n\r\n{1}",
                                resp,
                                contents,
                                contents.len()
                            );



                        return Some(response.as_bytes().to_vec());

                        }
                        None

                },
                HttpMessage::ClientRequest(met,data) =>{
                    match met{
                    HttpMethod::Post => {
                        if let Some(dat) = data{
                        let post = format!("POST / HTTP/1.0\r\n\
                                     Content-Length: {1}\r\n\r\n{0}",base64::encode(dat.create_packet(seq_num, ack_num)),dat.seq_length);
                        return Some(post.as_bytes().to_vec());

                        }else{
                            None
                        }
                    },
                    HttpMethod::Get =>{
                        let get = b"GET / HTTP/1.0\r\nContent-Length: 0\r\n\r\n";
                               return Some(get.to_vec());

                    },
                    _ =>{
                        return None;
                    }
                    }
                },
                _ => None
            }

        }

    }

        

    }




pub mod transmission{
    use std::fs::File;
    use std::io::Read;
    use std::convert::TryFrom;
    ///Packet Struct that includes Sequence length,
    /// sequence number, acknowledgement_number and the data itself
    #[derive( Clone)]
    pub struct Packet {
        ///Sequence Number
        pub seq_num: u32,
        ///Acknowledgement Number
        pub ack_num: u32,
        ///Sequence Length
        pub seq_length: u32,
        pub data: Vec<u8>
    }

    pub fn unpacku32(num: &u32) -> [u8; 4]{
        num.to_be_bytes()
    }
    pub fn packu32(arr: &[u8]) -> u32{

        u32::from_be_bytes(<[u8; 4]>::try_from(arr).unwrap())
    }

    impl Packet{
        pub fn new(data: &[u8]) -> Packet{
                Packet{
                    seq_length: u32::try_from(data.len()).unwrap(),
                    seq_num: 0,
                    ack_num: 0,
                    data: data.to_vec()
                }
        }



       pub fn create_packet(&mut self,seq: u32,ack: u32) -> Vec<u8>{
            self.seq_num = seq;
            self.ack_num = ack;
            let mut vec = Vec::new();
            vec.append(&mut unpacku32(&self.seq_num).to_vec());
            vec.append(&mut unpacku32(&self.ack_num).to_vec());
            vec.append(&mut unpacku32(&self.seq_length).to_vec());
            vec.append(&mut self.data.to_vec());
            vec

        }

    }
    impl From<Vec<u8>> for Packet{
        fn from(value: Vec<u8>) -> Packet{
            Packet{
                seq_num: 0,
                ack_num: 0,
                seq_length: u32::try_from(value.len()).unwrap(),
                data: value

            }
            
        }

    }
   impl Packet{
        pub fn parse_vec(value: Vec<u8>) -> Result<Self,()> {
          Ok(Packet{
                seq_num: packu32(&value[0..4]),
                ack_num: packu32(&value[4..8]),
                seq_length: packu32(&value[8..12]),
                data: Vec::from(&value[12..])
            })  
        }
   }


          

    pub fn get_file_as_byte(filename: &String) -> Vec<u8> {
        let mut f = match File::open(&filename){
            Ok(x) => x,
            Err(x) => panic!("File not found {}",x)
        };

        let metadata = f.metadata().unwrap();
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow lmao");
        buffer

    }


}










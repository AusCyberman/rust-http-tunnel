extern crate http_parser;
//Parsing
pub const DATA_PACKET_SIZE: usize = 50000;
pub const HEADER_SIZE: usize = 1000;
pub const HTML_DATA: usize = 1000;
pub const METADATA_SIZE: usize = 12;
pub const HTTP_SERVER_SIZE: usize = DATA_PACKET_SIZE+HTML_DATA+HEADER_SIZE;
pub const HTTP_CLIENT_SIZE: usize = DATA_PACKET_SIZE+HEADER_SIZE;



pub mod parser{

    ///Http Translation
    pub mod http{
        use std::ops::DerefMut;
        use http_parser::{HttpParserCallback, CallbackResult, ParseAction, HttpParser, HttpMethod};
        use crate::parser::http::HtmlData::NoData;
        use std::convert::TryFrom;

        pub struct HttpCallback{
            pub http_method: Option<HttpMethod>,
            pub status_code: Option<u16>,
            pub data: Option<Vec<u8>>
        }

        #[derive(Clone)]
        pub enum HtmlData{
        ValidData(Vec<u8>,Vec<u8>),
        CompleteData(Vec<u8>),
        NoData(Vec<u8>),
        InvalidData
        }
        pub enum HttpMessage{
            ServerResponse(u16,HtmlData),
            ClientRequest(HttpMethod,HtmlData),
            EmptyMessage
        }

    fn parseHtml(htmldat: HtmlData,dat: &u8) -> HtmlData{
        let start_char = b"<!--[";
        let end_char = b"]-->";
        //InData Completed

       match htmldat {
           HtmlData::ValidData(mut x,mut y) => {
               return if start_char.contains(dat) || end_char.contains(dat) {
                   y.push(*dat);

                   if y.len() > 0 && &end_char[..y.len()] != y.as_slice() {
                       return HtmlData::ValidData(x, Vec::new());
                   }
                   if y.as_slice() == &end_char[..] {
                       return HtmlData::CompleteData(x);
                   } else if y.as_slice() == &start_char[..] {
                       return HtmlData::InvalidData;
                   }
                   //println!("end {}",String::from_utf8_lossy(&[*dat]));
                   HtmlData::ValidData(x, y, )
               } else {
                   x.push(*dat);

                   HtmlData::ValidData(x, Vec::new())
               };
        },
           HtmlData::NoData(mut y)=>{
               y.push(*dat);
               if y.len() > 0 && &start_char[..y.len()] != y.as_slice()  {
                   return HtmlData::NoData(Vec::new());
               }
               if y.as_slice() == &start_char[..] {
                   return HtmlData::ValidData(Vec::new(),Vec::new());
               }
               //println!("{}",String::from_utf8_lossy(&[*dat]));
              return HtmlData::NoData(y);
           },
           y => y

    }}
    impl HttpCallback{
        pub fn default() -> HttpCallback{
            HttpCallback{
                http_method: None,
                status_code:None,
                data:None
            }
        }
    }
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

            if let Some(method) = callback.http_method{
                    if let  all@HttpMethod::Post | all@HttpMethod::Get = method {
                        //return HttpMessage::ClientRequest(all, HtmlData::CompleteData(Vec::from(base64::decode(callback.data.unwrap()).unwrap())));
                        return HttpMessage::ClientRequest(all,NoData(Vec::new()));
                    } else {
                        return HttpMessage::ClientRequest(callback.http_method.unwrap(), HtmlData::NoData(Vec::new()))
                    }
                    }else if let Some(status_code) = callback.status_code{
                                return HttpMessage::ServerResponse(status_code,callback.data.unwrap().iter().fold(HtmlData::NoData(Vec::new()),parseHtml));

                    }
                    println!("None");








            HttpMessage::EmptyMessage
        }}}


}

pub mod transmission{
    use crate::{DATA_PACKET_SIZE};
    use std::fs::File;
    use std::io::Read;
    use std::convert::TryFrom;
    use std::fmt::{Display, Formatter};
    use crate::parser::http::HtmlData;

    pub struct Packet<'a> {
        pub seq_length: u32,
        pub seq_num: u32,
        pub ack_num: u32,
        pub data: &'a [u8]
    }

    pub fn unpacku32(num: &u32) -> [u8; 4]{
        num.to_be_bytes()
    }
    pub fn packu32(arr: &[u8]) -> u32{

        u32::from_be_bytes(<[u8; 4]>::try_from(arr).unwrap())
    }


    impl<'a> Packet<'a>{
        pub fn new(data: &'a Vec<u8>) -> Packet<'a>{
                Packet{
                    seq_length: u32::try_from(data.len()).unwrap(),
                    seq_num: 10,
                    ack_num: 10,
                    data: data.as_slice()
                }
        }

    }

    impl<'T> TryFrom<&'T Vec<u8>> for Packet<'T>{
        type Error = HtmlData;

        fn try_from(value: &'T Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Packet{
                seq_num: packu32(&value[0..4]),
                ack_num: packu32(&value[4..8]),
                seq_length: packu32(&value[8..12]),
                data: &value[12..]
            })
        }
    }

    impl<'a> From<Packet<'a>> for Vec<u8>{
        fn from(p: Packet<'a>) -> Self {
            let mut vec = Vec::new();
            vec.append(&mut unpacku32(&p.seq_num).to_vec());
            vec.append(&mut unpacku32(&p.ack_num).to_vec());
            vec.append(&mut unpacku32(&p.seq_length).to_vec());
            vec.append(&mut p.data.to_vec());
            vec
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



pub struct ThreadPool;







extern crate http_parser;
use http_parser::{HttpParserCallback, CallbackResult, HttpParser, ParseAction};
//Parsing

pub mod parser{
    pub mod http{
    use http_parser::{HttpParserCallback, HttpParser, CallbackResult, ParseAction, HttpMethod};
        use std::ops::DerefMut;


        #[derive(Clone)]
        pub enum HtmlData{
        ValidData(Vec<u8>,Vec<u8>),
        CompleteData(Vec<u8>),
        NoData(Vec<u8>),
        InvalidData
    }
    fn parseHtml(htmldat: HtmlData,dat: &u8) -> HtmlData{
        let startChar = b"<!--[";
        let endChar = b"]-->";
        //InData Completed

       match htmldat {
           HtmlData::ValidData(mut x,mut y) => {
               return if startChar.contains(dat) || endChar.contains(dat) {
                   y.push(*dat);

                   if y.len() > 0 && &endChar[..y.len()] != y.as_slice() {
                       return HtmlData::ValidData(x, Vec::new());
                   }
                   if y.as_slice() == &endChar[..] {
                       return HtmlData::CompleteData(x);
                   } else if y.as_slice() == &startChar[..] {
                       return HtmlData::InvalidData;
                   }
                   //println!("end {}",String::from_utf8_lossy(&[*dat]));
                   HtmlData::ValidData(x, y, )
               } else {
                   x.push(*dat);

                   HtmlData::ValidData(x, Vec::new())
               };
            return HtmlData::ValidData(x,Vec::new())
        },
           HtmlData::NoData(mut y)=>{
               y.push(*dat);
               if y.len() > 0 && &startChar[..y.len()] != y.as_slice()  {
                   return HtmlData::NoData(Vec::new());
               }
               if y.as_slice() == &startChar[..] {
                   return HtmlData::ValidData(Vec::new(),Vec::new());
               }
               println!("{}",String::from_utf8_lossy(&[*dat]));
              return HtmlData::NoData(y);
           },
           y => y

    }}

    impl HttpParserCallback for HtmlData{
        fn on_message_begin(&mut self, parser: &mut HttpParser) -> CallbackResult {
            println!("Message Begin");
            Ok(ParseAction::None)
        }
        fn on_body(&mut self,parser: &mut HttpParser,data: &[u8]) -> CallbackResult{

            if let Some(HttpMethod::Post) | Some(HttpMethod::Put) = parser.method{
                *self = HtmlData::CompleteData(Vec::from(base64::decode(data).unwrap()));
            }else if let Some(x) = parser.status_code{
                println!("Status Code: {}", x);
                *self = data.iter().fold(HtmlData::NoData(Vec::new()),parseHtml);
            }

           Ok(ParseAction::None)
        }
    }}

    pub mod data{
        p
    }

}





pub struct ThreadPool;







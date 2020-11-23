extern crate http_parser;
use http_parser::{HttpParserCallback, CallbackResult, HttpParser, ParseAction};
//Parsing

pub mod parser{

    use http_parser::{HttpParserCallback, HttpParser, CallbackResult, ParseAction, HttpMethod};

    pub struct HttpCallback{
        data: Vec<u8>
    }
    struct HtmlData(Option<Vec<u8>>,Vec<u8>,bool,bool);

    fn parseHtml(htmldat: Option<HtmlData>,dat: &u8) -> Option<HtmlData>{
        let startChar = b"<!--[";
        let endChar = b"]-->";
                                                        //InData Completed
        if let Some(HtmlData(Some(mut x),mut y,z,false)) = htmldat{
            //println!("Found other chars: {}",String::from_utf8_lossy(&[*dat]));
               if startChar.contains(dat) || endChar.contains(dat){

                   y.push(*dat);

                  if !z && y.len() > 0 && &startChar[..y.len()] != y.as_slice()  {
                      return Some(HtmlData(Some(x),Vec::new(),z,false));
                  }


                    //println!("{} {} current:{}",String::from_utf8_lossy(&y),String::from_utf8_lossy(&endChar[..]),String::from_utf8_lossy(&[*dat]));
                   if y.as_slice() == &endChar[..]{
                     //  println!("it is");
                       if z{
                           return Some(HtmlData(Some(x),y,false,true));
                       }
                       return Some(HtmlData(Some(x),y,z,false));

                   }else if y.as_slice() == &startChar[..] {
                        if(z){
                            return None;
                        }
                       return Some(HtmlData(Some(Vec::new()),Vec::new(),true,false));

                   }
                   //println!("end {}",String::from_utf8_lossy(&[*dat]));
                   return Some(HtmlData(Some(x),y,z,false));
               }else if z {
                   x.push(*dat);

                   return Some(HtmlData(Some(x), Vec::new(), z, false));
               }
            return Some(HtmlData(Some(Vec::new()),y,z,false))
        }else{
            htmldat
        }

    }
    impl HttpCallback{
        pub fn default() -> HttpCallback{
            HttpCallback{ data: Vec::new()}
        }
        pub fn data(&mut self) -> &Vec<u8>{
            &self.data
        }
    }
    impl HttpParserCallback for HttpCallback{
        fn on_message_begin(&mut self, parser: &mut HttpParser) -> CallbackResult {
            println!("Message Begin");
            Ok(ParseAction::None)
        }
        fn on_body(&mut self,parser: &mut HttpParser,data: &[u8]) -> CallbackResult{

            if let Some(HttpMethod::Post) |Some(HttpMethod::Put) = parser.method{
                self.data = Vec::from(base64::decode(data).unwrap());
                println!("{}",String::from_utf8_lossy(&self.data));
            }else if let Some(x) = parser.status_code{
                println!("Status Code: {}", x);
                self.data = match data.iter().fold(Some(HtmlData(Some(Vec::new()),Vec::new(),false,false)),parseHtml){
                    Some(HtmlData(Some(x),_,false,_)) =>x,
                    _ => { println!("lmao fail");
                        return Err("Invalid Text".into())

                    }
                }
            }

           Ok(ParseAction::None)
        }




    }
}





pub struct ThreadPool;







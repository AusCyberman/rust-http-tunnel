extern crate http_parser;
use http_parser::{HttpParserCallback, CallbackResult, HttpParser, ParseAction};

pub struct ThreadPool;

pub struct HttpCallback(String);


impl HttpParserCallback for HttpCallback{
    fn on_message_begin(&mut self, parser: &mut HttpParser) -> CallbackResult {
       println!("Messag");

        Ok(ParseAction::None)
    }

}



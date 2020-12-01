use crate::transmission::Packet;
use crate::{HTML_DATA, HTML_PAGE_NAME};
use anyhow::Result;
use http_parser::{CallbackResult, HttpMethod, HttpParser, HttpParserCallback, ParseAction};
use std::io::Read;
use std::println;
pub struct HtmlData(Option<Vec<u8>>, Option<Vec<u8>>);
pub struct HttpCallback {
    pub http_method: Option<HttpMethod>,
    pub status_code: Option<u16>,
    pub data: Option<Vec<u8>>,
}
/// Includes data for packets
pub enum HttpMessage {
    ///Server Response packet includes status_code and HtmlData
    ServerResponse(u16, Option<Packet>),
    ClientRequest(HttpMethod, Option<Packet>),
    EmptyMessage,
}

fn parse_html(htmldat: HtmlData, dat: &u8) -> HtmlData {
    let start_char = b"<!--[";
    let end_char = b"]-->";

    match htmldat {
        HtmlData(Some(mut x), Some(mut y)) => {
            if start_char.contains(dat) || end_char.contains(dat) {
                y.push(*dat);

                if y.len() > 0 && &end_char[..y.len()] != y.as_slice() {
                    return HtmlData(Some(x), Some(Vec::new()));
                }
                if y.as_slice() == &end_char[..] {
                    return HtmlData(Some(x), None);
                } else if y.as_slice() == &start_char[..] {
                    return HtmlData(None, None);
                }
                //println!("end {}",String::from_utf8_lossy(&[*dat]));
                return HtmlData(Some(x), Some(y));
            } else {
                x.push(*dat);

                return HtmlData(Some(x), Some(Vec::new()));
            };
        }
        HtmlData(None, Some(mut y)) => {
            y.push(*dat);
            if y.len() > 0 && &start_char[..y.len()] != y.as_slice() {
                return HtmlData(None, Some(Vec::new()));
            }
            if y.as_slice() == &start_char[..] {
                return HtmlData(Some(Vec::new()), Some(Vec::new()));
            }
            return HtmlData(None, Some(y));
        }
        y => y,
    }
}
impl HttpCallback {
    pub fn default() -> HttpCallback {
        HttpCallback {
            http_method: None,
            status_code: None,
            data: None,
        }
    }
}
//Parse the http input and put it into the struct HttpCallback
impl HttpParserCallback for HttpCallback {
    fn on_body(&mut self, _: &mut HttpParser, data: &[u8]) -> CallbackResult {
        self.data = Some(Vec::from(data));
        println!("found data");

        Ok(ParseAction::None)
    }
    fn on_status(&mut self, _: &mut HttpParser, status: &[u8]) -> CallbackResult {
        println!("{}", String::from_utf8_lossy(status));
        Ok(ParseAction::None)
    }
    fn on_message_complete(&mut self, parser: &mut HttpParser) -> CallbackResult {
        if let Some(method) = parser.method {
            println!("METHOD: {}", method.to_string());
            self.http_method = Some(method);
        } else if let Some(x) = parser.status_code {
            self.status_code = Some(x);
            println!("code: {}", x);
        }
        if let Some(e) = parser.errno {
            println!("error: {}", e);
        };
        Ok(ParseAction::None)
    }
}
impl HttpMessage {
    pub fn parse(callback: HttpCallback) -> HttpMessage {
        if let Some(method) = (&callback).http_method {
            if let all @ HttpMethod::Post | all @ HttpMethod::Get = method {
                if let Some(data) = callback.data {
                    return HttpMessage::ClientRequest(
                        all,
                        Some(Packet::parse_u8_vec(base64::decode(&data[2..]).unwrap()).unwrap()),
                    );
                }
                return HttpMessage::ClientRequest(all, None);
            } else {
                return HttpMessage::ClientRequest(callback.http_method.unwrap(), None);
            }
        } else if let Some(status_code) = callback.status_code {
            println!("stat code from parse:  {}", status_code);
            if let HtmlData(Some(x), None) = callback
                .data
                .unwrap()
                .iter()
                .fold(HtmlData(None, Some(Vec::new())), parse_html)
            {
                return HttpMessage::ServerResponse(
                    status_code,
                    Some(Packet::parse_u8_vec(base64::decode(x).unwrap()).unwrap()),
                );
            } else {
                return HttpMessage::EmptyMessage;
            }
        }
        println!("Invalid Packet");

        HttpMessage::EmptyMessage
    }
    pub fn create_http_packet(&mut self, seq_num: u32, ack_num: u32) -> Option<Vec<u8>> {
        match self {
            //If input packet is a ServerResponse Packet, parse it and return the Vec
            //containing valid data
            HttpMessage::ServerResponse(resp, data) => {
                if let Some(dat) = data {
                    let mut extra_html = std::fs::File::open(HTML_PAGE_NAME).unwrap();
                    let mut extra_html_buf: [u8; HTML_DATA] = [0; HTML_DATA];
                    extra_html.read(&mut extra_html_buf).unwrap();
                    let contents = format!(
                        "<html><head><!--[{}]-->{}",
                        base64::encode(dat.create_u8_packet(seq_num, ack_num)),
                        String::from_utf8_lossy(&extra_html_buf)
                    );
                    let message = match resp {
                        200 => "OK",
                        404 => "Not Found",
                        _ => "Unknown",
                    };
                    let response = format!(
                        "HTTP/1.1 {} {}\r\nContent-Length: {3}\r\n\r\n{2}",
                        resp,
                        message,
                        contents,
                        contents.len()
                    );

                    println!("response, {}", response.len());
                    return Some(response.as_bytes().to_vec());
                }
                None
            }

            HttpMessage::ClientRequest(met, data) => match met {
                HttpMethod::Post => {
                    if let Some(dat) = data {
                        let data = format!(
                            "d={0}",
                            base64::encode(dat.create_u8_packet(seq_num, ack_num))
                        );
                        let post = format!(
                            "POST / HTTP/1.0\r\n\
                                    Content-Length: {1}\r\n\r\n{0}",
                            data,
                            data.len()
                        );
                        return Some(post.as_bytes().to_vec());
                    } else {
                        None
                    }
                }
                HttpMethod::Get => {
                    let get = b"GET / HTTP/1.0\r\nContent-Length: 0\r\n\r\n";
                    return Some(get.to_vec());
                }
                _ => {
                    return None;
                }
            },
            _ => None,
        }
    }
}

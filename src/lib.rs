extern crate http_parser;
//Parsing
pub const DATA_PACKET_SIZE: usize = DATA_SIZE+METADATA_SIZE;
pub const HEADER_SIZE: usize = 1000;
pub const HTML_DATA: usize = 1000;
pub const METADATA_SIZE: usize = 12;
pub const DATA_SIZE: usize = 50000;
pub const HTTP_SERVER_SIZE: usize = (DATA_PACKET_SIZE* 3) + HTML_DATA+HEADER_SIZE ;
pub const HTTP_CLIENT_SIZE: usize = (DATA_PACKET_SIZE * 3) +HEADER_SIZE;
pub const HTML_PAGE_NAME: &str = "index.html";


pub mod config;
pub mod http; 
pub mod transmission;











use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc,Mutex};
pub fn encode(data: Vec<u8>) -> String {
    base64::encode(data)
}
pub fn decode(data: Vec<u8>) -> Vec<u8> {
    base64::decode(data).unwrap()
}

fn unpacku32(num: &u32) -> [u8; 4] {
    num.to_be_bytes()
}
fn packu32(arr: &[u8]) -> u32 {
    u32::from_be_bytes(<[u8; 4]>::try_from(arr).unwrap())
}
///Packet Struct that includes Sequence length,
/// sequence number, acknowledgement_number and the data itself
#[derive(Clone)]
pub struct Packet {
    ///Sequence Number
    pub seq_num: u32,
    ///Acknowledgement Number
    pub ack_num: u32,
    ///Sequence Length
    pub seq_length: u32,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn parse_u8_vec(value: Vec<u8>) -> Result<Self, ()> {
        Ok(Packet {
            seq_num: packu32(&value[0..4]),
            ack_num: packu32(&value[4..8]),
            seq_length: packu32(&value[8..12]),
            data: Vec::from(&value[12..]),
        })
    }

    pub fn create_u8_packet(&mut self, seq: u32, ack: u32) -> Vec<u8> {
        self.seq_num = seq;
        self.ack_num = ack;
        let mut vec = Vec::new();
        vec.append(&mut unpacku32(&self.seq_num).to_vec());
        vec.append(&mut unpacku32(&self.ack_num).to_vec());
        vec.append(&mut unpacku32(&self.seq_length).to_vec());
        vec.append(&mut self.data);
        vec
    }
}
impl From<Vec<u8>> for Packet {
    fn from(value: Vec<u8>) -> Packet {
        Packet {
            seq_num: 0,
            ack_num: 0,
            seq_length: u32::try_from(value.len()).unwrap(),
            data: value,
        }
    }
}

pub fn get_file_as_byte(filename: &String) -> Vec<u8> {
    let mut f = match File::open(&filename) {
        Ok(x) => x,
        Err(x) => panic!("File not found {}", x),
    };

    let metadata = f.metadata().unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow lmao");
    buffer
}
type MutArc<T> = Arc<Mutex<T>>;

pub struct DataBuffer{
    counter: MutArc<u32>
}

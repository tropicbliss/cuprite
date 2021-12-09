use anyhow::{bail, Result};
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

const HEADER_SIZE: i32 = 10;
const MAX_MESSAGE_SIZE: usize = 4110;

pub struct Client {
    conn: TcpStream,
    id: i32,
}

impl Client {
    pub fn new(addr: &str) -> Result<Self> {
        let conn = TcpStream::connect(addr)?;
        Ok(Self { conn, id: 0 })
    }

    pub fn close(&mut self) -> Result<()> {
        self.conn.shutdown(Shutdown::Both)?;
        Ok(())
    }

    pub fn authenticate(&mut self, password: &str) -> Result<()> {
        self.send_message(MessageType::Login as i32, password)?;
        Ok(())
    }

    pub fn send_command(&mut self, command: &str) -> Result<()> {
        self.send_message(MessageType::Command as i32, command)?;
        Ok(())
    }

    fn increment_id(&mut self) {
        self.id += 1;
    }

    fn send_message(&mut self, msg_type: i32, msg_body: &str) -> Result<()> {
        self.increment_id();
        let req = Message {
            size: i32::try_from(msg_body.len())? + HEADER_SIZE,
            id: self.id,
            msg_type,
            body: msg_body.to_string(),
        };
        let mut resp_bytes = [0_u8; MAX_MESSAGE_SIZE];
        self.conn.write_all(&encode_message(&req)[..])?;
        self.conn.read_exact(&mut resp_bytes)?;
        let resp_id = decode_message(&resp_bytes)?;
        if self.id == resp_id {
            Ok(())
        } else {
            bail!("RCON request ID mismatch");
        }
    }
}

pub enum MessageType {
    Command = 2,
    Login,
}

#[derive(Debug)]
pub struct Message {
    pub size: i32,
    pub id: i32,
    pub msg_type: i32,
    pub body: String,
}

pub fn encode_message(msg: &Message) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    bytes.extend_from_slice(&msg.size.to_le_bytes());
    bytes.extend_from_slice(&msg.id.to_le_bytes());
    bytes.extend_from_slice(&msg.msg_type.to_le_bytes());
    bytes.extend_from_slice(msg.body.as_bytes());
    bytes.extend_from_slice(&[0, 0]);
    bytes
}

pub fn decode_message(bytes: &[u8; MAX_MESSAGE_SIZE]) -> Result<i32> {
    let id = i32::from_le_bytes(bytes[4..8].try_into()?);
    Ok(id)
}

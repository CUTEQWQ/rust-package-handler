extern crate mio;
use mio::*;
use mio::channel;

use std::net::TcpStream;
use std::ops::Shl;
use std::io::prelude::*;
use std::io::{self,Read};
use std::fs::File;
use std::cmp::PartialEq;
use std::clone::Clone;
use std::sync::{Arc,Mutex};
use std::mem::transmute;
use std::cell::RefCell;
use std::time::Duration;


//use std::time::SystemTime;

pub fn read_certain_bytes (mut stream: &TcpStream ,len: usize) -> Vec<u8> {
    let mut field :Vec<u8> = vec![0; len];
    //blocking mode
    //stream.set_read_timeout(Some(Duration::from_millis(100)));
    let mut loaded_len = match stream.read(&mut field) {
        Ok(m) => m,
        Err(e) => {
            match e.kind(){
                io::ErrorKind::WouldBlock => {
                    println!("Would block!");
                    0
                },
                _ => panic!("Got an error:{}",e),
            }
        },
    };
    if loaded_len == 0 {
        let a = Vec::new();
        return a;
    }
    while loaded_len < len {
        //println!("waiting for data");
        let new_len = stream.read(&mut field[loaded_len..]).unwrap();
        loaded_len = loaded_len + new_len;
    }
    field
}

//this method reads data from TcpStream
pub fn package_parser (mut stream: &TcpStream) -> (Vec<u8>, usize) {
    let head = read_certain_bytes(&stream, 4);
    if head.len() == 0 {
        return (head, 0);
    }
    let data_len_small = package_len_small(head);
    let len = data_len_small as usize;

    let body = read_certain_bytes(&stream, len);
    (body, len)
}
pub fn head_parser (mut stream: &TcpStream) -> String {
    //stream.set_nonblocking(false).expect("set blocking failed!");
    let (req_body, size) = package_parser(&stream);
    let mut request = String::new();
    if size == 0 {
        //println!("head_parser: size {:?}",size);
        return request;
    }
    for elem in req_body.iter() {
        request.push(*elem as char);
    }
    //println!("{}",request);
    request
}
pub fn head_parser_blocking (mut stream: &TcpStream, nonblockung: bool) -> String {
    stream.set_nonblocking(nonblockung).unwrap();
    let (req_body, size) = package_parser(&stream);
    let mut request = String::new();
    if size == 0 {
        //println!("head_parser: size {:?}",size);
        return request;
    }
    for elem in req_body.iter() {
        request.push(*elem as char);
    }
    //println!("{}",request);
    request
}
pub fn download_file (mut file :&File, mut stream: &TcpStream, head:Vec<u8>) -> usize{
////use for download_file which include package_head
//    let (message, file_size) = package_parser(&stream);
//    file.write(&message).unwrap();
//    file_size

    //download_file not include package_head
    let data_len_small = package_len_small(head);
    let len = data_len_small as usize;
    let message = read_certain_bytes(&stream, len);
    file.write(&message).unwrap();
    len
}
pub fn package_len_big (head: [u8; 4]) -> u32 {
    let mut len : u32 = 0;
    let mut fact = 24;
    let one: u32 = 1;
    for index in 0..4 {
        let temp = head[index] as u32;
        len = len + temp * one.shl(fact);
        fact -= 8;
    }
    len
}
pub fn package_len_small (head: Vec<u8>) -> u32 {
    let ptr = head.as_ptr() as *const u32;
    unsafe {
      *ptr
    }
}

pub fn create_request (request: String) -> Vec<u8> {
    let package = request.into_bytes();
    let request_size = package.len() as u32;

    let mut p :Vec<u8> = Vec::new();
    let head :[u8; 4] = unsafe{
        transmute(request_size.to_le())
    };
    for i in 0..4 {
        p.push(head[i]);
    }
    for elem in package.iter() {
        p.push(*elem);
    }
    p
}

pub fn create_package_message (len :u32, contents :&Arc<Vec<u8>>) -> Vec<u8> {
    let head :[u8; 4] = unsafe {
        transmute(len.to_le())
    };
    let mut vec : Vec<u8> = Vec::new();
    for i in 0..4 {
        vec.push(head[i]);
    }
    for elem in contents.iter() {
        vec.push(*elem);
    }
    vec
}
pub fn create_package_message_fortest (len :&u32, contents :Vec<u8>) -> Vec<u8> {
    let head :[u8; 4] = unsafe {
        transmute(len.to_le())
    };
    let mut vec : Vec<u8> = Vec::new();
    for i in 0..4 {
        vec.push(head[i]);
    }
    for elem in contents.iter() {
        vec.push(*elem);
    }
    vec
}


pub fn write_all(mut stream: &TcpStream, buf: &[u8]) -> io::Result<usize> {
    //let len = cmp::min(buf.len(), <wrlen_t>::max_value() as usize) as wrlen_t;
    let len = buf.len() as wrlen_t;
    let ret = cvt(unsafe {
        c::send(*stream.inner.as_inner(),
                buf.as_ptr() as *const c_void,
                len,
                MSG_NOSIGNAL)
    })?;
    Ok(ret as usize)
}


pub struct mystream {
    // asteam: Arc<RefCell<TcpStream>>,
    pub astream: Arc<Mutex<TcpStream>>,
}
impl mystream {
    pub fn new(mut stream: TcpStream) -> mystream {
        mystream{
            astream: Arc::new(Mutex::new(stream)),
        }
    }
}
impl Clone for mystream {
    fn clone(&self) -> mystream {
        mystream{
            astream:self.astream.clone(),
        }
    }
}

mod tests {
    use super::write_all;
    #[test]
    fn a () {

    }
}
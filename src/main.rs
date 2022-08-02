// simple-rust-file-transmission

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::env;


fn read_file(file_name: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_name)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    return Ok(data);
}

fn write_file(file_name: &str, data : Vec::<u8>) -> std::io::Result<()> {
    let mut file = File::create(file_name).expect("Unable to create file");   
    file.write_all(&data)?;
    return Ok(());
}

fn recive_package(mut stream: TcpStream) -> std::io::Result<Vec<u8>> {
    let mut package_len = [0 as u8; 8];
    stream.read_exact(&mut package_len)?;
    let len = u64::from_be_bytes(package_len);
  
    let mut data = vec![0; len.try_into().unwrap()];
    stream.read_exact(&mut data)?;
    return Ok(data);
}

fn send_packet(mut stream: TcpStream, data: Vec<u8>)-> std::io::Result<()>  {
  let length: u64 = data.len().try_into().unwrap();
  let len_bytes = length.to_be_bytes();
  stream.write(&len_bytes).unwrap();
  stream.write(&data)?;
  Ok(())
}


fn recive_file(file_name: &str) -> std::io::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:6666").unwrap();
  let (stream, _addr) = listener.accept()?;
  let data = recive_package(stream)?;
  write_file(file_name, data)?;
  return Ok(());
}

fn send_file(ip: &str, file_name: &str) -> std::io::Result<()> {
  let addr = format!("{}{}", ip, ":6666");
  let stream = TcpStream::connect(addr)?;
  let data = read_file(file_name)?;
  send_packet(stream, data)?;
  return Ok(());
}


fn print_usage(prog_name: &str) -> () {
  println!("usage:");
  println!("\t{} {}", prog_name, "-r <filename>");
  println!("\t{} {}", prog_name, "-s <IP> <filename>");
}



fn doit() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    let arg = &args[1].as_str();
    match arg {
      &"-r" => recive_file(&args[2]),
      &"-s" => send_file(&args[2], &args[3]),
      _ => Ok(print_usage(&args[0])),
    }?;
  } else {
    print_usage(&args[0]);
  }
  return Ok(());
}

fn main() -> std::io::Result<()> {
  doit()?;
  return Ok(());
}

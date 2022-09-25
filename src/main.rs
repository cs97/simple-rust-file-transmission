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

fn recive_file(file_name: &str) -> std::io::Result<()> {
	let tcp = easytcp::tcp::simple_listen("0.0.0.0", "6666")?;
  write_file(file_name, tcp.recive()?)?;
  return Ok(());
}

fn send_file(ip: &str, file_name: &str) -> std::io::Result<()> {
	let tcp = easytcp::tcp::simple_connect(ip, "6666")?;
	let data = read_file(file_name)?;
	tcp.send(data)?;
	return Ok(())
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

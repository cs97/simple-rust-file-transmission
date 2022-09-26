// simple-rust-file-transmission
use std::io::BufReader;
use std::io::prelude::*;
//use std::net::{TcpListener, TcpStream};
//use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::env;

/*
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
*/

fn recive_file_in_chunks(file_name: &str) -> std::io::Result<()> {
	let tcp = easytcp::tcp::simple_listen("0.0.0.0", "6666")?;

	let package_len = tcp.recive()?;
	let package_len_bytes: [u8; 8] = package_len[0..8].try_into().unwrap();
	let n = u64::from_be_bytes(package_len_bytes);

	let mut f = File::create(file_name)?;
	let mut pkglen = n;

	loop {
		if pkglen == 0 {
				break;
		}
		let data = tcp.recive()?;
		let datalen: u64 = data.len().try_into().unwrap();
		pkglen = pkglen - datalen;
		f.write_all(&data)?;

	}
	f.sync_data()?;
	return Ok(());
}

fn send_file_in_chunks(ip: &str, file_name: &str) -> std::io::Result<()> {
	let file = File::open(file_name)?;
	let file_length: u64 = file.metadata().unwrap().len();
	let mut br = BufReader::with_capacity(4096, file);

	let tcp = easytcp::tcp::simple_connect(ip, "6666")?;
	tcp.send(file_length.to_be_bytes().to_vec())?;

	loop {
		let buffer = br.fill_buf()?;
		let bufferlen = buffer.len();
		if bufferlen == 0 {
				break;
		}
		tcp.send(buffer.to_vec()).unwrap();
		br.consume(bufferlen);
	}
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

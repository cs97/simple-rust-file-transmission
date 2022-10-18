// simple-rust-file-transmission
use std::io::{self, Write};
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;


fn get_tcp_listen() -> std::io::Result<easytcp::tcp_aes_cbc::SecureTcp> {
	//let tcp = easytcp::tcp::listen("0.0.0.0", "6666")?;

	let key = "nice key";
	let tcp = easytcp::tcp_aes_cbc::listen("0.0.0.0", "6666", key)?;

	return Ok(tcp)
}

fn get_tcp_connect(ip: &str) -> std::io::Result<easytcp::tcp_aes_cbc::SecureTcp> {
	//let tcp = easytcp::tcp::connect(ip, "6666")?;

	let key = "nice key";
	let tcp = easytcp::tcp_aes_cbc::connect(ip, "6666", key)?;

	return Ok(tcp)
}


fn recive_file_in_chunks(file_name: &str) -> std::io::Result<()> {
	
	let tcp = get_tcp_listen()?;
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

	//let tcp = easytcp::tcp::connect(ip, "6666")?;
	let tcp = get_tcp_connect(ip)?;

	tcp.send(file_length.to_be_bytes().to_vec())?;
	let mut progress: usize = 0;

	loop {
		progressbar(progress, file_length.try_into().unwrap());
		let buffer = br.fill_buf()?;
		let bufferlen = buffer.len();
		if bufferlen == 0 {
				break;
		}
		tcp.send(buffer.to_vec()).unwrap();
		br.consume(bufferlen);
		progress = progress + 4096;
	}
	println!("");
	return Ok(());
}


fn progressbar(value: usize, target_value: usize) -> () {
	let mut percent = (value * 100) / target_value;
	if percent >= 100 {
		percent = 100;
	};
	let n = (4 * percent) / 10 ;

	let var1 = "=".repeat(n).to_string();
	let var2 = "-".repeat(40-n).to_string();
	let s = format!("[{}{}] {}%", var1, var2, percent);

	let stdout = io::stdout();
	let mut handle = stdout.lock();

	io::stdout().write_all("\r".as_bytes()).unwrap();
	handle.write_all(s.as_bytes()).unwrap();
	handle.flush().unwrap();
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
			&"-r" => recive_file_in_chunks(&args[2]),
			&"-s" => send_file_in_chunks(&args[2], &args[3]),
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

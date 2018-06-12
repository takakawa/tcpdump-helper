extern crate clap;

use clap::{Arg,App};
use std::fmt::Write;

fn main() {
	let matches  =   App::new("tcpdump-helper")
							.version("1.0")
							.author("Gaochuan <takakawa@163.com>")
							.about("Does something funny")
							.arg(Arg::with_name("url")
									.short("u")
									.long("url")
									.value_name("URL")
									.help("set the url to filter packets")
									.takes_value(true))
							.arg(Arg::with_name("method")
									.short("X")
									.long("method")
									.help("set the method to filter packets")
									.takes_value(true))
							.arg(Arg::with_name("port")
									.short("p")
									.long("port")
									.help("set the port to filter packets")
									.takes_value(true))
							.arg(Arg::with_name("dev")
									.short("i")
									.long("dev")
									.help("set the dev to filter packets")
									.takes_value(true))
							.arg(Arg::with_name("asic")
									.short("A")
									.long("asic")
									.help("print the packet by asic")
									.takes_value(false))
							.get_matches();

	let get_code = "0x47455420";
	let post_code = "0x504f5354";
	let put_code = "0x50555420";

	let mut tcpdump_command = "tcpdump ".to_string();


	let dev = matches.value_of("dev").unwrap_or("any");	
	write!(tcpdump_command,"-i {} ",dev);

	if matches.is_present("asic"){
		write!(tcpdump_command,"-Ann ");
	}

	let port = matches.value_of("port").unwrap_or("80");	
	write!(tcpdump_command,"\' port {} and ",port);

	if let Some(method) = matches.value_of("method"){
		tcpdump_command.push_str(" tcp[((tcp[12:1] & 0xf0) >> 2):4]=");
		let code = match method {
		  "GET" => get_code,
          "POST" => post_code,
          "PUT" => put_code,
			_ => get_code,
		};
        
		tcpdump_command.push_str(code);
	}

   if let Some(url) = matches.value_of("url") {
		let len =  std::cmp::max(4,url.len());
		write!(&mut tcpdump_command, " and {}","tcp[((tcp[12:1] & 0xf0) >> 2)+4:4]=0x");
		let mut i = 0;
		for &byte in  url.as_bytes() {
			write!(&mut tcpdump_command, "{:X}", byte).expect("unable to write");
            i=i+1;
            if i > 3 {
				break;
			}
		}
	}

	write!(tcpdump_command,"\'");


	println!("{}",tcpdump_command);
}

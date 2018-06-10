

mod server;
mod client;

extern crate clap;

use std::env;
use std::process::exit;
use std::io::{self, Read};
use clap::{Arg,App};
use std::thread;

fn main() -> io::Result<()>{

	


/*	let args : Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Error: Need to provide ip & port no");
		exit(1);
	}
	let port = &args[1];
	println!("Port number to use :: {:?}", port);
*/
	
	let matches = App::new("Multiparty Chat Server")
                          .version("1.0")
                          .author("Ranjan R. <rajeev2.ranjan@intel.com>")
                          .about("Starts a server to handle all chat clients")
                          .arg(Arg::with_name("host")
                               .short("h")
                               .long("hostname")
                               .value_name("HOSTNAME")
                               .help("Sets the hostname for the server")
			       .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("port")
			       .short("p")
			       .long("port_address")
                               .help("Sets the port address")
			       .value_name("PORT")
			       .takes_value(true)
                               .required(true))
                          .get_matches();

	println!("Hello, world!");
		
	let host = matches.value_of("host").unwrap_or("localhost");
	println!("Value for host: {}", host);
		
	let port = matches.value_of("port").unwrap_or("8008");
	println!("Value for port: {}", port);
	
	let mut buffer = String::new();
	println!("Enter your message : ");
	io::stdin().read_line(&mut buffer)?;

	println!("You typed in : {}",buffer );

	let th1 = thread::spawn( move || {
	
	});	
	
	Ok(())

}

mod server;
mod client;

extern crate clap;

use std::env;
use std::process::exit;
use std::io::{self, Read};
use clap::{Arg,App};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;

fn main() -> io::Result<()>{
	
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
	
	//Try reading messages from stdin
	let mut buffer = String::new();
	println!("Enter your message : ");
	io::stdin().read_line(&mut buffer)?;

	println!("You typed in : {}",buffer );

	//Creating a channel
	let (send, recv) = channel();


	//Spawning a thread
	let th1 = thread::spawn( move || {
		let handle = thread::current();
		println!("This is thread : {:?} ",handle.name());	
		send.send("\nHey!! from the thread...").unwrap();
		thread::sleep(Duration::from_secs(2)); // block for two seconds
		send.send("Delayed for 2 seconds\n").unwrap();
	});
	

	println!("{}", recv.recv().unwrap()); // Received immediately
	println!("Waiting...");
	println!("{}", recv.recv().unwrap()); // Received after 2 seconds
	
	thread::sleep(Duration::from_secs(5));	
	
	Ok(())

}

#[cfg(tests)]

mod tests{ 
 #[test]
 fn test_dummy() {
	assert_eq!(4, 2+2);
 }

}


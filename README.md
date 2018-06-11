# mpcs-rust
Multiparty Chat Server in Rust

The command line arguments need to be passed in the following manner when using cargo to run the binaries -
cargo run --bin mpcs -- -h localhost -p 4000

You could also use the long form of the arguments as -
cargo run --bin mpcs -- --hostname localhost --port_address 4000

Build -

1.) cd ./mpcs
2.) cargo build

Run - 

1.) cd ./mpcs
2.) cargo run --bin mpcs -- -h localhost -p 4000

Help -

1.) cd ./mpcs
2.) cargo run --bin mpcs -- --help


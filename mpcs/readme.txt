The command line arguments need to be passed in the following
manner when using cargo to run the binaries -

cargo run --bin mpcs -- -h localhost -p 4000

You could also use the long form of the arguments as -

cargo run --bin mpcs -- --hostname localhost --port_address 4000

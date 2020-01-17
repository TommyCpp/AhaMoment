use kvs::{Result, DEFAULT_IP, DEFAULT_PORT};
use clap::{App, Arg};


///
/// Start a KV store server
/// This binary has two attributes
/// --addr, default to be 127.0.0.1:4000, the address that server bind to
/// --engine, default to be kvs, must be either kvs or sled, the backend store engine that gonna used.
///
///
fn main() -> Result<()> {
    let default_addr = format!("{}:{}", DEFAULT_IP, DEFAULT_PORT);
    let matches = App::new("kvs-server")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .required(false).default_value(default_addr.as_str())
        )
        .arg(
            Arg::with_name("engine")
                .long("engine")
                .required(false)
                .possible_value("sled")
                .possible_value("kvs")
                .default_value("kvs")
        )
        .get_matches();

    let addr = matches.value_of("addr").unwrap();
    let engine = matches.value_of("engine").unwrap();

    println!("engine:{}", engine);
    println!("addr:{}", addr);

    Ok(())
}
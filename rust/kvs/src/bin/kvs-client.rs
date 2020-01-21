extern crate clap;

use std::env::current_dir;
use std::process::exit;
use env_logger;
use log::error;

use clap::{App, Arg, SubCommand};

use kvs::{Result, DEFAULT_PORT, DEFAULT_IP, KvClient};
use kvs::KvsEngine;
use std::net::{TcpStream, SocketAddr};

fn main() -> Result<()> {
    env_logger::init();
    let default_addr = format!("{}:{}", DEFAULT_IP, DEFAULT_PORT);
    let addr_arg = Arg::with_name("addr").long("addr").required(true).default_value(default_addr.as_str());

    let matches = App::new("kvs-client")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(SubCommand::with_name("get")
            .arg(Arg::with_name("key").required(true).index(1))
            .arg(addr_arg.clone()))
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("key").required(true).index(1))
                .arg(Arg::with_name("value").required(true).index(2))
                .arg(addr_arg.clone()),
        )
        .subcommand(SubCommand::with_name("rm")
            .arg(Arg::with_name("key").required(true).index(1))
            .arg(addr_arg.clone()))
        .get_matches();


    match matches.subcommand() {
        ("get", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            let addr = matches.value_of("addr").unwrap();
            let mut client = KvClient::new(addr.parse()?)?;

            match client.get(String::from(key)) {
                Ok(val) => {
                    println!("{}", val)
                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
        ("set", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            let value = matches.value_of("value").unwrap();
            let addr = matches.value_of("addr").unwrap();
            let mut client = KvClient::new(addr.parse()?)?;
            client.set(String::from(key), String::from(value));
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            let addr = matches.value_of("addr").unwrap();
            let mut client = KvClient::new(addr.parse()?)?;
            if let Err(err) = client.remove(String::from(key)) {
                error!("Key not found");
                exit(1);
            } else {
            }
        }
        _ => exit(1),
    }
    Ok(())
}
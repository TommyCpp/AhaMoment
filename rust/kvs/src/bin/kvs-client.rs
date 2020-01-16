extern crate clap;

use std::env::current_dir;
use std::process::exit;

use clap::{App, Arg, SubCommand};

use kvs::Result;
use kvs::KvsEngine;

fn main() -> Result<()> {
    let mut kvs = kvs::KvStore::open(current_dir()?.as_path()).unwrap();
    let matches = App::new("kvs-client")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(SubCommand::with_name("get").arg(Arg::with_name("key").required(true).index(1)))
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("key").required(true).index(1))
                .arg(Arg::with_name("value").required(true).index(2)),
        )
        .subcommand(SubCommand::with_name("rm").arg(Arg::with_name("key").required(true).index(1)))
        .get_matches();

    match matches.subcommand() {
        ("get", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            match kvs.get(String::from(key)) {
                Ok(Some(val)) => println!("{}", val),
                Ok(None) => {
                    println!("Key not found");
                    exit(0)
                }
                Err(why) => panic!("Other errors"),
            };
        }
        ("set", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            let value = matches.value_of("value").unwrap();
            kvs.set(String::from(key), String::from(value));
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            match    kvs.remove(String::from(key)) {
                Err(err) => match err {
                    kvs::Error::NotFoundError => {
                        println!("Key not found");
                        exit(1)
                    }
                    _ => panic!("Other error"),
                },
                Ok(_) => (),
            };
        }
        _ => exit(1),
    }

    Ok(())
}

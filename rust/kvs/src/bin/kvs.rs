extern crate clap;

use std::process::exit;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("kvs")
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
            let _key = matches.value_of("key").unwrap();
            unimplemented!()
        }
        ("set", Some(matches)) => {
            let _key = matches.value_of("key").unwrap();
            let _value = matches.value_of("value").unwrap();
            unimplemented!()
        }
        ("rm", Some(matches)) => {
            let _key = matches.value_of("key").unwrap();
            unimplemented!()
        }
        _ => exit(1),
    }
}

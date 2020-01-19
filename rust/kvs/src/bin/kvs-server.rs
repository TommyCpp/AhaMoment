use kvs::{Result, DEFAULT_IP, DEFAULT_PORT, SLED_STORE_NAME, KV_STORE_NAME, KvServer, SledStore, KvStore};
use std::net::{TcpListener, TcpStream, SocketAddr};
use clap::{App, Arg};
use log::{info, error};
use std::process::exit;
use std::env::current_dir;
use kvs::CommandProtocol;


///
/// Start a KV store server
/// This binary has two attributes
/// --addr, default to be 127.0.0.1:4000, the address that server bind to
/// --engine, default to be kvs, must be either kvs or sled, the backend store engine that gonna used.
///
///
fn main() -> Result<()> {
    env_logger::init();
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
                .possible_value(SLED_STORE_NAME)
                .possible_value(KV_STORE_NAME)
                .default_value(KV_STORE_NAME)
        )
        .get_matches();

    let addr = matches.value_of("addr").unwrap();
    let engine = matches.value_of("engine").unwrap();
    info!("Running KVS-Server version {} on addr {}, with backend storage engine of {}", env!("CARGO_PKG_VERSION"), addr, engine);

    if engine == SLED_STORE_NAME {
        let mut server = KvServer::<SledStore>::new(addr.parse()?, SledStore::open(current_dir()?.as_path())?)?;
        server.serve()
    } else {
        let mut server = KvServer::<KvStore>::new(addr.parse()?, KvStore::open(current_dir()?.as_path())?)?;
        server.serve()
    }


    Ok(())
}

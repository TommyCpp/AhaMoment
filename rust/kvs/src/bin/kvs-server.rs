use kvs::{Result, DEFAULT_IP, DEFAULT_PORT, SLED_STORE_NAME, KV_STORE_NAME, KvServer, SledStore, KvStore, SharedQueueThreadPool, ThreadPool};
use std::net::{TcpListener, TcpStream, SocketAddr};
use clap::{App, Arg};
use log::{info, error};
use std::process::exit;
use std::env::current_dir;
use kvs::Request;
use std::fs::File;
use std::io::{Write, Read};


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
                .default_value(SLED_STORE_NAME)
        )
        .get_matches();

    let addr = matches.value_of("addr").unwrap();
    let engine = matches.value_of("engine").unwrap();
    error!("Running KVS-Server version {} on addr {}, with backend storage engine of {}", env!("CARGO_PKG_VERSION"), addr, engine);

    ///
    ///
    macro_rules! start_server {
        ($engine:ty) => {
        //Start the KvServer with given engine
        KvServer::<$engine, SharedQueueThreadPool>::new(addr.parse()?, <$engine>::open(current_dir()?.as_path())?, SharedQueueThreadPool::new(4)?)?.serve();
        };
    }

    //create config file
    let conf_file_path = current_dir()?.as_path().join("server.conf");
    if conf_file_path.exists() {
        //if exists check if the the engine matches.
        let mut engine_in_config = String::new();
        File::open(conf_file_path)?.read_to_string(&mut engine_in_config);
        if engine_in_config != engine {
            exit(1)
        }
    } else {
        let mut conf_file = File::create(conf_file_path)?;
        conf_file.write(engine.as_ref());
    }

    if engine == SLED_STORE_NAME {
        start_server!(SledStore)
    } else {
        start_server!(KvStore)
    }


    Ok(())
}

use std::env::current_dir;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::exit;

use clap::{App, Arg};
use log::{error, info};

use kvs::{DEFAULT_IP, DEFAULT_PORT, KV_STORE_NAME, KvServer, KvStore, NAIVE_THREAD_POOL_NAME, NaiveThreadPool, RAYON_THREAD_POOL_NAME, Result, SHARED_QUEUE_THREAD_POOL_NAME, SharedQueueThreadPool, SLED_STORE_NAME, SledStore, ThreadPool, RayonThreadPool};
use kvs::Request;

///
/// Start a KV store server
/// This binary has two attributes
/// --addr, default to be 127.0.0.1:4000, the address that server bind to
/// --engine, default to be kvs, must be either kvs or sled, the backend store engine that gonna used.
/// --thread_pool, default to be sharequeue, must be either native, rayon or sharequeue, the thread_pool implementation that gonna be used
/// --num_thread, default to be the same with the cpu num
///
///
fn main() -> Result<()> {
    env_logger::init();
    let num_cpu = num_cpus::get().to_string();
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
        .arg(
            Arg::with_name("thread_pool")
                .required(false)
                .long("thread_pool")
                .possible_value(NAIVE_THREAD_POOL_NAME)
                .possible_value(SHARED_QUEUE_THREAD_POOL_NAME)
                .possible_value(RAYON_THREAD_POOL_NAME)
                .default_value(SHARED_QUEUE_THREAD_POOL_NAME)
        )
        .arg(
            Arg::with_name("num_thread")
                .required(false)
                .long("num_thread")
                .default_value(num_cpu.as_str())
        )
        .get_matches();

    let addr = matches.value_of("addr").unwrap();
    let engine = matches.value_of("engine").unwrap();
    let thread_num = matches.value_of("num_thread").unwrap().parse::<u32>().unwrap();
    let thread_pool_str = matches.value_of("thread_pool").unwrap();
    error!("Running KVS-Server version {} on addr {}, with backend storage engine of {}", env!("CARGO_PKG_VERSION"), addr, engine);

    ///
    ///todo: change macro_rule to fit new param
    macro_rules! start_server {
        ($engine:ty, $thread_pool:ty) => {
        //Start the KvServer with given engine
        KvServer::<$engine, $thread_pool>::new(addr.parse()?, <$engine>::open(current_dir()?.as_path())?, <$thread_pool>::new(thread_num)?)?.serve();
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
        match thread_pool_str {
            SHARED_QUEUE_THREAD_POOL_NAME => start_server!(SledStore, SharedQueueThreadPool),
            RAYON_THREAD_POOL_NAME => start_server!(SledStore, RayonThreadPool),
            NAIVE_THREAD_POOL_NAME => start_server!(SledStore, NaiveThreadPool),
            _ => exit(2)
        }
    } else {
        match thread_pool_str {
            SHARED_QUEUE_THREAD_POOL_NAME => start_server!(KvStore, SharedQueueThreadPool),
            RAYON_THREAD_POOL_NAME => start_server!(KvStore, RayonThreadPool),
            NAIVE_THREAD_POOL_NAME => start_server!(KvStore, NaiveThreadPool),
            _ => exit(2)
        }
    }


    Ok(())
}
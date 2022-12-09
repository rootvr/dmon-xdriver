use crate::cli::Args;
use crossbeam_channel::Sender;
use redis;
use std::env;
use std::process::Child;
use std::sync::{Arc, Mutex};

pub struct Broker {
    redis: redis::Connection,
}

impl Broker {
    pub fn new(cli_args: &Arc<Args>, wgen_handle: &Arc<Mutex<Child>>) -> Self {
        let redis_hn = match env::var("REDIS_HOSTNAME") {
            Ok(v) => v,
            Err(_) => format!("{}:{}", cli_args.redis_ipv4, cli_args.redis_port),
        };
        let redis_ps = env::var("REDIS_PASSWORD").unwrap_or_default();
        let redis_uri = match env::var("IS_TLS") {
            Ok(_) => "rediss",
            Err(_) => "redis",
        };
        let redis_url = format!("{}://:{}@{}", redis_uri, redis_ps, redis_hn);
        let redis_conn = match redis::Client::open(redis_url) {
            Ok(client) => match client.get_connection() {
                Ok(conn) => conn,
                Err(_) => {
                    kill!(wgen_handle, "failed to connect to Redis server");
                }
            },
            Err(_) => {
                kill!(wgen_handle, "invalid Redis connection url");
            }
        };
        Self { redis: redis_conn }
    }
    pub fn capture(
        &mut self,
        tx: Sender<String>,
        cli_args: &Arc<Args>,
        wgen_handle: &Arc<Mutex<Child>>,
    ) {
        let mut redis_broker = self.redis.as_pubsub();
        let channel = match &*cli_args.filter {
            "network" => "dmon_network_out",
            "structure" => "dmon_structure_out",
            "none" => "dmon_merged_out",
            _ => {
                kill!(wgen_handle, "invalid filter CLI arg");
            }
        };
        redis_broker.subscribe(&channel).unwrap();
        while let Ok(message) = redis_broker.get_message() {
            match wgen_handle.lock().unwrap().try_wait() {
                Ok(Some(_)) => break,
                Ok(None) => {}
                Err(_) => {
                    kill!(wgen_handle, "error during child process wait");
                }
            }
            let payload: String = message.get_payload().unwrap();
            if let Err(_) = tx.try_send(payload) {
                break;
            }
        }
    }
}

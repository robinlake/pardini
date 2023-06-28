use crate::thread_pool::ThreadPool;
use std::{
    fs::OpenOptions,
    io::{copy, BufReader},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

pub struct ServerOptions {
    pub addr: String,
    pub pool_size: usize,
    pub data_dir: String,
}

pub fn start_server(opts: ServerOptions) {
    let listener = TcpListener::bind(&opts.addr).expect("couldn't start TCP server");
    log::info!("server listening on {}", &opts.addr);

    let pool = ThreadPool::new(opts.pool_size);
    // accept connections and process them serially
    let dir = Arc::new(opts.data_dir.clone());
    for stream in listener.incoming() {
        let new_dir = dir.clone();
        pool.execute(move || consume_stream(stream.expect("Unable to unwrap stream"), &new_dir));
    }
}

fn consume_stream(stream: TcpStream, data_dir: &str) {
    let peer_name = stream
        .peer_addr()
        .and_then(|ok| Ok(ok.to_string()))
        .unwrap_or("Unknown".to_string());

    log::debug!("client {peer_name} connected");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("gpslog.txt")
        .expect("Unable to open file");

    copy(&mut BufReader::new(&stream), &mut file)
        .and_then(|copied_bytes| {
            log::debug!("client {peer_name} disconnected after copying {copied_bytes} bytes");
            Ok(())
        })
        .unwrap_or_else(|e| {
            if e.kind() == std::io::ErrorKind::ConnectionReset {
                log::debug!("client {peer_name} connection reset");
            } else {
                panic!("couldn't copy data into file: {e}")
            }
        });
}

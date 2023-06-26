use crate::thread_pool::ThreadPool;
use std::{
    fs::OpenOptions,
    io::{copy, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn start_server(addr: String, pool_size: usize) {
    let listener = TcpListener::bind(&addr).expect("couldn't start TCP server");
    log::info!("server listening on {}", &addr);

    let pool = ThreadPool::new(pool_size);
    // accept connections and process them serially
    for stream in listener.incoming() {
        pool.execute(|| handle_client(stream.expect("Unable to unwrap stream")));
    }
}

fn handle_client(stream: TcpStream) {
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

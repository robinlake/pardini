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

    log::debug!("client {} connected", peer_name);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("gpslog.txt")
        .expect("Unable to open file");

    let mut reader = BufReader::new(&stream);
    if let Err(e) = copy(&mut reader, &mut file) {
        match e.kind() {
            std::io::ErrorKind::ConnectionReset => {
                log::debug!("client {} connection reset", peer_name)
            }
            _ => log::error!("Unable to copy data: {}, {}", e.kind(), e),
        }
    } else {
        log::debug!("client {} disconnected", peer_name);
    }
}

use pardini::ThreadPool;
use std::{
    fs::OpenOptions,
    io::{copy, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("gpslog.txt")
        .expect("Unable to open file");
    let mut reader = BufReader::new(&stream);
    copy(&mut reader, &mut file).expect("Unable to write data");
    stream
        .write(b"Hello Peer!\r\n")
        .expect("unable to respond to client");
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("couldn't start TCP server");
    let pool = ThreadPool::new(4);
    // accept connections and process them serially
    for stream in listener.incoming() {
        thread::spawn(|| {
            // connection succeeded
            let stream = stream.expect("Unable to unwrap stream");
            handle_client(stream);
        });
        // handle_client(stream?);
    }
    Ok(())
}

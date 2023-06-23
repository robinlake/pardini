use std::{
    fs::OpenOptions,
    io::{copy, BufReader, Write},
    net::{TcpListener, TcpStream},
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
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("couldn't start TCP server");
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

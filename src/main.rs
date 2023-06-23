use std::{
    fs::OpenOptions,
    io::{copy, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(stream: TcpStream) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("gpslog.txt")
        .expect("Unable to open file");

    let mut reader = BufReader::new(&stream);
    match copy(&mut reader, &mut file) {
        Ok(_) => return,
        Err(e) => match e.kind() {
            std::io::ErrorKind::ConnectionReset => return,
            _ => eprintln!("Unable to copy data: {}, {}", e.kind(), e),
        },
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8084").expect("couldn't start TCP server");
    // accept connections and process them serially
    for stream in listener.incoming() {
        thread::spawn(|| handle_client(stream.expect("Unable to unwrap stream")));
    }
    Ok(())
}

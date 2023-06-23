use pardini::ThreadPool;
use std::{
    fs::OpenOptions,
    io::{copy, BufReader},
    net::{TcpListener, TcpStream},
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
    let listener = TcpListener::bind("0.0.0.0:8080").expect("couldn't start TCP server");
    let pool = ThreadPool::new(4);
    // accept connections and process them serially
    for stream in listener.incoming() {
        // thread::spawn(|| {
        //     // connection succeeded
        //     let stream = stream.expect("Unable to unwrap stream");
        //     handle_client(stream);
        // });
        // handle_client(stream?);
        pool.execute(|| handle_client(stream.expect("Unable to unwrap stream")));
    }
    Ok(())
}

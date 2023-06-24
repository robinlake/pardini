use clap::Parser;
use pardini::ThreadPool;
use std::{
    fs::OpenOptions,
    io::{copy, BufReader},
    net::{TcpListener, TcpStream},
};

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
    match copy(&mut reader, &mut file) {
        Ok(_) => log::debug!("client {} disconnected", peer_name),
        Err(e) => match e.kind() {
            std::io::ErrorKind::ConnectionReset => {
                log::debug!("client {} connection reset", peer_name);
            }
            _ => log::error!("Unable to copy data: {}, {}", e.kind(), e),
        },
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Host / port to listen on
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    addr: String,

    /// Thread pool size
    #[arg(short, long, default_value = "4")]
    pool_size: usize,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> std::io::Result<()> {
    let cli_args = Args::parse();

    env_logger::Builder::new()
        .filter_level(cli_args.verbose.log_level_filter())
        .init();

    let listener = TcpListener::bind(&cli_args.addr).expect("couldn't start TCP server");
    log::info!("server listening on {}", &cli_args.addr);

    let pool = ThreadPool::new(cli_args.pool_size);
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

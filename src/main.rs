use clap::Parser;
use pardini::tcp_server::start_server;

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
    start_server(cli_args.addr, cli_args.pool_size);
    Ok(())
}

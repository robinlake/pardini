use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{
    fs::OpenOptions,
    io::{copy, prelude::*, BufReader, Write},
    net::{TcpListener, TcpStream},
    str,
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

#[get("/")]
async fn hello() -> impl Responder {
    println!("Hello world!");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("req_body: {}", req_body);
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let _ = HttpServer::new(|| {
    //     App::new0()
    //         .service(hello)
    //         .service(echo)
    //         .route("/hey", web::get().to(manual_hello))
    // })
    // .bind(("192.168.50.20", 8080))?
    // .run()
    // .await;

    let listener = TcpListener::bind("0.0.0.0:8080").expect("couldn't start TCP server");
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

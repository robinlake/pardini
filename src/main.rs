use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    println!("New client! {}", stream.peer_addr().unwrap());
    let mut result: [u8; 128] = [0; 128];
    // result = stream.read(&mut [0; 128]).unwrap();
    stream.read(&mut result).unwrap();
    println!("result: {:?}", result);
    stream.write(b"Hello Peer!\r\n").unwrap();
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
    //     App::new()
    //         .service(hello)
    //         .service(echo)
    //         .route("/hey", web::get().to(manual_hello))
    // })
    // .bind(("192.168.50.20", 8080))?
    // .run()
    // .await;
    let listener = TcpListener::bind("192.168.50.20:8081").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

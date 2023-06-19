use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{
    fs::OpenOptions,
    io::{copy, prelude::*, BufReader, Write},
    net::{TcpListener, TcpStream},
    str,
};

fn handle_client(mut stream: TcpStream) {
    let mut file_ref = OpenOptions::new()
        .append(true)
        .open("gpslog.txt")
        .expect("Unable to open file");
    let mut reader = BufReader::new(&stream);
    copy(&mut reader, &mut file_ref).expect("Unable to write data");
    // file_ref.write_all(line).expect("Unable to write data");
    stream
        .write(b"Hello Peer!\r\n")
        .expect("unable to respond to client");
}
// fn handle_client(mut stream: TcpStream) {
//     println!("New client! {}", stream.peer_addr().unwrap());
//     let mut result: [u8; 128] = [0; 128];
//     stream.read(&mut result).unwrap();
//     let reader = BufReader::new(stream);
//     let mut mystr = String::new();
//     reader.read_line(&mut mystr).unwrap();
//     // let mystr = str::from_utf8(&result).unwrap();
//     // println!("result: {}", mystr);

//     let mut file_ref = OpenOptions::new()
//         .append(true)
//         .open("gpslog.txt")
//         .expect("Unable to open file");
//     file_ref.write_all(line).expect("Unable to write data");
//     stream.write(b"Hello Peer!\r\n").unwrap();
// }

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
    let listener = TcpListener::bind("0.0.0.0:8080").expect("couldn't start TCP server");
    // let listener = TcpListener::bind("192.168.50.20:8081").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

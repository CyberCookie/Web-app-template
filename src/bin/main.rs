use std::io::prelude::*;
use std::{ fs, thread, time, net };

use tokio_postgres::{ NoTls, Error };
use serde::{ Deserialize, Serialize };
use serde_json;


use tungstenite::{
    accept_hdr,
    Message,
    handshake::server::{ Request, Response }
};

use web_server::ThreadPool;



#[derive(Serialize, Deserialize)]
struct WSIncommingMsgPayload {
    qwerty: i32
}

#[derive(Serialize, Deserialize)]
struct WSMsg<P> {
    r#type: String,
    data: P
}


const HOST: &str = "127.0.0.1";
const HTTP_PORT: &str = "3005";
const WS_PORT: &str = "3012";
const DB_HOST: &str = "localhost";
const DB_USER: &str = "test_user_pg";
const DB_NAME: &str = "test_db";


#[tokio::main]
async fn main() -> Result<(), Error> {
    let (db_client, db_connection) =
        tokio_postgres::connect(
            &format!("host={} user={} dbname={}", DB_HOST, DB_USER, DB_NAME),
            NoTls
    ).await?;

    tokio::spawn(async move {
        if let Err(e) = db_connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let rows = db_client
        .query("SELECT (name) FROM person", &[])
        .await?;

    let value: &str = rows[0].get(0);

    println!("{}", value);



    let http_thread = thread::spawn(|| {
        run_simple_server(
            format!("{}:{}", HOST, HTTP_PORT)
        );
    });
    let ws_thread = thread::spawn(|| {
        run_web_socket_server(
            format!("{}:{}", HOST, WS_PORT)
        );
    });
    

    http_thread.join().unwrap();
    ws_thread.join().unwrap();

    Ok(())
}


fn run_web_socket_server(address: String) {
    let server = net::TcpListener::bind(address).unwrap();

    for stream in server.incoming() {
        thread::spawn(move || {
            let mut websocket = accept_hdr(
                stream.unwrap(),
                |_: &Request, response: Response| {
                    println!("Received a new ws handshake");
                    // println!("The request's path is: {}", req.uri().path());
                    // println!("The request's headers are:");
                    // for (ref header, _value) in req.headers() {
                    //     println!("* {}", header);
                    // }
    
                    // Let's add an additional header to our response to the client.
                    // let headers = response.headers_mut();
                    // headers.append("MyCustomHeader", ":)".parse().unwrap());
                    // headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                    Ok(response)
                }
            ).unwrap();



            websocket.write_message(
                Message::text(
                    serde_json::json!({
                        "type": "msg",
                        "data": "eboy"
                    }).to_string()
                )
            ).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() || msg.is_text() {
                    let mut ws_msg: WSMsg<WSIncommingMsgPayload> = serde_json::from_str(&msg.to_string()).unwrap();
                    ws_msg.data.qwerty = 40;

                    let msg_to_send = serde_json::to_string(&ws_msg).unwrap();

                    websocket.write_message(
                        Message::text(msg_to_send)
                    ).unwrap();
                }
            }
        });
    }
}


fn run_simple_server(address: String) {
    let listener = net::TcpListener::bind(address).unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
fn handle_connection(mut stream: net::TcpStream) {
    let mut buff = [0; 1024];

    stream.read(&mut buff).unwrap();

    let (status_line, filename) = if buff.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buff.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };


    let html = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        html.len(),
        html
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
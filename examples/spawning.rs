// cargo run --example spawning
// cargo run --example hello_tokio

use std::collections::HashMap;

use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Failed to bind");
    println!(
        "Listening on {}",
        listener.local_addr().expect("Failed to get local address")
    );

    loop {
        println!("Waiting for a connection accept ready...");
        // Semantically, `await` on an async function aims to be non-blocking.
        // However, whether the runtime (Tokio) actually suspends the current task
        // to execute other tasks is determined by its scheduler.
        // Here, similar to a classic Blocking I/O Server Loop, waiting for `accept()` ready
        // behaves like blocking I/O from this task's perspective.
        // Even if polling the `accept()` Future returns `Poll::Pending`, the current task
        // cannot proceed (effectively blocking it) until the Future becomes `Poll::Ready`.
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("Accepted connection from {:?}", addr);
                // A new task is spawned for each inbound socket. The socket is
                // moved to the new task and processed there.
                tokio::spawn(async move {
                    process(socket).await;
                });
                println!("Finished processing connection from {:?}", addr);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {:?}", e);
            }
        }
        println!("Preparing to accept next connection...");
    }
}

async fn process(socket: TcpStream) {
    // A hashmap is used to store data
    let mut db = HashMap::new();

    let mut conn = Connection::new(socket);
    while let Ok(frame) = conn.read_frame().await {
        if let None = frame {
            continue;
        }
        let resp = match Command::from_frame(frame.unwrap()) {
            Ok(cmd) => match cmd {
                Command::Set(cmd) => {
                    let key = cmd.key().to_string();
                    let value = cmd.value().to_vec();
                    db.insert(key, value);
                    Frame::Simple("OK".to_string())
                }
                Command::Get(cmd) => {
                    let key = cmd.key().to_string();
                    let value = db.get(&key).unwrap();
                    Frame::Bulk(value.clone().into())
                }
                _ => panic!("unimplemented {:?}", cmd),
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
                Frame::Error(e.to_string())
            }
        };
        conn.write_frame(&resp).await.unwrap();
    }
}

// cargo run --example spawning
// cargo run --example hello_tokio

use mini_redis::{Connection, Frame};
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
                process(socket).await;
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
    let mut conn = Connection::new(socket);
    match conn.read_frame().await {
        Ok(frame) => {
            println!("Got frame: {:?}", frame);
            // Handle the frame (e.g., process a command)
            let resp = Frame::Error("Unsupported command".to_string());
            conn.write_frame(&resp)
                .await
                .expect("Failed to write frame");
        }
        Err(e) => {
            println!("Failed to read frame: {}", e);
        }
    }
}

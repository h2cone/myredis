// cargo run --example async_await
// cargo expand --example async_await

async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    // returns a value representing the operation
    let say_world = say_world();

    println!("hello");

    say_world.await;
}

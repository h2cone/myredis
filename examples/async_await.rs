// cargo run --example async_await
// cargo expand --example async_await

async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    // return value of an async fn is an anonymous type that implements the Future trait
    let say_world = say_world();

    println!("hello");

    // thread may do other work while the operation processes in the background
    say_world.await;
}

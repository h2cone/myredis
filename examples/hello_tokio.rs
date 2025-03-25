use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379")
        // .await
        // .expect("Failed to connect to server");
        .await?;
    client
        .set("foo", "bar".into())
        // .await
        // .expect("Failed to set value");
        .await?;
    let result = client
        .get("foo")
        // .await
        // .expect("Failed to get value")
        // .expect("Value not found");
        .await?
        .expect("Value not found");
    println!("Got value from the server: {:?}", result);
    Ok(())
}

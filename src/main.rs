mod money;
mod account;
mod inventory;
mod api;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Hello, world!");
    Ok(())
}

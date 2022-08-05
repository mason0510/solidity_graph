
//test
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config::load_config();
    println!("{:?}", config.mysql);
    Ok(())
}

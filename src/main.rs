use std::env;

use anyhow::Result;

use yoimiya::notifier::Bark;

#[tokio::main]
async fn main() -> Result<()> {
    let arg: Vec<String> = env::args().collect();
    let mut bark = Bark::new().device_key(arg[1].clone());
    bark.title("Test".to_string())
        .body("Test Body".to_string())
        .group("Test Group".to_string())
        .run()
        .await?;
    Ok(())
}

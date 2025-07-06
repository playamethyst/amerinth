use amerinth::{Modrinth, UserAgent, users};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let auth = Modrinth::new(
        false,
        Some(
            UserAgent::builder("amerinth/examples")
                .author("getamethyst")
                .version("0.0.0")
                .contact("playamethyst.com")
                .build(),
        ),
    )?
    .pat(std::env::var("PAT")?, 31, 7, 2025)?;

    let payload = users::get(&auth, "abstralexis").await?;
    println!("{:?}", payload);

    Ok(())
}

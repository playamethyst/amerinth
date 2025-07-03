use amerinth::{Modrinth, UserAgent, misc};

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
    .pat_expires("h".to_string(), 31, 12, 2025)?;

    let payload = misc::forge(&auth, "fabric-apdsadsai").await?;
    println!("{:?}", payload);

    Ok(())
}

use amerinth::{Modrinth, UserAgent, misc};

#[tokio::main]
async fn main() -> Result<(), amerinth::ModrinthError> {
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

    let payload = misc::forge(&auth, "fabric-api").await?;
    println!("{:?}", payload);

    Ok(())
}

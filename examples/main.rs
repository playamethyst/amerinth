use amerinth::{Modrinth, UserAgent, misc};

#[tokio::main]
async fn main() -> Result<(), amerinth::ModrinthError> {
    let auth = Modrinth::new(Some(
        UserAgent::builder("amerinth/examples")
            .author("getamethyst")
            .version("0.0.0")
            .contact("playamethyst.com")
            .build(),
    ))?
    .pat_expires("h".to_string(), 31, 12, 2025)?;
    let statistics = misc::statistics(&auth).await;
    println!("{:?}", statistics);

    Ok(())
}

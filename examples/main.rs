use amerinth::{ModrinthAuth, UserAgent};

fn main() -> Result<(), amerinth::Error> {
    let auth = ModrinthAuth::new(Some(
        UserAgent::builder("amerinth/examples")
            .author("getamethyst")
            .version("0.0.0")
            .contact("playamethyst.com")
            .build(),
    ))?
    .pat_expires("h".to_string(), 31, 12, 2025)?;

    Ok(())
}

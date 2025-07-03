use amerinth::{ModrinthAuth, UserAgent};

fn main() {
    let auth = ModrinthAuth::new(Some(
        UserAgent::builder("amerinth/examples")
            .author("getamethyst")
            .version("0.0.0")
            .contact("playamethyst.com")
            .build(),
    ));
}

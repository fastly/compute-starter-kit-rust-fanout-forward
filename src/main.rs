use fastly::{Error, Request};

fn main() -> Result<(), Error> {
    // Log service version.
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    let req = Request::from_client();

    Ok(req.handoff_fanout("origin")?)
}

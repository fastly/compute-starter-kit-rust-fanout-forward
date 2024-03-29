use fastly::experimental::RequestUpgradeWebsocket;
use fastly::{Body, Error, Request};

fn take_non_empty_body(req: &mut Request) -> Result<Option<Body>, Error> {
    let mut chunks = req.get_body_mut().read_chunks(16_384);

    let chunk = match chunks.next() {
        Some(chunk) => chunk,
        None => return Ok(None),
    };

    let mut body: Vec<u8> = Vec::new();
    body.extend(chunk?);

    for chunk in chunks {
        body.extend(chunk?);
    }

    Ok(Some(Body::from(body)))
}

fn main() -> Result<(), Error> {
    // Log service version.
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    let mut req = Request::from_client();

    if let Some(body) = take_non_empty_body(&mut req)? {
        // Currently, Fanout only supports requests with empty bodies.
        // If the request has a body, forward it to the backend directly.

        return Ok(req.with_body(body).send("origin")?.send_to_client());
    }

    Ok(req.handoff_fanout("origin")?)
}

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
    let mut req = Request::from_client();

    if let Some(body) = take_non_empty_body(&mut req)? {
        // currently, Fanout only supports requests with empty bodies.
        // if the request has a body, forward it to the backend directly

        return Ok(req.with_body(body).send("backend")?.send_to_client());
    }

    Ok(req.handoff_fanout("backend")?)
}

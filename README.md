# Fanout forward Compute starter kit for Rust

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

Learn about [Fastly Compute with Fanout](https://www.fastly.com/documentation/guides/concepts/real-time-messaging/fanout/) using a basic starter that sends connections through the Fanout GRIP proxy to a backend.

**For more details about this and other starter kits for Compute, see the [Fastly Documentation Hub](https://www.fastly.com/documentation/solutions/starters/)**.

## Local testing

To test Fanout features in the local testing environment, first obtain [Pushpin](https://pushpin.org), the open-source GRIP proxy server that Fastly Fanout is based upon, and make sure it is available on the system path.

Create a Fastly Compute project based on this starter kit.

```term
mkdir my-fanout-project
cd my-fanout-project
fastly compute init --from=https://github.com/fastly/compute-starter-kit-rust-fanout-forward
```

The `fastly.toml` file included in this starter kit includes a `local_server.pushpin` section:
```toml
[local_server.pushpin]
enable = true
```

It also sets the backend named `"origin"` to `localhost:3000`.

Run the starter kit:
```term
fastly compute serve
```

The Fastly CLI starts Pushpin and then starts the starter kit app at http://localhost:7676/.

On the local testing environment, data can be sent to the connections via the GRIP publish endpoint at `http://localhost:5561/publish/`.

> **IMPORTANT:** Specifying remote backends when testing Fanout handoff in the local development environment is valid. However, because the publishing API runs locally, it is often convenient to also locally run any backends or applications that need to test publishing.

## Running the application on Fastly

The app expects a configured backend named "origin" that points to an origin server. For example, if the server is available at domain `example.com`, then you'll need to create a backend on your Compute service named "origin" with the destination host set to `example.com` and port `443`. Also set `Override Host` to the same host value.

You'll also need to [enable Fanout](https://www.fastly.com/documentation/guides/concepts/real-time-messaging/fanout/#enable-fanout) on your Fastly service to run this application. After deploying the app, you can enable Fanout on your service, type:

```shell
fastly products --enable=fanout
```

After setting up the backend configuration and enabling Fanout, incoming HTTP and WebSocket requests that arrive at the service will be processed by the fetch handler:

   1. WebSocket connections will be handed off to Fanout to reach the backend server. Fanout maintains a long-lived connection with the client, and uses the [WebSocket-over-HTTP protocol](https://pushpin.org/docs/protocols/websocket-over-http/) to transform the messages to and from the backend server.

   2. HTTP GET and HEAD requests will be handed off to Fanout to reach the backend server. The backend can include [GRIP control messages](https://pushpin.org/docs/protocols/grip/) in its response, instructing Fanout to maintain a long-lived connection with the client.

The GRIP publish endpoint is at `https://api.fastly.com/service/{service-id}/publish/`.

## Next Steps

The starter kit is written to send hand off all WebSocket and HTTP GET (and HEAD) traffic through Fanout. In an actual app we would be selective about which requests are handed off through Fanout, because requests that are handed off through Fanout do not pass through the Fastly cache.

For details, see [What to hand off to Fanout](https://www.fastly.com/documentation/guides/concepts/real-time-messaging/fanout/#what-to-hand-off-to-fanout) in the Developer Documentation.

The starter kit code contains a TODO section where you may insert additional conditions to check before setting the `use_fanout` variable to `true`.

For example, to check the request for the existence of a certain header:

```rust
  if let Some(_) = req.get_header("fanout") {
    use_fanout = true;
  }
```

## Notes

The code in this starter kit cannot be used with the [`fastly::main` attribute](https://docs.rs/fastly/0.11.2/fastly/attr.main.html) on the `main()` entry point. This is because a function decorated with `fastly::main` is expected to return a response, but handing off to Fanout is an action that does not create a response. Use an undecorated `main()` function instead, and use `Request::from_client()` and `Response::send_to_client()` as needed.

## Security issues

Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.

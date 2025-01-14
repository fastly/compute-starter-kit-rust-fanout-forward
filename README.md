# Fanout forward Compute starter kit for Rust

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

Learn about [Fastly Compute with Fanout](https://www.fastly.com/documentation/guides/concepts/real-time-messaging/fanout/) using a basic starter that sends connections through the Fanout GRIP proxy to a backend.

**For more details about this and other starter kits for Compute, see the [Fastly Documentation Hub](https://www.fastly.com/documentation/solutions/starters/)**.

## Setup

The app expects a configured backend named "origin" that points to an origin server. For example, if the server is available at domain `example.com`, then you'll need to create a backend on your Compute service named "origin" with the destination host set to `example.com` and port `443`. Also set `Override Host` to the same host value.

You'll also need to [enable Fanout](https://www.fastly.com/documentation/guides/concepts/real-time-messaging/fanout/#enable-fanout) on your Fastly service to run this application. To enable Fanout on your service, type:

```shell
fastly products --enable=fanout
```

> [!NOTE]
> This app is not currently supported in Fastly's [local development server](https://www.fastly.com/documentation/guides/compute/testing/#running-a-local-testing-server), as the development server does not support Fanout features. To experiment with Fanout, you will need to publish this project to your Fastly Compute service. using the `fastly compute publish` command.

## Running the application

After deploying the app and setting up the backend configuration, incoming HTTP and WebSocket requests that arrive at the service will be processed by the fetch handler:

1. WebSocket connections will be handed off to Fanout to reach the backend server. Fanout maintains a long-lived connection with the client, and uses the [WebSocket-over-HTTP protocol](https://pushpin.org/docs/protocols/websocket-over-http/) to transform the messages to and from the backend server.

2. HTTP GET and HEAD requests will be handed off to Fanout to reach the backend server. The backend can include [GRIP control messages](https://pushpin.org/docs/protocols/grip/) in its response, instructing Fanout to maintain a long-lived connection with the client.

## Next Steps

The starter kit is written to send all WebSocket and HTTP GET (and HEAD) traffic to Fanout. In an actual app we would be selective about which requests are handed off to Fanout, because requests that are handed off to Fanout do not pass through the Fastly cache.

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

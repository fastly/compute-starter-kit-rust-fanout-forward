# Compute@Edge Fanout forward starter kit for Rust

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

Learn about Fastly Compute@Edge with Fanout using a basic starter that sends connections through the Fanout GRIP proxy to a backend.

**For more details about this and other starter kits for Compute@Edge, see the [Fastly Developer Hub](https://developer.fastly.com/solutions/starters/)**.

## Setup

The app expects a configured backend named "backend" that points to a backend server. For example, if the server is available at domain `example.com`, then you'll need to create a backend on your Compute@Edge service named "backend" with the destination host set to `example.com` and port 443. Also set 'Override Host' to the same host value.

After deploying the app and setting up the backend configuration, all connections received by the service will be passed through the Fanout proxy to the backend. If WebSocket-over-HTTP mode is enabled on your service, then client WebSocket activity will be converted into HTTP when sending to the backend.

## Note

This app is not currently supported in Fastly's [local development server](https://developer.fastly.com/learning/compute/testing/#running-a-local-testing-server), as the development server does not support Fanout features. To experiment with Fanout, you will need to publish this project to your Fastly Compute@Edge service. using the `fastly compute publish` command.

## Security issues

Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.

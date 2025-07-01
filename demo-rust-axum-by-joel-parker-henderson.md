# Demo of Rust and axum web framework

Demonstration of:

- [Rust](https://www.rust-lang.org): programming language that focuses on reliability and stability.

- [axum](https://crates.io/crates/axum): web framework that focuses on ergonomics and modularity.

- [tower](https://crates.io/crates/tower): library for building robust clients and servers.

- [hyper](https://hyper.rs/): fast and safe HTTP library for the Rust language.

- [tokio](https://tokio.rs): platform for writing asynchronous I/O backed applications.

- [Serde](https://crates.io/crates/serde): serialization/deserialization framework.

## Feedback

Have an idea, suggestion, or feedback? Let us know via GitHub issues.

Have a code improvement or bug fix? We welcome GitHub pull requests.

## License

This demo uses the license Creative Commons Share-and-Share-Alike.

## Thanks

Thanks to all the above projects and their authors. Donate to them if you can.

Does this demo help your work? Donate here if you can via GitHub sponsors.

## Contact

Have feedback? Have thoughts about this? Want to contribute?

Contact the maintainer at <joel@joelparkerhenderson.com>.

---

# What is this?

This demo is a tutorial that teaches how to build features from the ground up
with axum and its ecosystem of tower middleware, hyper HTTP library, tokio
asynchronous platform, and Serde data conversions.

## What will you learn?

- Create a project using Rust and the axum web framework.

- Leverage capabilities of a hyper server and tower middleware.

- Create axum router routes and their handler functions.

- Create responses with HTTP status code OK and HTML text.

- Create a binary image and respond with a custom header.

- Handle HTTP verbs including GET, PUT, PATCH, POST, DELETE.

- Use axum extractors for query parameters and path parameters.

- Manage a data store and access it using RESTful routes.

- Create a tracing subscriber and emit tracing events.

## What is required?

Some knowledge of Rust programming is required, such as:

- How to create a Rust project, build it, and run it.

- How to write functions and their parameters

- How to use shell command line tools such as curl.

Some knowledge about web frameworks is required, such as:

- The general concepts of HTTP requests and responses.

- The general concepts of of RESTful routes and resources.

- The general concepts of formats for HTML, JSON, and text.

## What is helpful?

Some knowledge of web frameworks is helpful, such as:

- Rust web frameworks, such as Actix, Rocket, Warp, etc.

- Other languages' web frameworks, such as Rails, Phoenix, Express, etc.

- Other web-related frameworks, such as React, Vue, Svelte, etc.

Some knowledge of this stack can be helpful, such as:

- middleware programming e.g. with tower

- asynchronous application programming e.g. with tokio

- HTTP services programming e.g. with hyper

############
**Examples:**

- [Hello, World](examples/axum-hello-world/README.md)

- [Fallback router](examples/axum-fallback-router/README.md)

- [Graceful shutdown](examples/axum-graceful-shutdown/README.md)

**Examples:**

- [Hello handler function](examples/axum-hello-handler-function/README.md)

- [Epoch handler function](examples/axum-epoch-handler-function/README.md)

- [Uptime handler function](examples/axum-uptime-handler-function/README.md)

- [Count handler function](examples/axum-count-handler-function/README.md)

## Create axum routes and axum handlers

An axum "route" is how you declare you want to receive a specific inbound HTTP
requests; then you send the request to an axum "handler" function to do the
work and return a response.

This section shows how to:

- [Respond with a string of HTML](examples/axum-respond-with-a-string-of-html/README.md)

- [Respond with a file of HTML](examples/axum-respond-with-a-file-of-html/README.md)

- [Respond with HTTP status code](examples/axum-respond-with-http-status-code-ok/README.md)

- [Respond with the request URI](examples/axum-respond-with-the-request-uri/README.md)

- [Respond with a custom header and a custom image](examples/axum-respond-with-a-custom-header-and-a-custom-image/README.md)

- [Respond with a JSON payload](examples/axum-respond-with-a-json-payload/README.md)

- [Respond to multiple HTTP verbs](examples/axum-respond-to-multiple-http-verbs/README.md)

## Extractors

An axum "extractor" is how you pick apart the incoming request in order to get
any parts that your handler needs.

This section shows how to:

- [Extract path parameters](examples/axum-extract-path-parameters/README.md)

- [Extract query parameters](examples/axum-extract-query-parameters/README.md)

- [Extract JSON parameters](examples/axum-extract-json-parameters-parameters/README.md)

**More:**

- [RESTful routes and resources](examples/axum-restful-routes-and-resources/README.md)

- [Tracing subscriber](examples/axum-tracing-subscriber/README.md)

- [Bind + host + port + socket address](examples/axum-bind-host-port-socket-address/README.md)

**Ending:**

- [axum repository examples](doc/97-axum-repository-examples.md)

- [Ideas for next steps](doc/98-ideas-for-next-steps.md)

- [Conclusion](doc/99-conclusion.md)

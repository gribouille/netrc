# netrc

A [netrc](https://www.gnu.org/software/inetutils/manual/html_node/The-_002enetrc-file.html)
library for Rust, with support for [reqwest](https://crates.io/crates/reqwest)
via [reqwest-middleware](https://crates.io/crates/reqwest-middleware).


## reqwest-netrc

The `reqwest-netrc` crate is a middleware for [reqwest](https://crates.io/crates/reqwest)
to support the `netrc` file.

[![crates.io][crates-badge-2]][crate-url-2]
[![Documentation][doc-badge-2]][doc-url-2]
[![MIT licensed][mit-badge]][mit-url]
[![CI][actions-badge-2]][actions-url-2]


### Usage

To bring this crate into your repository, either add `reqwest-netrc` to your
`Cargo.toml`, or run:

```
> cargo add reqwest-netrc
```

### Example

The common scenario is to have a `~/.netrc` file or the `NETRC` environement variable defined:

```rust
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_netrc::NetrcMiddleware;

// ...

let client = ClientBuilder::new(Client::builder().build().unwrap())
    .with_init(NetrcMiddleware::new().unwrap())
    .build();

let res = client.get("https://domain.io/api/hello").send().await;

// ...
```

## rust-netrc

The `rust-netrc` crate is a parser for the `netrc` files.

[![crates.io][crates-badge]][crate-url]
[![Documentation][doc-badge]][doc-url]
[![MIT licensed][mit-badge]][mit-url]
[![CI][actions-badge]][actions-url]

### Usage

To bring this crate into your repository, either add `rust-netrc` to your
`Cargo.toml`, or run:

```
> cargo add rust-netrc
```

### Example

```rust
use netrc::Netrc;

fn main() {
  let nrc = Netrc::new().unwrap();

  for (host, auth) in nrc.hosts {
      println!("{host}: {auth:?}");
  }
}
```


## Contributing

Feedback and contributions are very welcome.


## License

This project is licensed under [MIT](./LICENSE).


[mit-badge]: https://img.shields.io/crates/l/rust-netrc.svg
[mit-url]: ./LICENSE

[crates-badge]: https://img.shields.io/crates/v/rust-netrc.svg
[crate-url]: https://crates.io/crates/rust-netrc
[doc-badge]: https://docs.rs/rust-netrc/badge.svg
[doc-url]: https://docs.rs/rust-netrc
[actions-badge]: https://github.com/gribouille/netrc/actions/workflows/rust-netrc.yml/badge.svg
[actions-url]: https://github.com/gribouille/netrc/actions/workflows/rust-netrc.yml

[crates-badge-2]: https://img.shields.io/crates/v/reqwest-netrc.svg
[crate-url-2]: https://crates.io/crates/reqwest-netrc
[doc-badge-2]: https://docs.rs/reqwest-netrc/badge.svg
[doc-url-2]: https://docs.rs/reqwest-netrc
[actions-badge-2]: https://github.com/gribouille/netrc/actions/workflows/reqwest-netrc.yml/badge.svg
[actions-url-2]: https://github.com/gribouille/netrc/actions/workflows/reqwest-netrc.yml
# netrc

[![crates.io](crates-badge)](crate-url)
[![Documentation](doc-badge)](doc-url)
[![MIT licensed](mit-badge)](mit-url)
[![CI](actions-badge)](actions-url)

A well-tested [netrc](netrc-url) library for Rust, with support for [Reqwest](reqwest-url)
via [Reqwest Middleware](reqwest-middleware-url).

[crates-badge]: https://img.shields.io/crates/v/rust-netrc.svg
[crate-url]: https://crates.io/crates/rust-netrc
[doc-badge]: https://docs.rs/rust-netrc/badge.svg
[doc-url]: https://docs.rs/rust-netrc
[mit-badge]: https://img.shields.io/crates/l/rust-netrc.svg
[mit-url]: ./LICENSE
[actions-badge]: https://github.com/gribouille/netrc/workflows/CI/badge.svg
[actions-url]: https://github.com/gribouille/netrc/actions?query=workflow%3ACI+branch%3Amaster
[netrc-url]: https://www.gnu.org/software/inetutils/manual/html_node/The-_002enetrc-file.html
[serde-url]: https://crates.io/crates/serde
[reqwest-url]: https://crates.io/crates/reqwest
[reqwest-middleware-url]: https://crates.io/crates/reqwest-middleware

## Usage

```
> cargo add rust-netrc
```

Basic usage:

```rust
use netrc::Netrc;

fn main() {
  let nrc = Netrc::new();

  println!(nrc.hosts["my.host"].login);
  println!(nrc.hosts["my.host"].account);
  println!(nrc.hosts["my.host"].password);
}
```

With [Reqwest](reqwest-url):

TODO


## Contributing

Feedback and contributions are very welcome.


## License

This project is licensed under [MIT](./LICENSE).
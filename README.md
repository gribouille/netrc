# netrc

[![crates.io][crates-badge]](crate-url)
[![Documentation][doc-badge]](doc-url)
[![MIT licensed][mit-badge]](mit-url)
[![CI][actions-badge]](actions-url)

A well-tested [netrc](https://www.gnu.org/software/inetutils/manual/html_node/The-_002enetrc-file.html)
library for Rust, with support for [Reqwest](https://crates.io/crates/reqwest)
via [Reqwest Middleware](https://crates.io/crates/reqwest-middleware).

[crates-badge]: https://img.shields.io/crates/v/rust-netrc.svg
[crate-url]: https://crates.io/crates/rust-netrc
[doc-badge]: https://docs.rs/rust-netrc/badge.svg
[doc-url]: https://docs.rs/rust-netrc
[mit-badge]: https://img.shields.io/crates/l/rust-netrc.svg
[mit-url]: ./LICENSE
[actions-badge]: https://github.com/gribouille/netrc/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/gribouille/netrc/actions/workflows/rust.yml

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

With `reqwest`:

TODO


## Contributing

Feedback and contributions are very welcome.


## License

This project is licensed under [MIT](./LICENSE).
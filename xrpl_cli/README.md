# XRPL CLI

A CLI for the XRP Ledger.

[![Crates.io](https://img.shields.io/crates/v/xrpl_cli)](https://crates.io/crates/xrpl_cli)

More information about this crate can be found in the [crate documentation][docs].

## Setup

To install the binary from source, run:

```sh
cargo install --path .
```

## Usage

```sh
xrpl --help

xrpl account ADDRESS info
xrpl account ADDRESS info --json --pretty
xrpl account ADDRESS info -jp

xrpl account ADDRESS balances
xrpl account ADDRESS balances --json --pretty
xrpl account ADDRESS balances -jp

xrpl account ADDRESS offers
xrpl account ADDRESS offers --pretty
xrpl account ADDRESS offers --json --pretty
xrpl account ADDRESS offers -jp

xrpl account ADDRESS offers list

xrpl account ADDRESS --public-key=".." --secret-key=".." offers remove <SEQUENCE>

xrpl ledger --closed
```

The `RUST_LOG` env variable is used to configure tracing, e.g.

```sh
RUST_LOG=debug xrpl account ADDRESS info -jp
```

## Status

This work is under active development and the API is expected to change.

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## License

This work is licensed under the Apache-2.0 License. See [LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/Apache-2.0.html> for details.

## Copyright

Copyright © 2022 [Georgios Moschovitis](https://gmosx.ninja).

[docs]: https://docs.rs/xrpl_cli
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

xrpl ledger --closed
```

The `RUST_LOG` env variable is used to configure tracing, e.g.

```sh
RUST_LOG=debug xrpl account ADDRESS info -jp
```

## Status

The software is under active development and the API is expected to change.

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## Contact

For questions, suggestions, etc, you can reach the maintainer on [Twitter](https://twitter.com/gmosx).

## License

The software is distributed under the terms of both the MIT license and the Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

## Copyright

Copyright © 2022 [George Moschovitis](https://gmosx.ninja).

[docs]: https://docs.rs/xrpl_cli
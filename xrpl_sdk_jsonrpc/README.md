# XRP Ledger JSONRPC client

A strongly-typed client for the XRP Ledger JSONRPC API.

This crate is an *unofficial*, community-driven effort.

[![Crates.io](https://img.shields.io/crates/v/xrpl_sdk_jsonrpc)](https://crates.io/crates/xrpl_sdk_jsonrpc)
[![Documentation](https://docs.rs/xrpl_sdk_jsonrpc/badge.svg)](https://docs.rs/xrpl_sdk_jsonrpc)

More information about this crate can be found in the [crate documentation][docs].

## Installation

```toml
[dependencies]
xrpl_sdk_jsonrpc = "0.6"
```

## Usage

```rust
let client = Client::new();

let account = env::var("XRPL_ACCOUNT_ADDRESS").expect("account not defined");

let req = AccountTxRequest::new(&account).limit(5);
let resp = client.call(req).await;

dbg!(&resp);
```

## Links

- https://github.com/XRPLF/xrpl-py

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

Copyright © 2021-2022 [George Moschovitis](https://gmosx.ninja).

[docs]: https://docs.rs/xrpl_sdk_jsonrpc
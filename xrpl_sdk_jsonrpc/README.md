# XRP Ledger JSONRPC client

A strongly-typed client for the XRP Ledger HTTP JSONRPC API.

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

## Status

This work is under active development and the API is expected to change.

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## License

This work is licensed under the Apache-2.0 License. See [LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/Apache-2.0.html> for details.

## Copyright

Copyright © 2022 [Georgios Moschovitis](https://gmosx.ninja).

[docs]: https://docs.rs/xrpl_sdk_jsonrpc
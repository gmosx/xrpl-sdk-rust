# XRP Ledger WebSocket Client

A strongly-typed client for the XRP Ledger WebSocket API.

This crate is an *unofficial*, community-driven effort.

[![Crates.io](https://img.shields.io/crates/v/xrpl_sdk_ws)](https://crates.io/crates/xrpl_sdk_ws)
[![Documentation](https://docs.rs/xrpl_sdk_ws/badge.svg)](https://docs.rs/xrpl_sdk_ws)

More information about this crate can be found in the [crate documentation][docs].

## Usage

```rust
let mut client = Client::connect(DEFAULT_WS_URL)
    .await
    .expect("cannot connect");

let req = AccountInfoRequest::new("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59").strict(true);

client.call(req).await.expect("cannot send request");

if let Some(msg) = client.messages.next().await {
    dbg!(&msg);
}
```

## Status

This work is under active development and the API is expected to change.

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## License

This work is licensed under the Apache-2.0 License. See [LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/Apache-2.0.html> for details.

## Copyright

Copyright © 2022 [Georgios Moschovitis](https://gmosx.ninja).

[docs]: https://docs.rs/xrpl_sdk_ws
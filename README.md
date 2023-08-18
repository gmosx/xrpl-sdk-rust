# XRP Ledger SDK

A Rust SDK for working with [XRP Ledger](https://xrpl.org) APIs.

This project is an *unofficial*, community-driven effort.

## Components

The SDK contains the following high-level crates:

- [xrpl_http_client](xrpl_http_client/)
- [xrpl_ws_client](xrpl_ws_client/)

Additionally, low-level crates are provided:

- [xrpl_types](xrpl_types/)
- [xrpl_api](xrpl_api/)
- [xrpl_address_codec](xrpl_address_codec/)
- [xrpl_binary_codec](xrpl_binary_codec/)

Finally, a convenient CLI is provided to demonstrate example usage:

- [xrpl_cli](xrpl_cli/)

## Usage

### JSONRPC Client example

```rust
let client = Client::new();

let account = env::var("XRPL_ACCOUNT_ADDRESS").expect("account not defined");

let req = AccountTxRequest::new(&account).limit(5);
let resp = client.call(req).await;

dbg!(&resp);
```

### WebSocket Client example

```rust
let mut client = Client::connect(DEFAULT_WS_URL)
    .await
    .expect("cannot connect");

let account = env::var("XRPL_ACCOUNT_ADDRESS").expect("account not defined");

let req = AccountInfoRequest::new(&account).strict(true);
client.call(req).await.expect("cannot send request");

if let Some(msg) = client.messages.next().await {
    dbg!(&msg);
}
```

## Links

- [Github Repository](https://github.com/gmosx/xrpl-sdk-rust)

## Status

This work is under active development (pre-alpha) and the API is expected to
change. It's not considered ready for use in production.

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## License

This work is licensed under the Apache-2.0 License. See [LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/Apache-2.0.html> for details.

## Copyright

Copyright © 2022 [Georgios Moschovitis](https://gmosx.ninja).

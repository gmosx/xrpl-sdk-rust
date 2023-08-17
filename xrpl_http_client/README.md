# XRP Ledger JSONRPC client

A strongly-typed client for the XRP Ledger HTTP JSONRPC API.

This crate is an *unofficial*, community-driven effort.

[![Crates.io](https://img.shields.io/crates/v/xrpl_http_client)](https://crates.io/crates/xrpl_http_client)
[![Documentation](https://docs.rs/xrpl_http_client/badge.svg)](https://docs.rs/xrpl_http_client)

More information about this crate can be found in the [crate documentation][docs].

## Installation

```toml
[dependencies]
xrpl_http_client = "0.12"
```

## Usage

```rust
let client = Client::new();

let account = "...";

let req = AccountTxRequest::new(&account).limit(5);
let resp = client.call(req).await;

dbg!(&resp);
```

```rust
let client = Client::new();

let account = "...";
let public_key = "...";
let secret_key = "...";

let offer_sequence = 123; // the sequence of the offer to cancel

let tx = Transaction::offer_cancel(account, offer_sequence);

let tx = client.prepare_transaction(tx).await?;

let public_key = hex::decode(public_key)?;
let secret_key = hex::decode(secret_key)?;

let tx = sign_transaction(tx, &public_key, &secret_key);

let tx_blob = serialize_transaction_to_hex(&tx);

let req = SubmitRequest::new(&tx_blob);
let resp = client.call(req).await?;

dbg!(resp);
```

## Status

This work is under active development and the API is expected to change.

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## License

This work is licensed under the Apache-2.0 License. See [LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/Apache-2.0.html> for details.

## Copyright

Copyright © 2022 [Georgios Moschovitis](https://gmosx.ninja).

[docs]: https://docs.rs/xrpl_http_client
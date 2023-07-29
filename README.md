# XRP Ledger SDK

A Rust SDK for working with [XRP Ledger](https://xrpl.org) APIs.

This project is an *unofficial*, community-driven effort.

## Components

The SDK contains the following high-level crates:

- [xrpl_sdk_jsonrpc](xrpl_sdk_jsonrpc/)
- [xrpl_sdk_ws](xrpl_sdk_ws/)

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

- [Github Repository](https://github.com/gmosx/xrpl_sdk_rust)

## Status

The software is under active development (pre-alpha) and the API is expected to
change. It's not considered ready for use in production.

### Roadmap

- Support for WebSocket API
- Introduce XRPL command line client (CLI)

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## Contact

For questions, suggestions, etc, you can reach the maintainer on [Twitter](https://twitter.com/gmosx).

## License

Each component of the XRPL SDK is individually licensed. Please check the corresponding directories for more details.

- xrpl_sdk_jsonrpc: [LICENSE-MIT](xrpl_sdk_jsonrpc/LICENSE-MIT), [LICENSE-APACHE](xrpl_sdk_jspnrpc/LICENSE-APACHE)
- xrpl_sdk_ws: [LICENSE-MIT](xrpl_sdk_ws/LICENSE-MIT), [LICENSE-APACHE](xrpl_sdk_ws/LICENSE-APACHE)
- xrpl_types: [LICENSE-MIT](xrpl_types/LICENSE-MIT), [LICENSE-APACHE](xrpl_types/LICENSE-APACHE)
- xrpl_api: [LICENSE-MIT](xrpl_types/LICENSE-MIT), [LICENSE-APACHE](xrpl_types/LICENSE-APACHE)
- xrpl_address_codec: [LICENSE-MIT](xrpl_address_codec/LICENSE-MIT), [LICENSE-APACHE](xrpl_address_codec/LICENSE-APACHE)
- xrpl_binary_codec: [LICENSE-MIT](xrpl_binary_codec/LICENSE-MIT), [LICENSE-APACHE](xrpl_binary_codec/LICENSE-APACHE)
- xrpl_cli: [LICENSE-MIT](xrpl_cli/LICENSE-MIT), [LICENSE-APACHE](xrpl_cli/LICENSE-APACHE)

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

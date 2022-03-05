# XRP Ledger JSONRPC client

A strongly-typed client for the XRP Ledger JSONRPC API.

This crate is an *unofficial*, community-driven effort.

## Installation

```toml
[dependencies]
xrpl_sdk_jsonrpc = "0.6"
```

## Usage

```rust
    let client = Client::new();

    let account = env::var("XRPL_ACCOUNT_ADDRESS").unwrap();

    let resp = client.account_tx(&account).limit(5).send().await;

    dbg!(&resp);
```

## FAQ

### Why provide both execute and send methods for API endpoint handlers?

Providing the lower-level `execute` method allows for more flexibility. Since `execute` is generic you can pass any type of object to deserialize the response to, e.g. you could deserialize to a `HashMap` instead of the 'default' response for each API call. Or you could use a custom struct with only the fields you are interested in.

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

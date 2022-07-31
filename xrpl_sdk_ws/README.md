# XRP Ledger WebSocket Client

A strongly-typed client for the XRP Ledger WebSocket API.

This crate is an *unofficial*, community-driven effort.

## Usage

```rust
let mut client = Client::connect(DEFAULT_WS_URL)
    .await
    .expect("cannot connect");

let req = SubscribeRequest::streams(&["ledger"]);
client.call(req).await.expect("cannot subscribe");

let (_, rx) = client.stream.split();

tokio::pin!(rx);

while let Some(msg) = rx.next().await {
    dbg!(&msg);
}
```

## Status

Under construction, nothing to see here.

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
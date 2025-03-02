# XRP Ledger Types

Core types and related functions for the XRP Ledger. Reused between Web and
WebSocket clients and potentially for server-side code.

The types in this crate are used when sending requests to the API.

[![Crates.io](https://img.shields.io/crates/v/xrpl_types)](https://crates.io/crates/xrpl_types)
[![Documentation](https://docs.rs/xrpl_types/badge.svg)](https://docs.rs/xrpl_types)

More information about this crate can be found in the [crate documentation][docs].

## Status

This work is under active development and the API is expected to change.

## `no_std` support

This crate is `no_std` compatible when disabling the default `std` feature.

```toml
xrpl_types = { version = "0.16.0", default-features = false }
```

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## License

This work is licensed under the Apache-2.0 License. See [LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/Apache-2.0.html> for details.

## Copyright

Copyright © 2022 [Georgios Moschovitis](https://gmosx.ninja).

[docs]: https://docs.rs/xrpl_types
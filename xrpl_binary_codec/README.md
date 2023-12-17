# XRP Ledger Binary Codec

Binary serialization for XRPL Protocol objects.

[![Crates.io](https://img.shields.io/crates/v/xrpl_binary_codec)](https://crates.io/crates/xrpl_binary_codec)
[![Documentation](https://docs.rs/xrpl_binary_codec/badge.svg)](https://docs.rs/xrpl_binary_codec)

More information about this crate can be found in the [crate documentation][docs].

## Links

- https://github.com/XRPLF/xrpl.js/tree/main/packages/ripple-binary-codec
- https://github.com/ripple/rippled/tree/develop/src/ripple/protocol

## Status

This work is under active development and the API is expected to change.

## `no_std` support

This crate is `no_std` compatible when disabling the default `std` feature.

```toml
xrpl_binary_codec = { version = "0.15.0", default-features = false }
```

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## License

This work is licensed under the Apache-2.0 License. See [LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/Apache-2.0.html> for details.

## Copyright

Copyright © 2022 [Georgios Moschovitis](https://gmosx.ninja).

[docs]: https://docs.rs/xrpl_binary_codec
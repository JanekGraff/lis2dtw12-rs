# LIS2DTW12-RS

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![dependency status][deps-image]][deps-link]
![MIT licensed][license-image]

A platform agnostic driver to interface with the LIS2DTW12 (3-axis accelerometer + temperature sensor).
The driver uses the `embedded-hal` traits and supports interfaces with I2C and SPI.
The driver supports async and blocking modes, selectable with the `async` and `blocking` features.

## Resources

- [LIS2DTW12 product page][product-page]
- [LIS2DTW12 datasheet][datasheet]

## Running examples

### NOTE

You may need to adjust the features for embassy depending on the chip you're using. This is just a configuration i was using when developing this.

The example can be run like this:

```bash
cargo run --example stm32l431-embassy-async --no-default-features --features "async"
```

## Contributing

If you have any problems feel free to open an issue, if i find the time i might review and fix it.

Also feel free to open PRs if you miss some features or find bugs. PRs for documentation, tests, examples, etc. are also very welcome!

## License

Dual licensed under your choice of either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[crate-image]: https://img.shields.io/crates/v/lis2dtw12.svg
[crate-link]: https://crates.io/crates/lis2dtw12
[docs-image]: https://docs.rs/lis2dtw12/badge.svg
[docs-link]: https://docs.rs/lis2dtw12/
[build-image]: https://github.com/JanekGraff/lis2dtw12-rs/actions/workflows/ci.yml/badge.svg?branch=main
[build-link]: https://github.com/JanekGraff/lis2dtw12-rs/actions
[deps-image]: https://deps.rs/repo/github/janekgraff/lis2dtw12-rs/status.svg
[deps-link]: https://deps.rs/repo/github/janekgraff/lis2dtw12-rs/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[product-page]: https://www.st.com/en/mems-and-sensors/lis2dtw12.html
[datasheet]: https://www.st.com/resource/en/datasheet/lis2dtw12.pdf
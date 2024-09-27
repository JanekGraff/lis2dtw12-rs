# LIS2DTW12-RS

<!-- [![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link] -->
[![Build Status][build-image]][build-link]
<!-- [![dependency status][deps-image]][deps-link] -->
![MIT licensed][license-image]

A platform agnostic driver to interface with the LIS2DTW12 (3-axis accelerometer + temperature sensor).
The driver uses the `embedded-hal` traits and supports interfaces with I2C and SPI.
The driver supports async and blocking modes, selectable with the `async` and `blocking` features.

## Resources

- [LIS2DH12 product page][product-page]
- [LIS2DH12 datasheet][datasheet]

## NOTE

This crate is still a WIP and at this point, it is just a theoretical implementation of the datasheet as my sensor has not arrived yet.

## Contributing

If you have any problems feel free to open an issue, if i find the time i might review and fix it.

Also feel free to open PRs if you miss some features or find bugs. PRs for documentation, tests, examples, etc. are also very welcome!

## License

Dual licensed under your choice of either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

<!-- [crate-image]: TBD
[crate-link]: TBD
[docs-image]: TBD
[docs-link]: TBD -->
[build-image]: https://github.com/JanekGraff/lis2dtw12-rs/actions/workflows/ci.yml/badge.svg?branch=main
[build-link]: https://github.com/JanekGraff/lis2dtw12-rs/actions
<!-- [deps-image]: TBD
[deps-link]: TBD -->
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[product-page]: https://www.st.com/en/mems-and-sensors/lis2dtw12.html
[datasheet]: https://www.st.com/resource/en/datasheet/lis2dtw12.pdf
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.1]

### Changed

- Example structure to support multiple examples in one crate.

### Added

- Example for tap detection

### Fixed

- Fix Low noise setting the wrong bit

## [v0.2.0]

### Changed

- Mode and LowPowerMode have been combined into one setting.

- Make seperate crate for example

### Added

- `destroy()` methods to I2C and SPI interface structs

### Fixed

- Raw acceleration data is now aligned depending on the set mode

- Conversion of raw results to mg

## [v0.1.5]

### Changed

- Reset settings interface
  - Add option to reset and block until the reset is complete
  - **OR**
  - Reset and poll function for the reset to complete (non-blocking)

### Added

- Configuration options for interrupt sources

- Function for reading all SRC registers

- Dump registers function

## [v0.1.4]

### Added

- Also build example in CI

- Add feature for choosing defmt/log

## [v0.1.3]

### Added

- Basic async embassy example for and STM32L431

### Fixed

- Fix default OutputDataRate setting

- Fix typo in README

## [v0.1.2]

### Fixed

- Properly re-export SlaveAddr enum in interface.rs

## [v0.1.1]

### Added

- Implementation of Interface trait for `embedded_hal[_async]` v1.0 SpiDevice
- Implementation of Interface trait for `embedded_hal[_async]` v1.0 SpiBus

### Fixed

- Fix formatting of docstrings

[Unreleased]: https://github.com/JanekGraff/li2dtw12-rs/compare/v0.2.1...HEAD
[v0.1.1]: https://github.com/JanekGraff/li2dtw12-rs/compare/v0.1.0...v0.1.1
[v0.1.2]: https://github.com/JanekGraff/li2dtw12-rs/compare/v0.1.1...v0.1.2
[v0.1.3]: https://github.com/JanekGraff/li2dtw12-rs/compare/v0.1.2...v0.1.3
[v0.1.4]: https://github.com/JanekGraff/li2dtw12-rs/compare/v0.1.3...v0.1.4
[v0.1.5]: https://github.com/JanekGraff/li2dtw12-rs/compare/v0.1.4...v0.1.5
[v0.2.0]: https://github.com/JanekGraff/li2dtw12-rs/compare/v0.1.5...v0.2.0
[v0.2.1]: https://github.com/JanekGraff/li2dtw12-rs/compare/v0.2.0...v0.2.1
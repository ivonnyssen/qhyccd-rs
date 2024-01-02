# qhyccd-rs

[![Crates.io](https://img.shields.io/crates/v/qhyccd-rs.svg)](https://crates.io/crates/qhyccd-rs)
[![Documentation](https://docs.rs/qhyccd-rs/badge.svg)](https://docs.rs/qhyccd-rs/)
[![Codecov](https://codecov.io/github/ivonnyssen/qhyccd-rs/coverage.svg?branch=main)](https://codecov.io/gh/ivonnyssen/qhyccd-rs)
[![Dependency status](https://deps.rs/repo/github/ivonnyssen/qhyccd-rs/status.svg)](https://deps.rs/repo/github/ivonnyssen/qhyccd-rs)

libqhyccd bindings for Rust.

Current bindings are not complete, but will grow as functionality is needed for the ASCOM alpaca drivers or other projects. It is very early in the dev cycle and I am still learning a ton about Rust, so things might be in flux for a while.

```toml
[dependencies]
qhyccd-rs = "0.1.5"
```

## Rust version requirements

qhyccd-rs works with stable Rust. The minimum required Rust version is 1.65.0.

## Version of libqhyccd

Currently the library works with  [QHYCCD SDK 23.09.06](https://www.qhyccd.com/html/prepub/log_en.html#!log_en.md#23.09.06) newer versions require openCV to be installed and do not link well necessarily on ARM-based systems. The focus of this development here is unix, specifically AARCH64-based flavors, although CI is testing for x64 compatibility as well.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <https://opensource.org/licenses/MIT>)

at your option.

The repository contains files from the QHYCCD SDK, these are not covered by these licenses and only provided, so builds in CI pass reasonably.

### Contribution

All contributions are welcome.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in qhyccd-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Quick Start

### Install libusb-1.0-dev

#### Debian / Ubuntu

```sh
sudo apt-get install libusb-1.0-0-dev
```

#### Fedora

```sh
sudo dnf install libusb1-devel
```

### Install QHYCCD SDK

#### ARM

```sh
wget https://www.qhyccd.com/file/repository/publish/SDK/230906/sdk_Arm64_23.09.06.tgz
tar xzvf sdk_Arm64_23.09.06.tgz 
cd sdk_Arm64_23.09.06/
sudo sh install.sh 
```

#### Linux_64

```sh
wget https://www.qhyccd.com/file/repository/publish/SDK/230906/sdk_linux64_23.09.06.tgz
tar xzvf sdk_linux64_23.09.06.tgz
cd sdk_linux64_23.09.06/
sudo sh install.sh 
```

## Usage Examples

[src/bin/LiveFrameMode.rs](https://github.com/ivonnyssen/qhyccd-rs/blob/main/src/bin/LiveFrameMode.rs)

[src/bin/SingleFrameMode.rs](https://github.com/ivonnyssen/qhyccd-rs/blob/main/src/bin/SingleFrameMode.rs)

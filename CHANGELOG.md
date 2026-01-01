# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.7] - 2025-01-01

### Changed

- **BREAKING**: Removed vendored feature from libqhyccd-sys - this change should
only affect the CI builds, as any real-world use of the library
needs the SDK installed locally
- Updated SDK version references from 24.12.26 to 25.09.29 in README
- CI/CD now uses system-installed SDK via [qhyccd-sdk-install](https://github.com/ivonnyssen/qhyccd-sdk-install) GitHub action
- Simplified build.rs to only link system libraries

### Removed

- Vendored SDK files no longer bundled with the crate
- All `--features libqhyccd-sys/vendored` flags from CI workflows

### Fixed

- Updated installation instructions in README to use correct SDK version

## [0.1.6] - Previous Release

- Previous functionality with vendored SDK support

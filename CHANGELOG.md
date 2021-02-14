# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.1] - 2017-12-21

### Added

- `Vec` now implements both `fmt::Debug`, `PartialEq` and `Eq`.

- `resize` and `resize_default` methods to `Vec`.

## [v0.2.0] - 2017-11-22

### Added

- A single producer single consumer mode to `RingBuffer`.

- A `truncate` method to `Vec`.

### Changed

- [breaking-change] Both `Vec::new` and `RingBuffer::new` no longer require an initial value. The
  signature of `new` is now `const fn() -> Self`.

- [breaking-change] The error type of all operations that may fail has changed from `()` to
  `BufferFullError`.

- Both `RingBuffer` and `Vec` now support arrays of *any* size for their backup storage.

## [v0.1.0] - 2017-04-27

- Initial release

[Unreleased]: https://github.com/japaric/heapless/compare/v0.2.1...HEAD
[v0.2.1]: https://github.com/japaric/heapless/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/heapless/compare/v0.1.0...v0.2.0

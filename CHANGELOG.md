# Changelog

All notable changes to this project are documented in this file. The project follows
[Semantic Versioning](https://semver.org/).

## [1.0.0] - 2026-08-15

### Added

- Typed request and capture-backed response contracts for the eight SSI FastConnect Data v2.2
  REST operations.
- Typed compatibility models for the official .NET client's `IntradaybyTick` operation.
- Typed channels and payloads for securities status, quote, trade, foreign room, market index,
  and realtime bar streams.
- Persistent streaming sessions with same-session channel switching and opt-in bounded reconnect
  plus resubscribe behavior.
- Raw REST, stream channel, and JSON escape hatches for forward compatibility.
- Optional JSON CLI and live examples.

### Changed

- Marked public enums as non-exhaustive before stabilizing the public Rust API.

### License

- Released under the MIT License.

[1.0.0]: https://github.com/nguyenthdat/fc-data/releases/tag/v1.0.0

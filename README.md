# vapor-rust

[![Crates.io](https://img.shields.io/crates/v/vapor.svg?style=flat-square)](https://crates.io/crates/vapor)
[![Crates.io](https://img.shields.io/crates/d/vapor.svg?style=flat-square)](https://crates.io/crates/vapor)

A client library for Rust applications to post metrics to [vapord](https://github.com/appalachian-io/vapor).

## Setup

In `Cargo.toml`:

```toml
vapor = "<latest-version>"
```

## Usage

```rust
// Setup
let vapor = Vapor::new("localhost", 13542);

// Gauges
vapor.gauge("test", 5);

// Events
vapor.event("audiod-recv.%h.audio-buffer-underrun");

// Sampled Gauges (0.1% of the time)
vapor.sample_gauge("audiod-recv.%h.audio-clock-diff", clock_diff, 0.001);

// Use `clone` to support multiple writers
{
  let vapor = vapor.clone();
  thread::spawn(move || {
    vapor.event("test");
  });
}
```

## Releasing

1) Update version in `Cargo.toml` and commit change
2) Create a tag and push it, e.g. : `git tag v0.1.0 && git push upstream v0.1.0`
3) Checkout the tag and publish: `git checkout v0.1.0 && cargo publish`

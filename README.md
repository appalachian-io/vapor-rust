#vapor-rust

A client library for Rust applications to post metrics to [vapord](https://github.com/appalachian-io/vapor).

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

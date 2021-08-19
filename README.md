# Backoff &emsp; [![Build Status]][ghactions] [![Latest Version]][crates.io]

[Build Status]: https://github.com/rust-playground/backoff-rs/actions/workflows/rust.yml/badge.svg
[ghactions]: https://github.com/rust-playground/backoff-rs/actions/workflows/rust.yml/badge.svg
[Latest Version]: https://img.shields.io/crates/v/backoff_rs.svg
[crates.io]: https://crates.io/crates/backoff_rs

Backoff provides the base components for implementing backoff and retry operations.

### Example
```rust
use backoff_rs::ExponentialBackoffBuilder;
use std::time::Duration;
fn main() {
    let bo = ExponentialBackoffBuilder::default()
        .factor(1.75)
        .interval(Duration::from_millis(500))
        .jitter(Duration::from_millis(150))
        .max(Duration::from_secs(5))
        .build();
    for attempt in 0..=5 {
        println!("{:?}", bo.duration(attempt));
    }
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Proteus by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
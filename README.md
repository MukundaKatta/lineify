# lineify

[![crates.io](https://img.shields.io/crates/v/lineify.svg)](https://crates.io/crates/lineify)

Token-stream → line-stream. Buffer partial lines, emit complete ones
on `\n`. CRLF supported.

```rust
use lineify::Lineifier;
let mut l = Lineifier::new();
let lines = l.push("hello\nworld\n");
```

Zero deps. MIT or Apache-2.0.

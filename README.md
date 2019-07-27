# efuse

[![Build Status](https://travis-ci.org/lucab/efuse-rs.svg?branch=master)](https://travis-ci.org/lucab/efuse-rs)
[![crates.io](https://img.shields.io/crates/v/efuse.svg)](https://crates.io/crates/efuse)
[![LoC](https://tokei.rs/b1/github/lucab/efuse-rs?category=code)](https://github.com/lucab/efuse-rs)
[![Documentation](https://docs.rs/efuse/badge.svg)](https://docs.rs/efuse)

A Rust library for software [fuses](https://en.wikipedia.org/wiki/Fuse_%28electrical%29).

This library provides boolean-like types that behave like software fuses: they can be "zapped" once, after which they remain in the toggled state forever.
It supports fuses with custom initial boolean state, as well as atomic fuses.

## Example

```rust
let initial_state = true;
let mut fuse = efuse::Fuse::new(initial_state);
assert_eq!(fuse.as_bool(), true);

fuse.zap();
assert_eq!(fuse.is_zapped(), true);
assert_eq!(fuse.as_bool(), false);

fuse.zap();
assert_eq!(fuse.as_bool(), false);

let already_zapped = fuse.zap_once();
assert!(already_zapped.is_err());
```

## License

Licensed under either of

 * MIT license - <http://opensource.org/licenses/MIT>
 * Apache License, Version 2.0 - <http://www.apache.org/licenses/LICENSE-2.0>

at your option.

# rustc-host

[![crates.io](https://img.shields.io/crates/v/rustc-host.svg)][`rustc-host`]
[![crates.io](https://img.shields.io/crates/d/rustc-host.svg)][`rustc-host`]

Combined crate (library + binary) for getting the so-called "host triple" of rustc.

## Usage as a library

Add this to your Cargo.toml:

```toml
[dependencies]
rustc-host = "0.1"
```

after that you can use it like this:

```rust
extern crate rustc_host;

fn main() {
    let host_triple = rustc_host::from_cli()
        .expect("failed to get host triple from rustc");
    println!("host triple: {}", host_triple);
}
```

## Usage as a binary

### Installation

```console
cargo install rustc-host
```

### Usage

```console
rustc-host
```

#### Example output

```text
x86_64-pc-windows-msvc
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`rustc-host`]: https://crates.io/crates/rustc-host

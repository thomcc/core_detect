# `core_detect`

This crate provides a `no_std` version of the `std::is_x86_feature_detected!` macro.

This is possible because x86 chips can just use the `cpuid` instruction to detect CPU features, whereas most other architectures require either reading files or querying the OS.

## Usage

Add `core_detect = "0.2"` to the `[dependencies]` section of your Cargo.toml.

```rust
if core_detect::is_x86_feature_detected!("ssse3") {
    println!("SSSE3 is available");
}
```

# License / Copyright

Much of this code is taken from the `stdarch` repository (for easy upgrading / maximal compatibility), and thus it uses the same copyright as Rust — MIT/Apache-2.0 dual license.

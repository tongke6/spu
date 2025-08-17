# Rust Frontend for SPU

This document describes the Rust frontend for the SPU library. The Rust frontend provides a safe and idiomatic Rust API for interacting with the SPU core library.

## Crates

The Rust frontend is composed of two crates:

- `libspu-sys`: This crate provides low-level, unsafe FFI (Foreign Function Interface) bindings to the C++ `libspu` library. It is generated using the `cxx` crate and is not intended for direct use by end-users.

- `spu-rs`: This crate provides a high-level, safe, and idiomatic Rust API that wraps the `libspu-sys` crate. It is the recommended way to use SPU from Rust.

## Building

The Rust frontend is built using Bazel. To build the Rust crates, you can use the following command:

```bash
bazel build //spu/spu-rs
```

To build the example application, you can use the following command:

```bash
bazel build //spu/spu-rs-example
```

## Usage

Here is a simple example of how to use the `spu-rs` crate:

```rust
use spu_rs::runtime::{RuntimeConfig, SpuRuntime};
use spu_rs::{FieldType, ProtocolKind};

fn main() {
    let config = RuntimeConfig {
        protocol: ProtocolKind::Semi2k,
        field: FieldType::Fm64,
        fxp_fraction_bits: 18,
    };

    // The SpuRuntime is not fully implemented yet.
    // let runtime = SpuRuntime::new(&config);

    println!("SPU Rust frontend is a work in progress.");
}
```

## Current Limitations

The Rust frontend is currently under development and has the following limitations:

- **Incomplete API**: Not all of the `libspu` C++ API is exposed in the Rust frontend yet.
- **Build Environment Issues**: The development of the Rust frontend was hindered by persistent issues with the build environment. This prevented the full implementation and testing of the `SpuRuntime` and the example application.
- **Link Context**: The creation and management of the `yacl::link::Context` from Rust is not yet implemented. This is a crucial component for the SPU runtime to function correctly.

This implementation should be considered a proof-of-concept. Further work is needed to make it a fully functional and robust frontend for SPU.

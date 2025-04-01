# Ethereum Related
## **alloy-primitives**

https://docs.rs/alloy-primitives/latest/alloy_primitives/

Primitive types shared by [alloy](https://github.com/alloy-rs), [foundry](https://github.com/foundry-rs/foundry), [revm](https://github.com/bluealloy/revm), and [reth](https://github.com/paradigmxyz/reth).

### **Types**

- Unsigned integers re-exported from [ruint](https://github.com/recmo/uint)
- Signed integers, as a wrapper around `ruint` integers
- Fixed-size byte arrays via [`FixedBytes`](https://docs.rs/alloy-primitives/latest/alloy_primitives/struct.FixedBytes.html)
    - [`wrap_fixed_bytes!`](https://docs.rs/alloy-primitives/latest/alloy_primitives/macro.wrap_fixed_bytes.html): macro for constructing named fixed bytes types
    - [`Address`](https://docs.rs/alloy-primitives/latest/alloy_primitives/struct.Address.html), which is a fixed-size byte array of 20 bytes, with EIP-55 and EIP-1191 checksum support
    - [`fixed_bytes!`](https://docs.rs/alloy-primitives/latest/alloy_primitives/macro.fixed_bytes.html), [`address!`](https://docs.rs/alloy-primitives/latest/alloy_primitives/macro.address.html) and other macros to construct the types at compile time

# General
## thiserror
**thiserror** provides a convenient derive macro for the standard library’s [`std::error::Error`](https://doc.rust-lang.org/core/error/trait.Error.html) trait.
https://docs.rs/thiserror/latest/thiserror/

## tracing
`tracing` is a framework for instrumenting Rust programs to collect structured, event-based diagnostic information.
https://docs.rs/tracing/latest/tracing/
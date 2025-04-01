# Introduction

reth, the execution client uses  the [Erigon staged-sync](https://erigon.substack.com/p/erigon-stage-sync-and-control-flows) node architecture. Whenever control flow reaches any stage, it attempts to process **all data available** for this stage at the moment. 

Here we use the `Stage` trait and the `SenderRecoveryStage` type to demonstrate how reth developer implementing the asynchronous stages.

# Reference

[Reth Book](https://reth.rs/#what-are-the-goals-of-reth)

[auto_impl - Rust](https://docs.rs/auto_impl/latest/auto_impl/)

[Poll in std::task - Rust](https://doc.rust-lang.org/std/task/enum.Poll.html)

[Introduction - Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
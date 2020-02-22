## Introduction
Mocking asynchronous traits is not as straightforward as one might think. 
Mockall (https://docs.rs/mockall/0.6.0/mockall/#external-traits) provides a great set of examples for various other
use cases, but mocking aync traits is not supported out of the box. This small Rust example demonstrates how to use 
mockall with async traits. It is based on the workaround listed here: https://github.com/asomers/mockall/issues/75

## Usage 
```shell script
cargo test
```

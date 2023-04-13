# SparkLite Client Library

This repository contains a Rust library that implements the client protocol for the SparkLite distributed data analysis framework.

It also contains a small command-line program that demonstrates the library's basic functionality.

See the [DESIGN](./DESIGN.md) file for more details about the design of the SparkLite client.

## Building
This project builds with a stable Rust toolchain. The MSRV is 1.68. If you don't already have one installed, [`rustup`](https://rustup.rs) can help you set one up.

You'll need a working SparkLite server already set up before you run the client. Visit the [sparklite-server](https://github.com/qaz365asd/sparklite-server) repository for instructions on how to set it up.

Once you have a working Rust toolchain and a SparkLite server is already running, run the following sequence of commands to build and run the SparkLite client:

```bash
$ git checkout https://github.com/adammitha/sparklite-client
$ cd sparklite-client
$ cargo build # This may take several minutes to download and compile all of the project's dependencies
$ cargo run <server-host> <server-port> # If the SparkLite server is running on the same machine, server-host will be localhost. The default port is 8000
```

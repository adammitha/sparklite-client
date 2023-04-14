# SparkLite Client Design

## Introduction and Motivation
SparkLite is a distributed data analysis system inspired by Spark. The `sparklite_client` library allows users to write applications that communicate with a SparkLite server.

## Components

### HTTP client
The SparkLite server communicates with clients via a simple REST over HTTP protocol using JSON-encoded messages. The core of the SparkLite client library is a simple retrying http client that handles retries, backoffs, and timeouts for higher level componenets in the system. Furthermore, the http client is generic over the underlying connection which allows us to substitute different transport layers to suit the application's specific needs. It also makes it possible to test the client library with deterministic simulation tools like [`turmoil`](https://docs.rs/turmoil/latest/turmoil/).

### SparkLite Client
The SparkLite client module is responsible for constructing HTTP requests from the user's input, processing responses and handling errors gracefully on behalf of the consumer of the SparkLite library.

The SparkLite client library defines several messages (see `src/message.rs`) that it can send to the SparkLite server. Because both the client and server are implemented in the same language, we were able to share these data structures between the two implementations to make communication simpler.

The package also contains a simple CLI that demonstrates the features of the client library. It implements commands for loading a dataset and applying a simple filter to the test dataset. See the screencast at the bottom of the README for a demonstration of how to use the CLI.

## Challenges and Lessons Learned
### Rust
One of the biggest challenges that I encountered when implementing this project was learning Rust. Although I've been exposed to Rust before with personal projects and at work, this is the most complex project that I've worked on from scratch.

We initially chose Rust for a couple of reasons: firstly, we wanted to take the opportunity to learn a language that was becoming increasingly prominent in the distributed systems community. Secondly, Rust's allows us to express our program's constraints and assumptions in the type system which ensures that many classes of errors get caught at compile time rather than runtime. In addition, the prohibition on mutable aliasing ensures that we avoid any nasty concurrency bugs that come along with multithreaded server implementations.

In exchange for these correctness guarantees, getting Rust code to compile tends to be a far more arduous task than is less picky languages. We found ourselves spending hours decoding obscure error messages from the Rust compiler due to subtle issues with ownership or lifetimes. While this additional work almost certainly produced more reliable, robust, and fault-tolerant code at the end, it make it difficult to hack-together a simple proof of concept to validate our ideas in the first place.

One aspect of the Rust ecosystem that we really appreciated was the robust ecosystem of third-party packages that the community has made available for building distributed systems. In particular, the `tokio` runtime and supporting packages make it really easy to stand up a networked client and server with very little boilerplate. Furthermore, the `serde` and `serde_json` crates make it easy to serialize our types across the network so we could easily share data structures between our client and server.

### Encoding functions on the wire
SparkLite's core data analysis interface consists of higher-order functions like `map`, `filter`, and `reduce` that users can apply to the dataset of their choice. Because these functions consume a function as input, we needed to come up with a way for users to provide a function across a network connection. The original Spark framework delegates this task to the underlying Java Virtual Machine, which provides the ability to serialize arbitrary JVM bytecode onto the wire and execute it on a remote machine. Unfortunately, this solution is not available in languages that compile to native code such as Rust. Copying raw x86_64 machine code across the wire seems crude and inelegant, and sacrifices portability and long-term maintainability.

Another option would be to use a platform-independent bytecode that can be compiled on the client and then executed on the server. WebAssembly is one example of such a format, with the added benefit that there are multiple high-quality Rust implementations that can be readily integrated into any Rust application or library. This option is probably the best solution as it allows users to provide functions in *any* language that can be compiled to WebAssembly, which will help maximize SparkLite's appeal to the widest range of users. However, given our inexperience with WebAssembly it is likely to be a significant implementation challenge.

The second option would be to specify an execution environment that users must comply with. The simplest of these environments would be a Posix shell, which would require that users write shell scripts using Posix-standard command line tools (e.g. `sort`, `uniq`, `sed`, `awk`, `grep` etc.) to manipulate their data. This option sacrifices the flexibility of a WebAssembly runtime in favour of a simpler implementation (i.e. `/bin/sh <shell_script.sh>`).

The final option is to provide a predefined list of functions that users can execute, and only let them select certain arguments for that function. We would statically define these functions on the SparkLite server, pass in the user-provided arguments and then execute on the provided dataset. While this is the most restrictive option from the end user's perspective, it's likely to be the easiest option from an implementation perspective - we simply need to write some Rust functions and define a schema for specifying which function to execute, and with which arguments.

Ultimately, we decided to go with a list of predefined functions that the user can choose from. Although it's the most limiting of the three options we considered, the implementation complexity fits well with the time constraints that we have for this project.

## Testing
The primary method for testing this client is through the provided command-line interface. See the following screencast for a demonstration.

[![asciicast](https://asciinema.org/a/577478.svg)](https://asciinema.org/a/577478?t=27)

The current server implementation only supports filtering based on the first column of the test dataset.

## Lessons for the future
Given the chance, I'd like to take more advantage of the `tokio` ecosystem which has a number of packages that abstract away a lot of the boilerplate associated with building distributed systems. For example, the `tower` crate has a number of `Layer`s that can be added onto your services, like timeouts, retries, and load balancing. Unfortunately, I struggled to use this library with my client due to the complexity of resolving the issues with Rust's borrow checker that arose when I attempted to use `tower` in the implementation of my http client, which forced me to implement these features myself.

`tokio` also has an excellent library for testing distributed systems called `turmoil`. It gives users a synthetic transport layer they can plug into their client and server implementations, which allows them to simulate flaky or partitioned networks in a deterministic fashion which allows developers to easily surface and reproduce subtle bugs in their logic.

# SparkLite Client Design

## Introduction
The `sparklite_client` library allows users to write applications that communicate with the SparkLite distributed data analysis system.

## Components

### HTTP client
The SparkLite server communicates with clients via a simple REST over HTTP protocol using JSON-encoded messages. The core of the SparkLite client library is a simple retrying http client that handles retries, backoffs, and timeouts for higher level componenets in the system. Furthermore, the http client is generic over the underlying connection which allows us to substitute different transport layers to suit the applications specific needs. It also makes it possible to test the client library with deterministic simulation tools like [`turmoil`](https://docs.rs/turmoil/latest/turmoil/).

### Messages


## Challenges

### Encoding functions on the wire
One of the most interesting design challenges that we encountered was figuring out how to serialize user defined functions on the wire that the SparkLite server can distribute and execute on our chosen dataset. The original Spark framework delegates this task to the underlying Java Virtual Machine, which provides the ability to serialize arbitrary JVM bytecode onto the wire and execute it on a remote machine. Unfortunately, this solution is not available in languages that compile to native code such as Rust. Copying raw x86_64 machine code across the wire seems like a particularly crude and inelegant solution to this problem, to say nothing of the fact that we'd be sacrificing portability as well.

Another option would be to use some platform-independent bytecode that can be compiled on the client and then executed on the server. WebAssembly is one example of such a format, with the added benefit that there are multiple high-quality Rust implementations that can be readily integrated into any Rust application or library. This option is probably the best solution as it allows users to provide functions in *any* language that can be compiled to WebAssembly, which will help maximize SparkLite's appeal to the widest range of users. However, given our inexperience with WebAssembly it is likely to be a significant implementation challenge.

The second option would be to specify a particular execution environment that users must comply with. The simplest of these environments would be a Posix shell, which would require that users write shell scripts using Posix-standard command line tools (e.g. `sort`, `uniq`, `sed`, `awk`, `grep` etc.) to manipulate their data. This is significantly less elegant than the WebAssembly option, but is probably going to be much easier to implement.

The final option is to provide a predefined list of functions that users can execute, and only let them select certain arguments for that function. We would statically define these functions on the SparkLite server, pass in the user-provided arguments and then execute on the provided dataset. While this is the most restrictive option from the end user's perspective, it's likely to be the easiest option from an implementation perspective - we simply need to write some Rust functions and define a schema for specifying which function to execute, and with which arguments.

## References
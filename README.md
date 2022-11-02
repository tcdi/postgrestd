# The Postgres Standard Library

`postgrestd` is a fork of the Rust standard library that implements a variant on the internal-only `std::sys` module plus additional targets that use this module. It replaces many of the bindings to the operating system with bindings to PostgreSQL's interfaces or to no-op functions, restricting what Rust code compiled with it can access through Safe Rust.

## Key Features

The goal of this fork is to prevent Safe Rust code from accessing the operating system's ability to start a subprocesses or to access the shell environment that Postgres has been started in, either of which can amount to allowing code to roam free in the computing environment.
 
In the typical implementation of the Rust standard library, it uses a module, `std::sys`, that defines various system-specific bindings which either address platform-specific datatype compatibility concerns or directly bind against the host's system library (which is usually the C standard library, AKA libc). In practice, all functions that require platform varying functionality are implemented through this module, though the module itself may pull in code from elsewhere. Code using `postgrestd` varies by instead either stubbing out the code (returning `Result::Err`) or using the Postgres C API instead when it's necessary to implement basic functionality for the Rust runtime to function.

The primary functions reimplemented through Postgres are a global allocator using `palloc`. This is actually built and linked through the `pallocator` crate to guarantee it is the used allocator, as std's default allocator may be overridden through `#[global_allocator]`, but only one `#[global_allocator]` may be linked in the entire build graph for a crate.

`Result::Err` allows Rust code that handles it appropriately to continue functioning. This currently is done for any function which is believed to violate the "trusted language" definition, to allow use in PL/Rust. Listing every single affected function is implausible, as there are many callers for `std::sys` and its internal API surface. This is also why other implementation strategies are not realistic: the surface functions of `std` can and will continue to slowly grow, and will have their internal implementations changed often, but they will remain implemented through the `std::sys` internal module, which changes much less frequently.

Code that relies on having access to the environment (i.e. doesn't handle an `Err` gracefully) will panic, which is converted via the `pgx` panic handler into raising an error that aborts the transaction.

Aborting a Rust runtime may be done at any point without being considered an `unsafe` operation, even if this means Drop implementations are not run. However, by replacing the Rust global allocator with one that uses Postgres memory contexts, aborted transactions do not actually leak memory: Postgres does the teardown of the aborted transaction's memory contexts, in a similar way that an operating system reaps a terminated process and reclaims its resources. It is possible for non-memory resources to leak, and handling those is not currently addressed, except by making it impossible to claim irrelevant resources such as e.g. file handles or threads.

As long as the transaction is terminated and Rust fully leaves the code behind, any broken invariants that may have been created as a result of failing to run Drop implementations are not witnessed and Rust's soundness is preserved. Implementing full and proper unwinding while still interfacing with Postgres error handling routines is expected to be plausible, which would prevent unnecessary resource leaks entirely and improve debugging, but is not presently done as it is currently considered a secondary goal and the implementation is expected to be "rather cursed".

## Get Started

`postgrestd` is intended to be used as part of [PL/Rust](https://github.com/tcdi/plrust).

## Getting Help

This is a part of the PL/Rust project. Join the [pgx] (Postgres Extensions in Rust) community on [Discord](https://discord.gg/kwsy38x5Kh) and ask for help in [plrust-#general](https://discord.com/channels/561648697805504526/835595007791726704)!

## Contributing

In addition to [TCDI](https://www.tcdi.com), the project is maintained by the [pgx][pgx] community. 

We are most definitely open to contributions of any kind. Bug Reports, Feature Requests, Documentation, and even sponsorships.


## License

`postgrestd` is distributed under the same terms that apply to the Rust standard library. This means it is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses. By contributing to this repository you accept that contributions to this repository may be upstreamed to the [rustc repo](https://github.com/rust-lang/rust) to be redistributed with the Rust standard library.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

[pgx]: https://github.com/tcdi/pgx

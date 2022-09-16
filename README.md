# The Postgres Standard Library

`postgrestd` is a fork of the Rust standard library that implements a variant on the internal-only `std::sys` module plus additional targets that use this module. It replaces many of the bindings to the operating system with bindings to PostgreSQL's interfaces or to no-op functions, restricting what Rust code compiled with it can access through Safe Rust.

## Key Features

The goal of this fork is to prevent Safe Rust code from accessing 

  * file handle operations
  * internals of the postgres process 
  * the OS with the permissions of the server process
  
In the typical implementation of the Rust standard library, it uses a module, `std::sys`, that defines various system-specific bindings which either address platform-specific datatype compatibility concerns or directly bind against the host's system library (which is usually the C standard library, AKA libc). Instead of directly interfacing with the C standard library, `postgrestd::sys` calls functions from the Postgres C API instead that implement similar functionality. When the functionality is not applicable or not desired, `Result::Err` that indicate unsupported functionality is returned, allowing Rust code that handles it appropriately to continue functioning. Code that relies on having access to the environment will panic, which is converted into raising an error that aborts the transaction.

Aborting a Rust runtime may be done at any point without being considered an unsafe operation, even if this means Drop implementations are not run. However, by replacing the Rust global allocator with one that uses Postgres memory contexts, aborted transactions do not actually leak memory: Postgres does the teardown of the aborted transaction's memory contexts, in a similar way that an operating system reaps a terminated process and reclaims its resources. It is possible for non-memory resources to leak, and handling those is not currently addressed, except by making it impossible to claim irrelevant resources such as e.g. file handles or threads.


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

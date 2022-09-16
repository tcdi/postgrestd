# The Postgres Standard Library

`postgrestd` is a fork of the Rust standard library that implements a variant on the internal-only `std::sys` module plus additional targets that use this module. It replaces many of the bindings to the operating system with bindings to PostgreSQL's interfaces or to no-op functions, restricting what Rust code compiled with it can access through Safe Rust.

## Key Features

The goal of the library is to prevent code from accessing 

  * file handle operations
  * internals of the postgres process 
  * the OS with the permissions of the server process
  
The library exposes certain calls into the environment, in the pattern of libc, instead of directly interfacing with the C standard library, the `postgrestd` equivalent functions are called. When it is not applicable we deny the code access, avoiding errors. Code that relies on having access to the environment will panic and abort the transaction.  

The key to managing this is replacing the allocator so that all memory allocations are inside the Postgres context, ensuring that data is torn down properly.


## Get Started

`postgrestd` is intended to be used as part of [PL/Rust](https://github.com/tcdi/plrust).

## Getting Help

This is a part of the PL/Rust project. Join the [pgx] (Postgres Extensions in Rust) community on [Discord](https://discord.gg/kwsy38x5Kh) and ask for help in [plrust-#general](https://discord.com/channels/561648697805504526/835595007791726704)!

## Contributing

In addition to [TCDI](https://www.tcdi.com), the project is maintained by the [pgx][pgx] community. 

We are most definitely open to contributions of any kind. Bug Reports, Feature Requests, Documentation, and even sponsorships.


## License

`postgrestd` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

[pgx]: https://github.com/tcdi/pgx

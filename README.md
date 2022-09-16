# The Postgres Standard Library

`Postgrestd` is a custom variant to the Rust Standard Library. This is the main source code repository for `postgrestd`. 

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

Join the [Postgres Rust Extension][pgx] (pgx) community on [Discord](https://discord.gg/kwsy38x5Kh).

## Contributing

In addition to [TCDI](https://www.tcdi.com), the project is maintained by the [pgx][pgx] community. 

We are most definitely open to contributions of any kind. Bug Reports, Feature Requests, Documentation, and even sponsorships.

If you'd like to contribute code via a Pull Request, please make it against our main branch.

## License

`postgrestd` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

[pgx]: https://github.com/tcdi/pgx

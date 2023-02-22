Rust `std` modules with impacted functionality:
- arch - SIMD and vendor intrinsics module.
    - Technically available but in practice unusable (it is almost entirely `unsafe`)
- fs - Filesystem manipulation operations.
    - May panic, return `Err("unsupported operation")`, or have arbitrary results (e.g. `is_file` always returns `false`)
- io - Traits, helpers, and type definitions for core I/O functionality.
    - Works on a case-by-case basis (usable with e.g. `Vec<u8>`, not with files).
- net - Networking primitives for TCP/UDP communication.
    - May panic, return `Err("unsupported operation")`, or have arbitrary results.
- os - OS-specific functionality.
    - May panic, return `Err("unsupported operation")`, or have arbitrary results.
- process - A module for working with processes.
    - May panic, return `Err("unsupported operation")`, or have arbitrary results.
- ptr - Manually manage memory through raw pointers.
    - Technically available but in practice unusable (it is almost entirely `unsafe`)
- thread - Native threads.
    - May panic, return `Err("unsupported operation")`, or have arbitrary results.


Other Rust `std` modules:
- alloc - Memory allocation APIs.
- any - Utilities for dynamic typing or type reflection.
- array - Utilities for the array primitive type.
- ascii - Operations on ASCII strings and characters.
- backtrace - Support for capturing a stack backtrace of an OS thread
- borrow - A module for working with borrowed data.
- boxed - The `Box<T>` type for heap allocation.
- cell - Shareable mutable containers.
- clone - The Clone trait for types that cannot be ‘implicitly copied’.
- cmp - Utilities for comparing and ordering values.
- collection - Collection types.
- convert - Traits for conversions between types.
- default - The Default trait for types with a default value.
- env - Inspection and manipulation of the process’s environment.
- error - Interfaces for working with Errors.
- ffi - Utilities related to FFI bindings.
- fmt - Utilities for formatting and printing Strings.
- future - Asynchronous basic functionality.
- hash - Generic hashing support.
- hint - Hints to compiler that affects how code should be emitted or optimized. Hints may be compile time or runtime.
- iter - Composable external iteration.
- marker - Primitive traits and types representing basic properties of types.
- mem - Basic functions for dealing with memory.
- num - Additional functionality for numerics.
- ops - Overloadable operators.
- option - Optional values.
- panic - Panic support in the standard library.
- path - Cross-platform path manipulation.
- pin - Types that pin data to its location in memory.
- prelude - The Rust Prelude
- primitive - This module reexports the primitive types to allow usage that is not possibly shadowed by other declared types.
- rc - Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.
- result - Error handling with the Result type.
- slice - Utilities for the slice primitive type.
- str - Utilities for the str primitive type.
- string - A UTF-8–encoded, growable string.
- sync - Useful synchronization primitives.
- task - Types and Traits for working with asynchronous tasks.
- time - Temporal quantification.
- vec - A contiguous growable array type with heap-allocated contents, written `Vec<T>`.
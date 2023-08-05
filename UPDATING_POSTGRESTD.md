This is a pretty rough set of steps, but should give you the basic idea for updating the rust version for PL/Rust and postgrestd.

Setup required (If this isn't where you put these, substitute paths as necessary):
- `~/work/plrust` should be a checkout of plrust.
- `~/work/postgrestd` should be a checkout of postgrestd.
- `~/work/rust` should be a checkout of rust (e.g. <https://github.com/rust-lang/rust>). Do not use a checkout that has the normal git hooks for rust-lang/rust set up, otherwise this process will fail.

Then, the rough steps are as follows:
1. `cd ~/work/postgrestd && git remote update && git checkout rust-$OLDVERSION && git checkout -b rust-$NEWVERSION`
2. `cd ~/work/rust`
   1. `git remote update && git checkout $NEWVERSION` (check out the tag for the new version)
   2. `./x.py build` (do enough of a build to fully setup the repo and submodules. You can kill this once that part is complete, and it's probably fine if this fails to compile)
   3. `git subtree push -P library ~/work/postgrestd "$NEWVERSION-library"` (push the subtree. This will take 1h-2h, so find something else to do while it runs)
3. `cd ~/work/postgrestd`
   1. `git subtree pull -P library . "$NEWVERSION-library"`

      Note: If `git subtree pull ...` tells you that you're "Already up to date." in this step despite changing nothing, it may be because there were no commits which modified files under `library/` between `$OLDVERSION` and `$NEWVERSION`. This pretty much can only ever happen when for patch updates (for example, this happened between Rust 1.71.0 and 1.71.1).

   2. Resolve merge conflicts.
   3. Push and fix issues.
   4. Audit the new stdlib APIs and internals for changes and make sure nothing new is added that we should not have exposed from PL/Rust. There's no real guide to how to do that, though, just go through the changes and use your best judgement. (Note: This is usually very time consuming). In particular keep an eye out for:
      1. New `std::io` APIs, and/or changes to their implementation.
      2. New `std::os::{unix, linux, darwin}` APIs, and/or changes to their implementation.
      3. New `std::{net, thread, backtrace, panic}` features, or changes to their implementation.
      4. Ditto for everything we document as being blocked in [block.md](./block.md).
      5. Removal of/changes to various `#[rustc_diagnostic_item]` that we use. (Many/all of these should be caught in PL/Rust? Any that aren't should have a test added).

      Note that many of these (the implementation changes) will not be noted in release notes (even in RELEASES.md). You must check the code.

4. In `~/work/plrust` and `~/work/postgrestd`, find all the references to the old version number and update them.
5. In plrust, migrate any rustc APIs that plrustc uses to the new versions (this is usually pretty straightforward, if you get stuck you can try and find similar version bumps in the source for clippy, miri, and other rustc-drivers).
6. Make sure CI passes and then you're good.

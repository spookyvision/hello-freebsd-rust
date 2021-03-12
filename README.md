# What
Hello world FreeBSD kernel module in Rust. Its main purpose is to log kernel module loading events like load or unload, helpfully *not* translating event enums into human-readable strings. It does so by hooking into the `module_event` using a C stub (`hello.c`) which forwards to the Rust implementation.
It implements `GlobalAlloc`, so you can do collections! It also implements it somewhat wrong, so you probably shouldn't! See the TODO comments in `src/os/malloc.rs`.

The directory layout is loosely based on [johalun/echo](https://github.com/johalun/echo), from which it also mostly inherits the `build.rs` script. The build process produces a spurious `-.d` file, lost interest finding the cause for that. `build.rs` also probably needs cleanup of the bazillion flags - more FreeBSD knowledge than I have is needed there.

`build.sh` builds and loads the module. It overrides `CARGO_TARGET_DIR` because I had the repo sitting on a slow as hell NAS.

# Prerequisites
- as of 2021-03-12: nightly Rust (for `build-std`)
- You need to install llvm9's libclang with the ports system. This takes *bloody ages* and also (on my system) required manually patching up a mismatched perl symlink in `/usr/local/bin` (some dependency required `perl5.32.0` but `perl5.32.1` was installed). I forgot which toggles are required to end up with libclang.

- You need to have the FreeBSD sources extracted like so:
```
fetch ftp://ftp.freebsd.org/pub/FreeBSD/releases/amd64/`uname -r`/src.txz
tar -C / -xzf src.txz
```
... after which you *probably* need to run `make kernel-toolchain` in `/usr/src`.

extern crate bindgen;

use std::path::PathBuf;

const FILEPATH_CODE: &'static str = "src/os/kernel_sys.rs";

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        // .no_unstable_rust()
        .use_core()
        // Use this prefix instead of ::std::os::raw
        .ctypes_prefix("cty")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Clang arguments
        .clang_arg("-O2")
        .clang_arg("-pipe")
        .clang_arg("-fno-strict-aliasing")
        .clang_arg("-Werror")
        .clang_arg("-D_KERNEL")
        .clang_arg("-DKLD_MODULE")
        .clang_arg("-nostdinc")
        .clang_arg("-I/usr/include")
        .clang_arg("-fno-common")
        .clang_arg("-fno-omit-frame-pointer")
        .clang_arg("-mno-omit-leaf-frame-pointer")
        .clang_arg("-MD")
        .clang_arg("-mcmodel=kernel")
        .clang_arg("-mno-red-zone")
        .clang_arg("-fno-asynchronous-unwind-tables")
        .clang_arg("-ffreestanding")
        .clang_arg("-fwrapv")
        .clang_arg("-fstack-protector")
        .clang_arg("-Wall")
        .clang_arg("-Wredundant-decls")
        .clang_arg("-Wnested-externs")
        .clang_arg("-Wstrict-prototypes")
        .clang_arg("-Wmissing-prototypes")
        .clang_arg("-Wpointer-arith")
        .clang_arg("-Winline")
        .clang_arg("-Wcast-qual")
        .clang_arg("-Wundef")
        .clang_arg("-Wno-pointer-sign")
        .clang_arg("-D__printf__=__freebsd_kprintf__")
        .clang_arg("-Wmissing-include-dirs")
        .clang_arg("-fdiagnostics-show-option")
        .clang_arg("-Wno-unknown-pragmas")
        .clang_arg("-Wno-error-tautological-compare")
        .clang_arg("-Wno-error-empty-body")
        .clang_arg("-std=iso9899:1999")
        // Command for building kernel modulde
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(FILEPATH_CODE);
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    // save some time - rerun only if this script has changed
    println!("cargo:rerun-if-changed=build.rs");
}

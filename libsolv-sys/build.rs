extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;


fn main() {

    // Compile the static inline functions into an archive
    // and directs Cargo to link it
    cc::Build::new()
        .file("static/bitmap.c")
        .file("static/dirpool.c")
        .file("static/pool.c")
        .file("static/poolarch.c")
        .file("static/queue.c")
        .file("static/repo.c")
        .file("static/repodata.c")
        .file("static/strpool.c")
        .compile("libsolv-static-functions.a");

    //pkg_config::probe_library("libsolvext").unwrap();

    // Direct Cargo to link the libsolv library
    pkg_config::probe_library("libsolv").unwrap();

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        // Prefer the libc crate's definitions for libc types
        .ctypes_prefix("libc")

        // allowlist libsolv's functions, types, and variables,
        // otherwise bindgen will bind all of libc
        .allowlist_type("Solver")
        .allowlist_type("Chksum")
        .allowlist_type("solv.*")
        .allowlist_function("pool.*")
        .allowlist_function("stringpool.*")
        .allowlist_function("transaction.*")
        .allowlist_function("solv.*")
        .allowlist_function("selection.*")
        .allowlist_function("repopagestore.*")
        .allowlist_function("repo.*")
        .allowlist_function("queue.*")
        .allowlist_function("policy.*")
        .allowlist_function("find.*")
        .allowlist_function("dirpool.*")
        .allowlist_function("datamatcher.*")
        .allowlist_function("dataiterator.*")
        .allowlist_function("map.*")
        .allowlist_var("DI.*")
        .allowlist_var("SOLV.*")
        .allowlist_var("REPO.*")
        .allowlist_var("SEARCH.*")
        .allowlist_var("EVRCMP.*")

        // Hide FILE from bindgen's output
        // Otherwise we get the OS's private file implementation
        .blocklist_type("FILE")
        .raw_line("use libc::FILE;")

        .rustified_enum("solv_knownid")

        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

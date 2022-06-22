extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;


fn main() {

    //pkg_config::probe_library("libsolvext").unwrap();

    pkg_config::probe_library("libsolvext").unwrap();


    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        // Prefer the libc crate's definitions for libc types
        .ctypes_prefix("libc")

        // <solv/testcase.h>
        .allowlist_var("TESTCASE.*")
        .allowlist_function("testcase.*")

        // <solv/solv_xfopen.h>
        .allowlist_function("solv_xfopen.*")

        // <solv/pool_fileconflicts.h>
        .allowlist_var("FINDFILECONFLICTS.*")
        .allowlist_function("pool_findfileconflicts")

        // <solv/repo_rpmdb.h>
        // TODO: VARS
        .allowlist_function("repo_add_rpm.*")
        .allowlist_function("rpm_state_.*")
        .allowlist_function("rpm_installedrpmdbids")
        .allowlist_function("rpm_by.*")
        .allowlist_function("rpm_query.*")
        .allowlist_function("rpm_iterate_filelist")

        // <solv/repo_repomdxml.h>
        .allowlist_function("repo_add_repo.*")
        // <solv/repo_rpmmd.h>
        .allowlist_function("repo_add_rpm.*")
        // <solv/repo_deltainfoxml.h>
        .allowlist_function("repo_add_delta.*")
        // <solv/repo_updateinfoxml.h>
        .allowlist_function("repo_add_update.*")

        // Don't let bindgen recreate libsolv's types
        .blocklist_type("Chksum")
        .blocklist_type("DUChanges")
        .blocklist_type("Dataiterator")
        .blocklist_type("Datamatcher")
        .blocklist_type("s_Datapos")
        .blocklist_type("Datapos")
        .blocklist_type("s_Dirpool")
        .blocklist_type("Dirpool")
        .blocklist_type("Hashtable")
        .blocklist_type("Hashval")
        .blocklist_type("Id")
        .blocklist_type("KeyValue")
        .blocklist_type("s_Map")
        .blocklist_type("Map")
        .blocklist_type("Offset")
        .blocklist_type("s_Pool")
        .blocklist_type("Pool")
        .blocklist_type("s_Queue")
        .blocklist_type("Queue")
        .blocklist_type("s_Reldep")
        .blocklist_type("Reldep")
        .blocklist_type("s_Repo")
        .blocklist_type("Repo")
        .blocklist_type("s_Repodata")
        .blocklist_type("Repodata")
        .blocklist_type("s_Repokey")
        .blocklist_type("Repokey")
        .blocklist_type("Rule")
        .blocklist_type("s_Solvable")
        .blocklist_type("Solvable")
        .blocklist_type("s_Solver")
        .blocklist_type("Solver")
        .blocklist_type("s_Stringpool")
        .blocklist_type("Stringpool")
        .blocklist_type("Transaction")

        // Hide FILE from bindgen's output
        // Otherwise we get the OS's private file implementation
        .blocklist_type("FILE")
        .raw_line("use libc::FILE;")

        // Import necessary structs from libsolv_sys
        .raw_line("use libsolv_sys::{Chksum, DUChanges, Dataiterator, Datamatcher, Datapos, Dirpool};")
        .raw_line("use libsolv_sys::{Hashtable, Hashval, Id, KeyValue, Map, Offset, Pool, Queue};")
        .raw_line("use libsolv_sys::{Reldep, Repo, s_Repo, Repodata, Repokey, Rule, Solvable, Solver, Stringpool};")
        .raw_line("use libsolv_sys::{Transaction};")

        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

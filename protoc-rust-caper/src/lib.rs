//! Code generator for caper RPC framework
//!
//! This crate provides one function [`run`], which can generate service provider
//! templates and client side stubs from `.proto` files. The best place to use
//! this function is the `build.rs` file. For more information about `build.rs`,
//! please refer to the [build scripts] section of the official cargo book.
//!
//! [`run`]: fn.run.html
//! [build scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//!
//! # Examples
//!
//! Say, we have a `awesome.proto` file in the project root directory (i.e. next
//! to `Cargo.toml`), and we want to generate rust code in `src/generated`.
//! We can add this to `build.rs`:
//!
//! ```no_run
//! extern crate protoc_rust_caper;
//!
//! fn main() {
//!     protoc_rust_caper::run(protoc_rust_caper::Args {
//!         out_dir: "src/generated",
//!         input: &["awesome.proto"],
//!         includes: &[],
//!         rust_protobuf: true
//!     }).expect("Failed to compile proto files");
//! }
//! ```
//!
//! # Acknowledgment
//!
//! The crate is a mirror of [protoc-rust-grpc].
//!
//! [protoc-rust-grpc]: https://crates.io/crates/protoc-rust-grpc

#![warn(missing_docs, missing_debug_implementations)]

extern crate inflector;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate protoc;
extern crate protoc_rust;
extern crate tempdir;

mod codegen;

use std::io;
use std::io::Read;
use std::io::Write;
use std::fs;

#[derive(Debug, Default)]
/// Argument passed to `run`
pub struct Args<'a> {
    /// where to put generated files
    pub out_dir: &'a str,
    /// dependency to other .proto files
    pub includes: &'a [&'a str],
    /// List of .proto files to compile
    pub input: &'a [&'a str],
    /// Generate rust-protobuf files along with caper
    ///
    /// Set this value to `false` to only generate service boilerplates and RPC
    /// stubs.
    pub rust_protobuf: bool,
}

/// Generate rust code
pub fn run(args: Args) -> io::Result<()> {
    let protoc = protoc::Protoc::from_env_path();
    let version = protoc.version().expect("protoc version");
    if !version.is_3() {
        panic!("protobuf must have version 3");
    }

    if args.rust_protobuf {
        protoc_rust::run(protoc_rust::Args {
            out_dir: args.out_dir,
            includes: args.includes,
            input: args.input,
        })?;
    }

    let temp_dir = tempdir::TempDir::new("protoc-rust")?;
    let temp_file = temp_dir.path().join("descriptor.pbbin");
    let temp_file = temp_file.to_str().expect("utf-8 file name");

    protoc.write_descriptor_set(protoc::DescriptorSetOutArgs {
        out: temp_file,
        includes: args.includes,
        input: args.input,
        include_imports: true,
    })?;

    let mut fds = Vec::new();
    let mut file = fs::File::open(temp_file)?;
    file.read_to_end(&mut fds)?;

    drop(file);
    drop(temp_dir);

    let fds: protobuf::descriptor::FileDescriptorSet =
        protobuf::parse_from_bytes(&fds).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let mut includes = args.includes;
    if includes.is_empty() {
        static DOT_SLICE: &'static [&'static str] = &["."];
        includes = DOT_SLICE;
    }

    let mut files_to_generate = Vec::new();
    'outer: for file in args.input {
        for include in includes {
            if let Some(truncated) = remove_path_prefix(file, include) {
                files_to_generate.push(truncated.to_owned());
                continue 'outer;
            }
        }

        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "file {:?} is not found in includes {:?}",
                file, args.includes
            ),
        ));
    }

    let gen_result = codegen::gen(fds.get_file(), &files_to_generate)?;

    for r in gen_result {
        let r: protobuf::compiler_plugin::GenResult = r;
        let file = format!("{}/{}", args.out_dir, r.name);
        let mut file = fs::File::create(&file)?;
        file.write_all(&r.content)?;
        file.flush()?;
    }

    Ok(())
}

fn remove_dot_slash(path: &str) -> &str {
    if path == "." {
        ""
    } else if path.starts_with("./") || path.starts_with(".\\") {
        &path[2..]
    } else {
        path
    }
}

fn remove_path_prefix<'a>(mut path: &'a str, mut prefix: &str) -> Option<&'a str> {
    path = remove_dot_slash(path);
    prefix = remove_dot_slash(prefix);

    if prefix == "" {
        return Some(path);
    }

    if prefix.ends_with("/") || prefix.ends_with("\\") {
        prefix = &prefix[..prefix.len() - 1];
    }

    if !path.starts_with(prefix) {
        return None;
    }

    if path.len() <= prefix.len() {
        return None;
    }

    if path.as_bytes()[prefix.len()] == b'/' || path.as_bytes()[prefix.len()] == b'\\' {
        return Some(&path[prefix.len() + 1..]);
    } else {
        return None;
    }
}

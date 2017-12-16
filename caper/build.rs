extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/message",
        input: &["src/message/meta.proto"],
        includes: &[],
    }).expect("Compile lib proto files failed.");
}

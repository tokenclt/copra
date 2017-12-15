extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/message",
        input: &["src/message/meta.proto"],
        includes: &[],
    }).expect("Compile lib proto files failed.");

    protoc_rust::run(protoc_rust::Args {
        out_dir: "examples/echo",
        input: &["examples/echo/message.proto"],
        includes: &[],
    }).expect("Compile proto files in echo example failed.");
}

extern crate protoc_rust;


// The work directory should be the workspace root
fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "copra/src/message",
        input: &[
            "copra/src/message/meta.proto",
            "copra/src/message/test.proto",
        ],
        includes: &[],
    }).expect("Failed to build copra proto files.");
}

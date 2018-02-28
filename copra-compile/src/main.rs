extern crate protoc_rust_copra;

fn generate_copra_internal_message() {
    protoc_rust_copra::run(protoc_rust_copra::Args {
        out_dir: "copra/src/message",
        input: &[
            "copra/src/message/meta.proto",
            "copra/src/message/test.proto",
        ],
        includes: &[],
        rust_protobuf: true,
    }).expect("Failed to build copra proto files");
}

fn generate_integration_test_message() {
    protoc_rust_copra::run(protoc_rust_copra::Args {
        out_dir: "copra/tests/generated",
        input: &["copra/tests/protos/simple.proto"],
        includes: &[],
        rust_protobuf: true,
    }).expect("Failed to build integration test proto files");
}

// The work directory should be the workspace root
fn main() {
    generate_copra_internal_message();
    generate_integration_test_message();
}

use std::env::Args;

/// This generates Device Plugin code (in v1beta1.rs) from pluginapi.proto
fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .include("data")
        .inputs(&[
            "./data/proto/discovery.proto"
        ])
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}

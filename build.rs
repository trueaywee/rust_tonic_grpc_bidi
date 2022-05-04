fn main() {
    tonic_build::compile_protos("proto/bidi_stream.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}

fn main() {
    let file_descriptors = protox::compile(["bidi_stream.proto"], ["./proto"]).unwrap();

    tonic_build::configure()
        .build_server(true)
        .compile_fds(file_descriptors)
        .unwrap();
}

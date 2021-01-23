fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/grpc/generated")
        .compile(&["src/grpc/proto/user_proto.proto"], &["src/grpc/proto"])?;
    Ok(())
}

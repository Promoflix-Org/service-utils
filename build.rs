fn main() {
    let auth = "./proto/auth_service.proto";
    let csv = "./proto/csv_service.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[auth, csv], &[".", "."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
}

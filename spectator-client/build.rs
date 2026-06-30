fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = std::env::var("PROTO_SRC")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("../protos"));

    let protos = &[
        proto_root.join("quic/common.proto"),
        proto_root.join("quic/combat.proto"),
        proto_root.join("grpc/spectator.proto"),
    ];

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .extern_path(".stargem.quic.combat", "crate::proto::combat")
        .extern_path(".stargem.quic.common", "crate::proto::common")
        .compile(protos, &[proto_root])?;
    Ok(())
}

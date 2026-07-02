fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = std::env::var("PROTO_SRC")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("../protos"));

    // First compile quic protos so their stubs are generated
    let quic_protos = &[
        proto_root.join("quic/common.proto"),
        proto_root.join("quic/combat.proto"),
    ];
    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .compile(quic_protos, &[proto_root.clone()])?;

    // Then compile spectator proto with extern_path to quic types
    let grpc_protos = &[proto_root.join("grpc/spectator.proto")];
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .extern_path(".stargem.quic.combat", "crate::proto::combat")
        .extern_path(".stargem.quic.common", "crate::proto::common")
        .compile(grpc_protos, &[proto_root])?;
    Ok(())
}
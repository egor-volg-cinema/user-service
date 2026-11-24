fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. vendored protoc, чтобы не зависеть от системного.
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    // SAFETY: build-скрипт однопоточный, гонок нет.
    unsafe { std::env::set_var("PROTOC", protoc); }

    // 2. Пересобирать build.rs при изменении .proto.
    println!("cargo:rerun-if-changed=proto/account.proto");
    println!("cargo:rerun-if-changed=proto/auth.proto");
    println!("cargo:rerun-if-changed=proto/users.proto");

    // 3. Генерация.
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(
            &["proto/account.proto", "proto/auth.proto", "proto/users.proto"],
            &["proto/"],
        )?;

    Ok(())
}
fn main() {
    tonic_build::configure()
    .type_attribute("user.GetUserRequest", "#[derive(Sized)]")
    .compile(&["user.proto"], &["../proto"])
    .unwrap_or_else(|e| {
        panic!("Failed to compile proto files: {}", e);
    });
}
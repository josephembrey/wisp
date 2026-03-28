fn main() {
    let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/lib/bindings.ts");
    app_lib::specta_builder()
        .export(app_lib::ts_export_config(), out)
        .expect("Failed to export typescript bindings");
}

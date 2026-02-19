fn main() {
    app_lib::specta_builder()
        .export(app_lib::ts_export_config(), "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");
}

const GUACAMOLE_SHARED: &str = "dist/include/guacamole";


fn collect_libguac_headers<P: AsRef<std::path::Path>>(include_path: P) -> impl Iterator<Item = String> {
    std::fs::read_dir(include_path)
        .expect("No such include directory!")
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().map(|m| m.is_file()).unwrap_or_default())
        .filter_map(|e| e.path().to_str().map(ToString::to_string))
}

fn main() {
    let out_dir = std::path::PathBuf::from(
        std::env::var("OUT_DIR")
            .expect("OUT_DIR not defined")
    );

    let mut builder = collect_libguac_headers(GUACAMOLE_SHARED)
        .fold(bindgen::builder(), |builder, path| builder.header(path))
        .generate_comments(false);

    if let Some(cflags) = option_env!("CFLAGS") {
        builder = builder.clang_args(cflags.split(' '));
    }

    builder.generate()
        .expect("Failed to generate bindings!")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Failed to write bindings to file!")
}
const GUACAMOLE_SHARED: &str = "dist/include/guacamole";
const GUACAMOLE_LIB: &str = "dist/lib";
const EXT: &str = "so";
#[cfg(target_os = "macos")]
const SHARED_EXT: &str = "dylib";
#[cfg(target_os = "linux")]
const SHARED_EXT: &str = "so";
#[cfg(target_os = "windows")]
const SHARED_EXT: &str = "dll";
const BINDINGS: &str = "bindings.rs";


fn collect_libguac_headers<P: AsRef<std::path::Path>>(
    include_path: P,
) -> impl Iterator<Item = String> {
    std::fs::read_dir(include_path)
        .expect("No such include directory!")
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().map(|m| m.is_file()).unwrap_or_default())
        .filter_map(|e| e.path().to_str().map(ToString::to_string))
}

fn main() {
    println!("cargo:rustc-link-search=native=./libguac-sys/dist/lib");
    println!("cargo:rustc-link-lib=static=guac");

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")
        .expect("OUT_DIR not defined"));

    if std::fs::metadata(out_dir.join(BINDINGS)).is_err() {
        let mut builder = collect_libguac_headers(GUACAMOLE_SHARED)
            .fold(bindgen::builder(), |builder, path| builder.header(path))
            .generate_comments(false);

        if let Some(cflags) = option_env!("CFLAGS") {
            builder = builder.clang_args(cflags.split(' '));
        }

        let shared_lib_out_dir = out_dir
            .join("..")
            .join("..")
            .join("..");
        let pattern = format!("{}/*.0.{}", GUACAMOLE_LIB, SHARED_EXT);
        for shared in glob::glob(&pattern).expect("Failed to list shared library").filter_map(Result::ok) {
            if let Some(filename) = std::path::PathBuf::from(&shared).file_name().and_then(|fname| fname.to_str()) {
                let filename = filename.replace(
                    &format!("0.{}", SHARED_EXT),
                    EXT
                );
                std::fs::copy(shared, shared_lib_out_dir.join(filename))
                    .expect("Failed to copy shared library");
            }
        }

        builder
            .generate()
            .expect("Failed to generate bindings!")
            .write_to_file(out_dir.join(BINDINGS))
            .expect("Failed to write bindings to file!")
    }
}

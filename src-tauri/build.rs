use std::env;
use std::path::PathBuf;
use std::process::Command;

const APP_COMMANDS: &[&str] = &[
    "get_vault_status",
    "create_vault",
    "unlock_with_password",
    "unlock_with_recovery_key",
    "lock_vault",
    "report_activity",
    "list_folders",
    "create_folder",
    "update_folder",
    "delete_folder",
    "list_items",
    "get_item",
    "create_item",
    "reveal_item_field",
    "copy_item_field",
    "update_item",
    "delete_item",
    "delete_items",
    "move_items",
    "import_file",
    "export_file",
    "change_password",
    "create_new_recovery_key",
    "get_biometric_status",
    "enable_biometric_unlock",
    "disable_biometric_unlock",
    "unlock_with_biometrics",
];

const MINIMUM_MACOS_VERSION: &str = "11.0";
const SWIFT_FRAMEWORKS: &[&str] = &[
    "Foundation",
    "AppKit",
    "Security",
    "LocalAuthentication",
    "CryptoKit",
];

fn swift_target() -> String {
    let architecture = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64".to_string(),
        Ok(other) => other.to_string(),
        Err(_) => "arm64".to_string(),
    };
    format!("{architecture}-apple-macos{MINIMUM_MACOS_VERSION}")
}

fn swift_runtime_library_paths(target: &str) -> Vec<String> {
    let output = Command::new("swiftc")
        .args(["-print-target-info", "-target", target])
        .output()
        .expect("swiftc is required to build the macOS Touch ID bridge");
    let info = String::from_utf8_lossy(&output.stdout).into_owned();

    let Some(key_position) = info.find("\"runtimeLibraryPaths\"") else {
        return Vec::new();
    };
    let after_key = &info[key_position..];
    let (Some(start), Some(end)) = (after_key.find('['), after_key.find(']')) else {
        return Vec::new();
    };
    after_key[start + 1..end]
        .split('"')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

fn build_swift_bridge() {
    let manifest_directory = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let output_directory = PathBuf::from(env::var("OUT_DIR").expect("output dir"));
    let swift_directory = manifest_directory.join("swift");
    let mut sources: Vec<PathBuf> = std::fs::read_dir(&swift_directory)
        .expect("the swift folder is missing")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "swift")
        })
        .collect();
    sources.sort();
    let library = output_directory.join("libhadesnative.a");

    let status = Command::new("swiftc")
        .args(["-O", "-parse-as-library", "-emit-library", "-static"])
        .args(["-module-name", "HadesNative", "-target", &swift_target()])
        .args(&sources)
        .arg("-o")
        .arg(&library)
        .status()
        .expect("swiftc is required to build the macOS Touch ID bridge");
    assert!(status.success(), "the Swift bridge failed to build");

    println!("cargo:rerun-if-changed=swift");
    println!(
        "cargo:rustc-link-search=native={}",
        output_directory.display()
    );
    println!("cargo:rustc-link-lib=static=hadesnative");
    for path in swift_runtime_library_paths(&swift_target()) {
        println!("cargo:rustc-link-search=native={path}");
    }
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    for framework in SWIFT_FRAMEWORKS {
        println!("cargo:rustc-link-lib=framework={framework}");
    }
}

fn main() {
    build_swift_bridge();
    let app_manifest = tauri_build::AppManifest::new().commands(APP_COMMANDS);
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(app_manifest))
        .expect("failed to run the Tauri build step");
}

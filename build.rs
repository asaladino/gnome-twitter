use std::process::Command;
use std::path::Path;
use std::fs::remove_file;
use std::str::from_utf8;

fn main() {
    // Remove old versions of the gresource to make sure we're using the latest version
    if Path::new("src/res/resources.gresource").exists() {
        remove_file("src/res/resources.gresource").unwrap();
    }

    // Compile Gresource
    let output =
        Command::new(option_env!("GRESOURCE_BINARY_PATH").unwrap_or("glib-compile-resources"))
            .args(&["--generate", "resources.xml"])
            .current_dir("src/res")
            .output()
            .unwrap();

    if !output.status.success() {
        println!("Failed to generate GResources!");
        println!(
            "glib-compile-resources stdout: {}",
            from_utf8(&output.stdout).unwrap()
        );
        println!(
            "glib-compile-resources stderr: {}",
            from_utf8(&output.stderr).unwrap()
        );
        panic!("Can't continue build without GResources!");
    }

    println!("cargo:rerun-if-env-changed=GXI_LOCALEDIR");
    println!("cargo:rerun-if-env-changed=GXI_VERSION");
    println!("cargo:rerun-if-env-changed=GXI_PLUGIN_DIR");
    println!("cargo:rerun-if-env-changed=GXI_XI_BINARY_PATH");
}
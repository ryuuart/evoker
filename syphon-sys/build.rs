use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .status()
        .expect("Failed to download Syphon-Framework repository.");

    Command::new("xcodebuild")
        .args(&[
            "-project",
            "Syphon-Framework/Syphon.xcodeproj",
            "-scheme",
            "Syphon",
            "build",
            "-configuration",
            "Debug",
        ])
        .arg(&format!("CONFIGURATION_BUILD_DIR={}", &out_dir))
        .status()
        .expect("Syphon-Framework build failed.");

    println!("cargo::rustc-link-search=framework={}", &out_dir);
    println!("cargo::rustc-link-arg=-Wl,-rpath,{}", &out_dir);
    println!("cargo::rustc-link-lib=framework=CoreGraphics");
    println!("cargo::rustc-link-lib=framework=Syphon");
}

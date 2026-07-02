use std::fs;

// Detect the installed CUDA toolkit major version and expose it as a `cuda_13` cfg,
// so the wrapper can switch between the CUDA 12.x and 13.x API shapes
// (`cudaGetDeviceProperties[_v2]`, and `cudaMemLocation.id` flat vs. anonymous union).
//
// Detection mirrors cudart-sys/build.rs: honor the `CUDA_VERSION` override first,
// otherwise read `/usr/local/cuda/version.json`. On a machine without CUDA (e.g. macOS)
// this simply sets no cfg — the crate does not build there anyway.
fn detect_cuda_version() -> Option<String> {
    if let Ok(v) = std::env::var("CUDA_VERSION") {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let content = fs::read_to_string("/usr/local/cuda/version.json").ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    value["cuda"]["version"].as_str().map(str::to_string)
}

fn main() {
    // Declare the custom cfg name so Rust 1.80+ does not warn about it.
    println!("cargo:rustc-check-cfg=cfg(cuda_13)");
    println!("cargo:rerun-if-env-changed=CUDA_VERSION");
    println!("cargo:rerun-if-changed=/usr/local/cuda/version.json");

    if let Some(version) = detect_cuda_version() {
        if version.starts_with("13.") {
            println!("cargo:rustc-cfg=cuda_13");
        }
    }
}

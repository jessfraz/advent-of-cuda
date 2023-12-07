#[cfg(not(target_os = "macos"))]
use cuda_builder::CudaBuilder;

#[cfg(not(target_os = "macos"))]
fn main() {
    let gpu_dir = std::env::current_dir().unwrap().join("gpu");
    let res_dir = std::env::current_dir().unwrap().join("resources");
    CudaBuilder::new(gpu_dir)
        .copy_to(res_dir.join("gpu.ptx"))
        .build()
        .unwrap();
}

#[cfg(target_os = "macos")]
fn main() {}

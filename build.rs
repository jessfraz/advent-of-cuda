use cuda_builder::CudaBuilder;

fn main() {
    CudaBuilder::new("../gpu")
        .copy_to("../resources/gpu.ptx")
        .build()
        .unwrap();
}

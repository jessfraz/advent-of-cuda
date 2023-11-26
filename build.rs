use cuda_builder::CudaBuilder;

fn main() {
    CudaBuilder::new(std::env::current_dir().unwrap())
        .build()
        .unwrap();
}

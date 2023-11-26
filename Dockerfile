FROM nvidia/cuda:11.5.2-cudnn8-devel-ubuntu20.04

# Update default packages
RUN apt-get update \
	&& apt-get install -y \
    build-essential \
    curl \
	libssl-dev \
	libtinfo-dev \
	libxml2-dev \
	pkg-config \
	xz-utils \
	zlib1g-dev \
	--no-install-recommends \
	&& rm -rf /var/lib/apt/lists/*

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# get prebuilt llvm
RUN curl -O "https://releases.llvm.org/7.0.1/clang+llvm-7.0.1-x86_64-linux-gnu-ubuntu-18.04.tar.xz" \
    && xz -d /clang+llvm-7.0.1-x86_64-linux-gnu-ubuntu-18.04.tar.xz \
    && tar xf /clang+llvm-7.0.1-x86_64-linux-gnu-ubuntu-18.04.tar -C /usr --strip-components=1 \
    && rm /clang+llvm-7.0.1-x86_64-linux-gnu-ubuntu-18.04.tar

# set env
ENV LLVM_CONFIG=/usr/bin/llvm-config
ENV CUDA_ROOT=/usr/local/cuda
ENV CUDA_PATH=$CUDA_ROOT
ENV LLVM_LINK_STATIC=1
ENV RUST_LOG=info
ENV PATH=$CUDA_ROOT/nvvm/lib64:/root/.cargo/bin:$PATH

# make ld aware of necessary *.so libraries
RUN echo $CUDA_ROOT/lib64 >> /etc/ld.so.conf &&\
    echo $CUDA_ROOT/compat >> /etc/ld.so.conf &&\
    echo $CUDA_ROOT/nvvm/lib64 >> /etc/ld.so.conf &&\
    ldconfig

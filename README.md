# advent-of-cuda

I want to get better at writing cuda kernels so I plan on doing advent of code
with cuda.

You can follow along here.

This for [advent of code 2023](https://adventofcode.com/).

Some days problems wont be able to be solved this way, but we will just take
them as we go! Probably will modify the problems just to learn things.

You will need cuda installed and llvm, [see
here](https://github.com/Rust-GPU/Rust-CUDA/blob/master/guide/src/guide/getting_started.md).

The kernels themselves are in the [`gpu/`](gpu/) directory. Then I test them from the code in [`src/`](src/).

> This repo depends on a super old toolchain because of the rust cuda sdk. In areas the rust cuda sdk won't work for what we want, we can wrap c++ in rust.

//! Advent of Code using CUDA and Rust. This is the GPU specific code.

#![deny(missing_docs)]
// This does a couple of things:
// - It only applies the attributes if we are compiling the crate for the GPU
//   (target_os = "cuda").
// - It declares the crate to be `no_std` on CUDA targets.
// - It registers a special attribute required by the codegen for things like
//   figuring out what functions are GPU kernels.
// - It explicitly includes `kernel` macro and `thread`
#![cfg_attr(
    target_os = "cuda",
    no_std,
    feature(register_attr),
    register_attr(nvvm_internal)
)]
// To use types such as slices or arrays inside of GPU kernels
// we allow `improper_cytypes_definitions`.
// This is because on the CPU, such types are not guaranteed to be passed a
// certain way, so they should not be used in `extern "C"` functions
// (which is what kernels are implicitly declared as). However,
// `rustc_codegen_nvvm` guarantees the way in which things like structs,
// slices, and arrays are passed.
#![allow(improper_ctypes_definitions)]

// To use `alloc` or things like printing from GPU kernels (which requires
// alloc), we declare `alloc`.
extern crate alloc;

pub mod add;

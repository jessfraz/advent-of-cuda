//! Advent of Code using CUDA and Rust.

#![deny(missing_docs)]

pub mod add;

/// The PTX code for the GPU kernel.
pub(crate) static PTX: &str = include_str!("../resources/gpu.ptx");

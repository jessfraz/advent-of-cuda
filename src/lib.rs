//! Advent of Code using CUDA and Rust.

#![deny(missing_docs)]

pub mod add;
pub mod day01;

/// The PTX code for the GPU kernel.
pub(crate) static PTX: &str = include_str!("../resources/gpu.ptx");

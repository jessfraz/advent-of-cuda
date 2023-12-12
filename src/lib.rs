//! Advent of Code using CUDA and Rust.

#![deny(missing_docs)]

#[cfg(not(target_os = "macos"))]
pub mod add;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
#[cfg(not(target_os = "macos"))]
pub mod gpu_day08;

/// The PTX code for the GPU kernel.
#[cfg(not(target_os = "macos"))]
pub(crate) static PTX: &str = include_str!("../resources/gpu.ptx");

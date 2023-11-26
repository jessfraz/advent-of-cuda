//! This example shows how to add two vectors together using CUDA.
use cuda_std::*;

/// Adds two vectors together and stores the result in `c`.
#[kernel]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn add(a: &[f32], b: &[f32], c: *mut f32) {
    let idx = thread::index_1d() as usize;
    if idx < a.len() {
        let elem = &mut *c.add(idx);
        *elem = a[idx] + b[idx];
    }
}

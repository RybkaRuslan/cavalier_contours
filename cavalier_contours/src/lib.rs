//! 2D geometry polyline/shape library for offsetting, combining, computing areas, path lengths,
//! winding numbers, etc.

pub use static_aabb2d_index;

#[macro_use]
mod macros;
#[macro_use]
pub mod core;
pub mod polyline;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

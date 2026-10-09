//! Renderer-agnostic barcode symbol encoders.
//!
//! Encoders turn ZPL field data into geometry in module units; renderers scale modules to
//! dots and draw them. `linear` holds the 1D symbologies, `matrix` the 2D ones.

pub mod linear;
pub mod matrix;

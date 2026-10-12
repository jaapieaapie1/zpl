//! Renderer-agnostic barcode symbol encoders.
//!
//! Encoders turn ZPL field data into geometry in module units; renderers scale modules to
//! dots and draw them. `linear` holds the 1D symbologies, `matrix` the square-module 2D ones,
//! `stacked` Codablock F and Code 49, `micropdf` MicroPDF417 and TLC39,
//! `databar` GS1 DataBar, `composite` GS1 composite symbols (^BR types 7-12 and `|` data), `maxicode` MaxiCode (hexagonal) and `postal` the height-modulated
//! postal codes.

pub mod composite;
pub mod databar;
pub mod linear;
pub mod matrix;
pub mod maxicode;
pub mod micropdf;
pub mod postal;
pub mod stacked;

/// A multi-row symbol whose rows can have different heights.
#[derive(Debug, Clone, PartialEq)]
pub struct Stacked {
    /// Width of every row in modules.
    pub width: u32,
    pub rows: Vec<StackedRow>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackedRow {
    /// `width` modules, `true` = dark.
    pub modules: Vec<bool>,
    /// How tall the row is drawn. See [`RowHeight`].
    pub height: RowHeight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RowHeight {
    /// A fixed number of modules (module width units), e.g. 1-module separator rows or
    /// symbology-defined row heights.
    Modules(u32),
    /// A share of the ZPL row/bar height parameter: the renderer gives every `Bar` row
    /// `bar_height * weight / sum(weights of Bar rows)` dots unless the command sets a
    /// per-row height, in which case each `Bar` row gets that height.
    Bar { weight: u32 },
}

/// One bar of a height-modulated (postal) symbol. `top` and `bottom` are fractions of the
/// full bar height measured from the top (0.0 = top, 1.0 = bottom), so a full bar is
/// `0.0..1.0`, a POSTNET short bar `0.6..1.0` and an IMb tracker `0.375..0.625`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PostalBar {
    pub top: f32,
    pub bottom: f32,
}

/// Common error for the encoders that don't define their own.
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolError {
    /// Character not encodable by the symbology.
    InvalidCharacter(char),
    /// Data length not allowed (or too long for the largest symbol).
    InvalidLength(usize),
    /// Malformed ZPL field data for the symbology (e.g. MaxiCode header).
    InvalidData(String),
    /// Variant not implemented.
    Unsupported,
}

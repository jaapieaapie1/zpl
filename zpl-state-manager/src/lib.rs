mod charset;
mod clock;
mod pages;
#[cfg(feature = "source-map")]
mod source_map;

#[cfg(feature = "source-map")]
pub use source_map::Source;

pub use pages::{Page, MAX_PAGES};
use pages::{PageLog, Serial};

use std::collections::HashMap;
use std::sync::Arc;

use zpl_parser::command::{
    BlockJustification, Code128Mode, Code49Interpretation, Command, DiagonalDirection, Encoding, FieldDirection,
    FontName, Justification, LineColor, LinearKind, Orientation, PostalKind, QrErrorCorrection, QrModel, TextLayout,
};

#[derive(Debug, Clone, PartialEq)]
pub enum DrawInstruction {
    DrawText {
        x: u32,
        y: u32,
        text: String,
        font: FontName,
        height: u32,
        width: u32,
        orientation: Orientation,
        justification: Justification,
        /// ^FO places the top of the text at `y`, ^FT the baseline.
        anchor: TextAnchor,
        /// ^FB / ^TB word-wrapped block.
        block: Option<TextBlock>,
        /// ^FP direction.
        direction: FieldDirection,
        /// ^FP extra gap between characters, in dots.
        char_gap: u32,
        /// TrueType/OpenType font downloaded with ~DU / ~DT / ~DY and selected through ^A@ or
        /// a ^CW alias; `None` means the built-in font for `font`.
        custom_font: Option<Arc<Vec<u8>>>,
        /// ^PA advanced layout (bidi, shaping, OpenType features).
        layout: TextLayout,
    },
    /// Code 128 (^BC).
    DrawBarcode {
        x: u32,
        y: u32,
        data: String,
        orientation: Orientation,
        height: u32,
        module_width: u32,
        wide_bar_ratio: f32,
        print_interpretation_line: bool,
        print_interpretation_line_above: bool,
        check_digit: bool,
        mode: Code128Mode,
    },
    /// Every other 1D symbology; `(x, y)` is the top-left of the bars.
    DrawLinear {
        x: u32,
        y: u32,
        kind: LinearKind,
        data: String,
        orientation: Orientation,
        height: u32,
        module_width: u32,
        wide_bar_ratio: f32,
        print_interpretation_line: bool,
        print_interpretation_line_above: bool,
    },
    DrawQrCode {
        x: u32,
        y: u32,
        data: String,
        orientation: Orientation,
        model: QrModel,
        magnification: u32,
        error_correction: QrErrorCorrection,
        mask: Option<u32>,
        /// Placed with ^FT: `y` is the bottom edge of the symbol.
        bottom: bool,
    },
    DrawDataMatrix {
        x: u32,
        y: u32,
        data: String,
        orientation: Orientation,
        /// Module size in dots; `None` means the symbol should be about `symbol_height` tall.
        module_height: Option<u32>,
        symbol_height: u32,
        columns: u32,
        rows: u32,
        escape: Option<char>,
        rectangular: bool,
        bottom: bool,
    },
    DrawPdf417 {
        x: u32,
        y: u32,
        data: String,
        orientation: Orientation,
        /// Row height in multiples of `module_width`; `None` means rows share `symbol_height`.
        row_height: Option<u32>,
        module_width: u32,
        symbol_height: u32,
        security: Option<u8>,
        columns: u32,
        rows: u32,
        truncated: bool,
        bottom: bool,
    },
    DrawAztec {
        x: u32,
        y: u32,
        data: String,
        orientation: Orientation,
        magnification: u32,
        ecc: u32,
        bottom: bool,
    },
    DrawBox {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        thickness: u32,
        color: LineColor,
        rounding: u32,
    },
    DrawCircle {
        x: u32,
        y: u32,
        diameter: u32,
        thickness: u32,
        color: LineColor,
    },
    DrawEllipse {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        thickness: u32,
        color: LineColor,
    },
    DrawDiagonalLine {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        thickness: u32,
        color: LineColor,
        direction: DiagonalDirection,
    },
    /// 1-bit bitmap (^GF, ^XG, ^IM): rows of `bytes_per_row` bytes, MSB first, set bit = black.
    DrawGraphicField {
        x: u32,
        y: u32,
        bytes_per_row: u32,
        data: Vec<u8>,
        magnification_x: u32,
        magnification_y: u32,
    },
    /// Symbols drawn from `zpl-symbols` encoders that have no dedicated variant. `bottom`: the
    /// origin came from ^FT, so `y` is the symbol's bottom edge.
    DrawSymbol {
        x: u32,
        y: u32,
        data: String,
        orientation: Orientation,
        bottom: bool,
        spec: SymbolSpec,
    },
    /// ^FR / ^LR: the inner element inverts whatever is underneath instead of painting black.
    Reverse(Box<DrawInstruction>),
}

impl DrawInstruction {
    /// The field data a text, barcode or symbol was drawn from, for swapping in another
    /// value (^SN / ^SF serialization). Renderers derive everything else, such as barcode
    /// interpretation lines, from it.
    pub fn field_data_mut(&mut self) -> Option<&mut String> {
        match self {
            DrawInstruction::DrawText { text: data, .. }
            | DrawInstruction::DrawBarcode { data, .. }
            | DrawInstruction::DrawLinear { data, .. }
            | DrawInstruction::DrawQrCode { data, .. }
            | DrawInstruction::DrawDataMatrix { data, .. }
            | DrawInstruction::DrawPdf417 { data, .. }
            | DrawInstruction::DrawAztec { data, .. }
            | DrawInstruction::DrawSymbol { data, .. } => Some(data),
            DrawInstruction::Reverse(inner) => inner.field_data_mut(),
            DrawInstruction::DrawBox { .. }
            | DrawInstruction::DrawCircle { .. }
            | DrawInstruction::DrawEllipse { .. }
            | DrawInstruction::DrawDiagonalLine { .. }
            | DrawInstruction::DrawGraphicField { .. } => None,
        }
    }
}

/// Symbology and resolved sizes (in dots) for `DrawInstruction::DrawSymbol`.
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolSpec {
    /// ^BD; fixed physical size.
    MaxiCode { mode: u8, symbol_number: u8, total_symbols: u8 },
    /// ^BR; `kind` is the ZPL type 1-12, `height` the linear component's bar height.
    DataBar { kind: u8, module_width: u32, separator_height: u32, height: u32, segments_per_row: u32 },
    /// ^BF
    MicroPdf417 { module_width: u32, row_height: u32, mode: u8 },
    /// ^BT
    Tlc39 { code39_module_width: u32, code39_ratio: f32, code39_height: u32, micro_module_width: u32, micro_row_height: u32 },
    /// ^BB; `mode` is A, E or F.
    Codablock { module_width: u32, row_height: u32, security: bool, columns: u32, rows: u32, mode: char },
    /// ^B4
    Code49 { module_width: u32, row_height: u32, interpretation: Code49Interpretation, start_mode: Option<u8> },
    /// ^BZ / ^B5
    Postal { kind: PostalKind, module_width: u32, height: u32, print_interpretation_line: bool, print_above: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAnchor {
    Top,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextBlock {
    pub width: u32,
    pub max_lines: u32,
    /// Extra dots between lines (^FB c).
    pub line_spacing: i32,
    pub justification: BlockJustification,
    /// Indent of every line but the first (^FB e).
    pub hanging_indent: u32,
    /// ^TB: lines beyond this height are dropped. ^FB instead overprints extra lines on the last one.
    pub clip_height: Option<u32>,
}

/// Printer resolution used unless [`StateManager::set_dpmm`] says otherwise: 8 dots/mm.
pub use zpl_parser::DEFAULT_DPMM;

/// The printhead resolutions Zebra makes, in dots per millimetre.
pub const DPMM_VALUES: [u32; 4] = [6, 8, 12, 24];

/// Nominal dots per inch of a `dpmm` printhead as Zebra (and Labelary) round them: 152,
/// 203, 300 and 600 rather than the exact 152.4, 203.2, 304.8 and 609.6.
pub fn nominal_dpi(dpmm: u32) -> u32 {
    match dpmm {
        6 => 152,
        8 => 203,
        12 => 300,
        24 => 600,
        _ => (dpmm as f32 * 25.4).round() as u32,
    }
}

/// Whole-label settings that apply to the page rather than to a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageSetup {
    /// Label size in format dots, the unit of every instruction on the page.
    pub width: u32,
    pub height: u32,
    /// ^POI: label printed rotated 180 degrees.
    pub inverted: bool,
    /// ^PMY: label printed mirrored horizontally.
    pub mirrored: bool,
    /// Printer resolution in dots per millimetre.
    pub dpmm: u32,
    /// Printer dots per format dot as a fraction (numerator, denominator): 2/1 after ^JMB,
    /// desired/base dpi after ^MUd,b,c, 1/1 normally.
    pub scale: (u32, u32),
}

impl Default for PageSetup {
    /// A 4 x 6 inch label at 8 dots/mm.
    fn default() -> Self {
        Self { width: 812, height: 1218, inverted: false, mirrored: false, dpmm: DEFAULT_DPMM, scale: (1, 1) }
    }
}

impl PageSetup {
    /// Printer dots per format dot.
    pub fn dot_scale(&self) -> f32 {
        self.scale.0 as f32 / self.scale.1 as f32
    }

    /// Format dots per inch: the printer's nominal dpi divided by [`PageSetup::dot_scale`].
    pub fn format_dpi(&self) -> f32 {
        nominal_dpi(self.dpmm) as f32 / self.dot_scale()
    }
}

#[derive(Debug, Clone)]
enum FieldMode {
    Text,
    GraphicSymbol {
        orientation: Orientation,
        height: u32,
        width: u32,
    },
    Code128 {
        orientation: Orientation,
        height: u32,
        print_interpretation_line: bool,
        print_interpretation_line_above: bool,
        check_digit: bool,
        mode: Code128Mode,
    },
    Linear {
        kind: LinearKind,
        orientation: Orientation,
        height: u32,
        print_interpretation_line: bool,
        print_interpretation_line_above: bool,
    },
    QrCode {
        orientation: Orientation,
        model: QrModel,
        magnification: u32,
        error_correction: QrErrorCorrection,
        mask: Option<u32>,
    },
    DataMatrix {
        orientation: Orientation,
        module_height: Option<u32>,
        columns: u32,
        rows: u32,
        escape: Option<char>,
        rectangular: bool,
    },
    Pdf417 {
        orientation: Orientation,
        row_height: Option<u32>,
        security: Option<u8>,
        columns: u32,
        rows: u32,
        truncated: bool,
    },
    Aztec {
        orientation: Orientation,
        magnification: u32,
        ecc: u32,
    },
    Symbol {
        orientation: Orientation,
        spec: SymbolSpec,
    },
}

/// Settings that only last until the field's ^FS.
#[derive(Debug, Clone)]
struct Field {
    data: Option<String>,
    mode: FieldMode,
    hex_indicator: Option<char>,
    reverse: bool,
    block: Option<TextBlock>,
    direction: FieldDirection,
    char_gap: u32,
    number: Option<u32>,
    /// ^TB orientation override for the text field.
    orientation: Option<Orientation>,
    /// ^FC primary/secondary/tertiary clock indicators.
    clock: Option<(char, Option<char>, Option<char>)>,
    /// ^FE indicator.
    embed: Option<char>,
    /// ^SN / ^SF: how the data changes from one printed copy to the next.
    serial: Option<Serial>,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            data: None,
            mode: FieldMode::Text,
            hex_indicator: None,
            reverse: false,
            block: None,
            direction: FieldDirection::Horizontal,
            char_gap: 0,
            number: None,
            orientation: None,
            clock: None,
            embed: None,
            serial: None,
        }
    }
}

#[derive(Debug)]
pub struct StateManager {
    label_home_x: u32,
    label_home_y: u32,
    label_shift: i32,
    label_top: i32,

    field_origin_x: u32,
    field_origin_y: u32,
    /// Origin came from ^FT (baseline / bottom-left) rather than ^FO (top-left).
    field_typeset: bool,
    field_justification: Justification,

    default_orientation: Orientation,
    default_justification: Justification,

    current_font: FontName,
    current_font_orientation: Orientation,
    current_font_height: u32,
    current_font_width: u32,
    current_custom_font: Option<Arc<Vec<u8>>>,
    text_layout: TextLayout,

    print_width: Option<u32>,
    label_length: Option<u32>,
    /// Printer resolution, ^JMB half density and the ^MU base/desired dpi conversion.
    dpmm: u32,
    half_density: bool,
    conversion: (u32, u32),
    inverted: bool,
    mirrored: bool,
    label_reverse: bool,

    media_darkness: i32,
    character_encoding: Encoding,

    barcode_module_width: u32,
    barcode_wide_bar_ratio: f32,
    barcode_height: u32,

    field: Field,

    /// ^DF formats and ~DG / ~DY graphics, keyed by `stored_name`. They outlive formats.
    formats: HashMap<String, Vec<Command>>,
    graphics: HashMap<String, (u32, Vec<u8>)>,
    /// ^DF in progress: commands are stored until ^XZ.
    recording: Option<(String, Vec<Command>)>,
    /// ^XF in this format: the stored format is drawn at ^XZ with these ^FN values.
    recall: Option<String>,
    field_values: HashMap<u32, String>,
    /// While a stored format is replayed: the ^FN values supplied by the recalling format.
    replay_values: Option<HashMap<u32, String>>,
    /// ^FN data of the fields printed so far in this format, for ^FE references.
    format_values: HashMap<u32, String>,

    /// ^ST clock (else the system clock, UTC) and the ^SO offsets of clocks 2 and 3.
    clock: Option<clock::DateTime>,
    clock_offsets: [clock::Offset; 2],

    /// Downloaded fonts (~DU / ~DT / ~DY) and ^CW letter aliases, keyed by `stored_name`.
    fonts: HashMap<String, Arc<Vec<u8>>>,
    font_aliases: HashMap<char, String>,

    /// ^IS images (the drawing of a whole format) and the pending save of this format.
    images: HashMap<String, Vec<DrawInstruction>>,
    save_image: Option<(String, bool)>,
    /// Index of this format's first instruction in the output.
    format_start: usize,

    /// ^PQ total labels and replicates of each serial number, for this format.
    quantity: u32,
    replicates: u32,
    /// Where each label ends, recorded only by [`StateManager::process_pages`].
    page_log: Option<PageLog>,
    max_pages: usize,
}

/// ^GS symbols by field data character.
fn graphic_symbol(c: char) -> Option<char> {
    match c.to_ascii_uppercase() {
        'A' => Some('\u{00AE}'),
        'B' => Some('\u{00A9}'),
        'C' => Some('\u{2122}'),
        'D' => Some('\u{24C1}'),
        'E' => Some('\u{24D2}'),
        _ => None,
    }
}

/// Object names are matched without their storage device (`R:`, `E:`, ...) and extension,
/// so `~DGR:LOGO.GRF` is found by `^XGE:LOGO.GRF` and `~DYR:LOGO,...` alike.
fn stored_name(name: &str) -> String {
    let name = name.rsplit(':').next().unwrap_or(name);
    let name = name.split('.').next().unwrap_or(name);
    name.trim().to_ascii_uppercase()
}

/// Decodes ^FH escapes (`<indicator>` + two hex digits). Each run of consecutive escaped
/// bytes is decoded with the ^CI character set; literal text is kept as is.
fn decode_hex_escapes(data: &str, indicator: char, encoding: Encoding) -> String {
    let hex = |c: Option<char>| c.and_then(|c| c.to_digit(16));
    let mut out = String::with_capacity(data.len());
    let mut run = Vec::new();
    let mut rest = data;
    while let Some(c) = rest.chars().next() {
        if c == indicator {
            let mut it = rest[c.len_utf8()..].chars();
            if let (Some(h), Some(l)) = (hex(it.next()), hex(it.next())) {
                run.push((h * 16 + l) as u8);
                rest = &rest[c.len_utf8() + 2..];
                continue;
            }
        }
        if !run.is_empty() {
            out.push_str(&charset::decode(&run, encoding));
            run.clear();
        }
        out.push(c);
        rest = &rest[c.len_utf8()..];
    }
    if !run.is_empty() {
        out.push_str(&charset::decode(&run, encoding));
    }
    out
}

/// Expands ^FE references to ^FN data: `#n#` inserts field n, `#n,f,x,y#` / `#n,b,x,y#` inserts
/// `y` characters starting at position `x` (1-based) counted from the front or back.
fn embed_fields(data: &str, indicator: char, lookup: impl Fn(u32) -> Option<String>) -> String {
    let mut out = String::with_capacity(data.len());
    let mut parts = data.split(indicator);
    out.push_str(parts.next().unwrap_or(""));
    let mut parts = parts.peekable();
    while let Some(spec) = parts.next() {
        let mut fields = spec.split(',');
        let number = fields.next().and_then(|n| n.trim().parse::<u32>().ok());
        match (number, parts.peek().is_some()) {
            (Some(n), true) => {
                let value = lookup(n).unwrap_or_default();
                let chars: Vec<char> = value.chars().collect();
                let rest: Vec<&str> = fields.collect();
                let piece: String = match rest.as_slice() {
                    [] => value,
                    [dir, x, y] => {
                        let (x, y) = (x.trim().parse::<usize>().unwrap_or(0), y.trim().parse::<usize>().unwrap_or(0));
                        if x == 0 {
                            String::new()
                        } else if dir.trim().eq_ignore_ascii_case("b") {
                            // Counting back from the last character, then taking y characters onwards.
                            let start = chars.len().saturating_sub(x);
                            chars[start..].iter().take(y).collect()
                        } else {
                            chars.iter().skip(x - 1).take(y).collect()
                        }
                    }
                    _ => String::new(),
                };
                out.push_str(&piece);
                out.push_str(parts.next().unwrap_or(""));
            }
            _ => {
                out.push(indicator);
                out.push_str(spec);
            }
        }
    }
    out
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            label_home_x: 0,
            label_home_y: 0,
            label_shift: 0,
            label_top: 0,

            field_origin_x: 0,
            field_origin_y: 0,
            field_typeset: false,
            field_justification: Justification::Left,

            default_orientation: Orientation::Normal,
            default_justification: Justification::Left,

            current_font: FontName::FontA,
            current_font_orientation: Orientation::Normal,
            current_font_height: 9,
            current_font_width: 5,
            current_custom_font: None,
            text_layout: TextLayout::default(),

            print_width: None,
            label_length: None,
            dpmm: DEFAULT_DPMM,
            half_density: false,
            conversion: (1, 1),
            inverted: false,
            mirrored: false,
            label_reverse: false,

            media_darkness: 0,
            character_encoding: Encoding::Usa1,

            barcode_module_width: 2,
            barcode_wide_bar_ratio: 3.0,
            barcode_height: 10,

            field: Field::default(),

            formats: HashMap::new(),
            graphics: HashMap::new(),
            recording: None,
            recall: None,
            field_values: HashMap::new(),
            replay_values: None,
            format_values: HashMap::new(),

            clock: None,
            clock_offsets: Default::default(),

            fonts: HashMap::new(),
            font_aliases: HashMap::new(),

            images: HashMap::new(),
            save_image: None,
            format_start: 0,

            quantity: 1,
            replicates: 1,
            page_log: None,
            max_pages: MAX_PAGES,
        }
    }

    pub fn process(&mut self, commands: Vec<Command>) -> Vec<DrawInstruction> {
        let mut instructions = Vec::new();

        for command in commands {
            self.process_command(command, &mut instructions);
        }

        instructions
    }

    /// Like [`StateManager::process`], but split into the labels that would be printed: one
    /// page per ^XA...^XZ format that draws anything, repeated for ^PQ copies with ^SN /
    /// ^SF fields serialized. Always returns at least one (possibly blank) page and at most
    /// [`MAX_PAGES`] (see [`StateManager::set_max_pages`]).
    ///
    /// Page settings (^PW, ^LL, ^PO, ^PM) carry over from one format to the next, as on a
    /// printer; each page has the settings in effect at its ^XZ.
    pub fn process_pages(&mut self, commands: Vec<Command>) -> Vec<Page> {
        self.page_log = Some(PageLog::default());
        let mut out = Vec::new();
        for command in commands {
            self.process_command(command, &mut out);
        }
        let extra = vec![(); out.len()];
        self.paginate(out, extra).into_iter().map(|(page, _)| page).collect()
    }

    /// Caps the number of pages [`StateManager::process_pages`] returns, so `^PQ99999999`
    /// cannot exhaust memory. Defaults to [`MAX_PAGES`].
    pub fn set_max_pages(&mut self, max_pages: usize) {
        self.max_pages = max_pages.max(1);
    }

    /// Ends the page log started by `process_pages`, cutting `out` and the parallel `extra`
    /// into pages. Anything drawn after the last ^XZ is a final page.
    fn paginate<T: Clone>(&mut self, out: Vec<DrawInstruction>, extra: Vec<T>) -> Vec<(Page, Vec<T>)> {
        let mut log = self.page_log.take().unwrap_or_default();
        log.page_break(out.len(), self.page_setup(), self.quantity, self.replicates);
        log.paginate(out, extra, self.page_setup(), self.max_pages)
    }

    /// Sets the real-time clock used by ^FC fields to `secs` since the Unix epoch, in
    /// the printer's local time. A ^ST in the label still overrides it.
    pub fn set_clock_unix(&mut self, secs: i64) {
        self.clock = Some(clock::DateTime::from_unix(secs));
    }

    /// Sets the printer resolution in dots per millimetre (6, 8, 12 or 24; default 8). It
    /// sizes the default 4 x 6 inch label; parse with [`zpl_parser::Parser::with_dpmm`] at
    /// the same resolution so ^MU millimetres and inches match.
    pub fn set_dpmm(&mut self, dpmm: u32) {
        self.dpmm = dpmm.max(1);
    }

    /// Label size in format dots: ^PW / ^LL, else a 4 x 6 inch label at the printer's
    /// resolution (812 x 1218 at 8 dots/mm) shrunk by the ^JM / ^MU format scale.
    pub fn get_label_dimensions(&self) -> (u32, u32) {
        let (num, den) = self.format_scale();
        let dpi = nominal_dpi(self.dpmm);
        let inches = |n: u32| (n * dpi * den + num / 2) / num;
        let width = self.print_width.unwrap_or_else(|| inches(4));
        let height = self.label_length.unwrap_or_else(|| inches(6));
        (width, height)
    }

    fn format_scale(&self) -> (u32, u32) {
        let (base, desired) = self.conversion;
        (desired * if self.half_density { 2 } else { 1 }, base)
    }

    pub fn page_setup(&self) -> PageSetup {
        let (width, height) = self.get_label_dimensions();
        PageSetup {
            width,
            height,
            inverted: self.inverted,
            mirrored: self.mirrored,
            dpmm: self.dpmm,
            scale: self.format_scale(),
        }
    }

    /// Field origin in label coordinates: ^LH home, minus the ^LS shift, plus the ^LT offset.
    fn origin(&self) -> (u32, u32) {
        let x = self.label_home_x as i64 + self.field_origin_x as i64 - self.label_shift as i64;
        let y = self.label_home_y as i64 + self.field_origin_y as i64 + self.label_top as i64;
        (x.clamp(0, u32::MAX as i64) as u32, y.clamp(0, u32::MAX as i64) as u32)
    }

    /// Top-left of a graphic `height` dots tall: ^FT places its bottom edge on the origin.
    fn graphic_origin(&self, height: u32) -> (u32, u32) {
        let (x, y) = self.origin();
        if self.field_typeset { (x, y.saturating_sub(height)) } else { (x, y) }
    }

    fn emit(&self, out: &mut Vec<DrawInstruction>, instruction: DrawInstruction) {
        if self.field.reverse || self.label_reverse {
            out.push(DrawInstruction::Reverse(Box::new(instruction)));
        } else {
            out.push(instruction);
        }
    }

    /// Width 0 means "not given": scalable fonts then use the height, bitmap fonts the
    /// height's magnification.
    fn set_font_size(&mut self, height: Option<u32>, width: Option<u32>) {
        match (height, width) {
            (Some(h), w) => {
                self.current_font_height = h;
                self.current_font_width = w.unwrap_or(0);
            }
            // Width only: scalable fonts keep their aspect ratio.
            (None, Some(w)) => {
                self.current_font_height = w;
                self.current_font_width = w;
            }
            (None, None) => {}
        }
    }

    fn process_command(&mut self, command: Command, out: &mut Vec<DrawInstruction>) {
        if let Some((_, stored)) = &mut self.recording {
            if command != Command::EndFormat {
                stored.push(command);
                return;
            }
        }

        match command {
            Command::StartFormat => {
                self.reset_format_state();
                self.format_start = out.len();
            }
            Command::EndFormat => {
                self.end_format(out);
                self.format_start = out.len();
                let setup = self.page_setup();
                if let Some(log) = &mut self.page_log {
                    log.page_break(out.len(), setup, self.quantity, self.replicates);
                }
            }

            Command::LabelHome { x, y } => {
                self.label_home_x = x;
                self.label_home_y = y;
            }
            Command::LabelShift { shift } => self.label_shift = shift,
            Command::LabelTop { offset } => self.label_top = offset,

            Command::FieldOrigin { x, y, justification } => {
                self.field_origin_x = x;
                self.field_origin_y = y;
                self.field_typeset = false;
                self.field_justification = justification.unwrap_or(self.default_justification);
            }
            Command::FieldTypeset { x, y, justification } => {
                self.field_origin_x = x;
                self.field_origin_y = y;
                self.field_typeset = true;
                self.field_justification = justification.unwrap_or(self.default_justification);
            }

            Command::FieldOrientation { orientation, justification } => {
                self.default_orientation = orientation;
                if let Some(j) = justification {
                    self.default_justification = j;
                }
            }

            Command::Font { font, orientation, height, width } => {
                self.current_font = font;
                self.current_custom_font = self.aliased_font(font);
                self.current_font_orientation = orientation.unwrap_or(self.default_orientation);
                self.set_font_size(height, width);
            }

            Command::ChangeFont { font, height, width } => {
                self.current_font = font;
                self.current_custom_font = self.aliased_font(font);
                self.set_font_size(height, width);
            }

            Command::PrintWidth { width } => self.print_width = Some(width),
            Command::LabelLength { length, .. } => self.label_length = Some(length),
            Command::PrintOrientation { inverted } => self.inverted = inverted,
            Command::PrintMirror { enabled } => self.mirrored = enabled,
            Command::DotsPerMm { half } => self.half_density = half,
            Command::FormatConversion { base, desired } => self.conversion = (base, desired),
            Command::LabelReverse { enabled } => self.label_reverse = enabled,
            Command::MediaDarkness { value } => self.media_darkness = value,
            Command::CharacterEncoding { encoding } => self.character_encoding = encoding,

            Command::BarcodeFieldDefaults { module_width, wide_bar_ratio, height } => {
                self.barcode_module_width = module_width;
                self.barcode_wide_bar_ratio = wide_bar_ratio;
                self.barcode_height = height;
            }

            Command::FieldBlock { width, max_lines, line_spacing, justification, hanging_indent } => {
                self.field.block = Some(TextBlock {
                    width,
                    max_lines: max_lines.max(1),
                    line_spacing,
                    justification,
                    hanging_indent,
                    clip_height: None,
                });
            }
            Command::TextBlock { orientation, width, height } => {
                self.field.orientation = orientation;
                self.field.block = Some(TextBlock {
                    width,
                    max_lines: u32::MAX,
                    line_spacing: 0,
                    justification: BlockJustification::Left,
                    hanging_indent: 0,
                    clip_height: Some(height),
                });
            }
            Command::FieldParameter { direction, char_gap } => {
                self.field.direction = direction;
                self.field.char_gap = char_gap;
            }
            Command::FieldHexIndicator { indicator } => self.field.hex_indicator = Some(indicator),
            Command::FieldReverse => self.field.reverse = true,
            Command::FieldNumber { number } => self.field.number = Some(number),

            Command::FielData { data } => {
                let mut data = match self.field.hex_indicator {
                    Some(ind) => decode_hex_escapes(&data, ind, self.character_encoding),
                    None => data,
                };
                if let Some(ind) = self.field.embed {
                    data = embed_fields(&data, ind, |n| {
                        self.format_values.get(&n).or_else(|| self.replay_values.as_ref()?.get(&n)).cloned()
                    });
                }
                if let Some((primary, secondary, tertiary)) = self.field.clock {
                    let now = self.clock.unwrap_or_else(clock::DateTime::now);
                    let mut clocks = vec![(primary, now)];
                    clocks.extend(secondary.map(|c| (c, now.offset(&self.clock_offsets[0]))));
                    clocks.extend(tertiary.map(|c| (c, now.offset(&self.clock_offsets[1]))));
                    data = clock::expand(&data, &clocks);
                }
                self.field.data = Some(data);
            }
            Command::FieldClock { primary, secondary, tertiary } => {
                self.field.clock = Some((primary, secondary, tertiary));
            }
            Command::SetClock { month, day, year, hour, minute, second } => {
                self.clock = Some(clock::DateTime {
                    year: year as i64,
                    month: month.clamp(1, 12),
                    day: day.clamp(1, 31),
                    hour: hour.min(23),
                    minute: minute.min(59),
                    second: second.min(59),
                });
            }
            Command::ClockOffset { clock, months, days, years, hours, minutes, seconds } => {
                self.clock_offsets[usize::from(clock == 3)] =
                    clock::Offset { months, days, years, hours, minutes, seconds };
            }
            Command::AdvancedText { layout } => self.text_layout = layout,
            Command::FieldEmbed { indicator } => self.field.embed = Some(indicator),
            Command::ImageSave { name, print } => self.save_image = Some((stored_name(&name), print)),
            Command::ImageLoad { name } => {
                if let Some(image) = self.images.get(&stored_name(&name)) {
                    out.extend(image.iter().cloned());
                }
            }
            Command::DownloadFont { name, data } => {
                self.fonts.insert(stored_name(&name), Arc::new(data));
            }
            Command::FontAlias { letter, name } => {
                self.font_aliases.insert(letter, stored_name(&name));
            }
            Command::FontByName { orientation, height, width, name } => {
                self.current_font = FontName::Custom('@');
                self.current_custom_font = self.fonts.get(&stored_name(&name)).cloned();
                self.current_font_orientation = orientation.unwrap_or(self.default_orientation);
                self.set_font_size(height, width);
            }
            Command::MaxiCode { mode, symbol_number, total_symbols } => {
                self.set_symbol(Some(Orientation::Normal), SymbolSpec::MaxiCode { mode, symbol_number, total_symbols });
            }
            Command::DataBar { orientation, kind, magnification, separator_height, height, segments_per_row } => {
                let spec = SymbolSpec::DataBar {
                    kind,
                    module_width: magnification.unwrap_or(2),
                    separator_height,
                    height: height.unwrap_or(25),
                    segments_per_row,
                };
                self.set_symbol(orientation, spec);
            }
            Command::MicroPdf417 { orientation, height, mode } => {
                let spec = SymbolSpec::MicroPdf417 {
                    module_width: self.barcode_module_width,
                    row_height: height.unwrap_or(self.barcode_height),
                    mode,
                };
                self.set_symbol(orientation, spec);
            }
            Command::Tlc39 {
                orientation,
                code39_module_width,
                code39_ratio,
                code39_height,
                micro_row_height,
                micro_module_width,
            } => {
                let spec = SymbolSpec::Tlc39 {
                    code39_module_width,
                    code39_ratio,
                    code39_height,
                    micro_module_width,
                    micro_row_height,
                };
                self.set_symbol(orientation, spec);
            }
            Command::Codablock { orientation, row_height, security, columns, rows, mode } => {
                let module_width = self.barcode_module_width;
                let spec = SymbolSpec::Codablock {
                    module_width,
                    row_height: row_height * module_width,
                    security,
                    columns,
                    rows,
                    mode,
                };
                self.set_symbol(orientation, spec);
            }
            Command::Code49 { orientation, row_height, interpretation, start_mode } => {
                let module_width = self.barcode_module_width;
                let spec = SymbolSpec::Code49 {
                    module_width,
                    row_height: row_height.unwrap_or(self.barcode_height) * module_width,
                    interpretation,
                    start_mode,
                };
                self.set_symbol(orientation, spec);
            }
            Command::Postal { kind, orientation, height, print_interpretation_line, print_above } => {
                let spec = SymbolSpec::Postal {
                    kind,
                    module_width: self.barcode_module_width,
                    height: height.unwrap_or(self.barcode_height),
                    print_interpretation_line,
                    print_above,
                };
                self.set_symbol(orientation, spec);
            }
            Command::SerializationData { start, increment, leading_zeros } => {
                self.field.data = Some(start);
                self.field.serial = Some(Serial::Number { increment, leading_zeros });
            }
            Command::SerializationField { mask, increment } => {
                self.field.serial = Some(Serial::Mask { mask, increment });
            }

            Command::FieldSeperator => {
                let field = std::mem::take(&mut self.field);
                self.finish_field(field, out);
            }

            Command::GraphicSymbol { orientation, height, width } => {
                let height = height.unwrap_or(self.current_font_height);
                self.field.mode = FieldMode::GraphicSymbol {
                    orientation: orientation.unwrap_or(self.default_orientation),
                    height,
                    width: width.unwrap_or(height),
                };
            }

            Command::Code128 { orientation, height, print_interpetation_line, print_above, check_digit, mode } => {
                self.field.mode = FieldMode::Code128 {
                    orientation: orientation.unwrap_or(self.default_orientation),
                    height: height.unwrap_or(self.barcode_height),
                    print_interpretation_line: print_interpetation_line,
                    print_interpretation_line_above: print_above,
                    check_digit,
                    mode,
                };
            }

            Command::Code39 { orientation, check_digit, height, print_interpetation_line, print_above } => {
                self.field.mode = FieldMode::Linear {
                    kind: LinearKind::Code39 { check_digit },
                    orientation,
                    height: height.unwrap_or(self.barcode_height),
                    print_interpretation_line: print_interpetation_line,
                    print_interpretation_line_above: print_above,
                };
            }

            Command::LinearBarcode { kind, orientation, height, print_interpretation_line, print_above } => {
                self.field.mode = FieldMode::Linear {
                    kind,
                    orientation: orientation.unwrap_or(self.default_orientation),
                    height: height.unwrap_or(self.barcode_height),
                    print_interpretation_line,
                    print_interpretation_line_above: print_above,
                };
            }

            Command::QrCode { orientation, model, magnification, error_correction, mask } => {
                self.field.mode = FieldMode::QrCode { orientation, model, magnification, error_correction, mask };
            }

            Command::DataMatrix { orientation, module_height, columns, rows, escape, rectangular } => {
                self.field.mode = FieldMode::DataMatrix {
                    orientation: orientation.unwrap_or(self.default_orientation),
                    module_height,
                    columns,
                    rows,
                    escape,
                    rectangular,
                };
            }

            Command::Pdf417 { orientation, row_height, security, columns, rows, truncated } => {
                self.field.mode = FieldMode::Pdf417 {
                    orientation: orientation.unwrap_or(self.default_orientation),
                    row_height,
                    security,
                    columns,
                    rows,
                    truncated,
                };
            }

            Command::Aztec { orientation, magnification, ecc } => {
                self.field.mode = FieldMode::Aztec {
                    orientation: orientation.unwrap_or(self.default_orientation),
                    // Zebra's default at 8 dots/mm.
                    magnification: magnification.unwrap_or(2).max(1),
                    ecc,
                };
            }

            Command::GraphicBox { width, height, thickness, line_color, rounding } => {
                let (x, y) = self.graphic_origin(height.max(thickness));
                self.emit(out, DrawInstruction::DrawBox { x, y, width, height, thickness, color: line_color, rounding });
            }

            Command::GraphicCircle { diameter, thickness, color } => {
                let (x, y) = self.graphic_origin(diameter);
                self.emit(out, DrawInstruction::DrawCircle { x, y, diameter, thickness, color });
            }

            Command::GraphicEllipse { width, height, thickness, color } => {
                let (x, y) = self.graphic_origin(height);
                self.emit(out, DrawInstruction::DrawEllipse { x, y, width, height, thickness, color });
            }

            Command::GraphicDiagonalLine { width, height, thickness, color, direction } => {
                let (x, y) = self.graphic_origin(height);
                self.emit(out, DrawInstruction::DrawDiagonalLine { x, y, width, height, thickness, color, direction });
            }

            Command::GraphicField { bytes_per_row, data, .. } => {
                self.emit_graphic(out, bytes_per_row, data, 1, 1);
            }

            Command::DownloadGraphic { name, bytes_per_row, data } => {
                self.graphics.insert(stored_name(&name), (bytes_per_row, data));
            }

            Command::RecallGraphic { name, magnification_x, magnification_y } => {
                if let Some((bytes_per_row, data)) = self.graphics.get(&stored_name(&name)).cloned() {
                    self.emit_graphic(out, bytes_per_row, data, magnification_x, magnification_y);
                }
            }

            Command::DownloadFormat { name } => self.recording = Some((stored_name(&name), Vec::new())),
            Command::RecallFormat { name } => self.recall = Some(stored_name(&name)),

            Command::PrintQuantity { total, replicate_count, .. } => {
                self.quantity = total.max(1);
                self.replicates = replicate_count.unwrap_or(1).max(1);
            }
            Command::Comment { .. } => {}
            Command::Unknown { .. } => {}
        }
    }

    fn set_symbol(&mut self, orientation: Option<Orientation>, spec: SymbolSpec) {
        self.field.mode = FieldMode::Symbol { orientation: orientation.unwrap_or(self.default_orientation), spec };
    }

    /// The downloaded font a ^CW alias maps `font` to, if any.
    fn aliased_font(&self, font: FontName) -> Option<Arc<Vec<u8>>> {
        let letter = match font {
            FontName::Custom(c) => c,
            other => format!("{other:?}").chars().last()?,
        };
        self.fonts.get(self.font_aliases.get(&letter.to_ascii_uppercase())?).cloned()
    }

    fn emit_graphic(&self, out: &mut Vec<DrawInstruction>, bytes_per_row: u32, data: Vec<u8>, mx: u32, my: u32) {
        if bytes_per_row == 0 {
            return;
        }
        let (mx, my) = (mx.max(1), my.max(1));
        let rows = data.len().div_ceil(bytes_per_row as usize) as u32;
        let (x, y) = self.graphic_origin(rows * my);
        self.emit(
            out,
            DrawInstruction::DrawGraphicField { x, y, bytes_per_row, data, magnification_x: mx, magnification_y: my },
        );
    }

    fn finish_field(&mut self, field: Field, out: &mut Vec<DrawInstruction>) {
        // While a format recalls a stored one, its ^FN fields only supply data.
        if let (Some(number), Some(_)) = (field.number, &self.recall) {
            if let Some(data) = field.data {
                self.field_values.insert(number, data);
            }
            return;
        }
        let replayed = field.number.and_then(|n| self.replay_values.as_ref()?.get(&n).cloned());
        let Some(data) = replayed.or(field.data) else { return };
        if let Some(n) = field.number {
            self.format_values.insert(n, data.clone());
        }

        if let (Some(serial), Some(log)) = (field.serial, &mut self.page_log) {
            // ^GS data picks symbols rather than printing; it isn't serialized.
            if !matches!(field.mode, FieldMode::GraphicSymbol { .. }) {
                log.serial(out.len(), data.clone(), serial);
            }
        }

        let (x, y) = self.origin();
        let bottom = self.field_typeset;
        // Reverse is per field; graphics already emitted use the live `self.field`.
        let saved_reverse = std::mem::replace(&mut self.field.reverse, field.reverse);

        let instruction = match field.mode {
            FieldMode::Text => DrawInstruction::DrawText {
                x,
                y,
                text: data,
                font: self.current_font,
                height: self.current_font_height,
                width: self.current_font_width,
                orientation: field.orientation.unwrap_or(self.current_font_orientation),
                justification: self.field_justification,
                anchor: if bottom { TextAnchor::Baseline } else { TextAnchor::Top },
                block: field.block,
                direction: field.direction,
                char_gap: field.char_gap,
                custom_font: self.current_custom_font.clone(),
                layout: self.text_layout,
            },
            FieldMode::GraphicSymbol { orientation, height, width } => DrawInstruction::DrawText {
                x,
                y,
                text: data.chars().filter_map(graphic_symbol).collect(),
                // The symbols are scalable outlines, not part of a bitmap font.
                font: FontName::Font0,
                height,
                width,
                orientation,
                justification: self.field_justification,
                anchor: if bottom { TextAnchor::Baseline } else { TextAnchor::Top },
                block: None,
                direction: FieldDirection::Horizontal,
                char_gap: 0,
                custom_font: None,
                layout: self.text_layout,
            },
            FieldMode::Code128 {
                orientation,
                height,
                print_interpretation_line,
                print_interpretation_line_above,
                check_digit,
                mode,
            } => {
                let y = if bottom && orientation == Orientation::Normal { y.saturating_sub(height) } else { y };
                DrawInstruction::DrawBarcode {
                    x,
                    y,
                    data,
                    orientation,
                    height,
                    module_width: self.barcode_module_width,
                    wide_bar_ratio: self.barcode_wide_bar_ratio,
                    print_interpretation_line,
                    print_interpretation_line_above,
                    check_digit,
                    mode,
                }
            }
            FieldMode::Linear { kind, orientation, height, print_interpretation_line, print_interpretation_line_above } => {
                let y = if bottom && orientation == Orientation::Normal { y.saturating_sub(height) } else { y };
                DrawInstruction::DrawLinear {
                    x,
                    y,
                    kind,
                    data,
                    orientation,
                    height,
                    module_width: self.barcode_module_width,
                    wide_bar_ratio: self.barcode_wide_bar_ratio,
                    print_interpretation_line,
                    print_interpretation_line_above,
                }
            }
            FieldMode::QrCode { orientation, model, magnification, error_correction, mask } => {
                DrawInstruction::DrawQrCode {
                    x,
                    y,
                    data,
                    orientation,
                    model,
                    magnification,
                    error_correction,
                    mask,
                    bottom,
                }
            }
            FieldMode::DataMatrix { orientation, module_height, columns, rows, escape, rectangular } => {
                DrawInstruction::DrawDataMatrix {
                    x,
                    y,
                    data,
                    orientation,
                    module_height,
                    symbol_height: self.barcode_height,
                    columns,
                    rows,
                    escape,
                    rectangular,
                    bottom,
                }
            }
            FieldMode::Pdf417 { orientation, row_height, security, columns, rows, truncated } => {
                DrawInstruction::DrawPdf417 {
                    x,
                    y,
                    data,
                    orientation,
                    row_height,
                    module_width: self.barcode_module_width,
                    symbol_height: self.barcode_height,
                    security,
                    columns,
                    rows,
                    truncated,
                    bottom,
                }
            }
            FieldMode::Aztec { orientation, magnification, ecc } => {
                DrawInstruction::DrawAztec { x, y, data, orientation, magnification, ecc, bottom }
            }
            FieldMode::Symbol { orientation, spec } => DrawInstruction::DrawSymbol { x, y, data, orientation, bottom, spec },
        };

        self.emit(out, instruction);
        self.field.reverse = saved_reverse;
    }

    fn end_format(&mut self, out: &mut Vec<DrawInstruction>) {
        if let Some((name, commands)) = self.recording.take() {
            self.formats.insert(name, commands);
            return;
        }
        if let Some(name) = self.recall.take() {
            if let Some(commands) = self.formats.get(&name).cloned() {
                self.replay_values = Some(std::mem::take(&mut self.field_values));
                self.field = Field::default();
                for command in commands {
                    self.process_command(command, out);
                }
                self.replay_values = None;
            }
        }
        if let Some((name, print)) = self.save_image.take() {
            let start = self.format_start.min(out.len());
            let image = if print { out[start..].to_vec() } else { out.split_off(start) };
            self.images.insert(name, image);
        }
    }

    fn reset_format_state(&mut self) {
        self.field_origin_x = 0;
        self.field_origin_y = 0;
        self.field_typeset = false;
        self.field_justification = self.default_justification;
        self.field = Field::default();
        self.recall = None;
        self.field_values.clear();
        self.format_values.clear();
        self.save_image = None;
        self.label_reverse = false;
        self.quantity = 1;
        self.replicates = 1;
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zpl_parser::parser::Parser;

    fn run(zpl: &str) -> Vec<DrawInstruction> {
        StateManager::new().process(Parser::new(zpl).parse().unwrap())
    }

    fn texts(ins: &[DrawInstruction]) -> Vec<String> {
        ins.iter()
            .filter_map(|i| match i {
                DrawInstruction::DrawText { text, .. } => Some(text.clone()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn hex_escapes() {
        assert_eq!(decode_hex_escapes("a_41_7e_", '_', Encoding::Usa1), "aA~_");
        assert_eq!(decode_hex_escapes("_C3_A9", '_', Encoding::Utf8), "é");
        assert_eq!(decode_hex_escapes("_E9", '_', Encoding::Cp1252), "é");
        assert_eq!(decode_hex_escapes("_82", '_', Encoding::Usa1), "é");
        assert_eq!(decode_hex_escapes("#41", '#', Encoding::Usa1), "A");
        let ins = run("^XA^FO1,1^FH^FDx_41^FS^FO1,1^FDx_41^FS^XZ");
        assert_eq!(texts(&run("^XA^CI28^FO1,1^FH^FD_C3_A9^FS^CI27^FO1,1^FH^FD_E9^FS^XZ")), ["é", "é"]);
        assert_eq!(texts(&ins), ["xA", "x_41"]);
    }

    #[test]
    fn field_block_is_per_field() {
        let ins = run("^XA^FO1,1^FB100,2^FDa^FS^FO1,1^FDb^FS^XZ");
        let blocks: Vec<_> = ins
            .iter()
            .map(|i| match i {
                DrawInstruction::DrawText { block, .. } => block.is_some(),
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(blocks, [true, false]);
    }

    #[test]
    fn typeset_anchors() {
        let ins = run("^XA^FT10,100^A0N,20^FDa^FS^FT10,100^GB50,30,2^FS^XZ");
        assert!(matches!(ins[0], DrawInstruction::DrawText { y: 100, anchor: TextAnchor::Baseline, .. }));
        assert!(matches!(ins[1], DrawInstruction::DrawBox { y: 70, .. }));
    }

    #[test]
    fn reverse_wraps_any_field() {
        let ins = run("^XA^FO1,1^FR^FDa^FS^FO1,1^FDb^FS^LRY^FO1,1^GB5,5,5^FS^XZ");
        assert!(matches!(ins[0], DrawInstruction::Reverse(_)));
        assert!(matches!(ins[1], DrawInstruction::DrawText { .. }));
        assert!(matches!(ins[2], DrawInstruction::Reverse(_)));
    }

    #[test]
    fn label_shift_and_top() {
        let ins = run("^XA^LS20^LT5^LH10,10^FO50,50^GB1,1,1^FS^XZ");
        assert!(matches!(ins[0], DrawInstruction::DrawBox { x: 40, y: 65, .. }));
    }

    #[test]
    fn stored_format_is_filled_in() {
        let zpl = "^XA^DFR:T.ZPL^FS^FO10,10^FN1^FDdefault^FS^FO10,50^FDstatic^FS^XZ\
                   ^XA^XFR:T.ZPL^FS^FN1^FDhello^FS^XZ";
        assert_eq!(texts(&run(zpl)), ["hello", "static"]);
    }

    #[test]
    fn stored_graphic_is_recalled() {
        let ins = run("~DGR:LOGO.GRF,2,1,FF00^XA^FO5,5^XGR:LOGO.GRF,2,3^FS^XZ");
        assert_eq!(
            ins,
            [DrawInstruction::DrawGraphicField {
                x: 5,
                y: 5,
                bytes_per_row: 1,
                data: vec![0xFF, 0x00],
                magnification_x: 2,
                magnification_y: 3
            }]
        );
    }

    #[test]
    fn field_orientation_default() {
        let ins = run("^XA^FWR^FO1,1^A0,20^FDa^FS^XZ");
        assert!(matches!(ins[0], DrawInstruction::DrawText { orientation: Orientation::Rotated90, .. }));
    }

    #[test]
    fn field_embedding() {
        let zpl = "^XA^FN2^FDField FN 2 Data^FS^FE#^FD#2# and #2,f,1,5#|#2,b,4,4#|#9#^FS^XZ";
        assert_eq!(texts(&run(zpl))[1], "Field FN 2 Data and Field|Data|");
        assert_eq!(embed_fields("50# off", '#', |_| None), "50# off");
    }

    #[test]
    fn clock_fields() {
        let zpl = "^XA^ST10,13,2026,14,5,9^SO2,0,1^FO1,1^FC%,{^FD%Y-%m-%d {d %%^FS^XZ";
        assert_eq!(texts(&run(zpl)), ["2026-10-13 14 %%"]);
    }

    #[test]
    fn saved_images_and_fonts() {
        let ins = run("^XA^FO1,1^FDbg^FS^ISR:BG.GRF,N^XZ^XA^ILR:BG.GRF^FO1,1^FDfg^FS^XZ");
        assert_eq!(texts(&ins), ["bg", "fg"]);
        let ins = run("~DUR:MYFONT.TTF,2,ABCD^XA^CWQ,R:MYFONT.TTF^FO1,1^AQN,20^FDx^FS^A@N,20,20,R:MYFONT.TTF^FDy^FS^A0N,20^FDz^FS^XZ");
        let fonts: Vec<_> = ins
            .iter()
            .map(|i| match i {
                DrawInstruction::DrawText { custom_font, .. } => custom_font.as_deref().cloned(),
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(fonts, [Some(vec![0xAB, 0xCD]), Some(vec![0xAB, 0xCD]), None]);
    }

    #[test]
    fn new_symbols() {
        let ins = run("^XA^BY3^FO10,10^BD^FDdata^FS^FO1,1^BRN,6,4,1,50,10^FD(01)123^FS^FO1,1^BB,4,N,10,2^FDabc^FS^XZ");
        assert!(matches!(ins[0], DrawInstruction::DrawSymbol { spec: SymbolSpec::MaxiCode { mode: 2, .. }, .. }));
        assert!(matches!(
            ins[1],
            DrawInstruction::DrawSymbol { spec: SymbolSpec::DataBar { kind: 6, module_width: 4, height: 50, segments_per_row: 10, .. }, .. }
        ));
        assert!(matches!(ins[2], DrawInstruction::DrawSymbol { spec: SymbolSpec::Codablock { row_height: 12, mode: 'F', .. }, .. }));
    }

    #[test]
    fn units_of_measure() {
        let ins = run("^XA^MUM^FO10,2.5^GB25,25,1^FS^MUD^FO10,10^GB5,5,1^FS^XZ");
        assert!(matches!(ins[0], DrawInstruction::DrawBox { x: 80, y: 20, width: 200, .. }));
        assert!(matches!(ins[1], DrawInstruction::DrawBox { x: 10, y: 10, width: 5, .. }));
    }

    fn pages_at(zpl: &str, dpmm: u32) -> Vec<Page> {
        let mut state = StateManager::new();
        state.set_dpmm(dpmm);
        state.process_pages(Parser::with_dpmm(zpl, dpmm).parse().unwrap())
    }

    #[test]
    fn default_label_is_4_by_6_inches_at_the_printer_resolution() {
        let size = |zpl: &str, dpmm| {
            let setup = pages_at(zpl, dpmm)[0].setup;
            (setup.width, setup.height)
        };
        assert_eq!(size("^XA^FO1,1^FDa^FS^XZ", 6), (608, 912));
        assert_eq!(size("^XA^FO1,1^FDa^FS^XZ", 8), (812, 1218));
        assert_eq!(size("^XA^FO1,1^FDa^FS^XZ", 12), (1200, 1800));
        assert_eq!(size("^XA^FO1,1^FDa^FS^XZ", 24), (2400, 3600));
        assert_eq!(pages("^XA^XZ")[0].setup.dpmm, 8);
        // ^PW / ^LL are dots whatever the resolution.
        assert_eq!(size("^XA^PW300^LL200^XZ", 12), (300, 200));
        // ^MU millimetres follow it too.
        assert_eq!(size("^XA^MUM^PW50^LL25^XZ", 12), (600, 300));
    }

    #[test]
    fn half_density_doubles_the_format() {
        let page = &pages_at("^XA^JMB^FO10,10^GB20,20,2^FS^XZ", 8)[0];
        // Instructions stay in format dots; the page scale maps them to printer dots, and
        // the default label shrinks so that it is still 4 x 6 inches.
        assert!(matches!(page.instructions[0], DrawInstruction::DrawBox { x: 10, y: 10, width: 20, .. }));
        assert_eq!(page.setup.scale, (2, 1));
        assert_eq!(page.setup.dot_scale(), 2.0);
        assert_eq!((page.setup.width, page.setup.height), (406, 609));
        assert_eq!(page.setup.format_dpi(), 101.5);
        // ^JM persists into later formats until ^JMA.
        let pages = pages_at("^XA^JMB^FO1,1^FDa^FS^XZ^XA^FO1,1^FDb^FS^XZ^XA^JMA^FO1,1^FDc^FS^XZ", 12);
        let scales: Vec<_> = pages.iter().map(|p| p.setup.scale).collect();
        assert_eq!(scales, [(2, 1), (2, 1), (1, 1)]);
    }

    #[test]
    fn format_conversion_scales_the_page() {
        let setup = pages_at("^XA^MUd,200,600^FO1,1^FDa^FS^XZ", 24)[0].setup;
        assert_eq!((setup.scale, setup.dot_scale()), ((600, 200), 3.0));
        assert_eq!((setup.width, setup.height), (800, 1200));
        // Matching values turn it off again.
        assert_eq!(pages_at("^XA^MUd,200,600^MUd,300,300^XZ", 24)[0].setup.scale, (300, 300));
    }

    #[test]
    fn line_breaks_in_field_data_are_ignored() {
        let ins = run("^XA^FO1,1^FDVAN:\n^FS^FO1,1^FDab\r\ncd^FS^FO1,1^FVx\r\n^FS^FO1,1^FDplain^FS^XZ");
        assert_eq!(texts(&ins), ["VAN:", "abcd", "x", "plain"]);
    }

    #[test]
    fn bad_parameters_skip_the_command() {
        let ins = run("^XA^FOx,1^GB5,5,1^FS^CI99^FO1,1^FDok^FS^XZ");
        assert_eq!(texts(&ins), ["ok"]);
    }

    fn pages(zpl: &str) -> Vec<Page> {
        StateManager::new().process_pages(Parser::new(zpl).parse().unwrap())
    }

    fn page_texts(pages: &[Page]) -> Vec<Vec<String>> {
        pages.iter().map(|p| texts(&p.instructions)).collect()
    }

    #[test]
    fn one_page_per_label() {
        let zpl = "^XA^PW400^FO1,1^FDa^FS^XZ^XA^FO1,1^FDb^FS^FO1,1^FDc^FS^XZ^XA^LL300^FO1,1^FDd^FS^XZ";
        let pages = pages(zpl);
        assert_eq!(page_texts(&pages), [vec!["a"], vec!["b", "c"], vec!["d"]]);
        // Page settings carry over to later labels.
        let sizes: Vec<_> = pages.iter().map(|p| (p.setup.width, p.setup.height)).collect();
        assert_eq!(sizes, [(400, 1218), (400, 1218), (400, 300)]);
        // The flat output is unchanged.
        assert_eq!(texts(&run(zpl)), ["a", "b", "c", "d"]);
    }

    #[test]
    fn formats_that_draw_nothing_are_skipped() {
        let zpl = "~DGR:LOGO.GRF,2,1,FF00^XA^DFR:T.ZPL^FS^FO1,1^FN1^FS^XZ^XA^XZ\
                   ^XA^FO1,1^FDbg^FS^ISR:BG.GRF,N^XZ^XA^XFR:T.ZPL^FN1^FDhi^FS^XZ^FO1,1^FDafter^FS";
        assert_eq!(page_texts(&pages(zpl)), [vec!["hi"], vec!["after"]]);
        // No ^XA / ^XZ at all.
        assert_eq!(page_texts(&pages("^FO1,1^FDloose^FS")), [vec!["loose"]]);
    }

    #[test]
    fn blank_input_gives_one_blank_page() {
        for zpl in ["", "^XA^XZ", "^XA^PW100^LL50^DFR:T.ZPL^FS^XZ"] {
            let pages = pages(zpl);
            assert_eq!(pages.len(), 1, "{zpl}");
            assert!(pages[0].instructions.is_empty());
        }
        assert_eq!(pages("^XA^PW100^LL50^XZ")[0].setup.width, 100);
    }

    #[test]
    fn print_quantity_copies() {
        let pages = pages("^XA^PQ3^FO1,1^FDa^FS^XZ^XA^FO1,1^FDb^FS^XZ");
        assert_eq!(page_texts(&pages), [vec!["a"], vec!["a"], vec!["a"], vec!["b"]]);
    }

    #[test]
    fn serialized_copies() {
        let zpl = "^XA^PQ4^FO1,1^SNA008,1,Y^FS^FO1,1^FDstatic^FS^FO1,1^FR^BCN,50^SN10,-5^FS^XZ";
        let pages = pages(zpl);
        let data = |p: &Page| -> Vec<String> {
            p.instructions.clone().iter_mut().map(|i| i.field_data_mut().unwrap().clone()).collect()
        };
        let all: Vec<_> = pages.iter().map(data).collect();
        assert_eq!(
            all,
            // 10 - 15 wraps around to 95.
            [["A008", "static", "10"], ["A009", "static", "5"], ["A010", "static", "0"], ["A011", "static", "95"]]
        );
        assert!(matches!(&pages[1].instructions[2], DrawInstruction::Reverse(inner) if matches!(**inner, DrawInstruction::DrawBarcode { .. })));
    }

    #[test]
    fn serial_replicates() {
        let pages = pages("^XA^PQ5,0,2^FO1,1^FDLOT-1^SFD^FS^XZ");
        assert_eq!(page_texts(&pages), [["LOT-1"], ["LOT-1"], ["LOT-2"], ["LOT-2"], ["LOT-3"]]);
        // ^PQ resets at the next format.
        assert_eq!(page_texts(&self::pages("^XA^PQ2^FO1,1^SN1^FS^XZ^XA^FO1,1^SN1^FS^XZ")), [["1"], ["2"], ["1"]]);
    }

    #[test]
    fn page_count_is_capped() {
        assert_eq!(pages("^XA^PQ99999999^FO1,1^FDa^FS^XZ").len(), MAX_PAGES);
        let mut state = StateManager::new();
        state.set_max_pages(3);
        let pages = state.process_pages(Parser::new("^XA^PQ2^FO1,1^FDa^FS^XZ^XA^PQ5^FO1,1^FDb^FS^XZ").parse().unwrap());
        assert_eq!(page_texts(&pages), [["a"], ["a"], ["b"]]);
    }

    #[test]
    fn graphic_symbol() {
        let ins = run("^XA^FO1,1^GSN,30,30^FDAC^FS^XZ");
        assert_eq!(texts(&ins), ["\u{00AE}\u{2122}"]);
    }
}

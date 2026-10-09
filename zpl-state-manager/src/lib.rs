use std::collections::HashMap;

use zpl_parser::command::{
    BlockJustification, Code128Mode, Command, DiagonalDirection, Encoding, FieldDirection, FontName,
    Justification, LineColor, LinearKind, Orientation, QrErrorCorrection, QrModel,
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
    /// ^FR / ^LR: the inner element inverts whatever is underneath instead of painting black.
    Reverse(Box<DrawInstruction>),
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

/// Whole-label settings that apply to the page rather than to a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageSetup {
    pub width: u32,
    pub height: u32,
    /// ^POI: label printed rotated 180 degrees.
    pub inverted: bool,
    /// ^PMY: label printed mirrored horizontally.
    pub mirrored: bool,
}

#[derive(Debug, Clone)]
enum FieldMode {
    Text,
    Symbol {
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

    print_width: Option<u32>,
    label_length: Option<u32>,
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

/// Decodes ^FH escapes (`<indicator>` + two hex digits). Escaped bytes are UTF-8 under
/// ^CI28 and single-byte (Latin-1) otherwise.
fn decode_hex_escapes(data: &str, indicator: char, utf8: bool) -> String {
    let hex = |b: u8| (b as char).to_digit(16);
    let bytes = data.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    let mut ind = [0; 4];
    let ind = indicator.encode_utf8(&mut ind).as_bytes();
    while i < bytes.len() {
        if bytes[i..].starts_with(ind) {
            let at = i + ind.len();
            if let (Some(&h), Some(&l)) = (bytes.get(at), bytes.get(at + 1)) {
                if let (Some(h), Some(l)) = (hex(h), hex(l)) {
                    let byte = (h * 16 + l) as u8;
                    if utf8 || byte < 0x80 {
                        out.push(byte);
                    } else {
                        let mut buf = [0; 4];
                        out.extend_from_slice((byte as char).encode_utf8(&mut buf).as_bytes());
                    }
                    i = at + 2;
                    continue;
                }
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8(out).unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned())
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

            print_width: None,
            label_length: None,
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
        }
    }

    pub fn process(&mut self, commands: Vec<Command>) -> Vec<DrawInstruction> {
        let mut instructions = Vec::new();

        for command in commands {
            self.process_command(command, &mut instructions);
        }

        instructions
    }

    pub fn get_label_dimensions(&self) -> (u32, u32) {
        let width = self.print_width.unwrap_or(812);
        let height = self.label_length.unwrap_or(1218);
        (width, height)
    }

    pub fn page_setup(&self) -> PageSetup {
        let (width, height) = self.get_label_dimensions();
        PageSetup { width, height, inverted: self.inverted, mirrored: self.mirrored }
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

    fn set_font_size(&mut self, height: Option<u32>, width: Option<u32>) {
        // Scalable fonts keep their aspect ratio when only one dimension is given
        if let Some(h) = height {
            self.current_font_height = h;
        }
        if let Some(w) = width.or(height) {
            self.current_font_width = w;
        }
        if width.is_some() && height.is_none() {
            self.current_font_height = self.current_font_width;
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
            Command::StartFormat => self.reset_format_state(),
            Command::EndFormat => self.end_format(out),

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
                self.current_font_orientation = orientation.unwrap_or(self.default_orientation);
                self.set_font_size(height, width);
            }

            Command::ChangeFont { font, height, width } => {
                self.current_font = font;
                self.set_font_size(height, width);
            }

            Command::PrintWidth { width } => self.print_width = Some(width),
            Command::LabelLength { length, .. } => self.label_length = Some(length),
            Command::PrintOrientation { inverted } => self.inverted = inverted,
            Command::PrintMirror { enabled } => self.mirrored = enabled,
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
                let data = match self.field.hex_indicator {
                    Some(ind) => decode_hex_escapes(&data, ind, self.character_encoding == Encoding::Utf8),
                    None => data,
                };
                self.field.data = Some(data);
            }
            Command::SerializationData { start } => self.field.data = Some(start),

            Command::FieldSeperator => {
                let field = std::mem::take(&mut self.field);
                self.finish_field(field, out);
            }

            Command::GraphicSymbol { orientation, height, width } => {
                let height = height.unwrap_or(self.current_font_height);
                self.field.mode = FieldMode::Symbol {
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

            Command::PrintQuality { .. } => {}
            Command::Comment { .. } => {}
            Command::Unknown { .. } => {}
        }
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
            },
            FieldMode::Symbol { orientation, height, width } => DrawInstruction::DrawText {
                x,
                y,
                text: data.chars().filter_map(graphic_symbol).collect(),
                font: self.current_font,
                height,
                width,
                orientation,
                justification: self.field_justification,
                anchor: if bottom { TextAnchor::Baseline } else { TextAnchor::Top },
                block: None,
                direction: FieldDirection::Horizontal,
                char_gap: 0,
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
        };

        self.emit(out, instruction);
        self.field.reverse = saved_reverse;
    }

    fn end_format(&mut self, out: &mut Vec<DrawInstruction>) {
        if let Some((name, commands)) = self.recording.take() {
            self.formats.insert(name, commands);
            return;
        }
        let Some(name) = self.recall.take() else { return };
        let Some(commands) = self.formats.get(&name).cloned() else { return };
        self.replay_values = Some(std::mem::take(&mut self.field_values));
        self.field = Field::default();
        for command in commands {
            self.process_command(command, out);
        }
        self.replay_values = None;
    }

    fn reset_format_state(&mut self) {
        self.field_origin_x = 0;
        self.field_origin_y = 0;
        self.field_typeset = false;
        self.field_justification = self.default_justification;
        self.field = Field::default();
        self.recall = None;
        self.field_values.clear();
        self.label_reverse = false;
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
        assert_eq!(decode_hex_escapes("a_41_7e_", '_', false), "aA~_");
        assert_eq!(decode_hex_escapes("_C3_A9", '_', true), "é");
        assert_eq!(decode_hex_escapes("_E9", '_', false), "é");
        assert_eq!(decode_hex_escapes("#41", '#', false), "A");
        let ins = run("^XA^FO1,1^FH^FDx_41^FS^FO1,1^FDx_41^FS^XZ");
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
    fn graphic_symbol() {
        let ins = run("^XA^FO1,1^GSN,30,30^FDAC^FS^XZ");
        assert_eq!(texts(&ins), ["\u{00AE}\u{2122}"]);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    StartFormat,
    EndFormat,
    FieldSeperator,

    FieldOrigin {
        x: u32,
        y: u32,
        justification: Option<Justification>,
    },

    LabelHome {
        x: u32,
        y: u32,
    },

    LabelTop {
        offset: i32,
    },

    PrintWidth {
        width: u32,
    },

    LabelLength {
        length: u32,
        apply_to_all_media: Option<bool>,
    },

    Font {
        font: FontName,
        orientation: Option<Orientation>,
        height: Option<u32>,
        width: Option<u32>,
    },

    ChangeFont {
        font: FontName,
        height: Option<u32>,
        width: Option<u32>,
    },

    FielData {
        data: String,
    },

    FieldBlock {
        width: u32,
        max_lines: u32,
        line_spacing: i32,
        justification: BlockJustification,
        hanging_indent: u32,
    },

    FieldTypeset {
        x: u32,
        y: u32,
        justification: Option<Justification>,
    },

    FieldReverse,

    BarcodeFieldDefaults {
        module_width: u32,
        wide_bar_ratio: f32,
        height: u32,
    },

    Code128 {
        orientation: Option<Orientation>,
        height: Option<u32>,
        print_interpetation_line: bool,
        print_above: bool,
        check_digit: bool,
        mode: Code128Mode,
    },

    Code39 {
        orientation: Orientation,
        height: Option<u32>,
        print_interpetation_line: bool,
        print_above: bool,
        check_digit: bool,
    },

    QrCode {
        orientation: Orientation,
        model: QrModel,
        magnification: u32,
        error_correction: QrErrorCorrection,
        mask: Option<u32>,
    },

    GraphicBox {
        width: u32,
        height: u32,
        thickness: u32,
        line_color: LineColor,
        rounding: u32,
    },

    GraphicCircle {
        diameter: u32,
        thickness: u32,
        color: LineColor,
    },

    GraphicDiagonalLine {
        width: u32,
        height: u32,
        thickness: u32,
        color: LineColor,
        direction: DiagonalDirection,
    },

    GraphicField {
        compression: CompressionType,
        binary_byte_count: u32,
        graphic_field_count: u32,
        bytes_per_row: u32,
        data: Vec<u8>,
    },

    CharacterEncoding {
        encoding: Encoding,
    },

    /// ^PQ: print `total` labels, each serial number (^SN / ^SF) `replicate_count` times.
    PrintQuantity {
        total: u32,
        pause_count: Option<u32>,
        replicate_count: Option<u32>,
        override_pause: bool,
    },

    MediaDarkness {
        value: i32,
    },

    Comment {
        text: String,
    },

    /// ^FH: field data of the current field contains `<indicator>XX` hex escapes.
    FieldHexIndicator {
        indicator: char,
    },

    /// ^FW: default orientation (and justification) for following fields.
    FieldOrientation {
        orientation: Orientation,
        justification: Option<Justification>,
    },

    /// ^FP: field direction and additional inter-character gap in dots.
    FieldParameter {
        direction: FieldDirection,
        char_gap: u32,
    },

    /// ^TB: text block, wraps within `width` and clips to `height`.
    TextBlock {
        orientation: Option<Orientation>,
        width: u32,
        height: u32,
    },

    /// ^FN: field number, used by ^DF templates and filled in by ^XF formats.
    FieldNumber {
        number: u32,
    },

    /// ^DF: the rest of this format (up to ^XZ) is stored under `name` instead of printed.
    DownloadFormat {
        name: String,
    },

    /// ^XF: recall a stored format.
    RecallFormat {
        name: String,
    },

    /// ~DG / ~DY (GRF): store a bitmap under `name`. `data` is decoded rows, MSB first.
    DownloadGraphic {
        name: String,
        bytes_per_row: u32,
        data: Vec<u8>,
    },

    /// ^XG (and ^IM, with magnification 1): draw a stored bitmap at the field origin.
    RecallGraphic {
        name: String,
        magnification_x: u32,
        magnification_y: u32,
    },

    /// ^LR: reverse-print every field.
    LabelReverse {
        enabled: bool,
    },

    /// ^PO: print orientation; `inverted` rotates the whole label 180 degrees.
    PrintOrientation {
        inverted: bool,
    },

    /// ^PM: mirror the whole label horizontally.
    PrintMirror {
        enabled: bool,
    },

    /// ^JM: `half` (B) prints at half the printhead's dots per millimetre, every format
    /// dot covering 2 x 2 printer dots.
    DotsPerMm {
        half: bool,
    },

    /// ^MUa,b,c with b and c: the format was written for `base` dpi and is scaled to
    /// `desired` dpi. Equal values turn the conversion off.
    FormatConversion {
        base: u32,
        desired: u32,
    },

    /// ^LS: shift all fields left by `shift` dots (negative shifts right).
    LabelShift {
        shift: i32,
    },

    /// ^GE: ellipse.
    GraphicEllipse {
        width: u32,
        height: u32,
        thickness: u32,
        color: LineColor,
    },

    /// ^GS: graphic symbol; the field data selects the symbol (A = (R), B = (C), C = TM, ...).
    GraphicSymbol {
        orientation: Option<Orientation>,
        height: Option<u32>,
        width: Option<u32>,
    },

    /// ^SN: serialized field data. The last run of digits in `start` changes by `increment`
    /// on each label; `leading_zeros` pads it to its original width.
    SerializationData {
        start: String,
        increment: i64,
        leading_zeros: bool,
    },

    /// ^SF: serializes the field's ^FD data. `mask` and `increment` are right-aligned to the
    /// data; mask characters are D (decimal), H (hex), O (octal), A (letters), N (0-9A-Z)
    /// and % (not incremented), upper-cased here.
    SerializationField {
        mask: String,
        increment: String,
    },

    /// 1D barcodes other than Code 128 (^B1 ^B2 ^B3 ^B8 ^B9 ^BA ^BE ^BI ^BJ ^BK ^BM ^BP ^BS ^BU).
    LinearBarcode {
        kind: LinearKind,
        orientation: Option<Orientation>,
        height: Option<u32>,
        print_interpretation_line: bool,
        print_above: bool,
    },

    /// ^BX
    DataMatrix {
        orientation: Option<Orientation>,
        module_height: Option<u32>,
        columns: u32,
        rows: u32,
        escape: Option<char>,
        rectangular: bool,
    },

    /// ^B7. `row_height` is in multiples of the module width.
    Pdf417 {
        orientation: Option<Orientation>,
        row_height: Option<u32>,
        security: Option<u8>,
        columns: u32,
        rows: u32,
        truncated: bool,
    },

    /// ^BD
    MaxiCode {
        mode: u8,
        symbol_number: u8,
        total_symbols: u8,
    },

    /// ^BR. `kind` is the ZPL symbology type 1-12.
    DataBar {
        orientation: Option<Orientation>,
        kind: u8,
        magnification: Option<u32>,
        separator_height: u32,
        height: Option<u32>,
        segments_per_row: u32,
    },

    /// ^BF
    MicroPdf417 {
        orientation: Option<Orientation>,
        height: Option<u32>,
        mode: u8,
    },

    /// ^BT
    Tlc39 {
        orientation: Option<Orientation>,
        code39_module_width: u32,
        code39_ratio: f32,
        code39_height: u32,
        micro_row_height: u32,
        micro_module_width: u32,
    },

    /// ^BB. `row_height` is in multiples of the module width.
    Codablock {
        orientation: Option<Orientation>,
        row_height: u32,
        security: bool,
        columns: u32,
        rows: u32,
        mode: char,
    },

    /// ^B4. `row_height` is in multiples of the module width.
    Code49 {
        orientation: Option<Orientation>,
        row_height: Option<u32>,
        interpretation: Code49Interpretation,
        start_mode: Option<u8>,
    },

    /// ^BZ (POSTNET, PLANET, USPS IMb) and ^B5 (PLANET).
    Postal {
        kind: PostalKind,
        orientation: Option<Orientation>,
        height: Option<u32>,
        print_interpretation_line: bool,
        print_above: bool,
    },

    /// ^FC: clock indicator characters for the field data of this field.
    FieldClock {
        primary: char,
        secondary: Option<char>,
        tertiary: Option<char>,
    },

    /// ^ST: set the real-time clock used by ^FC fields.
    SetClock {
        month: u32,
        day: u32,
        year: u32,
        hour: u32,
        minute: u32,
        second: u32,
    },

    /// ^SO: offset of the secondary (2) or tertiary (3) clock.
    ClockOffset {
        clock: u8,
        months: i32,
        days: i32,
        years: i32,
        hours: i32,
        minutes: i32,
        seconds: i32,
    },

    /// ^FE: field data may embed `^FN` values as `#n#` or `#n,f|b,x,y#`.
    FieldEmbed {
        indicator: char,
    },

    /// ^IL: draw a stored label image (saved with ^IS) as the background of this format.
    ImageLoad {
        name: String,
    },

    /// ^IS: store this format's image under `name` at ^XZ.
    ImageSave {
        name: String,
        print: bool,
    },

    /// ~DU / ~DT / ~DY (TrueType/OpenType): store a font under `name`.
    DownloadFont {
        name: String,
        data: Vec<u8>,
    },

    /// ^CW: font letter `letter` now refers to the stored font `name`.
    FontAlias {
        letter: char,
        name: String,
    },

    /// ^A@: select a stored font by name.
    FontByName {
        orientation: Option<Orientation>,
        height: Option<u32>,
        width: Option<u32>,
        name: String,
    },

    /// ^PA: advanced text layout.
    AdvancedText {
        layout: TextLayout,
    },

    /// ^BO / ^B0
    Aztec {
        orientation: Option<Orientation>,
        magnification: Option<u32>,
        ecc: u32,
    },

    Unknown {
        prefix: CommandPrefix,
        name: String,
        params: Vec<String>,
    },
}

/// ^PA settings. All off is the classic one-glyph-per-character layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TextLayout {
    /// Missing glyphs print the font's .notdef glyph (usually a box) instead of a space.
    pub default_glyph: bool,
    /// Unicode bidirectional reordering (right-to-left scripts).
    pub bidi: bool,
    /// Contextual shaping (e.g. Arabic joining forms).
    pub shaping: bool,
    /// OpenType layout features (ligatures, kerning, mark positioning).
    pub opentype: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code49Interpretation {
    None,
    Above,
    Below,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostalKind {
    Postnet,
    Planet,
    UspsImb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldDirection {
    Horizontal,
    Vertical,
    Reverse,
}

/// 1D symbologies handled by `Command::LinearBarcode`, with their symbology-specific options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinearKind {
    Code11 { check_digits: u8 },
    Interleaved2of5 { check_digit: bool },
    Code39 { check_digit: bool },
    Ean8,
    UpcE { print_check_digit: bool },
    Code93 { print_check_digit: bool },
    Ean13,
    Industrial2of5,
    Standard2of5,
    Codabar { check_digit: bool, start: char, stop: char },
    Msi { check: MsiCheck, print_check_digit: bool },
    Plessey { print_check_digit: bool },
    UpcEanExtension,
    UpcA { print_check_digit: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsiCheck {
    None,
    Mod10,
    Mod10Mod10,
    Mod11Mod10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandPrefix {
    Carot,
    Tilde,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Usa1,
    Usa2,
    Uk,
    Holland,
    DenmarkNorway,
    SwedenFinland,
    Germany,
    France1,
    France2,
    Italy,
    Spain,
    Misc,
    Japan,
    Zebra,
    DoubleByteAsian,
    ShiftJIS,
    EUCJPCN,
    UCS2BigEndian,
    SingleByteAsian,
    MultibyteAsian,
    Utf8,
    Utf16BigEndian,
    Utf16LittleEndian,
    /// Zebra Code Page 1252 (^CI27) and Windows code pages 1250-1255 (^CI31-36).
    Cp1252,
    Cp1250,
    Cp1251,
    Cp1253,
    Cp1254,
    Cp1255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    Ascii,
    Binary,
    CompressedBinary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagonalDirection {
    /// ^GD L, left-leaning: top-left to bottom-right (`\`).
    LeftToRight,
    /// ^GD R or `/`, right-leaning: top-right to bottom-left (`/`).
    RightToLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontName {
    Font0,
    Font1,
    Font2,
    Font3,
    Font4,
    Font5,
    Font6,
    Font7,
    Font8,
    Font9,
    FontA,
    FontB,
    FontC,
    FontD,
    FontE,
    FontF,
    FontG,
    FontH,
    FontI,
    FontJ,
    FontK,
    FontL,
    FontM,
    FontO,
    FontP,
    FontQ,
    FontR,
    FontS,
    FontT,
    FontU,
    FontV,
    /// The ^GS symbol font: a 24 x 24 dot matrix magnified in whole steps.
    Symbol,
    Custom(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Normal,
    Rotated90,
    Rotated180,
    Rotated270,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Justification {
    Left,
    Right,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockJustification {
    Left,
    Center,
    Right,
    Justified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code128Mode {
    AutoMode,
    UccCase,
    Auto,
    Subset(Code128Subset),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code128Subset {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QrModel {
    Model1,
    Model2,
    Micro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QrErrorCorrection {
    HighReliability,
    Standard,
    HighDensity,
    UltraHighReliability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineColor {
    Black,
    White,
}

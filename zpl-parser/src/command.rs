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

    PrintQuality {
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

    /// ^SN: serialized field; rendered with its starting value.
    SerializationData {
        start: String,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    Ascii,
    Binary,
    CompressedBinary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagonalDirection {
    LeftToRight,
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

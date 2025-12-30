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

    Unknown {
        prefix: CommandPrefix,
        name: String,
        params: Vec<String>,
    },
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

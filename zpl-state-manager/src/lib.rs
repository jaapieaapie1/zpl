use zpl_parser::command::{
    Command, Orientation, FontName, Justification, LineColor,
    CompressionType, DiagonalDirection, QrModel, QrErrorCorrection,
    Encoding, BlockJustification, Code128Mode
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
    },
    DrawBarcode {
        x: u32,
        y: u32,
        data: String,
        orientation: Orientation,
        height: u32,
        module_width: u32,
        print_interpretation_line: bool,
        print_interpretation_line_above: bool,
        check_digit: bool,
        mode: Code128Mode,
    },
    DrawCode39 {
        x: u32,
        y: u32,
        data: String,
        orientation: Orientation,
        check_digit: bool,
        height: u32,
        module_width: u32,
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
    DrawDiagonalLine {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        thickness: u32,
        color: LineColor,
        direction: DiagonalDirection,
    },
    DrawGraphicField {
        x: u32,
        y: u32,
        compression: CompressionType,
        bytes_per_row: u32,
        data: Vec<u8>,
    },
}

#[derive(Debug)]
pub struct StateManager {
    label_home_x: u32,
    label_home_y: u32,

    field_origin_x: u32,
    field_origin_y: u32,
    field_justification: Justification,

    current_font: FontName,
    current_font_orientation: Orientation,
    current_font_height: u32,
    current_font_width: u32,

    print_width: Option<u32>,
    label_length: Option<u32>,
    label_top: i32,

    media_darkness: i32,
    character_encoding: Encoding,

    barcode_module_width: u32,
    barcode_wide_bar_ratio: f32,
    barcode_height: u32,

    field_block_width: u32,
    field_block_max_lines: u32,
    field_block_line_spacing: i32,
    field_block_justification: BlockJustification,
    field_block_hanging_indent: u32,

    pending_field_data: Option<String>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            label_home_x: 0,
            label_home_y: 0,

            field_origin_x: 0,
            field_origin_y: 0,
            field_justification: Justification::Left,

            current_font: FontName::FontA,
            current_font_orientation: Orientation::Normal,
            current_font_height: 9,
            current_font_width: 5,

            print_width: None,
            label_length: None,
            label_top: 0,

            media_darkness: 0,
            character_encoding: Encoding::Usa1,

            barcode_module_width: 2,
            barcode_wide_bar_ratio: 3.0,
            barcode_height: 10,

            field_block_width: 0,
            field_block_max_lines: 1,
            field_block_line_spacing: 0,
            field_block_justification: BlockJustification::Left,
            field_block_hanging_indent: 0,

            pending_field_data: None,
        }
    }

    pub fn process(&mut self, commands: Vec<Command>) -> Vec<DrawInstruction> {
        let mut instructions = Vec::new();

        for command in commands {
            if let Some(instruction) = self.process_command(command) {
                instructions.push(instruction);
            }
        }

        instructions
    }

    pub fn get_label_dimensions(&self) -> (u32, u32) {
        let width = self.print_width.unwrap_or(812);
        let height = self.label_length.unwrap_or(1218);
        (width, height)
    }

    fn process_command(&mut self, command: Command) -> Option<DrawInstruction> {
        match command {
            Command::StartFormat => {
                self.reset_format_state();
                None
            }
            Command::EndFormat => None,

            Command::LabelHome { x, y } => {
                self.label_home_x = x;
                self.label_home_y = y;
                None
            }

            Command::FieldOrigin { x, y, justification } => {
                self.field_origin_x = x;
                self.field_origin_y = y;
                if let Some(j) = justification {
                    self.field_justification = j;
                }
                None
            }

            Command::Font { font, orientation, height, width } => {
                self.current_font = font;
                if let Some(o) = orientation {
                    self.current_font_orientation = o;
                } else {
                    self.current_font_orientation = Orientation::Normal;
                }
                if let Some(h) = height {
                    self.current_font_height = h;
                }
                if let Some(w) = width {
                    self.current_font_width = w;
                }
                None
            }

            Command::ChangeFont { font, height, width } => {
                self.current_font = font;
                if let Some(h) = height {
                    self.current_font_height = h;
                }
                if let Some(w) = width {
                    self.current_font_width = w;
                }
                None
            }

            Command::PrintWidth { width } => {
                self.print_width = Some(width);
                None
            }

            Command::LabelLength { length, .. } => {
                self.label_length = Some(length);
                None
            }

            Command::LabelTop { offset } => {
                self.label_top = offset;
                None
            }

            Command::MediaDarkness { value } => {
                self.media_darkness = value;
                None
            }

            Command::CharacterEncoding { encoding } => {
                self.character_encoding = encoding;
                None
            }

            Command::BarcodeFieldDefaults { module_width, wide_bar_ratio, height } => {
                self.barcode_module_width = module_width;
                self.barcode_wide_bar_ratio = wide_bar_ratio;
                self.barcode_height = height;
                None
            }

            Command::FieldBlock { width, max_lines, line_spacing, justification, hanging_indent } => {
                self.field_block_width = width;
                self.field_block_max_lines = max_lines;
                self.field_block_line_spacing = line_spacing;
                self.field_block_justification = justification;
                self.field_block_hanging_indent = hanging_indent;
                None
            }

            Command::FielData { data } => {
                self.pending_field_data = Some(data);
                None
            }

            Command::FieldSeperator => {
                if let Some(data) = self.pending_field_data.take() {
                    Some(DrawInstruction::DrawText {
                        x: self.label_home_x + self.field_origin_x,
                        y: self.label_home_y + self.field_origin_y,
                        text: data,
                        font: self.current_font,
                        height: self.current_font_height,
                        width: self.current_font_width,
                        orientation: self.current_font_orientation,
                        justification: self.field_justification,
                    })
                } else {
                    None
                }
            }

            Command::Code128 { orientation, height, print_interpetation_line,
                              print_above, check_digit, mode } => {
                self.pending_field_data.take().map(|data| {
                    DrawInstruction::DrawBarcode {
                        x: self.label_home_x + self.field_origin_x,
                        y: self.label_home_y + self.field_origin_y,
                        data,
                        orientation: orientation.unwrap_or(Orientation::Normal),
                        height: height.unwrap_or(self.barcode_height),
                        module_width: self.barcode_module_width,
                        print_interpretation_line: print_interpetation_line,
                        print_interpretation_line_above: print_above,
                        check_digit,
                        mode,
                    }
                })
            }

            Command::Code39 { orientation, check_digit, height,
                             print_interpetation_line, print_above } => {
                self.pending_field_data.take().map(|data| {
                    DrawInstruction::DrawCode39 {
                        x: self.label_home_x + self.field_origin_x,
                        y: self.label_home_y + self.field_origin_y,
                        data,
                        orientation,
                        check_digit,
                        height: height.unwrap_or(self.barcode_height),
                        module_width: self.barcode_module_width,
                        print_interpretation_line: print_interpetation_line,
                        print_interpretation_line_above: print_above,
                    }
                })
            }

            Command::QrCode { orientation, model, magnification, error_correction, mask } => {
                self.pending_field_data.take().map(|data| {
                    DrawInstruction::DrawQrCode {
                        x: self.label_home_x + self.field_origin_x,
                        y: self.label_home_y + self.field_origin_y,
                        data,
                        orientation,
                        model,
                        magnification,
                        error_correction,
                        mask,
                    }
                })
            }

            Command::GraphicBox { width, height, thickness, line_color, rounding } => {
                Some(DrawInstruction::DrawBox {
                    x: self.label_home_x + self.field_origin_x,
                    y: self.label_home_y + self.field_origin_y,
                    width,
                    height,
                    thickness,
                    color: line_color,
                    rounding,
                })
            }

            Command::GraphicCircle { diameter, thickness, color } => {
                Some(DrawInstruction::DrawCircle {
                    x: self.label_home_x + self.field_origin_x,
                    y: self.label_home_y + self.field_origin_y,
                    diameter,
                    thickness,
                    color,
                })
            }

            Command::GraphicDiagonalLine { width, height, thickness, color, direction } => {
                Some(DrawInstruction::DrawDiagonalLine {
                    x: self.label_home_x + self.field_origin_x,
                    y: self.label_home_y + self.field_origin_y,
                    width,
                    height,
                    thickness,
                    color,
                    direction,
                })
            }

            Command::GraphicField { compression, bytes_per_row, data, .. } => {
                Some(DrawInstruction::DrawGraphicField {
                    x: self.label_home_x + self.field_origin_x,
                    y: self.label_home_y + self.field_origin_y,
                    compression,
                    bytes_per_row,
                    data,
                })
            }

            Command::FieldTypeset { x, y, justification } => {
                self.field_origin_x = x;
                self.field_origin_y = y;
                if let Some(j) = justification {
                    self.field_justification = j;
                }
                None
            }
            Command::PrintQuality { .. } => None,
            Command::Comment { .. } => None,
            Command::Unknown { .. } => None,
        }
    }

    fn reset_format_state(&mut self) {
        self.field_origin_x = 0;
        self.field_origin_y = 0;
        self.field_justification = Justification::Left;
        self.pending_field_data = None;
        self.field_block_width = 0;
        self.field_block_max_lines = 1;
        self.field_block_line_spacing = 0;
        self.field_block_justification = BlockJustification::Left;
        self.field_block_hanging_indent = 0;
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

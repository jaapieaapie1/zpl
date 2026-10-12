use zpl_tokenizer::{CommandPrefix, Span, Token, Tokenizer};

use crate::{
    command::{
        BlockJustification, Code128Mode, Command, CommandPrefix as CmdPrefix, CompressionType,
        DiagonalDirection, Encoding, FontName, Justification, LineColor, Orientation,
        QrErrorCorrection, QrModel,
    },
    error::ParseError,
};

mod extra;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    peeked: Option<Token<'a>>,
    /// Format dots per ^MU unit: 1 for dots; for millimetres and inches the printer's dots
    /// per mm (or inch), divided by the ^JM / ^MU format scale so they stay physical sizes.
    unit_scale: std::cell::Cell<f32>,
    /// Printer resolution in dots per millimetre.
    dpmm: u32,
    /// ^MU units (`D`, `M` or `I`), ^MU base/desired dpi conversion and ^JM half density,
    /// which together decide `unit_scale`.
    units: std::cell::Cell<char>,
    conversion: std::cell::Cell<(u32, u32)>,
    half_density: std::cell::Cell<bool>,
}

/// Printer resolution assumed by [`Parser::new`]: 8 dots/mm (203 dpi).
pub const DEFAULT_DPMM: u32 = 8;

impl<'a> Parser<'a> {
    pub fn new(zpl: &'a str) -> Self {
        Self::with_dpmm(zpl, DEFAULT_DPMM)
    }

    /// Parser for a `dpmm` dots/mm printer (6, 8, 12 or 24): ^MU millimetres and inches
    /// are converted to that printer's dots.
    pub fn with_dpmm(zpl: &'a str, dpmm: u32) -> Self {
        Self {
            tokenizer: Tokenizer::new(zpl),
            peeked: None,
            unit_scale: std::cell::Cell::new(1.0),
            dpmm: dpmm.max(1),
            units: std::cell::Cell::new('D'),
            conversion: std::cell::Cell::new((1, 1)),
            half_density: std::cell::Cell::new(false),
        }
    }

    /// Recomputes `unit_scale` after ^MU or ^JM. Physical units use the printer's dots/mm
    /// divided by the format scale (desired/base dpi, doubled by ^JMB), so `^MUm^FO10,10`
    /// lands 10 mm in whatever the format is scaled to.
    fn update_unit_scale(&self) {
        let (base, desired) = self.conversion.get();
        let format_scale = desired as f32 / base as f32 * if self.half_density.get() { 2.0 } else { 1.0 };
        let per_mm = self.dpmm as f32 / format_scale;
        self.unit_scale.set(match self.units.get() {
            'I' => per_mm * 25.4,
            'M' => per_mm,
            _ => 1.0,
        });
    }

    pub fn parse(mut self) -> Result<Vec<Command>, ParseError> {
        let mut commands = Vec::new();

        while let Some(command) = self.parse_next()? {
            commands.push(command);
        }

        Ok(commands)
    }

    /// Like [`Parser::parse`], plus the source span of each command: byte offsets into the
    /// input, without surrounding whitespace.
    #[cfg(feature = "source-map")]
    pub fn parse_with_spans(mut self) -> Result<(Vec<Command>, Vec<Span>), ParseError> {
        let mut commands = Vec::new();
        let mut spans = Vec::new();

        loop {
            let start = self.tokenizer.position();
            let Some(command) = self.parse_next()? else { break };
            let end = self.tokenizer.position();
            let text = &self.tokenizer.input()[start..end];
            let start = start + (text.len() - text.trim_start().len());
            let end = start + text.trim().len();
            commands.push(command);
            spans.push(Span { start, end });
        }

        Ok((commands, spans))
    }

    #[inline]
    fn next_token(&mut self) -> Option<Token<'a>> {
        if let Some(token) = self.peeked.take() {
            Some(token)
        } else {
            self.tokenizer.next()
        }
    }

    pub fn parse_next(&mut self) -> Result<Option<Command>, ParseError> {
        let token = match self.next_token() {
            Some(t) => t,
            None => return Ok(None),
        };

        let command = match token {
            Token::StartToken => Command::StartFormat,
            Token::EndToken => Command::EndFormat,
            Token::FieldSeparator => Command::FieldSeperator,
            Token::FieldData { data, .. } => Command::FielData {
                data: data.to_string(),
            },

            Token::Command {
                prefix,
                name,
                params,
                span,
            } => {
                // Printers skip a command with bad parameters instead of rejecting the label.
                self.parse_command(prefix, name, &params, span).unwrap_or_else(|_| Command::Unknown {
                    prefix: match prefix {
                        CommandPrefix::Caret => CmdPrefix::Carot,
                        CommandPrefix::Tilde => CmdPrefix::Tilde,
                    },
                    name: name.to_string(),
                    params: params.iter().map(|s| s.to_string()).collect(),
                })
            }

            Token::Unknown { content, .. } => Command::Unknown {
                prefix: CmdPrefix::Carot,
                name: "UNKNOWN".to_string(),
                params: vec![content.to_string()],
            },
        };

        Ok(Some(command))
    }

    fn parse_command(
        &self,
        prefix: CommandPrefix,
        name: &str,
        params: &[&str],
        span: Span,
    ) -> Result<Command, ParseError> {
        let name_upper = name.to_uppercase();

        match (prefix, name_upper.as_str()) {
            (CommandPrefix::Caret, "FO") => self.parse_field_origin(params, span),
            (CommandPrefix::Caret, "FT") => self.parse_field_typeset(params, span),
            (CommandPrefix::Caret, "FB") => self.parse_field_block(params, span),
            (CommandPrefix::Caret, "FR") => Ok(Command::FieldReverse),
            (CommandPrefix::Caret, "LH") => self.parse_label_home(params, span),
            (CommandPrefix::Caret, "LT") => self.parse_label_top(params, span),
            (CommandPrefix::Caret, "PW") => self.parse_print_width(params, span),
            (CommandPrefix::Caret, "LL") => self.parse_label_length(params, span),

            (CommandPrefix::Caret, "A") => self.parse_font(params, span),
            (CommandPrefix::Caret, "CF") => self.parse_change_font(params, span),

            (CommandPrefix::Caret, "GB") => self.parse_graphic_box(params, span),
            (CommandPrefix::Caret, "GF") => self.parse_graphic_field(params, span),
            (CommandPrefix::Caret, "GC") => self.parse_graphic_circle(params, span),
            (CommandPrefix::Caret, "GD") => self.parse_graphic_diagonal_line(params, span),

            (CommandPrefix::Caret, "BC") => self.parse_code128(params, span),
            (CommandPrefix::Caret, "BY") => self.parse_barcode_field_defaults(params, span),
            (CommandPrefix::Caret, "BQ") => self.parse_qr_code(params, span),
            (CommandPrefix::Caret, "B3") => self.parse_code39(params, span),

            (CommandPrefix::Caret, "CI") => self.parse_character_encoding(params, span),
            (CommandPrefix::Caret, "PQ") => self.parse_print_quantity(params, span),
            (CommandPrefix::Caret, "MD") => self.parse_media_darkness(params, span),
            (CommandPrefix::Caret, "FX") => self.parse_comment(params, span),

            _ => {
                if let Some(command) = self.parse_extra(prefix == CommandPrefix::Tilde, &name_upper, params, span) {
                    return command;
                }
                Ok(Command::Unknown {
                prefix: match prefix {
                    CommandPrefix::Caret => CmdPrefix::Carot,
                    CommandPrefix::Tilde => CmdPrefix::Tilde,
                },
                name: name.to_string(),
                params: params.iter().map(|s| s.to_string()).collect(),
                })
            }
        }
    }

    fn parse_field_origin(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^FOx,y[,justification]

        let x = self.dim_param(params, 0, "x", span)?;
        let y = self.dim_param(params, 1, "y", span)?;

        let justification = if params.len() >= 3 {
            self.parse_justification(params[2], span)?
        } else {
            None
        };

        Ok(Command::FieldOrigin {
            x,
            y,
            justification,
        })
    }

    fn parse_field_typeset(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^FTx,y (Zebra continues after the previous field when omitted; we use 0)

        let x = self.opt_dim(params, 0, span)?.unwrap_or(0);
        let y = self.opt_dim(params, 1, span)?.unwrap_or(0);

        let justification = if params.len() >= 3 {
            self.parse_justification(params[2], span)?
        } else {
            None
        };

        Ok(Command::FieldTypeset {
            x,
            y,
            justification,
        })
    }

    fn parse_label_home(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^LHx,y

        let x = self.dim_param(params, 0, "x", span)?;
        let y = self.dim_param(params, 1, "y", span)?;

        Ok(Command::LabelHome { x, y })
    }

    fn parse_label_top(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^LTx

        let offset = self.dim_i32(params.first().copied().unwrap_or("0"), span)?;

        Ok(Command::LabelTop { offset })
    }

    fn parse_print_width(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^PWa

        let width = self.dim_param(params, 0, "a", span)?;

        Ok(Command::PrintWidth { width })
    }

    fn parse_label_length(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^LLy,x

        let length = self.dim_param(params, 0, "y", span)?;

        let apply_to_all_media = if params.len() >= 2 && !params[1].is_empty() {
            Some(self.parse_yes_no(params[1], span)?)
        } else {
            None
        };

        Ok(Command::LabelLength {
            length,
            apply_to_all_media,
        })
    }

    fn parse_font(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^A@o,h,w,d:o.x selects a stored font by name.
        if params.first().is_some_and(|p| p.starts_with('@')) {
            let orientation = match params[0][1..].chars().next() {
                Some(c) => Some(self.parse_orientation(&c.to_string(), span)?),
                None => None,
            };
            return Ok(Command::FontByName {
                orientation,
                height: self.opt_dim(params, 1, span)?,
                width: self.opt_dim(params, 2, span)?,
                name: params.get(3).map(|s| s.trim().to_string()).unwrap_or_default(),
            });
        }

        // ^Afo,h,w
        let (font_name, orientation) =
            self.parse_font_name_orientation(params, 0, "font_orientation", span)?;

        let height = if params.len() > 1 && !params[1].is_empty() {
            Some(self.dim_param(params, 1, "h", span)?)
        } else {
            None
        };

        let width = if params.len() > 2 && !params[2].is_empty() {
            Some(self.dim_param(params, 2, "w", span)?)
        } else {
            None
        };

        Ok(Command::Font {
            font: font_name,
            orientation,
            height,
            width,
        })
    }

    fn parse_change_font(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^CFf,h,w

        if params.is_empty() || params[0].is_empty() {
            return Err(ParseError::MissingRequiredParameter {
                command: "CF",
                parameter: "f",
                span,
            });
        }

        let font_char = params[0]
            .chars()
            .next()
            .ok_or(ParseError::MissingRequiredParameter {
                command: "CF",
                parameter: "f",
                span,
            })?;

        let font = self.convert_font(font_char);

        let height = if params.len() > 1 && !params[1].is_empty() {
            Some(self.dim(params[1], span)?)
        } else {
            None
        };

        let width = if params.len() > 2 && !params[2].is_empty() {
            Some(self.dim(params[2], span)?)
        } else {
            None
        };

        Ok(Command::ChangeFont {
            font,
            height,
            width,
        })
    }

    fn parse_field_block(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^FBa,b,c,d,e
        let width = if params.is_empty() || params[0].is_empty() {
            0
        } else {
            self.dim(params[0], span)?
        };

        let max_lines = if params.len() < 2 || params[1].is_empty() {
            1
        } else {
            self.parse_u32(params[1], span)?
        };

        let line_spacing = if params.len() < 3 || params[2].is_empty() {
            0
        } else {
            self.dim_i32(params[2], span)?
        };

        let justification = if params.len() < 4 || params[3].is_empty() {
            BlockJustification::Left
        } else {
            self.parse_block_justification(params[3], span)?
        };

        let hanging_indent = if params.len() < 5 || params[4].is_empty() {
            0
        } else {
            self.dim(params[4], span)?
        };

        Ok(Command::FieldBlock {
            width,
            max_lines,
            line_spacing,
            justification,
            hanging_indent,
        })
    }

    fn parse_graphic_box(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^GBw,h,t,c,r
        let thickness = if params.len() >= 3 && !params[2].is_empty() {
            self.dim(params[2], span)?
        } else {
            1
        };

        let width = if params.is_empty() || params[0].is_empty() {
            thickness
        } else {
            self.dim(params[0], span)?
        };

        let height = if params.len() < 2 || params[1].is_empty() {
            thickness
        } else {
            self.dim(params[1], span)?
        };

        let line_color = if params.len() < 4 || params[3].is_empty() {
            LineColor::Black
        } else {
            self.parse_line_color(params[3], span)?
        };

        let rounding = if params.len() < 5 || params[4].is_empty() {
            0
        } else {
            self.parse_u32(params[4], span)?
        };

        Ok(Command::GraphicBox {
            width,
            height,
            thickness,
            line_color,
            rounding,
        })
    }

    fn parse_graphic_field(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^GFa,b,c,d,data
        let compression = if params.is_empty() || params[0].is_empty() {
            CompressionType::Ascii
        } else {
            self.parse_compression_type(params[0], span)?
        };

        let binary_byte_count = self.parse_u32_param(params, 1, "b", span)?;
        let graphic_field_count = self.parse_u32_param(params, 2, "c", span)?;
        let bytes_per_row = self.parse_u32_param(params, 3, "d", span)?;

        let data = if params.len() >= 5 {
            // The data itself may contain commas (ASCII compression), so rejoin the
            // remaining params that the tokenizer split on ','.
            let raw = params[4..].join(",");

            // Check for Z64 compression format (:Z64:data:checksum)
            if raw.starts_with(":Z64:") || raw.starts_with(":B64:") {
                let compact: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
                self.decode_z64_data(&compact)?
            } else {
                match compression {
                    CompressionType::Ascii => self.decode_hex_data(&raw, bytes_per_row),
                    CompressionType::Binary | CompressionType::CompressedBinary => {
                        raw.as_bytes().to_vec()
                    }
                }
            }
        } else {
            Vec::new()
        };

        Ok(Command::GraphicField {
            compression,
            binary_byte_count,
            graphic_field_count,
            bytes_per_row,
            data,
        })
    }

    /// Decodes ASCII hex graphic data, including ZPL's ASCII compression:
    /// - `G`-`Y` repeat the next hex digit 1-19 times, `g`-`z` add 20-400 (combinable)
    /// - `,` fills the rest of the row with `0`
    /// - `!` fills the rest of the row with `F`
    /// - `:` repeats the previous row
    fn decode_hex_data(&self, hex_str: &str, bytes_per_row: u32) -> Vec<u8> {
        let row_len = (bytes_per_row as usize) * 2;
        // One flat nibble buffer. Complete rows occupy whole multiples of `row_len`, so the
        // row being built starts at the last such multiple and the previous row precedes it.
        let mut nibbles: Vec<u8> = Vec::with_capacity(hex_str.len() * 2);
        let row_start = |len: usize| len / row_len * row_len;
        let mut repeat: usize = 0;

        for &c in hex_str.as_bytes() {
            match c {
                b'G'..=b'Y' => repeat += (c - b'F') as usize,
                b'g'..=b'z' => repeat += (c - b'f') as usize * 20,
                b',' | b'!' if row_len > 0 => {
                    let fill = if c == b',' { 0x0 } else { 0xF };
                    let start = row_start(nibbles.len());
                    nibbles.resize(start + row_len, fill);
                    repeat = 0;
                }
                b':' if row_len > 0 => {
                    let mut start = row_start(nibbles.len());
                    if nibbles.len() > start {
                        nibbles.resize(start + row_len, 0);
                        start += row_len;
                    }
                    if start >= row_len {
                        nibbles.extend_from_within(start - row_len..start);
                    } else {
                        nibbles.resize(row_len, 0);
                    }
                    repeat = 0;
                }
                _ => {
                    let value = match c {
                        b'0'..=b'9' => c - b'0',
                        b'A'..=b'F' => c - b'A' + 10,
                        b'a'..=b'f' => c - b'a' + 10,
                        _ => continue,
                    };
                    nibbles.resize(nibbles.len() + repeat.max(1), value);
                    repeat = 0;
                }
            }
        }

        nibbles.chunks_exact(2).map(|pair| (pair[0] << 4) | pair[1]).collect()
    }

    fn decode_z64_data(&self, data: &str) -> Result<Vec<u8>, ParseError> {
        use base64::{Engine as _, engine::general_purpose};

        // Format is :Z64:base64data:checksum or :B64:base64data:checksum
        let is_compressed = data.starts_with(":Z64:");

        // Extract the base64 data between the markers
        let start_marker = if is_compressed { ":Z64:" } else { ":B64:" };
        let data_part = data.strip_prefix(start_marker)
            .ok_or_else(|| ParseError::InvalidZ64Data {
                message: "Missing Z64 or B64 prefix".to_string(),
            })?;

        // Split by the ending colon to separate data from checksum
        let parts: Vec<&str> = data_part.split(':').collect();
        if parts.is_empty() {
            return Err(ParseError::InvalidZ64Data {
                message: "Empty Z64 data".to_string(),
            });
        }

        let base64_data = parts[0];

        // Decode base64
        let compressed_bytes = general_purpose::STANDARD
            .decode(base64_data)
            .map_err(|e| ParseError::InvalidZ64Data {
                message: format!("Base64 decode error: {}", e),
            })?;

        // Decompress if Z64 (compressed), otherwise return as-is if B64 (uncompressed)
        if is_compressed {
            self.decompress_lz77(&compressed_bytes)
        } else {
            Ok(compressed_bytes)
        }
    }

    fn decompress_lz77(&self, compressed: &[u8]) -> Result<Vec<u8>, ParseError> {
        use flate2::read::ZlibDecoder;
        use std::io::Read;

        let mut decoder = ZlibDecoder::new(compressed);
        let mut decompressed = Vec::new();

        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| ParseError::InvalidZ64Data {
                message: format!("ZLIB decompression error: {}", e),
            })?;

        Ok(decompressed)
    }

    fn parse_code128(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^BCo,h,f,g,e,m
        let orientation = if params.is_empty() || params[0].is_empty() {
            None
        } else {
            Some(self.parse_orientation(params[0], span)?)
        };

        let height = if params.len() < 2 || params[1].is_empty() {
            None
        } else {
            Some(self.dim(params[1], span)?)
        };

        let print_interpetation_line = if params.len() < 3 || params[2].is_empty() {
            true
        } else {
            self.parse_yes_no(params[2], span)?
        };

        let print_above = if params.len() < 4 || params[3].is_empty() {
            false
        } else {
            self.parse_yes_no(params[3], span)?
        };

        let check_digit = if params.len() < 5 || params[4].is_empty() {
            false
        } else {
            self.parse_yes_no(params[4], span)?
        };

        let mode = if params.len() < 6 || params[5].is_empty() {
            Code128Mode::AutoMode
        } else {
            self.parse_code128_mode(params[5], span)?
        };

        Ok(Command::Code128 {
            orientation,
            height,
            print_interpetation_line,
            print_above,
            check_digit,
            mode,
        })
    }

    fn parse_barcode_field_defaults(
        &self,
        params: &[&str],
        span: Span,
    ) -> Result<Command, ParseError> {
        // ^BYw,r,h
        let module_width = if params.is_empty() || params[0].is_empty() {
            2
        } else {
            self.parse_u32(params[0], span)?
        };

        let wide_bar_ratio = if params.len() < 2 || params[1].is_empty() {
            3.0
        } else {
            self.parse_f32(params[1], span)?
        };

        let height = if params.len() < 3 || params[2].is_empty() {
            10
        } else {
            self.dim(params[2], span)?
        };

        Ok(Command::BarcodeFieldDefaults {
            module_width,
            wide_bar_ratio,
            height,
        })
    }

    fn parse_qr_code(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^BQa,b,c,d,e
        let orientation = if params.is_empty() || params[0].is_empty() {
            Orientation::Normal
        } else {
            self.parse_orientation(params[0], span)?
        };

        let model = if params.len() < 2 || params[1].is_empty() {
            QrModel::Model2
        } else {
            self.parse_qr_model(params[1], span)?
        };

        let magnification = if params.len() < 3 || params[2].is_empty() {
            1
        } else {
            self.parse_u32(params[2], span)?
        };

        let error_correction = if params.len() < 4 || params[3].is_empty() {
            QrErrorCorrection::HighReliability
        } else {
            self.parse_qr_error_correction(params[3], span)?
        };

        let mask = if params.len() < 5 || params[4].is_empty() {
            Some(7)
        } else {
            Some(self.parse_u32(params[4], span)?)
        };

        Ok(Command::QrCode {
            orientation,
            model,
            magnification,
            error_correction,
            mask,
        })
    }

    fn parse_code39(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^B3o,e,h,f,g
        let orientation = if params.is_empty() || params[0].is_empty() {
            Orientation::Normal
        } else {
            self.parse_orientation(params[0], span)?
        };

        let check_digit = if params.len() < 2 || params[1].is_empty() {
            false
        } else {
            self.parse_yes_no(params[1], span)?
        };

        let height = if params.len() < 3 || params[2].is_empty() {
            None
        } else {
            Some(self.dim(params[2], span)?)
        };

        let print_interpetation_line = if params.len() < 4 || params[3].is_empty() {
            true
        } else {
            self.parse_yes_no(params[3], span)?
        };

        let print_above = if params.len() < 5 || params[4].is_empty() {
            false
        } else {
            self.parse_yes_no(params[4], span)?
        };

        Ok(Command::Code39 {
            orientation,
            height,
            print_interpetation_line,
            print_above,
            check_digit,
        })
    }

    fn parse_character_encoding(
        &self,
        params: &[&str],
        span: Span,
    ) -> Result<Command, ParseError> {
        // ^CIa,s1,d1,s2,d2,...
        let encoding = if params.is_empty() || params[0].is_empty() {
            Encoding::Usa1
        } else {
            self.parse_encoding(params[0], span)?
        };

        Ok(Command::CharacterEncoding { encoding })
    }

    fn parse_print_quantity(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^PQq,p,r,o,e
        let total = if params.is_empty() || params[0].is_empty() {
            1
        } else {
            self.parse_u32(params[0], span)?
        };

        let pause_count = if params.len() < 2 || params[1].is_empty() {
            None
        } else {
            let val = self.parse_u32(params[1], span)?;
            if val == 0 {
                None
            } else {
                Some(val)
            }
        };

        let replicate_count = if params.len() < 3 || params[2].is_empty() {
            None
        } else {
            let val = self.parse_u32(params[2], span)?;
            if val == 0 {
                None
            } else {
                Some(val)
            }
        };

        let override_pause = if params.len() < 4 || params[3].is_empty() {
            false
        } else {
            self.parse_yes_no(params[3], span)?
        };

        Ok(Command::PrintQuantity {
            total,
            pause_count,
            replicate_count,
            override_pause,
        })
    }

    fn parse_media_darkness(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^MDa
        let value = if params.is_empty() || params[0].is_empty() {
            0
        } else {
            self.parse_i32(params[0], span)?
        };

        Ok(Command::MediaDarkness { value })
    }

    fn parse_comment(&self, params: &[&str], _span: Span) -> Result<Command, ParseError> {
        // ^FXc
        let text = if params.is_empty() {
            String::new()
        } else {
            params[0].to_string()
        };

        Ok(Command::Comment { text })
    }

    fn parse_graphic_circle(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^GCd,t,c
        let diameter = if params.is_empty() || params[0].is_empty() {
            3
        } else {
            self.dim(params[0], span)?
        };

        let thickness = if params.len() < 2 || params[1].is_empty() {
            1
        } else {
            self.dim(params[1], span)?
        };

        let color = if params.len() < 3 || params[2].is_empty() {
            LineColor::Black
        } else {
            self.parse_line_color(params[2], span)?
        };

        Ok(Command::GraphicCircle {
            diameter,
            thickness,
            color,
        })
    }

    fn parse_graphic_diagonal_line(
        &self,
        params: &[&str],
        span: Span,
    ) -> Result<Command, ParseError> {
        // ^GDw,h,t,c,o
        let thickness = if params.len() < 3 || params[2].is_empty() {
            1
        } else {
            self.dim(params[2], span)?
        };

        let width = if params.is_empty() || params[0].is_empty() {
            if thickness >= 3 {
                thickness
            } else {
                3
            }
        } else {
            self.dim(params[0], span)?
        };

        let height = if params.len() < 2 || params[1].is_empty() {
            if thickness >= 3 {
                thickness
            } else {
                3
            }
        } else {
            self.dim(params[1], span)?
        };

        let color = if params.len() < 4 || params[3].is_empty() {
            LineColor::Black
        } else {
            self.parse_line_color(params[3], span)?
        };

        let direction = if params.len() < 5 || params[4].is_empty() {
            DiagonalDirection::RightToLeft
        } else {
            self.parse_diagonal_direction(params[4], span)?
        };

        Ok(Command::GraphicDiagonalLine {
            width,
            height,
            thickness,
            color,
            direction,
        })
    }

    fn parse_font_name_orientation(
        &self,
        params: &[&str],
        index: usize,
        name: &'static str,
        span: Span,
    ) -> Result<(FontName, Option<Orientation>), ParseError> {
        if index >= params.len() || params[index].is_empty() {
            return Err(ParseError::MissingRequiredParameter {
                command: "",
                parameter: name,
                span,
            });
        }

        let mut param_chars = params[index].chars();

        let Some(font_name) = param_chars.next() else {
            return Err(ParseError::MissingRequiredParameter {
                command: "",
                parameter: name,
                span,
            });
        };

        let font = self.convert_font(font_name);

        let orientation = param_chars
            .next()
            .map(
                |orientation_char| match orientation_char.to_ascii_uppercase() {
                    'N' => Ok(Orientation::Normal),
                    'R' => Ok(Orientation::Rotated90),
                    'I' => Ok(Orientation::Rotated180),
                    'B' => Ok(Orientation::Rotated270),
                    _ => Err(ParseError::InvalidOrientation),
                },
            )
            .transpose()?;

        Ok((font, orientation))
    }

    fn convert_font(&self, font_char: char) -> FontName {
        match font_char.to_ascii_uppercase() {
            'A' => FontName::FontA,
            'B' => FontName::FontB,
            'C' => FontName::FontC,
            'D' => FontName::FontD,
            'E' => FontName::FontE,
            'F' => FontName::FontF,
            'G' => FontName::FontG,
            'H' => FontName::FontH,
            'I' => FontName::FontI,
            'J' => FontName::FontJ,
            'K' => FontName::FontK,
            'L' => FontName::FontL,
            'M' => FontName::FontM,
            'O' => FontName::FontO,
            'P' => FontName::FontP,
            'Q' => FontName::FontQ,
            'R' => FontName::FontR,
            'S' => FontName::FontS,
            'T' => FontName::FontT,
            'U' => FontName::FontU,
            'V' => FontName::FontV,
            '0' => FontName::Font0,
            '1' => FontName::Font1,
            '2' => FontName::Font2,
            '3' => FontName::Font3,
            '4' => FontName::Font4,
            '5' => FontName::Font5,
            '6' => FontName::Font6,
            '7' => FontName::Font7,
            '8' => FontName::Font8,
            '9' => FontName::Font9,
            a => FontName::Custom(a),
        }
    }

    fn parse_justification(
        &self,
        s: &str,
        span: Span,
    ) -> Result<Option<Justification>, ParseError> {
        match s.to_uppercase().as_str() {
            "0" | "L" => Ok(Some(Justification::Left)),
            "1" | "R" => Ok(Some(Justification::Right)),
            "2" | "A" => Ok(Some(Justification::Auto)),
            "" => Ok(None),
            _ => Err(ParseError::InvalidJustification {
                value: s.to_string(),
                span,
            }),
        }
    }

    fn parse_yes_no(&self, s: &str, span: Span) -> Result<bool, ParseError> {
        match s.to_uppercase().as_str() {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => Err(ParseError::InvalidYesNo {
                value: s.to_string(),
                span,
            }),
        }
    }

    fn parse_block_justification(
        &self,
        s: &str,
        span: Span,
    ) -> Result<BlockJustification, ParseError> {
        match s.to_uppercase().as_str() {
            "L" => Ok(BlockJustification::Left),
            "C" => Ok(BlockJustification::Center),
            "R" => Ok(BlockJustification::Right),
            "J" => Ok(BlockJustification::Justified),
            _ => Err(ParseError::InvalidBlockJustification {
                value: s.to_string(),
                span,
            }),
        }
    }

    fn parse_line_color(&self, s: &str, span: Span) -> Result<LineColor, ParseError> {
        match s.to_uppercase().as_str() {
            "B" => Ok(LineColor::Black),
            "W" => Ok(LineColor::White),
            _ => Err(ParseError::InvalidLineColor {
                value: s.to_string(),
                span,
            }),
        }
    }

    fn parse_compression_type(&self, s: &str, span: Span) -> Result<CompressionType, ParseError> {
        match s.to_uppercase().as_str() {
            "A" => Ok(CompressionType::Ascii),
            "B" => Ok(CompressionType::Binary),
            "C" => Ok(CompressionType::CompressedBinary),
            _ => Err(ParseError::InvalidCompressionType {
                value: s.to_string(),
                span,
            }),
        }
    }

    fn parse_orientation(&self, s: &str, _span: Span) -> Result<Orientation, ParseError> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Orientation::Normal),
            "R" => Ok(Orientation::Rotated90),
            "I" => Ok(Orientation::Rotated180),
            "B" => Ok(Orientation::Rotated270),
            _ => Err(ParseError::InvalidOrientation),
        }
    }

    fn parse_code128_mode(&self, s: &str, span: Span) -> Result<Code128Mode, ParseError> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Code128Mode::AutoMode),
            "U" => Ok(Code128Mode::UccCase),
            "A" => Ok(Code128Mode::Auto),
            "D" => Ok(Code128Mode::Auto),
            _ => Err(ParseError::InvalidCode128Mode {
                value: s.to_string(),
                span,
            }),
        }
    }

    fn parse_qr_model(&self, s: &str, span: Span) -> Result<QrModel, ParseError> {
        match s {
            "1" => Ok(QrModel::Model1),
            "2" => Ok(QrModel::Model2),
            _ => Err(ParseError::InvalidNumber {
                value: s.to_string(),
                span,
            }),
        }
    }

    fn parse_qr_error_correction(
        &self,
        s: &str,
        span: Span,
    ) -> Result<QrErrorCorrection, ParseError> {
        match s.to_uppercase().as_str() {
            "H" => Ok(QrErrorCorrection::UltraHighReliability),
            "Q" => Ok(QrErrorCorrection::HighReliability),
            "M" => Ok(QrErrorCorrection::Standard),
            "L" => Ok(QrErrorCorrection::HighDensity),
            _ => Err(ParseError::InvalidNumber {
                value: s.to_string(),
                span,
            }),
        }
    }

    fn parse_encoding(&self, s: &str, span: Span) -> Result<Encoding, ParseError> {
        let num = self.parse_u32(s, span)?;
        match num {
            0 => Ok(Encoding::Usa1),
            1 => Ok(Encoding::Usa2),
            2 => Ok(Encoding::Uk),
            3 => Ok(Encoding::Holland),
            4 => Ok(Encoding::DenmarkNorway),
            5 => Ok(Encoding::SwedenFinland),
            6 => Ok(Encoding::Germany),
            7 => Ok(Encoding::France1),
            8 => Ok(Encoding::France2),
            9 => Ok(Encoding::Italy),
            10 => Ok(Encoding::Spain),
            11 => Ok(Encoding::Misc),
            12 => Ok(Encoding::Japan),
            13 => Ok(Encoding::Zebra),
            14 => Ok(Encoding::DoubleByteAsian),
            15 => Ok(Encoding::ShiftJIS),
            16 => Ok(Encoding::EUCJPCN),
            17 => Ok(Encoding::UCS2BigEndian),
            24 => Ok(Encoding::SingleByteAsian),
            26 => Ok(Encoding::MultibyteAsian),
            28 => Ok(Encoding::Utf8),
            29 => Ok(Encoding::Utf16BigEndian),
            30 => Ok(Encoding::Utf16LittleEndian),
            27 => Ok(Encoding::Cp1252),
            31 => Ok(Encoding::Cp1250),
            33 => Ok(Encoding::Cp1251),
            34 => Ok(Encoding::Cp1253),
            35 => Ok(Encoding::Cp1254),
            36 => Ok(Encoding::Cp1255),
            _ => Err(ParseError::InvalidNumber {
                value: s.to_string(),
                span,
            }),
        }
    }

    fn parse_diagonal_direction(
        &self,
        s: &str,
        span: Span,
    ) -> Result<DiagonalDirection, ParseError> {
        match s.to_uppercase().as_str() {
            "R" | "/" => Ok(DiagonalDirection::LeftToRight),
            "L" | "\\" => Ok(DiagonalDirection::RightToLeft),
            _ => Err(ParseError::InvalidNumber {
                value: s.to_string(),
                span,
            }),
        }
    }

    /// A length in the current ^MU units, converted to dots. Inches and millimetres may be
    /// fractional.
    fn dim(&self, s: &str, span: Span) -> Result<u32, ParseError> {
        let scale = self.unit_scale.get();
        if scale == 1.0 {
            return self.parse_u32(s, span);
        }
        let v = self.parse_f32(s, span)?;
        if v < 0.0 {
            return Err(ParseError::InvalidNumber { value: s.to_string(), span });
        }
        Ok((v * scale).round() as u32)
    }

    fn dim_i32(&self, s: &str, span: Span) -> Result<i32, ParseError> {
        let scale = self.unit_scale.get();
        if scale == 1.0 {
            return self.parse_i32(s, span);
        }
        Ok((self.parse_f32(s, span)? * scale).round() as i32)
    }

    fn dim_param(&self, params: &[&str], index: usize, name: &'static str, span: Span) -> Result<u32, ParseError> {
        match params.get(index) {
            Some(s) => self.dim(s, span),
            None => Err(ParseError::MissingRequiredParameter { command: "", parameter: name, span }),
        }
    }

    fn opt_dim(&self, params: &[&str], index: usize, span: Span) -> Result<Option<u32>, ParseError> {
        match params.get(index) {
            Some(s) if !s.is_empty() => self.dim(s, span).map(Some),
            _ => Ok(None),
        }
    }

    fn parse_u32_param(
        &self,
        params: &[&str],
        index: usize,
        name: &'static str,
        span: Span,
    ) -> Result<u32, ParseError> {
        if index >= params.len() {
            return Err(ParseError::MissingRequiredParameter {
                command: "",
                parameter: name,
                span,
            });
        }

        self.parse_u32(params[index], span)
    }

    fn parse_optional_u32(
        &self,
        params: &[&str],
        index: usize,
        span: Span,
    ) -> Result<Option<u32>, ParseError> {
        if index >= params.len() || params[index].is_empty() {
            return Ok(None);
        }

        Ok(Some(self.parse_u32(params[index], span)?))
    }

    #[inline]
    fn parse_u32(&self, s: &str, span: Span) -> Result<u32, ParseError> {
        s.parse::<u32>().map_err(|_| ParseError::InvalidNumber {
            value: s.to_string(),
            span,
        })
    }

    #[inline]
    fn parse_f32(&self, s: &str, span: Span) -> Result<f32, ParseError> {
        s.parse::<f32>().map_err(|_| ParseError::InvalidNumber {
            value: s.to_string(),
            span,
        })
    }

    #[inline]
    fn parse_i32(&self, s: &str, span: Span) -> Result<i32, ParseError> {
        s.parse::<i32>().map_err(|_| ParseError::InvalidNumber {
            value: s.to_string(),
            span,
        })
    }
}

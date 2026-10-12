//! Field modifiers, stored objects, page settings and the less common barcodes.

use zpl_tokenizer::Span;

use super::Parser;
use crate::{
    command::{Code49Interpretation, Command, FieldDirection, LinearKind, LineColor, MsiCheck, Orientation, PostalKind, TextLayout},
    error::ParseError,
};

/// Plain ASCII hex (no ZPL compression), ignoring anything that isn't a hex digit.
fn decode_hex(raw: &str) -> Vec<u8> {
    let digits: Vec<u8> = raw.bytes().filter_map(|b| (b as char).to_digit(16).map(|d| d as u8)).collect();
    digits.chunks_exact(2).map(|p| p[0] << 4 | p[1]).collect()
}

/// Parameter `i`, or "" when absent.
fn param<'p>(params: &[&'p str], i: usize) -> &'p str {
    params.get(i).copied().unwrap_or("").trim()
}

fn first_char(params: &[&str], i: usize) -> Option<char> {
    param(params, i).chars().next()
}

impl Parser<'_> {
    fn opt_orientation(&self, params: &[&str], i: usize, span: Span) -> Result<Option<Orientation>, ParseError> {
        match param(params, i) {
            "" => Ok(None),
            s => self.parse_orientation(s, span).map(Some),
        }
    }

    fn yes_no_or(&self, params: &[&str], i: usize, default: bool, span: Span) -> Result<bool, ParseError> {
        match param(params, i) {
            "" => Ok(default),
            s => self.parse_yes_no(s, span),
        }
    }

    fn u32_or(&self, params: &[&str], i: usize, default: u32, span: Span) -> Result<u32, ParseError> {
        Ok(self.parse_optional_u32(params, i, span)?.unwrap_or(default))
    }

    /// A length parameter in ^MU units, or `default` dots.
    fn dim_or(&self, params: &[&str], i: usize, default: u32, span: Span) -> Result<u32, ParseError> {
        Ok(self.opt_dim(params, i, span)?.unwrap_or(default))
    }

    pub(super) fn parse_extra(&self, tilde: bool, name: &str, params: &[&str], span: Span) -> Option<Result<Command, ParseError>> {
        Some(match (tilde, name) {
            (false, "FH") => Ok(Command::FieldHexIndicator { indicator: first_char(params, 0).unwrap_or('_') }),
            (false, "FW") => self.parse_field_orientation(params, span),
            (false, "FP") => self.parse_field_parameter(params, span),
            (false, "TB") => self.parse_text_block(params, span),
            (false, "FN") => Ok(Command::FieldNumber {
                number: param(params, 0).chars().take_while(char::is_ascii_digit).collect::<String>().parse().unwrap_or(0),
            }),
            (false, "SN") => Ok(Command::SerializationData {
                start: match params.first() {
                    None | Some(&"") => "1".to_string(),
                    Some(v) => v.to_string(),
                },
                increment: match param(params, 1) {
                    "" => 1,
                    s => s.parse().ok()?,
                },
                leading_zeros: self.yes_no_or(params, 2, false, span).ok()?,
            }),
            (false, "SF") => {
                let mask = param(params, 0).to_ascii_uppercase();
                if mask.is_empty() || !mask.chars().all(|c| matches!(c, 'D' | 'H' | 'O' | 'A' | 'N' | '%')) {
                    return None;
                }
                let increment = match param(params, 1) {
                    "" => "1".to_string(),
                    s => s.to_string(),
                };
                Ok(Command::SerializationField { mask, increment })
            }
            (false, "DF") => Ok(Command::DownloadFormat { name: param(params, 0).to_string() }),
            (false, "XF") => Ok(Command::RecallFormat { name: param(params, 0).to_string() }),
            (true, "DG") => self.parse_download_graphic(params, span),
            (true, "DY") => return self.parse_download_object(params, span),
            (false, "XG") => Ok(Command::RecallGraphic {
                name: param(params, 0).to_string(),
                magnification_x: self.u32_or(params, 1, 1, span).ok()?,
                magnification_y: self.u32_or(params, 2, 1, span).ok()?,
            }),
            (false, "IM") => Ok(Command::RecallGraphic { name: param(params, 0).to_string(), magnification_x: 1, magnification_y: 1 }),
            (false, "LR") => self.yes_no_or(params, 0, false, span).map(|enabled| Command::LabelReverse { enabled }),
            (false, "PM") => self.yes_no_or(params, 0, false, span).map(|enabled| Command::PrintMirror { enabled }),
            (false, "PO") => Ok(Command::PrintOrientation { inverted: param(params, 0).eq_ignore_ascii_case("I") }),
            (false, "LS") => Ok(Command::LabelShift {
                shift: match param(params, 0) {
                    "" => 0,
                    s => self.dim_i32(s, span).ok()?,
                },
            }),
            (false, "GE") => self.parse_graphic_ellipse(params, span),
            (false, "GS") => self.parse_graphic_symbol(params, span),
            (false, "BX") => self.parse_data_matrix(params, span),
            (false, "BD") => Ok(Command::MaxiCode {
                mode: self.u32_or(params, 0, 2, span).ok()?.clamp(2, 6) as u8,
                symbol_number: self.u32_or(params, 1, 1, span).ok()?.clamp(1, 8) as u8,
                total_symbols: self.u32_or(params, 2, 1, span).ok()?.clamp(1, 8) as u8,
            }),
            (false, "BR") => Ok(Command::DataBar {
                // ^BR defaults to rotated.
                orientation: Some(self.opt_orientation(params, 0, span).ok()?.unwrap_or(Orientation::Rotated90)),
                kind: self.u32_or(params, 1, 1, span).ok()?.clamp(1, 12) as u8,
                magnification: self.parse_optional_u32(params, 2, span).ok()?.map(|m| m.clamp(1, 10)),
                separator_height: self.u32_or(params, 3, 1, span).ok()?.clamp(1, 2),
                height: self.opt_dim(params, 4, span).ok()?,
                segments_per_row: self.u32_or(params, 5, 22, span).ok()?,
            }),
            (false, "BF") => Ok(Command::MicroPdf417 {
                orientation: self.opt_orientation(params, 0, span).ok()?,
                height: self.opt_dim(params, 1, span).ok()?,
                mode: self.u32_or(params, 2, 0, span).ok()?.min(33) as u8,
            }),
            (false, "BT") => Ok(Command::Tlc39 {
                orientation: self.opt_orientation(params, 0, span).ok()?,
                code39_module_width: self.dim_or(params, 1, 2, span).ok()?,
                code39_ratio: match param(params, 2) {
                    "" => 2.0,
                    s => self.parse_f32(s, span).ok()?.clamp(2.0, 3.0),
                },
                code39_height: self.dim_or(params, 3, 40, span).ok()?,
                micro_row_height: self.dim_or(params, 4, 4, span).ok()?,
                micro_module_width: self.dim_or(params, 5, 2, span).ok()?,
            }),
            (false, "BB") => Ok(Command::Codablock {
                orientation: self.opt_orientation(params, 0, span).ok()?,
                row_height: self.u32_or(params, 1, 8, span).ok()?,
                security: self.yes_no_or(params, 2, true, span).ok()?,
                columns: self.u32_or(params, 3, 0, span).ok()?,
                rows: self.u32_or(params, 4, 0, span).ok()?,
                mode: first_char(params, 5).map(|c| c.to_ascii_uppercase()).filter(|c| matches!(c, 'A' | 'E' | 'F')).unwrap_or('F'),
            }),
            (false, "B4") => Ok(Command::Code49 {
                orientation: self.opt_orientation(params, 0, span).ok()?,
                row_height: self.parse_optional_u32(params, 1, span).ok()?,
                interpretation: match param(params, 2).to_ascii_uppercase().as_str() {
                    "A" => Code49Interpretation::Above,
                    "B" => Code49Interpretation::Below,
                    _ => Code49Interpretation::None,
                },
                start_mode: first_char(params, 3).and_then(|c| c.to_digit(10)).map(|d| d as u8),
            }),
            (false, "B5") => Ok(Command::Postal {
                kind: PostalKind::Planet,
                orientation: self.opt_orientation(params, 0, span).ok()?,
                height: self.opt_dim(params, 1, span).ok()?,
                print_interpretation_line: self.yes_no_or(params, 2, false, span).ok()?,
                print_above: self.yes_no_or(params, 3, false, span).ok()?,
            }),
            (false, "BZ") => Ok(Command::Postal {
                kind: match param(params, 4) {
                    "1" => PostalKind::Planet,
                    "3" => PostalKind::UspsImb,
                    _ => PostalKind::Postnet,
                },
                orientation: self.opt_orientation(params, 0, span).ok()?,
                height: self.opt_dim(params, 1, span).ok()?,
                print_interpretation_line: self.yes_no_or(params, 2, false, span).ok()?,
                print_above: self.yes_no_or(params, 3, false, span).ok()?,
            }),
            (false, "MU") => {
                let units = first_char(params, 0).map(|c| c.to_ascii_uppercase());
                self.units.set(units.filter(|u| matches!(u, 'I' | 'M')).unwrap_or('D'));
                // Base and desired dpi must both be given, else they are ignored.
                let dpi = |i| param(params, i).parse::<u32>().ok().filter(|v| matches!(v, 150 | 200 | 300 | 600));
                let command = match (dpi(1), dpi(2)) {
                    (Some(base), Some(desired)) => {
                        self.conversion.set((base, desired));
                        Command::FormatConversion { base, desired }
                    }
                    _ => Command::Comment { text: String::new() },
                };
                self.update_unit_scale();
                Ok(command)
            }
            (false, "JM") => {
                let half = param(params, 0).eq_ignore_ascii_case("B");
                self.half_density.set(half);
                self.update_unit_scale();
                Ok(Command::DotsPerMm { half })
            }
            (false, "FC") => Ok(Command::FieldClock {
                primary: first_char(params, 0).unwrap_or('%'),
                secondary: first_char(params, 1),
                tertiary: first_char(params, 2),
            }),
            (false, "ST") => Ok(Command::SetClock {
                month: self.u32_or(params, 0, 1, span).ok()?,
                day: self.u32_or(params, 1, 1, span).ok()?,
                year: self.u32_or(params, 2, 2000, span).ok()?,
                hour: self.u32_or(params, 3, 0, span).ok()?,
                minute: self.u32_or(params, 4, 0, span).ok()?,
                second: self.u32_or(params, 5, 0, span).ok()?,
            }),
            (false, "SO") => {
                let n = |i| match param(params, i) {
                    "" => Some(0),
                    s => self.parse_i32(s, span).ok(),
                };
                Ok(Command::ClockOffset {
                    clock: if param(params, 0) == "3" { 3 } else { 2 },
                    months: n(1)?,
                    days: n(2)?,
                    years: n(3)?,
                    hours: n(4)?,
                    minutes: n(5)?,
                    seconds: n(6)?,
                })
            }
            (false, "PA") => {
                let on = |i| param(params, i) == "1";
                Ok(Command::AdvancedText {
                    layout: TextLayout { default_glyph: on(0), bidi: on(1), shaping: on(2), opentype: on(3) },
                })
            }
            (false, "FE") => Ok(Command::FieldEmbed { indicator: first_char(params, 0).unwrap_or('#') }),
            (false, "IL") => Ok(Command::ImageLoad { name: param(params, 0).to_string() }),
            (false, "IS") => Ok(Command::ImageSave {
                name: param(params, 0).to_string(),
                print: self.yes_no_or(params, 1, true, span).ok()?,
            }),
            (false, "CW") => Ok(Command::FontAlias {
                letter: first_char(params, 0)?.to_ascii_uppercase(),
                name: param(params, 1).to_string(),
            }),
            (true, "DU" | "DT") => self.parse_download_font(params, span),
            (false, "B7") => self.parse_pdf417(params, span),
            (false, "BO" | "B0") => self.parse_aztec(params, span),
            // ^BLo,h,g: LOGMARS is Code 39 with a mandatory check digit.
            (false, "BL") => Ok(Command::LinearBarcode {
                kind: LinearKind::Code39 { check_digit: true },
                orientation: self.opt_orientation(params, 0, span).ok()?,
                height: self.opt_dim(params, 1, span).ok()?,
                print_interpretation_line: true,
                print_above: self.yes_no_or(params, 2, false, span).ok()?,
            }),
            (false, name) => return self.parse_linear(name, params, span),
            _ => return None,
        })
    }

    fn parse_field_orientation(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^FWr,z
        Ok(Command::FieldOrientation {
            orientation: self.opt_orientation(params, 0, span)?.unwrap_or(Orientation::Normal),
            justification: self.parse_justification(param(params, 1), span)?,
        })
    }

    fn parse_field_parameter(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^FPd,g
        let direction = match param(params, 0).to_ascii_uppercase().as_str() {
            "V" => FieldDirection::Vertical,
            "R" => FieldDirection::Reverse,
            _ => FieldDirection::Horizontal,
        };
        Ok(Command::FieldParameter { direction, char_gap: self.u32_or(params, 1, 0, span)? })
    }

    fn parse_text_block(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^TBa,b,c
        Ok(Command::TextBlock {
            orientation: self.opt_orientation(params, 0, span)?,
            width: self.dim_or(params, 1, 0, span)?,
            height: self.dim_or(params, 2, 0, span)?,
        })
    }

    fn parse_graphic_ellipse(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^GEw,h,t,c
        let color = match param(params, 3) {
            "" => LineColor::Black,
            s => self.parse_line_color(s, span)?,
        };
        Ok(Command::GraphicEllipse {
            width: self.dim_or(params, 0, 3, span)?,
            height: self.dim_or(params, 1, 3, span)?,
            thickness: self.dim_or(params, 2, 1, span)?,
            color,
        })
    }

    fn parse_graphic_symbol(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^GSo,h,w
        Ok(Command::GraphicSymbol {
            orientation: self.opt_orientation(params, 0, span)?,
            height: self.opt_dim(params, 1, span)?,
            width: self.opt_dim(params, 2, span)?,
        })
    }

    /// Decodes ~DG / ~DY bitmap data: Z64/B64, or (compressed) ASCII hex.
    fn decode_graphic(&self, raw: &str, bytes_per_row: u32) -> Result<Vec<u8>, ParseError> {
        let raw = raw.trim();
        if raw.starts_with(":Z64:") || raw.starts_with(":B64:") {
            let compact: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
            self.decode_z64_data(&compact)
        } else {
            Ok(self.decode_hex_data(raw, bytes_per_row))
        }
    }

    fn parse_download_graphic(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ~DGd:o.x,t,w,data
        let bytes_per_row = self.u32_or(params, 2, 0, span)?;
        let raw = params.get(3..).map(|rest| rest.join(",")).unwrap_or_default();
        Ok(Command::DownloadGraphic {
            name: param(params, 0).to_string(),
            bytes_per_row,
            data: self.decode_graphic(&raw, bytes_per_row)?,
        })
    }

    /// ~DYd:f,b,x,t,w,data. Only GRF bitmaps (`x` = G) can be drawn; other objects (fonts,
    /// PNG, ...) are ignored.
    fn parse_download_object(&self, params: &[&str], span: Span) -> Option<Result<Command, ParseError>> {
        let format = param(params, 1).to_ascii_uppercase();
        let raw = || params.get(5..).map(|rest| rest.join(",")).unwrap_or_default();
        match param(params, 2).to_ascii_uppercase().as_str() {
            "G" => {}
            // TrueType (T), TrueType extension (E) and OpenType (O... firmware dependent).
            "T" | "E" | "O" => {
                let raw = raw();
                let data = if raw.trim_start().starts_with(':') {
                    match self.decode_graphic(&raw, 0) {
                        Ok(d) => d,
                        Err(e) => return Some(Err(e)),
                    }
                } else if format == "A" {
                    decode_hex(&raw)
                } else {
                    raw.into_bytes()
                };
                return Some(Ok(Command::DownloadFont { name: param(params, 0).to_string(), data }));
            }
            _ => return Some(Ok(Command::Comment { text: String::new() })),
        }
        let bytes_per_row = match self.u32_or(params, 4, 0, span) {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let raw = params.get(5..).map(|rest| rest.join(",")).unwrap_or_default();
        let data = if param(params, 1).eq_ignore_ascii_case("A") || raw.trim_start().starts_with(':') {
            self.decode_graphic(&raw, bytes_per_row)
        } else {
            Ok(raw.into_bytes())
        };
        Some(data.map(|data| Command::DownloadGraphic { name: param(params, 0).to_string(), bytes_per_row, data }))
    }

    /// ~DUd:o.x,s,data / ~DTd:o.x,s,data: the font file as ASCII hex.
    fn parse_download_font(&self, params: &[&str], _span: Span) -> Result<Command, ParseError> {
        Ok(Command::DownloadFont { name: param(params, 0).to_string(), data: decode_hex(&params.get(2..).map(|p| p.join(",")).unwrap_or_default()) })
    }

    fn parse_data_matrix(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^BXo,h,s,c,r,f,g,a
        Ok(Command::DataMatrix {
            orientation: self.opt_orientation(params, 0, span)?,
            module_height: self.parse_optional_u32(params, 1, span)?.filter(|&h| h > 0),
            columns: self.u32_or(params, 3, 0, span)?,
            rows: self.u32_or(params, 4, 0, span)?,
            // ECC 200 always honours the escape character (default `~`).
            escape: Some(first_char(params, 6).unwrap_or('~')),
            rectangular: param(params, 7) == "2",
        })
    }

    fn parse_pdf417(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^B7o,h,s,c,r,t
        Ok(Command::Pdf417 {
            orientation: self.opt_orientation(params, 0, span)?,
            row_height: self.parse_optional_u32(params, 1, span)?.filter(|&h| h > 0),
            security: self.parse_optional_u32(params, 2, span)?.map(|s| s.min(8) as u8),
            columns: self.u32_or(params, 3, 0, span)?,
            rows: self.u32_or(params, 4, 0, span)?,
            truncated: self.yes_no_or(params, 5, false, span)?,
        })
    }

    fn parse_aztec(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^BOa,b,c,d,e,f,g
        Ok(Command::Aztec {
            orientation: self.opt_orientation(params, 0, span)?,
            magnification: self.parse_optional_u32(params, 1, span)?,
            ecc: self.u32_or(params, 3, 0, span)?,
        })
    }

    /// The 1D barcodes. Most share `o,h,f,g` (orientation, height, interpretation line,
    /// line above); a few put a check-digit flag first (`o,e,h,f,g`) or last.
    fn parse_linear(&self, name: &str, params: &[&str], span: Span) -> Option<Result<Command, ParseError>> {
        // Parameter index of the height; f and g follow it.
        let check_first = matches!(name, "B1" | "BK" | "BM" | "BP");
        let h = if check_first { 2 } else { 1 };
        let check_last = param(params, h + 3);
        let check_first_param = param(params, 1);
        let yes = |s: &str, default: bool| match s {
            "" => Ok(default),
            s => self.parse_yes_no(s, span),
        };

        let kind = (|| -> Result<Option<LinearKind>, ParseError> {
            Ok(Some(match name {
                "B1" => LinearKind::Code11 { check_digits: if yes(check_first_param, false)? { 1 } else { 2 } },
                "B2" => LinearKind::Interleaved2of5 { check_digit: yes(check_last, false)? },
                "B8" => LinearKind::Ean8,
                "B9" => LinearKind::UpcE { print_check_digit: yes(check_last, true)? },
                "BA" => LinearKind::Code93 { print_check_digit: yes(check_last, false)? },
                "BE" => LinearKind::Ean13,
                "BI" => LinearKind::Industrial2of5,
                "BJ" => LinearKind::Standard2of5,
                "BK" => LinearKind::Codabar {
                    check_digit: yes(check_first_param, false)?,
                    start: first_char(params, 5).unwrap_or('A').to_ascii_uppercase(),
                    stop: first_char(params, 6).unwrap_or('A').to_ascii_uppercase(),
                },
                "BM" => LinearKind::Msi {
                    check: match check_first_param.to_ascii_uppercase().as_str() {
                        "A" => MsiCheck::None,
                        "C" => MsiCheck::Mod10Mod10,
                        "D" => MsiCheck::Mod11Mod10,
                        _ => MsiCheck::Mod10,
                    },
                    print_check_digit: yes(param(params, 5), false)?,
                },
                "BP" => LinearKind::Plessey { print_check_digit: yes(check_first_param, false)? },
                "BS" => LinearKind::UpcEanExtension,
                "BU" => LinearKind::UpcA { print_check_digit: yes(check_last, true)? },
                _ => return Ok(None),
            }))
        })();
        let kind = match kind {
            Ok(Some(kind)) => kind,
            Ok(None) => return None,
            Err(e) => return Some(Err(e)),
        };

        // ^BS prints its digits above the bars by default.
        let above_default = name == "BS";
        Some((|| {
            Ok(Command::LinearBarcode {
                kind,
                orientation: self.opt_orientation(params, 0, span)?,
                height: self.opt_dim(params, h, span)?,
                print_interpretation_line: yes(param(params, h + 1), true)?,
                print_above: yes(param(params, h + 2), above_default)?,
            })
        })())
    }
}

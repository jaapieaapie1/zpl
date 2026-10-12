//! Which part of the ZPL source produced each draw instruction, for editors.

use zpl_parser::command::Command;
use zpl_parser::Span;

use crate::{DrawInstruction, StateManager};

/// Where a draw instruction came from, as byte offsets into the ZPL source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Source {
    /// The field, from its ^FO/^FT (or first command after the previous field) through
    /// the command that drew it, usually ^FS.
    pub field: Span,
    /// The ^FD/^FV command with the field's data, for fields finished by ^FS.
    pub data: Option<Span>,
}

impl StateManager {
    /// Like [`StateManager::process`], plus the [`Source`] of every instruction. `spans`
    /// holds each command's span, as returned by `Parser::parse_with_spans`.
    ///
    /// Instructions drawn by recalled formats (^XF) or stored images (^IL) map to the
    /// command that recalled them.
    pub fn process_mapped(&mut self, commands: Vec<Command>, spans: &[Span]) -> (Vec<DrawInstruction>, Vec<Source>) {
        let mut out = Vec::new();
        let mut sources: Vec<Source> = Vec::new();
        let mut field_start: Option<usize> = None;
        let mut data: Option<Span> = None;
        // Index of the first instruction of the current field.
        let mut field_first = 0;

        for (command, &span) in commands.into_iter().zip(spans) {
            let separator = matches!(command, Command::FieldSeperator);
            let ends_field = separator || matches!(command, Command::StartFormat | Command::EndFormat);
            match command {
                Command::FieldOrigin { .. } | Command::FieldTypeset { .. } => {
                    field_start.get_or_insert(span.start);
                }
                Command::FielData { .. } => data = Some(span),
                _ => {}
            }

            self.process_command(command, &mut out);

            // ^IS can take instructions back out; everything new came from this command.
            sources.truncate(out.len());
            field_first = field_first.min(sources.len());
            let source = Source {
                field: Span { start: field_start.unwrap_or(span.start), end: span.end },
                data: if separator { data } else { None },
            };
            sources.resize(out.len(), source);

            if separator {
                // Graphics like ^GB draw before the ^FS; their field still ends there.
                for source in &mut sources[field_first..] {
                    source.field.end = span.end;
                }
            }
            if ends_field {
                field_start = None;
                data = None;
                field_first = sources.len();
            }
        }

        (out, sources)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zpl_parser::Parser;

    fn mapped(zpl: &str) -> Vec<(String, Option<String>)> {
        let (commands, spans) = Parser::new(zpl).parse_with_spans().unwrap();
        let (instructions, sources) = StateManager::new().process_mapped(commands, &spans);
        assert_eq!(instructions.len(), sources.len());
        sources
            .iter()
            .map(|s| (zpl[s.field.start..s.field.end].to_string(), s.data.map(|d| zpl[d.start..d.end].to_string())))
            .collect()
    }

    #[test]
    fn maps_fields_and_data() {
        let zpl = "^XA\n^CF0,30\n^FO50,50^FDHello^FS\n^FO10,10^GB100,100,3^FS\n  ^FT20,200^A0N,20,20^FDWorld^FS\n^XZ";
        assert_eq!(
            mapped(zpl),
            vec![
                ("^FO50,50^FDHello^FS".to_string(), Some("^FDHello".to_string())),
                ("^FO10,10^GB100,100,3^FS".to_string(), None),
                ("^FT20,200^A0N,20,20^FDWorld^FS".to_string(), Some("^FDWorld".to_string())),
            ]
        );
    }

    #[test]
    fn recalled_format_maps_to_recall() {
        let zpl = "^XA^DFR:L.ZPL^FS^FO10,10^FN1^FS^XZ^XA^XFR:L.ZPL^FN1^FDhi^FS^XZ";
        let sources = mapped(zpl);
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].0, "^XZ");
    }
}

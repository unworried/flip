use alloc::string::String;
use core::cmp;

use crate::source::Source;

use super::Diagnostic;

pub struct DiagnosticsDisplay<'a> {
    text: &'a Source, // May need more info => SourceCode struct
    diagnostics: &'a [Diagnostic],
}

const MESSAGE_PADDING: usize = 16;
const COLOR_RED: &str = "\x1b[31m\x1b[1m";
const COLOR_ORANGE: &str = "\x1b[33m\x1b[1m";
const COLOR_RESET: &str = "\x1b[0m";

impl<'a> DiagnosticsDisplay<'a> {
    pub fn new(text: &'a Source, diagnostics: &'a [Diagnostic]) -> Self {
        Self { text, diagnostics }
    }

    /// Formats diagnostic in desired format for user presentation
    pub fn stringify(&self, diagnostic: &Diagnostic) -> String {
        let line_index = self.text.line_index(diagnostic.span.start);
        let line = self.text.line(line_index);
        let line_start = self.text.line_start(line_index);

        let column = diagnostic.span.start - line_start;

        let prefix_start = cmp::max(0, column as isize - MESSAGE_PADDING as isize) as usize;
        let prefix_end = column;

        let prefix = &line[prefix_start..prefix_end];

        let suffix_start = cmp::min(column + diagnostic.span.length(), line.len());
        let suffix_end = cmp::min(suffix_start + MESSAGE_PADDING, line.len());

        let span = &line[prefix_end..suffix_start];

        let suffix = &line[suffix_start..suffix_end];

        let indent = cmp::min(MESSAGE_PADDING, column);

        let indicators = format!(
            "{:indent$}{}",
            "",
            "^".repeat(diagnostic.span.length()),
            indent = indent
        );

        let pointer = format!("{:indent$}|", "", indent = indent);
        let message = format!(
            "{:indent$}+-- {}{} ({}{}{}:{}{}{}){}",
            "",
            COLOR_ORANGE,
            diagnostic.message,
            COLOR_RESET,
            line_index + 1,
            COLOR_ORANGE,
            COLOR_RESET,
            column + 1,
            COLOR_ORANGE,
            COLOR_RESET,
            indent = indent
        );

        format!(
            "{}{}{}{}{}\n{}\n{}\n{}\n",
            prefix, COLOR_RED, span, COLOR_RESET, suffix, indicators, pointer, message
        )
    }

    pub fn print(&self) {
        for diagnostic in self.diagnostics {
            println!("{}", self.stringify(diagnostic));
        }
    }
}

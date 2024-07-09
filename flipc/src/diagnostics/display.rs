//! diagnostics/display.rs - Module for displaying diagnostics to the user. The display module
//! takes the source code and the diagnostics and formats them for user presentation.
use alloc::string::String;
use core::cmp;

use super::Diagnostic;
use crate::error::Result;
use crate::escape_codes::Color;
use crate::source::Source;

pub struct DiagnosticsDisplay<'a> {
    text: &'a Source, // May need more info => SourceCode struct
    diagnostics: &'a [Diagnostic],
}

const MESSAGE_PADDING: usize = 16;

impl<'a> DiagnosticsDisplay<'a> {
    pub fn new(text: &'a Source, diagnostics: &'a [Diagnostic]) -> Self {
        Self { text, diagnostics }
    }

    /// Formats diagnostic in desired format for user presentation
    pub fn stringify(&self, diagnostic: &Diagnostic) -> Result<String> {
        let line_index = self.text.line_index(diagnostic.span.start);
        let line = self.text.line(line_index)?;
        let line_start = self.text.line_start(line_index);

        let column = diagnostic.span.start - line_start;

        let prefix_start = cmp::max(0, column as isize - MESSAGE_PADDING as isize) as usize;
        let prefix_end = column;

        //println!("{}, {}", prefix_start, prefix_end);
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
            Color::Orange,
            diagnostic.message,
            Color::Reset,
            line_index + 1,
            Color::Orange,
            Color::Reset,
            column + 1,
            Color::Orange,
            Color::Reset,
            indent = indent
        );

        Ok(format!(
            "{}{}{}{}{}\n{}\n{}\n{}\n",
            prefix,
            Color::Red,
            span,
            Color::Reset,
            suffix,
            indicators,
            pointer,
            message
        ))
    }

    pub fn print(&self) -> Result<()> {
        for diagnostic in self.diagnostics {
            eprintln!("{}", self.stringify(diagnostic)?);
        }

        Ok(())
    }
}

//! diagnostics/display.rs - Module for displaying diagnostics to the user. The display module
//! takes the source code and the diagnostics and formats them for user presentation.
use alloc::string::String;
use core::cmp;

use super::Diagnostic;
use crate::diagnostics::DiagnosticKind;
use crate::error::Result;
use crate::escape_codes::Color;
use crate::source::Source;
use crate::span::Span;

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
    pub fn stringify(&self, message: &str, span: &Span) -> Result<String> {
        let line_index = self.text.line_index(span.start);
        let line = self.text.line(line_index)?;
        let line_start = self.text.line_start(line_index);

        let column = span.start - line_start;

        /*let line_start = self.text[..diagnostic.span.start]
          .rfind('\n')
          .map_or(0, |pos| pos + 1);
        let line_end = self.text[diagnostic.span.start..]
        .find('\n')
        .map_or(self.text.len(), |pos| pos + diagnostic.span.start);*/

        //let prefix_start = cmp::max(0, column as isize - MESSAGE_PADDING as isize) as usize;
        //let prefix_end = column;

        //println!("{}, {}", prefix_start, prefix_end);
        //let prefix = &line[prefix_start..prefix_end];

        let line_length = line.len();
        let prefix_start = cmp::min(
            cmp::max(0, column as isize - MESSAGE_PADDING as isize) as usize,
            line_length,
        );
        let prefix_end = cmp::min(column, line_length);

        // TODO: Review this, possibly revert back to previous code - commit <= a980151
        // Done to highlight issues that may arise from the new code / WIP changes
        assert!(prefix_start <= prefix_end);
        let prefix = &line[prefix_start..prefix_end];

        let suffix_start = cmp::min(column + span.length(), line.len());
        let suffix_end = cmp::min(suffix_start + MESSAGE_PADDING, line.len());

        let line_span = &line[prefix_end..suffix_start];

        let suffix = &line[suffix_start..suffix_end];

        let indent = cmp::min(MESSAGE_PADDING, column);

        let indicators = format!(
            "{:indent$}{}",
            "",
            "^".repeat(span.length()),
            indent = indent
        );

        let pointer = format!("{:indent$}|", "", indent = indent);
        let message = format!(
            "{:indent$}+-- {}{} ({}{}{}:{}{}{}){}",
            "",
            Color::Orange,
            message,
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
            line_span,
            Color::Reset,
            suffix,
            indicators,
            pointer,
            message
        ))
    }

    pub fn print(&self) -> Result<()> {
        let mut program_diagnostics = Vec::new();

        for diagnostic in self.diagnostics {
            if diagnostic.span.is_none() {
                program_diagnostics.push(diagnostic);
                continue;
            }

            eprintln!(
                "{}",
                self.stringify(&diagnostic.message, &diagnostic.span.expect("unreachable"))?
            );
        }

        eprintln!();
        for diagnostic in program_diagnostics {
            eprintln!(
                "{}[{}]{}: {}",
                Color::Red,
                diagnostic.kind,
                Color::Reset,
                diagnostic.message
            );
        }
        eprintln!();

        Ok(())
    }
}

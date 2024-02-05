use alloc::vec::Vec;

#[derive(Debug, PartialEq, Clone, Default, PartialOrd)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn combine(mut spans: Vec<&Span>) -> Span {
        spans.sort_by(|a, b| a.start.cmp(&b.start));

        // TODO: Deal with errors
        let start = spans.first().unwrap().start;
        let end = spans.last().unwrap().end;

        Span::new(start, end)
    }

    pub fn length(&self) -> usize {
        (self.end - self.start) + 1
    }
}

use super::Expression;
use crate::parser::Parse;

pub struct Print {
    expression: Expression,
}

impl<'a> Parse<'a> for Print {
    fn parse(parser: &mut crate::parser::Parser<'a>) -> Self {
        let expression = Expression::parse(parser);

        Self {
            expression,
        }
    }
}

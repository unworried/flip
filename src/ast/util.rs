#[macro_export]
macro_rules! print_stmt {
    ($expression:expr) => {
        $crate::ast::Statement::Print($crate::ast::statement::Print {
            expression: $expression,
        })
    };
}

#[macro_export]
macro_rules! if_stmt {
    ($condition:expr, $resolution:expr) => {
        $crate::ast::Statement::If($crate::ast::statement::If {
            condition: $condition,
            resolution: $resolution,
        })
    };
}

#[macro_export]
macro_rules! while_stmt {
    ($condition:expr, $resolution:expr) => {
        $crate::ast::Statement::While($crate::ast::statement::While {
            condition: $condition,
            resolution: $resolution,
        })
    };
}

#[macro_export]
macro_rules! literal {
    ($kind:tt => $value:expr) => {
        $crate::ast::Expression::Literal($crate::ast::expression::Literal::$kind(
            $value.to_string(),
        ))
    };
}

#[macro_export]
macro_rules! string_literal {
    ($value:expr) => {
        $crate::literal!(String => $value)
    };
}

#[macro_export]
macro_rules! primitive {
    ($kind:tt => $value:expr) => {
        $crate::ast::Expression::Primitive($crate::ast::expression::Primitive::$kind($value))
    };
}

#[macro_export]
macro_rules! int_primitive {
    ($value:expr) => {
        $crate::primitive!(Int => $value)
    };
}

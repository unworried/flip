#[macro_export]
macro_rules! print_stmt {
    ($expression:expr) => {
        $crate::ast::Stmt::Print($crate::ast::statement::Print {
            expression: $expression,
        })
    };
}

#[macro_export]
macro_rules! if_stmt {
    ($condition:expr, $resolution:expr) => {
        $crate::ast::Stmt::If($crate::ast::statement::If {
            condition: $condition,
            resolution: $resolution,
        })
    };
}

#[macro_export]
macro_rules! while_stmt {
    ($condition:expr, $resolution:expr) => {
        $crate::ast::Stmt::While($crate::ast::statement::While {
            condition: $condition,
            resolution: $resolution,
        })
    };
}

#[macro_export]
macro_rules! label_stmt {
    ($ident:expr) => {
        $crate::ast::Stmt::Label($crate::ast::statement::Label { ident: $ident })
    };
}

#[macro_export]
macro_rules! goto_stmt {
    ($ident:expr) => {
        $crate::ast::Stmt::Goto($crate::ast::statement::Goto { ident: $ident })
    };
}

#[macro_export]
macro_rules! let_stmt {
    ($ident:expr, $expression:expr) => {
        $crate::ast::Stmt::Let($crate::ast::statement::Let {
            ident: $ident,
            expression: $expression,
        })
    };
}

#[macro_export]
macro_rules! literal {
    ($kind:tt => $value:expr) => {
        $crate::ast::Expr::Literal($crate::ast::expression::Literal::$kind($value.to_string()))
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
        $crate::ast::Expr::Primitive($crate::ast::expression::Primitive::$kind($value))
    };
}

#[macro_export]
macro_rules! int_primitive {
    ($value:expr) => {
        $crate::primitive!(Int => $value)
    };
}

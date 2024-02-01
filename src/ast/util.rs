#[macro_export]
macro_rules! stmt {
    ($statement:expr) => {
        $crate::ast::Item {
            kind: $crate::ast::ItemKind::Statement($statement),
        }
    };
}

#[macro_export]
macro_rules! print_stmt {
    ($expression:expr) => {
        $crate::stmt!($crate::ast::Stmt {
            kind: $crate::ast::StmtKind::Print($expression),
        })
    };
}

#[macro_export]
macro_rules! if_stmt {
    ($condition:expr, $resolution:expr) => {
        $crate::stmt!($crate::ast::Stmt {
            kind: $crate::ast::StmtKind::If($condition, $resolution),
        })
    };
}

#[macro_export]
macro_rules! while_stmt {
    ($condition:expr, $resolution:expr) => {
        $crate::stmt!($crate::ast::Stmt {
            kind: $crate::ast::StmtKind::While($condition, $resolution),
        })
    };
}

#[macro_export]
macro_rules! let_stmt {
    ($ident:expr, $expression:expr) => {
        $crate::stmt!($crate::ast::Stmt {
            kind: $crate::ast::StmtKind::Let($ident, $expression),
        })
    };
}

#[macro_export]
macro_rules! literal {
    ($kind:tt => $value:expr) => {
        $crate::ast::Expr {
            kind: $crate::ast::ExprKind::Literal($crate::ast::expression::Literal::$kind($value)),
        }
    };
}

#[macro_export]
macro_rules! string_literal {
    ($value:expr) => {
        $crate::literal!(String => $value.to_string())
    };
}

#[macro_export]
macro_rules! int_literal {
    ($value:expr) => {
        $crate::literal!(Integer => $value)
    };
}

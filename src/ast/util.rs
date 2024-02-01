#[macro_export]
macro_rules! print_stmt {
    ($expression:expr) => {
        $crate::ast::Stmt {
            kind: $crate::ast::StmtKind::Print($expression),
        }
    };
}

#[macro_export]
macro_rules! if_stmt {
    ($condition:expr, $resolution:expr) => {
        $crate::ast::Stmt {
            kind: $crate::ast::StmtKind::If($condition, $resolution),
        }
    };
}

#[macro_export]
macro_rules! while_stmt {
    ($condition:expr, $resolution:expr) => {
        $crate::ast::Stmt {
            kind: $crate::ast::StmtKind::While($condition, $resolution),
        }
    };
}

#[macro_export]
macro_rules! label_stmt {
    ($label:expr) => {
        $crate::ast::Stmt {
            kind: $crate::ast::StmtKind::Label($label),
        }
    };
}

#[macro_export]
macro_rules! goto_stmt {
    ($label:expr) => {
        $crate::ast::Stmt {
            kind: $crate::ast::StmtKind::Goto($label),
        }
    };
}

#[macro_export]
macro_rules! let_stmt {
    ($ident:expr, $expression:expr) => {
        $crate::ast::Stmt {
            kind: $crate::ast::StmtKind::Let($ident, $expression),
        }
    };
}

#[macro_export]
macro_rules! input_stmt {
    ($ident:expr) => {
        $crate::ast::Stmt {
            kind: $crate::ast::StmtKind::Input($ident),
        }
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

use super::{
    Assignment, Ast, Binary, Call, Definition, Function, Ident, If, Literal, Program, Sequence,
    Unary, Variable, While,
};

pub trait Walkable {
    fn walk<V: Visitor>(&self, visitor: &mut V);
}

pub trait Visitor: Sized {
    fn visit_program(&mut self, program: &Program) {
        program.functions.iter().for_each(|func| func.walk(self));
    }

    fn visit_function(&mut self, program: &Function) {
        program.body.walk(self);
    }

    fn visit_ast(&mut self, ast: &Ast) {
        ast.walk(self);
    }

    fn visit_sequence(&mut self, seq: &Sequence) {
        seq.expressions.iter().for_each(|stmt| stmt.walk(self));
    }

    fn visit_binary(&mut self, bin: &Binary) {
        bin.left.walk(self);
        bin.right.walk(self);
    }

    fn visit_unary(&mut self, un: &Unary) {
        un.operand.walk(self);
    }

    fn visit_literal(&mut self, _lit: &Literal) {}

    fn visit_definition(&mut self, def: &Definition) {
        def.pattern.name.walk(self);
        def.value.walk(self);
    }

    fn visit_assignment(&mut self, def: &Assignment) {
        def.pattern.name.walk(self);
        def.value.walk(self);
    }

    fn visit_if(&mut self, if_expr: &If) {
        if_expr.condition.walk(self);
        if_expr.then.walk(self);
    }

    fn visit_while(&mut self, while_expr: &While) {
        while_expr.condition.walk(self);
        while_expr.then.walk(self);
    }

    fn visit_variable(&mut self, _var: &Variable) {}

    fn visit_call(&mut self, call: &Call) {
        call.arguments.iter().for_each(|arg| arg.walk(self));
    }

    fn visit_return(&mut self, ret: &Ast) {
        ret.walk(self);
    }
}

impl Walkable for Function {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_function(self);
    }
}

impl Walkable for Ast {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self {
            Ast::Sequence(seq) => visitor.visit_sequence(seq),
            Ast::Definition(def) => visitor.visit_definition(def),
            Ast::Assignment(def) => visitor.visit_assignment(def),
            Ast::If(if_expr) => visitor.visit_if(if_expr),
            Ast::While(while_expr) => visitor.visit_while(while_expr),
            Ast::Literal(lit) => visitor.visit_literal(lit),
            Ast::Binary(bin) => visitor.visit_binary(bin),
            Ast::Unary(un) => visitor.visit_unary(un),
            Ast::Variable(var) => visitor.visit_variable(var),
            Ast::Call(call) => visitor.visit_call(call),
            Ast::Return(ret) => visitor.visit_return(ret),
            Ast::Error => {}
        }
    }
}

impl Walkable for Ident {
    fn walk<V: Visitor>(&self, _visitor: &mut V) {
        // Do nothing
    }
}

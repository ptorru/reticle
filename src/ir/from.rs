use crate::ir::ast::*;

impl From<ExprTup> for Expr {
    fn from(tup: ExprTup) -> Self {
        Expr::Tup(tup)
    }
}

impl From<ExprTerm> for Expr {
    fn from(term: ExprTerm) -> Self {
        Expr::Term(term)
    }
}

impl From<InstrCall> for Instr {
    fn from(instr: InstrCall) -> Self {
        Instr::Call(instr)
    }
}

impl From<InstrWire> for Instr {
    fn from(instr: InstrWire) -> Self {
        Instr::Wire(instr)
    }
}

impl From<InstrComp> for Instr {
    fn from(instr: InstrComp) -> Self {
        Instr::Comp(instr)
    }
}

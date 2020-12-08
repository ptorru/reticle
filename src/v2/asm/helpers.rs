use crate::v2::asm::ast::*;

impl Loc {
    pub fn prim(&self) -> &Prim {
        &self.prim
    }
    pub fn x(&self) -> &ExprCoord {
        &self.x
    }
    pub fn y(&self) -> &ExprCoord {
        &self.y
    }
}

impl InstrAsm {
    pub fn dst(&self) -> &Expr {
        &self.dst
    }
    pub fn op(&self) -> &AsmOp {
        &self.op
    }
    pub fn attr(&self) -> &Expr {
        &self.attr
    }
    pub fn arg(&self) -> &Expr {
        &self.arg
    }
    pub fn loc(&self) -> &Loc {
        &self.loc
    }
}

impl Prog {
    pub fn sig(&self) -> &Sig {
        &self.sig
    }
    pub fn body(&self) -> &Vec<Instr> {
        &self.body
    }
}

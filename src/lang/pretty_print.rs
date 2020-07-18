use crate::lang::ast::*;
use crate::util::pretty_print::{PrettyPrint, PRETTY_INDENT};
use pretty::RcDoc;

impl PrettyPrint for Ty {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Ty::Bool => RcDoc::text("bool"),
            Ty::UInt(width) => RcDoc::text("u").append(RcDoc::as_string(width)),
            Ty::SInt(width) => RcDoc::text("i").append(RcDoc::as_string(width)),
            Ty::Vector(dtype, len) => dtype
                .to_doc()
                .append(RcDoc::text("<"))
                .append(RcDoc::as_string(len))
                .append(RcDoc::text(">")),
        }
    }
}

impl PrettyPrint for Loc {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Loc::Var => RcDoc::text("??"),
            Loc::Lut => RcDoc::text("lut"),
            Loc::Lum => RcDoc::text("lum"),
            Loc::Dsp => RcDoc::text("dsp"),
            Loc::Ram => RcDoc::text("ram"),
        }
    }
}

impl PrettyPrint for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Ref(n, _) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for StdOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            _ => panic!("WIP"),
        }
    }
}

impl PrettyPrint for PlacedOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            PlacedOp::Reg => RcDoc::text("reg"),
            PlacedOp::Add => RcDoc::text("add"),
            PlacedOp::Sub => RcDoc::text("sub"),
            PlacedOp::Mul => RcDoc::text("mul"),
            _ => panic!("WIP"),
        }
    }
}

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Instr::Placed {
                id,
                ty,
                op,
                attrs,
                params,
                loc,
            } => {
                let out_doc = RcDoc::as_string(id)
                    .append(RcDoc::text(":"))
                    .append(RcDoc::space())
                    .append(ty.to_doc())
                    .append(RcDoc::space())
                    .append(RcDoc::text("="))
                    .append(RcDoc::space());
                let attrs_doc = match attrs.is_empty() {
                    true => RcDoc::nil(),
                    false => RcDoc::text("[")
                        .append(RcDoc::intersperse(
                            attrs.iter().map(|a| a.to_doc()),
                            RcDoc::text(",").append(RcDoc::space()),
                        ))
                        .append(RcDoc::text("]")),
                };
                let params_doc = match params.is_empty() {
                    true => panic!("Error: must have at least one param"),
                    false => RcDoc::text("(")
                        .append(RcDoc::intersperse(
                            params.iter().map(|p| p.to_doc()),
                            RcDoc::text(",").append(RcDoc::space()),
                        ))
                        .append(RcDoc::text(")")),
                };
                let loc_doc = RcDoc::text("@").append(loc.to_doc());
                out_doc
                    .append(op.to_doc())
                    .append(attrs_doc)
                    .append(params_doc)
                    .append(RcDoc::space())
                    .append(loc_doc)
            }
            _ => panic!("WIP"),
        }
    }
}

impl PrettyPrint for Port {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Port::Input { id, ty } => RcDoc::as_string(id)
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(ty.to_doc()),
            Port::Output { id, ty } => RcDoc::as_string(id)
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(ty.to_doc()),
        }
    }
}

impl PrettyPrint for Def {
    fn to_doc(&self) -> RcDoc<()> {
        let inputs_doc = RcDoc::intersperse(
            self.sig.inputs().iter().map(|i| i.to_doc()),
            RcDoc::text(",").append(RcDoc::space()),
        );
        let outputs_doc = RcDoc::intersperse(
            self.sig.outputs().iter().map(|o| o.to_doc()),
            RcDoc::text(",").append(RcDoc::space()),
        );
        let mut body_doc = RcDoc::nil();
        for decl in self.body().iter() {
            body_doc = body_doc
                .append(RcDoc::hardline())
                .append(decl.to_doc())
                .append(RcDoc::text(";"));
        }
        body_doc = body_doc.nest(PRETTY_INDENT).group();
        RcDoc::text("def")
            .append(RcDoc::space())
            .append(RcDoc::as_string(self.id()))
            .append(RcDoc::text("("))
            .append(inputs_doc)
            .append(RcDoc::text(")"))
            .append(RcDoc::space())
            .append(RcDoc::text("->"))
            .append(RcDoc::space())
            .append(RcDoc::text("("))
            .append(outputs_doc)
            .append(RcDoc::text(")"))
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(body_doc)
            .append(RcDoc::hardline())
            .append(RcDoc::text("}"))
    }
}

impl PrettyPrint for Prog {
    fn to_doc(&self) -> RcDoc<()> {
        let mut defs_doc = RcDoc::nil();
        for d in self.defs().iter() {
            defs_doc = defs_doc.append(d.to_doc());
        }
        defs_doc
    }
}

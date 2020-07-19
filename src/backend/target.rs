use crate::backend::asm::ast as asm;
use crate::passes::select::instr as sel;
use serde::{Deserialize, Serialize};
use serde_json;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Expr {
    Input(String, String),
    BinOp(String, Rc<Expr>, Rc<Expr>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Instr {
    pub name: String,
    pub cost: u32,
    pub loc: String,
    pub ty: String,
    pub output: String,
    pub expr: Expr,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Def {
    pub instr: Vec<Instr>,
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub asm: asm::Instr,
    pub pattern: sel::Pattern,
}

#[derive(Clone, Debug)]
pub struct Target {
    pub tiles: Vec<Tile>,
}

impl Expr {
    pub fn to_instr_mut(&self, instr: &mut Vec<sel::Instr>, op_ty: sel::Ty, op_loc: sel::Loc) {
        match self {
            Expr::Input(ty, loc) => {
                let op = sel::Op::In;
                let ty = sel::Ty::from_str(ty).unwrap();
                let inp_loc = sel::Loc::from_str(loc).unwrap();
                instr.push(sel::Instr::new(op, ty, inp_loc));
            }
            Expr::BinOp(op, lhs, rhs) => {
                let op = sel::Op::from_str(op).unwrap();
                instr.push(sel::Instr::new(op, op_ty.clone(), op_loc.clone()));
                lhs.to_instr_mut(instr, op_ty.clone(), op_loc.clone());
                rhs.to_instr_mut(instr, op_ty.clone(), op_loc.clone());
            }
        }
    }
}

impl Instr {
    fn to_asm_instr(&self) -> asm::Instr {
        let ty = sel::Ty::from_str(&self.ty).unwrap();
        let loc = sel::Loc::from_str(&self.loc).unwrap();
        let asm_loc = asm::Loc {
            ty: loc,
            x: asm::LocExpr::Hole,
            y: asm::LocExpr::Hole,
        };
        asm::Instr {
            ty: ty,
            op: self.name.to_string(),
            loc: asm_loc,
            dst: None,
            params: Vec::new(),
        }
    }
    fn to_pattern(&self) -> sel::Pattern {
        let ty = sel::Ty::from_str(&self.ty).unwrap();
        let loc = sel::Loc::from_str(&self.loc).unwrap();
        let mut instr: Vec<sel::Instr> = Vec::new();
        self.expr.to_instr_mut(&mut instr, ty, loc);
        sel::Pattern {
            name: self.name.to_string(),
            cost: self.cost.clone(),
            instr: instr.to_vec(),
        }
    }
}

impl From<Instr> for Tile {
    fn from(instr: Instr) -> Self {
        Tile {
            asm: instr.to_asm_instr(),
            pattern: instr.to_pattern(),
        }
    }
}

impl From<Def> for Target {
    fn from(def: Def) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        for instr in def.instr.iter() {
            tiles.push(Tile::from(instr.clone()));
        }
        Target {
            tiles: tiles.to_vec(),
        }
    }
}

impl FromStr for Target {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let def: Def = serde_json::from_str(input).expect("Error: parsing json");
        Ok(Target::from(def))
    }
}

pub mod carry;
pub mod dsp;
pub mod errors;
pub mod fdre;
pub mod fdse;
pub mod gnd;
pub mod loc;
pub mod lut;
pub mod port;
pub mod vcc;

use crate::errors::Error;
use crate::gnd::Gnd;
use crate::port::Output;
use crate::vcc::Vcc;
use bline::{input_try_from_sig, wire_try_from_expr};
use std::collections::HashSet;
use std::convert::TryInto;
use verilog::ast as vl;
use xir::ast as xir;

pub const CLOCK: &str = "clock";
pub const RESET: &str = "reset";

fn vec_decl_try_from_instr_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Decl>, Error> {
    Ok(wire_try_from_expr(instr.dst())?)
}

fn vec_decl_try_from_instr_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Decl>, Error> {
    Ok(wire_try_from_expr(instr.dst())?)
}

fn vec_decl_try_from_instr(instr: &xir::Instr) -> Result<Vec<vl::Decl>, Error> {
    match instr {
        xir::Instr::Basc(instr) => Ok(vec_decl_try_from_instr_basc(instr)?),
        xir::Instr::Mach(instr) => Ok(vec_decl_try_from_instr_mach(instr)?),
    }
}

fn tmp_name_try_from_term(term: &xir::ExprTerm) -> Result<xir::Id, Error> {
    let dst: xir::Id = term.clone().try_into()?;
    Ok(format!("_{}", dst))
}

pub fn try_from_xir_prog(prog: &xir::Prog) -> Result<vl::Module, Error> {
    let id = prog.sig().id();
    let mut module = vl::Module::new(&id);
    let input = input_try_from_sig(prog.sig())?;
    for i in input {
        module.add_port(i.clone());
    }
    let mut decl: Vec<vl::Decl> = Vec::new();
    let dsp_outputs = Output::dsp();
    for i in prog.body() {
        let d: Vec<vl::Decl> = vec_decl_try_from_instr(i)?;
        decl.extend(d);
        if let Some(instr) = i.mach() {
            if instr.op().is_dsp() {
                let term = instr.dst().get_term(0)?;
                let name = tmp_name_try_from_term(term)?;
                if let Some(width) = dsp_outputs.get_width("P") {
                    decl.push(vl::Decl::new_wire(&name, u64::from(*width)));
                }
            }
        }
    }
    let decl_set: HashSet<vl::Decl> = decl.into_iter().collect();
    let output: Vec<vl::Decl> = wire_try_from_expr(prog.sig().output())?;
    let output_set: HashSet<vl::Decl> = output.into_iter().collect();
    let gnd = Gnd::default();
    let vcc = Vcc::default();
    module.add_decl(vl::Decl::new_wire(&gnd.name(), 1));
    module.add_decl(vl::Decl::new_wire(&vcc.name(), 1));
    for d in decl_set.difference(&output_set) {
        module.add_decl(d.clone());
    }
    module.add_stmt(gnd.to_stmt());
    module.add_stmt(vcc.to_stmt());
    Ok(module)
}

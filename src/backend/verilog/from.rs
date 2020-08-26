use crate::backend::asm::ast::*;
use vast::v05::ast as Verilog;

fn to_verilog_port(port: Port) -> Vec<Verilog::Port> {
    let mut ports: Vec<Verilog::Port> = Vec::new();
    match port {
        Port::Input { id, ty } => {
            if ty.is_vector() {
                for i in 0..ty.length() {
                    let name = format!("{}_{}", id, i);
                    let port = Verilog::Port::new_input(&name, ty.width());
                    ports.push(port);
                }
            } else {
                let port = Verilog::Port::new_input(&id, ty.width());
                ports.push(port);
            }
        }
        Port::Output { id, ty } => {
            if ty.is_vector() {
                for i in 0..ty.length() {
                    let name = format!("{}_{}", id, i);
                    let port = Verilog::Port::new_output(&name, ty.width());
                    ports.push(port);
                }
            } else {
                let port = Verilog::Port::new_output(&id, ty.width());
                ports.push(port);
            }
        }
    }
    ports
}

impl From<Prog> for Verilog::Module {
    fn from(prog: Prog) -> Self {
        let mut ports: Vec<Verilog::Port> = Vec::new();
        for input in prog.inputs().iter() {
            ports.extend(to_verilog_port(input.clone()));
        }
        for output in prog.outputs().iter() {
            ports.extend(to_verilog_port(output.clone()));
        }
        let mut module = Verilog::Module::new(&prog.id());
        for port in ports.iter() {
            module.add_port(port.clone());
        }
        module
    }
}

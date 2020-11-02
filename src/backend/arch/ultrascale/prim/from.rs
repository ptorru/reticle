use crate::backend::arch::ultrascale::prim::ast::*;
use crate::backend::verilog;

fn lut_width(ty: LutTy) -> u32 {
    match ty {
        LutTy::Lut1 => 2,
        LutTy::Lut2 => 4,
        LutTy::Lut3 => 8,
        LutTy::Lut4 => 16,
        LutTy::Lut5 => 32,
        LutTy::Lut6 => 64,
    }
}

fn convert_literal(vcc: &Expr, gnd: &Expr, width: i64, value: i64) -> verilog::Expr {
    if width == 1 {
        let mask = value & 1;
        let is_one = mask == 1;
        if is_one {
            verilog::Expr::from(vcc.clone())
        } else {
            verilog::Expr::from(gnd.clone())
        }
    } else {
        let mut concat = verilog::ExprConcat::default();
        for i in 0..width {
            let shift = value >> i;
            let mask = shift & 1;
            let is_one = mask == 1;
            if is_one {
                concat.add_expr(verilog::Expr::from(vcc.clone()));
            } else {
                concat.add_expr(verilog::Expr::from(gnd.clone()));
            }
        }
        verilog::Expr::from(concat)
    }
}

impl From<Expr> for verilog::Expr {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Ref(name) => verilog::Expr::new_ref(&name),
            Expr::Index(name, index) => verilog::Expr::new_index_bit(&name, index as i32),
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Lut> for verilog::Stmt {
    fn from(lut: Lut) -> Self {
        let init = lut.get_attr("init");
        let a = lut.input("a");
        let y = lut.output("y");
        let mut inst = verilog::Instance::new(&lut.id(), &lut.ty().to_string());
        let width = lut_width(lut.ty().clone());
        inst.add_param("INIT", verilog::Expr::new_ulit_hex(width, &init));
        inst.connect("I0", verilog::Expr::from(a.clone()));
        match lut.ty() {
            LutTy::Lut2 => {
                let b = lut.input("b");
                inst.connect("I1", verilog::Expr::from(b.clone()));
            }
            LutTy::Lut3 => {
                let b = lut.input("b");
                let c = lut.input("c");
                inst.connect("I1", verilog::Expr::from(b.clone()));
                inst.connect("I2", verilog::Expr::from(c.clone()));
            }
            LutTy::Lut4 => {
                let b = lut.input("b");
                let c = lut.input("c");
                let d = lut.input("d");
                inst.connect("I1", verilog::Expr::from(b.clone()));
                inst.connect("I2", verilog::Expr::from(c.clone()));
                inst.connect("I3", verilog::Expr::from(d.clone()));
            }
            LutTy::Lut5 => {
                let b = lut.input("b");
                let c = lut.input("c");
                let d = lut.input("d");
                let e = lut.input("e");
                inst.connect("I1", verilog::Expr::from(b.clone()));
                inst.connect("I2", verilog::Expr::from(c.clone()));
                inst.connect("I3", verilog::Expr::from(d.clone()));
                inst.connect("I4", verilog::Expr::from(e.clone()));
            }
            LutTy::Lut6 => {
                let b = lut.input("b");
                let c = lut.input("c");
                let d = lut.input("d");
                let e = lut.input("e");
                let f = lut.input("f");
                inst.connect("I1", verilog::Expr::from(b.clone()));
                inst.connect("I2", verilog::Expr::from(c.clone()));
                inst.connect("I3", verilog::Expr::from(d.clone()));
                inst.connect("I4", verilog::Expr::from(e.clone()));
                inst.connect("I5", verilog::Expr::from(f.clone()));
            }
            _ => (),
        }
        inst.connect("O", verilog::Expr::from(y.clone()));
        verilog::Stmt::from(inst)
    }
}

impl From<Reg> for verilog::Stmt {
    fn from(reg: Reg) -> Self {
        let a = reg.input("a");
        let clock = reg.input("clock");
        let reset = reg.input("reset");
        let en = reg.input("en");
        let output = reg.output("y");
        let mut inst = verilog::Instance::new(&reg.id(), &reg.ty().to_string());
        inst.add_param("IS_C_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_D_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        if reg.is_fdre() {
            inst.add_param("INIT", verilog::Expr::new_ulit_bin(1, "0"));
            inst.add_param("IS_R_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        } else {
            inst.add_param("INIT", verilog::Expr::new_ulit_bin(1, "1"));
            inst.add_param("IS_S_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        }
        inst.connect("C", verilog::Expr::from(clock.clone()));
        inst.connect("CE", verilog::Expr::from(en.clone()));
        inst.connect("D", verilog::Expr::from(a.clone()));
        inst.connect("Q", verilog::Expr::from(output.clone()));
        if reg.is_fdre() {
            inst.connect("R", verilog::Expr::from(reset.clone()));
        } else {
            inst.connect("S", verilog::Expr::from(reset.clone()));
        }
        verilog::Stmt::from(inst)
    }
}

impl From<DspLoc> for verilog::Attribute {
    fn from(loc: DspLoc) -> Self {
        let mut attr = verilog::Attribute::default();
        let loc = format!("DSP48E2_X{}Y{}", loc.x(), loc.y());
        attr.add_stmt("LOC", &loc);
        attr
    }
}

impl From<DspVector> for verilog::Stmt {
    fn from(dsp: DspVector) -> Self {
        let mut inst = verilog::Instance::new(&dsp.id(), "DSP48E2");
        if let Some(loc) = dsp.loc() {
            inst.set_attr(verilog::Attribute::from(loc.clone()));
        }
        let gnd = dsp.input("gnd");
        let vcc = dsp.input("vcc");
        let clock = dsp.input("clock");
        let reset = dsp.input("reset");
        let a = dsp.input("a");
        let b = dsp.input("b");
        let y = dsp.output("y");
        // clock
        inst.connect("CLK", verilog::Expr::from(clock.clone()));
        // resets
        inst.connect("RSTA", verilog::Expr::from(reset.clone()));
        inst.connect("RSTALLCARRYIN", verilog::Expr::from(reset.clone()));
        inst.connect("RSTALUMODE", verilog::Expr::from(reset.clone()));
        inst.connect("RSTB", verilog::Expr::from(reset.clone()));
        inst.connect("RSTC", verilog::Expr::from(reset.clone()));
        inst.connect("RSTCTRL", verilog::Expr::from(reset.clone()));
        inst.connect("RSTD", verilog::Expr::from(reset.clone()));
        inst.connect("RSTINMODE", verilog::Expr::from(reset.clone()));
        inst.connect("RSTM", verilog::Expr::from(reset.clone()));
        inst.connect("RSTP", verilog::Expr::from(reset.clone()));
        // operands
        inst.connect(
            "A",
            verilog::Expr::new_slice(
                &b.id(),
                verilog::Expr::new_int(47),
                verilog::Expr::new_int(18),
            ),
        );
        inst.connect(
            "B",
            verilog::Expr::new_slice(
                &b.id(),
                verilog::Expr::new_int(17),
                verilog::Expr::new_int(0),
            ),
        );
        inst.connect("C", verilog::Expr::from(a.clone()));
        inst.connect("P", verilog::Expr::from(y.clone()));
        // derive attributes
        match dsp.op() {
            DspVectorOp::Add => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("NONE"));
                inst.connect("ALUMODE", convert_literal(&vcc, &gnd, 4, 0));
                inst.connect("INMODE", convert_literal(&vcc, &gnd, 5, 0));
                inst.connect("OPMODE", convert_literal(&vcc, &gnd, 9, 51));
            }
            DspVectorOp::Sub => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("NONE"));
                inst.connect("ALUMODE", convert_literal(&vcc, &gnd, 4, 3));
                inst.connect("INMODE", convert_literal(&vcc, &gnd, 5, 0));
                inst.connect("OPMODE", convert_literal(&vcc, &gnd, 9, 51));
            }
            DspVectorOp::And => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("NONE"));
                inst.connect("ALUMODE", convert_literal(&vcc, &gnd, 4, 12));
                inst.connect("INMODE", convert_literal(&vcc, &gnd, 5, 0));
                inst.connect("OPMODE", convert_literal(&vcc, &gnd, 9, 51));
            }
            DspVectorOp::Or => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("NONE"));
                inst.connect("ALUMODE", convert_literal(&vcc, &gnd, 4, 12));
                inst.connect("INMODE", convert_literal(&vcc, &gnd, 5, 0));
                inst.connect("OPMODE", convert_literal(&vcc, &gnd, 9, 59));
            }
            DspVectorOp::Xor => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("NONE"));
                inst.connect("ALUMODE", convert_literal(&vcc, &gnd, 4, 4));
                inst.connect("INMODE", convert_literal(&vcc, &gnd, 5, 0));
                inst.connect("OPMODE", convert_literal(&vcc, &gnd, 9, 51));
            }
            DspVectorOp::Nand => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("NONE"));
                inst.connect("ALUMODE", convert_literal(&vcc, &gnd, 4, 14));
                inst.connect("INMODE", convert_literal(&vcc, &gnd, 5, 0));
                inst.connect("OPMODE", convert_literal(&vcc, &gnd, 9, 51));
            }
            DspVectorOp::Nor => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("NONE"));
                inst.connect("ALUMODE", convert_literal(&vcc, &gnd, 4, 14));
                inst.connect("INMODE", convert_literal(&vcc, &gnd, 5, 0));
                inst.connect("OPMODE", convert_literal(&vcc, &gnd, 9, 59));
            }
            DspVectorOp::Xnor => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("NONE"));
                inst.connect("ALUMODE", convert_literal(&vcc, &gnd, 4, 5));
                inst.connect("INMODE", convert_literal(&vcc, &gnd, 5, 0));
                inst.connect("OPMODE", convert_literal(&vcc, &gnd, 9, 51));
            }
        }
        match dsp.get_param("length") {
            1 => inst.add_param("USE_SIMD", verilog::Expr::new_str("ONE48")),
            2 => inst.add_param("USE_SIMD", verilog::Expr::new_str("TWO24")),
            3 => inst.add_param("USE_SIMD", verilog::Expr::new_str("FOUR12")),
            4 => inst.add_param("USE_SIMD", verilog::Expr::new_str("FOUR12")),
            _ => unimplemented!(),
        }
        // derive registers
        if dsp.has_reg("a") {
            let na = dsp.reg("a") as i32;
            assert_eq!(na, 1, "Error: more than one register");
            let en_a = dsp.input("en_a");
            inst.add_param("CREG", verilog::Expr::new_int(na));
            inst.connect("CEC", verilog::Expr::from(en_a.clone()));
        } else {
            inst.add_param("CREG", verilog::Expr::new_int(0));
            inst.connect("CEC", convert_literal(&vcc, &gnd, 1, 0));
        }
        if dsp.has_reg("b") {
            let nb = dsp.reg("b") as i32;
            let en_b = dsp.input("en_b");
            assert!(nb >= 0, "Error: negative number of registers");
            assert!(nb < 3, "Error: more than three registers");
            inst.add_param("AREG", verilog::Expr::new_int(nb));
            inst.add_param("BREG", verilog::Expr::new_int(nb));
            inst.add_param("ACASCREG", verilog::Expr::new_int(nb));
            inst.add_param("BCASCREG", verilog::Expr::new_int(nb));
            inst.connect("CEA1", verilog::Expr::from(en_b.clone()));
            inst.connect("CEB1", verilog::Expr::from(en_b.clone()));
            inst.connect("CEA2", verilog::Expr::from(en_b.clone()));
            inst.connect("CEB2", verilog::Expr::from(en_b.clone()));
        } else {
            inst.add_param("AREG", verilog::Expr::new_int(0));
            inst.add_param("BREG", verilog::Expr::new_int(0));
            inst.add_param("ACASCREG", verilog::Expr::new_int(0));
            inst.add_param("BCASCREG", verilog::Expr::new_int(0));
            inst.connect("CEA1", convert_literal(&vcc, &gnd, 1, 0));
            inst.connect("CEA2", convert_literal(&vcc, &gnd, 1, 0));
            inst.connect("CEB1", convert_literal(&vcc, &gnd, 1, 0));
            inst.connect("CEB2", convert_literal(&vcc, &gnd, 1, 0));
        }
        if dsp.has_reg("y") {
            let ny = dsp.reg("y") as i32;
            let en_y = dsp.input("en_y");
            assert_eq!(ny, 1, "Error: more than one register");
            inst.add_param("PREG", verilog::Expr::new_int(ny));
            inst.connect("CEP", verilog::Expr::from(en_y.clone()));
        } else {
            inst.add_param("PREG", verilog::Expr::new_int(0));
            inst.connect("CEP", convert_literal(&vcc, &gnd, 1, 0));
        }
        // default params
        inst.add_param("A_INPUT", verilog::Expr::new_str("DIRECT"));
        inst.add_param("AMULTSEL", verilog::Expr::new_str("A"));
        inst.add_param("B_INPUT", verilog::Expr::new_str("DIRECT"));
        inst.add_param("BMULTSEL", verilog::Expr::new_str("B"));
        inst.add_param("PREADDINSEL", verilog::Expr::new_str("A"));
        inst.add_param("RND", verilog::Expr::new_ulit_hex(48, "0"));
        inst.add_param("USE_WIDEXOR", verilog::Expr::new_str("FALSE"));
        inst.add_param("XORSIMD", verilog::Expr::new_str("XOR24_48_96"));
        inst.add_param("AUTORESET_PATDET", verilog::Expr::new_str("NO_RESET"));
        inst.add_param("AUTORESET_PRIORITY", verilog::Expr::new_str("RESET"));
        inst.add_param("MASK", verilog::Expr::new_ulit_hex(48, "3fffffffffff"));
        inst.add_param("PATTERN", verilog::Expr::new_ulit_hex(48, "0"));
        inst.add_param("SEL_MASK", verilog::Expr::new_str("MASK"));
        inst.add_param("SEL_PATTERN", verilog::Expr::new_str("PATTERN"));
        inst.add_param("USE_PATTERN_DETECT", verilog::Expr::new_str("NO_PATDET"));
        inst.add_param(
            "IS_ALUMODE_INVERTED",
            verilog::Expr::new_ulit_bin(4, "0000"),
        );
        inst.add_param("IS_CARRYIN_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_CLK_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param(
            "IS_INMODE_INVERTED",
            verilog::Expr::new_ulit_bin(5, "00000"),
        );
        inst.add_param(
            "IS_OPMODE_INVERTED",
            verilog::Expr::new_ulit_bin(9, "000000000"),
        );
        inst.add_param(
            "IS_RSTALLCARRYIN_INVERTED",
            verilog::Expr::new_ulit_bin(1, "0"),
        );
        inst.add_param(
            "IS_RSTALUMODE_INVERTED",
            verilog::Expr::new_ulit_bin(1, "0"),
        );
        inst.add_param("IS_RSTA_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTB_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTCTRL_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTC_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTD_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTINMODE_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTM_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTP_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        // default clock enable
        inst.connect("CEAD", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CEALUMODE", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CECARRYIN", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CECTRL", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CED", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CEINMODE", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CEM", convert_literal(&vcc, &gnd, 1, 0));
        // default registers
        inst.add_param("ADREG", verilog::Expr::new_int(0));
        inst.add_param("ALUMODEREG", verilog::Expr::new_int(0));
        inst.add_param("CARRYINREG", verilog::Expr::new_int(0));
        inst.add_param("CARRYINSELREG", verilog::Expr::new_int(0));
        inst.add_param("DREG", verilog::Expr::new_int(0));
        inst.add_param("INMODEREG", verilog::Expr::new_int(0));
        inst.add_param("OPMODEREG", verilog::Expr::new_int(0));
        inst.add_param("MREG", verilog::Expr::new_int(0));
        // default input values
        inst.connect("ACIN", convert_literal(&vcc, &gnd, 30, 0));
        inst.connect("BCIN", convert_literal(&vcc, &gnd, 18, 0));
        inst.connect("CARRYCASCIN", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("MULTSIGNIN", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("PCIN", convert_literal(&vcc, &gnd, 48, 0));
        inst.connect("CARRYIN", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CARRYINSEL", convert_literal(&vcc, &gnd, 3, 0));
        inst.connect("D", convert_literal(&vcc, &gnd, 27, 0));
        // unused outputs
        inst.connect("ACOUT", verilog::Expr::from(Expr::default()));
        inst.connect("BCOUT", verilog::Expr::from(Expr::default()));
        inst.connect("CARRYCASCOUT", verilog::Expr::from(Expr::default()));
        inst.connect("MULTSIGNOUT", verilog::Expr::from(Expr::default()));
        inst.connect("PCOUT", verilog::Expr::from(Expr::default()));
        inst.connect("OVERFLOW", verilog::Expr::from(Expr::default()));
        inst.connect("PATTERNBDETECT", verilog::Expr::from(Expr::default()));
        inst.connect("PATTERNDETECT", verilog::Expr::from(Expr::default()));
        inst.connect("UNDERFLOW", verilog::Expr::from(Expr::default()));
        inst.connect("CARRYOUT", verilog::Expr::from(Expr::default()));
        inst.connect("XOROUT", verilog::Expr::from(Expr::default()));
        verilog::Stmt::from(inst)
    }
}

impl From<DspFused> for verilog::Stmt {
    fn from(dsp: DspFused) -> Self {
        let mut inst = verilog::Instance::new(&dsp.id(), "DSP48E2");
        let gnd = dsp.input("gnd");
        let vcc = dsp.input("vcc");
        let clock = dsp.input("clock");
        let reset = dsp.input("reset");
        let a = dsp.input("a");
        let b = dsp.input("b");
        let y = dsp.output("y");
        // clock
        inst.connect("CLK", verilog::Expr::from(clock.clone()));
        // resets
        inst.connect("RSTA", verilog::Expr::from(reset.clone()));
        inst.connect("RSTALLCARRYIN", verilog::Expr::from(reset.clone()));
        inst.connect("RSTALUMODE", verilog::Expr::from(reset.clone()));
        inst.connect("RSTB", verilog::Expr::from(reset.clone()));
        inst.connect("RSTC", verilog::Expr::from(reset.clone()));
        inst.connect("RSTCTRL", verilog::Expr::from(reset.clone()));
        inst.connect("RSTD", verilog::Expr::from(reset.clone()));
        inst.connect("RSTINMODE", verilog::Expr::from(reset.clone()));
        inst.connect("RSTM", verilog::Expr::from(reset.clone()));
        inst.connect("RSTP", verilog::Expr::from(reset.clone()));
        // operands
        inst.connect("A", verilog::Expr::from(a.clone()));
        inst.connect("B", verilog::Expr::from(b.clone()));
        inst.connect("P", verilog::Expr::from(y.clone()));
        // derive attributes
        match dsp.op() {
            DspFusedOp::Mul => {
                inst.add_param("USE_MULT", verilog::Expr::new_str("MULTIPLY"));
                inst.connect("ALUMODE", convert_literal(&vcc, &gnd, 4, 0));
                inst.connect("INMODE", convert_literal(&vcc, &gnd, 5, 0));
                inst.connect("OPMODE", convert_literal(&vcc, &gnd, 9, 5));
                inst.connect("C", convert_literal(&vcc, &gnd, 48, 0));
            }
            DspFusedOp::MulAdd => {
                let c = dsp.input("c");
                inst.add_param("USE_MULT", verilog::Expr::new_str("MULTIPLY"));
                inst.connect("ALUMODE", convert_literal(&vcc, &gnd, 4, 0));
                inst.connect("INMODE", convert_literal(&vcc, &gnd, 5, 0));
                inst.connect("OPMODE", convert_literal(&vcc, &gnd, 9, 53));
                inst.connect("C", verilog::Expr::from(c.clone()));
            }
        }
        // derive registers
        if dsp.has_reg("a") {
            let na = dsp.reg("a") as i32;
            let en_a = dsp.input("en_a");
            inst.add_param("AREG", verilog::Expr::new_int(na));
            inst.add_param("ACASCREG", verilog::Expr::new_int(na));
            inst.connect("CEA1", verilog::Expr::from(en_a.clone()));
            inst.connect("CEA2", verilog::Expr::from(en_a.clone()));
        } else {
            inst.add_param("AREG", verilog::Expr::new_int(0));
            inst.add_param("ACASCREG", verilog::Expr::new_int(0));
            inst.connect("CEA1", convert_literal(&vcc, &gnd, 1, 0));
            inst.connect("CEA2", convert_literal(&vcc, &gnd, 1, 0));
        }
        if dsp.has_reg("b") {
            let nb = dsp.reg("b") as i32;
            let en_b = dsp.input("en_b");
            inst.add_param("BREG", verilog::Expr::new_int(nb));
            inst.add_param("BCASCREG", verilog::Expr::new_int(nb));
            inst.connect("CEB1", verilog::Expr::from(en_b.clone()));
            inst.connect("CEB2", verilog::Expr::from(en_b.clone()));
        } else {
            inst.add_param("BREG", verilog::Expr::new_int(0));
            inst.add_param("BCASCREG", verilog::Expr::new_int(0));
            inst.connect("CEB1", convert_literal(&vcc, &gnd, 1, 0));
            inst.connect("CEB2", convert_literal(&vcc, &gnd, 1, 0));
        }
        if dsp.has_reg("c") {
            let nc = dsp.reg("c") as i32;
            let en_c = dsp.input("en_c");
            inst.add_param("CREG", verilog::Expr::new_int(nc));
            inst.connect("CEC", verilog::Expr::from(en_c.clone()));
        } else {
            inst.add_param("CREG", verilog::Expr::new_int(0));
            inst.connect("CEC", convert_literal(&vcc, &gnd, 1, 0));
        }
        if dsp.has_reg("mul") {
            let nmul = dsp.reg("mul") as i32;
            let en_mul = dsp.input("en_mul");
            assert_eq!(nmul, 1, "Error: more than one register");
            inst.add_param("MREG", verilog::Expr::new_int(nmul));
            inst.connect("CEM", verilog::Expr::from(en_mul.clone()));
        } else {
            inst.add_param("MREG", verilog::Expr::new_int(0));
            inst.connect("CEM", convert_literal(&vcc, &gnd, 1, 0));
        }
        if dsp.has_reg("y") {
            let en_y = dsp.input("en_y");
            inst.add_param("PREG", verilog::Expr::new_int(1));
            inst.connect("CEP", verilog::Expr::from(en_y.clone()));
        } else {
            inst.add_param("PREG", verilog::Expr::new_int(0));
            inst.connect("CEP", convert_literal(&vcc, &gnd, 1, 0));
        }
        // default params
        inst.add_param("USE_SIMD", verilog::Expr::new_str("ONE48"));
        inst.add_param("A_INPUT", verilog::Expr::new_str("DIRECT"));
        inst.add_param("AMULTSEL", verilog::Expr::new_str("A"));
        inst.add_param("B_INPUT", verilog::Expr::new_str("DIRECT"));
        inst.add_param("BMULTSEL", verilog::Expr::new_str("B"));
        inst.add_param("PREADDINSEL", verilog::Expr::new_str("A"));
        inst.add_param("RND", verilog::Expr::new_ulit_hex(48, "0"));
        inst.add_param("USE_WIDEXOR", verilog::Expr::new_str("FALSE"));
        inst.add_param("XORSIMD", verilog::Expr::new_str("XOR24_48_96"));
        inst.add_param("AUTORESET_PATDET", verilog::Expr::new_str("NO_RESET"));
        inst.add_param("AUTORESET_PRIORITY", verilog::Expr::new_str("RESET"));
        inst.add_param("MASK", verilog::Expr::new_ulit_hex(48, "3fffffffffff"));
        inst.add_param("PATTERN", verilog::Expr::new_ulit_hex(48, "0"));
        inst.add_param("SEL_MASK", verilog::Expr::new_str("MASK"));
        inst.add_param("SEL_PATTERN", verilog::Expr::new_str("PATTERN"));
        inst.add_param("USE_PATTERN_DETECT", verilog::Expr::new_str("NO_PATDET"));
        inst.add_param(
            "IS_ALUMODE_INVERTED",
            verilog::Expr::new_ulit_bin(4, "0000"),
        );
        inst.add_param("IS_CARRYIN_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_CLK_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param(
            "IS_INMODE_INVERTED",
            verilog::Expr::new_ulit_bin(5, "00000"),
        );
        inst.add_param(
            "IS_OPMODE_INVERTED",
            verilog::Expr::new_ulit_bin(9, "000000000"),
        );
        inst.add_param(
            "IS_RSTALLCARRYIN_INVERTED",
            verilog::Expr::new_ulit_bin(1, "0"),
        );
        inst.add_param(
            "IS_RSTALUMODE_INVERTED",
            verilog::Expr::new_ulit_bin(1, "0"),
        );
        inst.add_param("IS_RSTA_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTB_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTCTRL_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTC_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTD_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTINMODE_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTM_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        inst.add_param("IS_RSTP_INVERTED", verilog::Expr::new_ulit_bin(1, "0"));
        // default clock enable
        inst.connect("CEAD", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CEALUMODE", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CECARRYIN", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CECTRL", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CED", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CEINMODE", convert_literal(&vcc, &gnd, 1, 0));
        // default registers
        inst.add_param("ADREG", verilog::Expr::new_int(0));
        inst.add_param("ALUMODEREG", verilog::Expr::new_int(0));
        inst.add_param("CARRYINREG", verilog::Expr::new_int(0));
        inst.add_param("CARRYINSELREG", verilog::Expr::new_int(0));
        inst.add_param("DREG", verilog::Expr::new_int(0));
        inst.add_param("INMODEREG", verilog::Expr::new_int(0));
        inst.add_param("OPMODEREG", verilog::Expr::new_int(0));
        // default input values
        inst.connect("ACIN", convert_literal(&vcc, &gnd, 30, 0));
        inst.connect("BCIN", convert_literal(&vcc, &gnd, 18, 0));
        inst.connect("CARRYCASCIN", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("MULTSIGNIN", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("PCIN", convert_literal(&vcc, &gnd, 48, 0));
        inst.connect("CARRYIN", convert_literal(&vcc, &gnd, 1, 0));
        inst.connect("CARRYINSEL", convert_literal(&vcc, &gnd, 3, 0));
        inst.connect("D", convert_literal(&vcc, &gnd, 27, 0));
        // unused outputs
        inst.connect("ACOUT", verilog::Expr::from(Expr::default()));
        inst.connect("BCOUT", verilog::Expr::from(Expr::default()));
        inst.connect("CARRYCASCOUT", verilog::Expr::from(Expr::default()));
        inst.connect("MULTSIGNOUT", verilog::Expr::from(Expr::default()));
        inst.connect("PCOUT", verilog::Expr::from(Expr::default()));
        inst.connect("OVERFLOW", verilog::Expr::from(Expr::default()));
        inst.connect("PATTERNBDETECT", verilog::Expr::from(Expr::default()));
        inst.connect("PATTERNDETECT", verilog::Expr::from(Expr::default()));
        inst.connect("UNDERFLOW", verilog::Expr::from(Expr::default()));
        inst.connect("CARRYOUT", verilog::Expr::from(Expr::default()));
        inst.connect("XOROUT", verilog::Expr::from(Expr::default()));
        verilog::Stmt::from(inst)
    }
}

impl From<Vcc> for verilog::Stmt {
    fn from(vcc: Vcc) -> Self {
        let y = vcc.output("y");
        let mut inst = verilog::Instance::new(&vcc.id(), "VCC");
        inst.connect("P", verilog::Expr::from(y.clone()));
        verilog::Stmt::from(inst)
    }
}

impl From<Gnd> for verilog::Stmt {
    fn from(gnd: Gnd) -> Self {
        let y = gnd.output("y");
        let mut inst = verilog::Instance::new(&gnd.id(), "GND");
        inst.connect("G", verilog::Expr::from(y.clone()));
        verilog::Stmt::from(inst)
    }
}

impl From<Const> for verilog::Stmt {
    fn from(constant: Const) -> Self {
        let gnd = constant.input("gnd");
        let vcc = constant.input("vcc");
        let width = constant.get_param("width");
        let value = constant.get_param("value");
        let expr = convert_literal(&vcc, &gnd, width, value);
        let out = verilog::Expr::new_ref(&constant.id());
        let assign = verilog::Parallel::Assign(out, expr);
        verilog::Stmt::from(assign)
    }
}

impl From<Carry> for verilog::Stmt {
    fn from(carry: Carry) -> Self {
        let gnd = carry.input("gnd");
        let a = carry.input("a");
        let b = carry.input("b");
        let ci = carry.input("ci");
        let y = carry.output("y");
        let mut inst = verilog::Instance::new(&carry.id(), "CARRY8");
        inst.add_param("CARRY_TYPE", verilog::Expr::new_str("SINGLE_CY8"));
        inst.connect("DI", verilog::Expr::from(a.clone()));
        inst.connect("S", verilog::Expr::from(b.clone()));
        inst.connect("O", verilog::Expr::from(y.clone()));
        inst.connect("CI", verilog::Expr::from(ci.clone()));
        inst.connect("CI_TOP", verilog::Expr::from(gnd.clone()));
        // unused output
        inst.connect("CO", verilog::Expr::from(Expr::default()));
        verilog::Stmt::from(inst)
    }
}

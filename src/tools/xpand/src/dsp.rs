use verilog::ast as vl;

#[derive(Clone, Debug)]
pub enum InputTy {
    Direct,
    Cascade,
}

#[derive(Clone, Debug)]
pub enum AMultSel {
    A,
    AD,
}

#[derive(Clone, Debug)]
pub enum BMultSel {
    B,
    AD,
}

#[derive(Clone, Debug)]
pub enum PreAddInSel {
    A,
    B,
}

#[derive(Clone, Debug)]
pub enum UseMult {
    Multiply,
    Dynamic,
    None,
}

#[derive(Clone, Debug)]
pub enum UseSimd {
    One,
    Two,
    Four,
}

#[derive(Clone, Debug)]
pub enum XorSimd {
    One,
    Two,
}

#[derive(Clone, Debug)]
pub enum AutoResetPatDet {
    NoReset,
    ResetMatch,
    ResetNotMatch,
}

#[derive(Clone, Debug)]
pub enum AutoResetPriority {
    Reset,
    Cep,
}

#[derive(Clone, Debug)]
pub enum SelMask {
    C,
    Mask,
    RoundModeOne,
    RoundModeTwo,
}

#[derive(Clone, Debug)]
pub enum SelPattern {
    C,
    Pattern,
}

#[derive(Clone, Debug)]
pub enum UsePatternDetect {
    NoPatDet,
    PatDet,
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub a_input: InputTy,
    pub b_input: InputTy,
    pub a_multsel: AMultSel,
    pub b_multsel: BMultSel,
    pub preaddinsel: PreAddInSel,
    pub rnd: u64,
    pub use_mult: UseMult,
    pub use_simd: UseSimd,
    pub use_widexor: bool,
    pub xorsimd: XorSimd,
    pub autoreset_patdet: AutoResetPatDet,
    pub autoreset_priority: AutoResetPriority,
    pub mask: u64,
    pub pattern: u64,
    pub sel_mask: SelMask,
    pub sel_pattern: SelPattern,
    pub use_pattern_detect: UsePatternDetect,
    pub is_alumode_inverted: u64,
    pub is_carryin_inverted: bool,
    pub is_clk_inverted: bool,
    pub is_inmode_inverted: u64,
    pub is_opmode_inverted: u64,
    pub is_rstallcarryin_inverted: bool,
    pub is_rstalumode_inverted: bool,
}

#[derive(Clone, Debug)]
pub struct Dsp {
    pub name: String,
    pub prim: String,
    pub attr: Attr,
}

impl Default for InputTy {
    fn default() -> Self {
        InputTy::Direct
    }
}

impl Default for AMultSel {
    fn default() -> Self {
        AMultSel::A
    }
}

impl Default for BMultSel {
    fn default() -> Self {
        BMultSel::B
    }
}

impl Default for PreAddInSel {
    fn default() -> Self {
        PreAddInSel::A
    }
}

impl Default for UseMult {
    fn default() -> Self {
        UseMult::Multiply
    }
}

impl Default for UseSimd {
    fn default() -> Self {
        UseSimd::One
    }
}

impl Default for XorSimd {
    fn default() -> Self {
        XorSimd::Two
    }
}

impl Default for AutoResetPatDet {
    fn default() -> Self {
        AutoResetPatDet::NoReset
    }
}

impl Default for AutoResetPriority {
    fn default() -> Self {
        AutoResetPriority::Reset
    }
}

impl Default for SelMask {
    fn default() -> Self {
        SelMask::Mask
    }
}

impl Default for SelPattern {
    fn default() -> Self {
        SelPattern::Pattern
    }
}

impl Default for UsePatternDetect {
    fn default() -> Self {
        UsePatternDetect::NoPatDet
    }
}

impl Default for Attr {
    fn default() -> Self {
        Attr {
            a_input: InputTy::default(),
            b_input: InputTy::default(),
            a_multsel: AMultSel::default(),
            b_multsel: BMultSel::default(),
            preaddinsel: PreAddInSel::default(),
            rnd: 0,
            use_mult: UseMult::default(),
            use_simd: UseSimd::default(),
            use_widexor: false,
            xorsimd: XorSimd::default(),
            autoreset_patdet: AutoResetPatDet::default(),
            autoreset_priority: AutoResetPriority::default(),
            mask: u64::from_str_radix("3fffffffffff", 16).unwrap(),
            pattern: 0,
            sel_mask: SelMask::default(),
            sel_pattern: SelPattern::default(),
            use_pattern_detect: UsePatternDetect::default(),
            is_alumode_inverted: 0,
            is_carryin_inverted: false,
            is_clk_inverted: false,
            is_inmode_inverted: 0,
            is_opmode_inverted: 0,
            is_rstallcarryin_inverted: false,
            is_rstalumode_inverted: false,
        }
    }
}

impl Default for Dsp {
    fn default() -> Self {
        Dsp {
            name: String::new(),
            prim: "DSP48E2".to_string(),
            attr: Attr::default(),
        }
    }
}

impl InputTy {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            InputTy::Direct => vl::Expr::new_str("DIRECT"),
            InputTy::Cascade => vl::Expr::new_str("CASCADE"),
        }
    }
}

impl AMultSel {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            AMultSel::A => vl::Expr::new_str("A"),
            AMultSel::AD => vl::Expr::new_str("AD"),
        }
    }
}

impl BMultSel {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            BMultSel::B => vl::Expr::new_str("B"),
            BMultSel::AD => vl::Expr::new_str("AD"),
        }
    }
}

impl PreAddInSel {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            PreAddInSel::A => vl::Expr::new_str("A"),
            PreAddInSel::B => vl::Expr::new_str("B"),
        }
    }
}

impl UseMult {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            UseMult::Multiply => vl::Expr::new_str("MULTIPLY"),
            UseMult::Dynamic => vl::Expr::new_str("DYNAMIC"),
            UseMult::None => vl::Expr::new_str("NONE"),
        }
    }
}

impl UseSimd {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            UseSimd::One => vl::Expr::new_str("ONE48"),
            UseSimd::Two => vl::Expr::new_str("TWO24"),
            UseSimd::Four => vl::Expr::new_str("FOUR12"),
        }
    }
}

impl XorSimd {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            XorSimd::One => vl::Expr::new_str("XOR12"),
            XorSimd::Two => vl::Expr::new_str("XOR24_48_96"),
        }
    }
}

impl AutoResetPatDet {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            AutoResetPatDet::NoReset => vl::Expr::new_str("NO_RESET"),
            AutoResetPatDet::ResetMatch => vl::Expr::new_str("RESET_MATCH"),
            AutoResetPatDet::ResetNotMatch => vl::Expr::new_str("RESET_NOT_MATCH"),
        }
    }
}

impl AutoResetPriority {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            AutoResetPriority::Reset => vl::Expr::new_str("RESET"),
            AutoResetPriority::Cep => vl::Expr::new_str("CEP"),
        }
    }
}

impl SelMask {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            SelMask::C => vl::Expr::new_str("C"),
            SelMask::Mask => vl::Expr::new_str("MASK"),
            SelMask::RoundModeOne => vl::Expr::new_str("ROUNDING_MODE1"),
            SelMask::RoundModeTwo => vl::Expr::new_str("ROUNDING_MODE2"),
        }
    }
}

impl SelPattern {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            SelPattern::C => vl::Expr::new_str("C"),
            SelPattern::Pattern => vl::Expr::new_str("PATTERN"),
        }
    }
}

impl UsePatternDetect {
    pub fn to_expr(&self) -> vl::Expr {
        match self {
            UsePatternDetect::NoPatDet => vl::Expr::new_str("NO_PATDET"),
            UsePatternDetect::PatDet => vl::Expr::new_str("PATDET"),
        }
    }
}

impl Dsp {
    pub fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.name, &self.prim);
        inst.add_param("A_INPUT", self.attr.a_input.to_expr());
        inst.add_param("B_INPUT", self.attr.b_input.to_expr());
        inst.add_param("AMULTSEL", self.attr.a_multsel.to_expr());
        inst.add_param("BMULTSEL", self.attr.b_multsel.to_expr());
        inst.add_param("PREADDINSEL", self.attr.preaddinsel.to_expr());
        inst.add_param(
            "RND",
            vl::Expr::new_ulit_hex(48, &format!("{:x}", self.attr.rnd)),
        );
        inst.add_param("USE_MULT", self.attr.use_mult.to_expr());
        inst.add_param("USE_SIMD", self.attr.use_simd.to_expr());
        inst.add_param(
            "USE_WIDEXOR",
            vl::Expr::new_str(&format!("{}", self.attr.use_widexor).to_uppercase()),
        );
        inst.add_param("XORSIMD", self.attr.xorsimd.to_expr());
        inst.add_param("AUTORESET_PATDET", self.attr.autoreset_patdet.to_expr());
        inst.add_param("AUTORESET_PRIORITY", self.attr.autoreset_priority.to_expr());
        inst.add_param(
            "MASK",
            vl::Expr::new_ulit_hex(48, &format!("{:x}", self.attr.mask)),
        );
        inst.add_param(
            "PATTERN",
            vl::Expr::new_ulit_hex(48, &format!("{:x}", self.attr.pattern)),
        );
        inst.add_param("SEL_MASK", self.attr.sel_mask.to_expr());
        inst.add_param("SEL_PATTERN", self.attr.sel_pattern.to_expr());
        inst.add_param("USE_PATTERN_DETECT", self.attr.use_pattern_detect.to_expr());
        inst.add_param(
            "IS_ALUMODE_INVERTED",
            vl::Expr::new_ulit_hex(4, &format!("{:x}", self.attr.is_alumode_inverted)),
        );
        inst.add_param(
            "IS_CARRYIN_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_carryin_inverted))),
        );
        inst.add_param(
            "IS_CLK_INVERTED",
            vl::Expr::new_ulit_bin(1, &format!("{}", u64::from(self.attr.is_clk_inverted))),
        );
        inst.add_param(
            "IS_INMODE_INVERTED",
            vl::Expr::new_ulit_hex(5, &format!("{:x}", self.attr.is_inmode_inverted)),
        );
        inst.add_param(
            "IS_OPMODE_INVERTED",
            vl::Expr::new_ulit_hex(9, &format!("{:x}", self.attr.is_opmode_inverted)),
        );
        inst.add_param(
            "IS_RSTALLCARRYIN_INVERTED",
            vl::Expr::new_ulit_bin(
                1,
                &format!("{}", u64::from(self.attr.is_rstallcarryin_inverted)),
            ),
        );
        inst.add_param(
            "IS_RSTALUMODE_INVERTED",
            vl::Expr::new_ulit_bin(
                1,
                &format!("{}", u64::from(self.attr.is_rstalumode_inverted)),
            ),
        );
        //instance.add_param("IS_RSTA_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
        //instance.add_param("IS_RSTB_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
        //instance.add_param("IS_RSTCTRL_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
        //instance.add_param("IS_RSTC_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
        //instance.add_param("IS_RSTD_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
        //instance.add_param("IS_RSTINMODE_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
        //instance.add_param("IS_RSTM_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
        //instance.add_param("IS_RSTP_INVERTED", vl::Expr::new_ulit_bin(1, "0"));
        inst
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

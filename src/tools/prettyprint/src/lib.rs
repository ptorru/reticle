pub use pretty::RcDoc;

pub const PRETTY_WIDTH: usize = 100;
pub const PRETTY_INDENT: isize = 4;

pub trait PrettyPrint {
    fn to_doc(&self) -> RcDoc<()>;

    fn to_pretty_with_width(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }

    fn to_pretty(&self) -> String {
        self.to_pretty_with_width(PRETTY_WIDTH)
    }
}

pub trait PrettyHelper<'a>: Sized {
    fn surround(self, pre: &'a str, post: &'a str) -> Self;
    fn parens(self) -> Self {
        self.surround("(", ")")
    }
    fn brackets(self) -> Self {
        self.surround("[", "]")
    }
    fn braces(self) -> Self {
        self.surround("{", "}")
    }
    fn comps(self) -> Self {
        self.surround("<", ">")
    }
    fn quotes(self) -> Self {
        self.surround("\"", "\"")
    }
}

impl<'a, A> PrettyHelper<'a> for RcDoc<'a, A> {
    fn surround(self, l: &'a str, r: &'a str) -> Self {
        RcDoc::text(l).append(self).append(RcDoc::text(r))
    }
}

pub fn intersperse<'a>(iter: impl Iterator<Item = RcDoc<'a>>, separator: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::intersperse(iter, separator)
}

#[must_use]
pub fn block(body: RcDoc<()>) -> RcDoc<()> {
    RcDoc::nil()
        .append(RcDoc::hardline())
        .append(body)
        .nest(PRETTY_INDENT)
        .append(RcDoc::hardline())
}

#[must_use]
pub fn block_with_braces<'a>(name: RcDoc<'a>, body: RcDoc<'a>) -> RcDoc<'a> {
    name.append(RcDoc::space()).append(block(body).braces())
}

#[must_use]
pub fn block_with_parens<'a>(name: RcDoc<'a>, body: RcDoc<'a>) -> RcDoc<'a> {
    name.append(RcDoc::space()).append(block(body).parens())
}

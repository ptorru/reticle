use reticle::asm::parser::parse_from_file;

fn main() {
    let prog = parse_from_file("examples/asm/example.rasm");
    println!("{}", prog);
}

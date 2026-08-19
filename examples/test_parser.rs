
use typst_core::compiler::parse::parser::Parser;
use typst_core::compiler::parse::SyntaxMode;

fn main() {
    let cases = ["$ g_i(x^*) $", "$ f(x^*) $", "$ (x^*) $", "$ g_i(x) $"];
    for c in cases {
        let p = Parser::new(c, 0, SyntaxMode::Markup).parse();
        println!("=== SOURCE: {} ===", c);
        println!("{:#?}", p);
    }
}

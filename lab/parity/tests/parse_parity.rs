use pretty_assertions::assert_eq;

// compact.rs está em src/ — referenciar via path relativo
#[path = "../src/compact.rs"]
mod compact;
use compact::{compact_cristalino, compact_original};

fn assert_paridade(input: &str) {
    let orig  = typst_syntax::parse(input);
    let crist = typst_core::rules::parse::parse(input);
    assert_eq!(
        compact_original(&orig),
        compact_cristalino(&crist),
        "markup input: {:?}", input
    );
}

fn assert_paridade_math(input: &str) {
    let orig  = typst_syntax::parse_math(input);
    let crist = typst_core::rules::parse::parse_math(input);
    assert_eq!(
        compact_original(&orig),
        compact_cristalino(&crist),
        "math input: {:?}", input
    );
}

fn assert_paridade_code(input: &str) {
    let orig  = typst_syntax::parse_code(input);
    let crist = typst_core::rules::parse::parse_code(input);
    assert_eq!(
        compact_original(&orig),
        compact_cristalino(&crist),
        "code input: {:?}", input
    );
}

// ── Markup ───────────────────────────────────────────────────────────────────

#[test] fn vazio()              { assert_paridade(""); }
#[test] fn texto_simples()      { assert_paridade("Hello, world!"); }
#[test] fn enfase()             { assert_paridade("Hello *world* and _emphasis_"); }
#[test] fn heading_multiplo()   { assert_paridade("= H1\n== H2\n=== H3"); }
#[test] fn lista_bullets()      { assert_paridade("- item 1\n- item 2"); }
#[test] fn lista_enum()         { assert_paridade("+ first\n+ second"); }
#[test] fn link()               { assert_paridade("https://typst.org"); }
#[test] fn escape()             { assert_paridade("\\# escaped"); }
#[test] fn raw_inline()         { assert_paridade("`code`"); }
#[test] fn raw_block()          { assert_paridade("```rust\nfn main() {}\n```"); }
#[test] fn strong_nested()      { assert_paridade("*bold _and italic_*"); }
#[test] fn label_ref()          { assert_paridade("= Section <sec>\nSee @sec"); }
#[test] fn parbreak()           { assert_paridade("Para 1\n\nPara 2"); }

// Trivia — fronteira crítica identificada no diagnóstico
#[test] fn espaco_simples()       { assert_paridade("a b"); }
#[test] fn espacos_consecutivos() { assert_paridade("a  b   c"); }
#[test] fn tab_e_espaco()         { assert_paridade("a\t b"); }
#[test] fn newline_simples()      { assert_paridade("a\nb"); }
#[test] fn newline_vs_parbreak()  { assert_paridade("a\nb\n\nc"); }

// Recuperação de erros — cristalino deve imitar o oráculo
#[test] fn error_brace_aberto()  { assert_paridade("#{{{broken"); }
#[test] fn error_hash_sozinho()  { assert_paridade("#"); }
#[test] fn error_dollar_sozinho(){ assert_paridade("$"); }
#[test] fn error_bracket()       { assert_paridade("[unclosed"); }

// ── Math ─────────────────────────────────────────────────────────────────────

#[test] fn math_simples()    { assert_paridade_math("x^2 + 1"); }
#[test] fn math_fraccao()    { assert_paridade_math("a/b"); }
#[test] fn math_attach()     { assert_paridade_math("x_1^2"); }
#[test] fn math_equation()   { assert_paridade("$x^2 + 1$"); }
#[test] fn math_block()      { assert_paridade("$ sum_(i=0)^n i $"); }
#[test] fn math_delimited()  { assert_paridade_math("[a + b]"); }
#[test] fn math_primes()     { assert_paridade_math("a'''"); }

// ── Code ──────────────────────────────────────────────────────────────────────

#[test] fn let_binding()   { assert_paridade_code("let x = 1"); }
#[test] fn func_call()     { assert_paridade("#f(x, y)"); }
#[test] fn if_else()       { assert_paridade_code("if x { y } else { z }"); }
#[test] fn closure()       { assert_paridade_code("let f = (x) => x + 1"); }
#[test] fn set_rule()      { assert_paridade("#set text(size: 12pt)"); }
#[test] fn show_rule()     { assert_paridade("#show heading: it => it.body"); }
#[test] fn for_loop()      { assert_paridade_code("for i in range(10) { i }"); }
#[test] fn import()        { assert_paridade("#import \"f.typ\": a, b"); }
#[test] fn destructuring() { assert_paridade_code("let (a, b) = (1, 2)"); }
#[test] fn context_expr()  { assert_paridade("#context text.lang"); }

// ── Corpus de ficheiros ───────────────────────────────────────────────────────

fn corpus_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus")
}

#[test]
fn corpus_completo() {
    let mut falhas: Vec<(String, String)> = Vec::new();

    for entry in walkdir::WalkDir::new(corpus_dir())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |x| x == "typ"))
    {
        let path  = entry.path();
        let input = std::fs::read_to_string(path).unwrap();
        let orig  = typst_syntax::parse(&input);
        let crist = typst_core::rules::parse::parse(&input);
        let co    = compact_original(&orig);
        let cc    = compact_cristalino(&crist);

        if co != cc {
            falhas.push((
                path.display().to_string(),
                format!("expected:\n{:#?}\n\nactual:\n{:#?}", co, cc),
            ));
        }
    }

    if !falhas.is_empty() {
        let msg = falhas.iter()
            .map(|(p, d)| format!("── {} ──\n{}", p, d))
            .collect::<Vec<_>>()
            .join("\n\n");
        panic!("Paridade falhou em {} ficheiro(s):\n\n{}", falhas.len(), msg);
    }
}

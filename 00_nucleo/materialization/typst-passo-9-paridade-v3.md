# Passo 9 — Testes de paridade de parsing (v3)

## Contexto

Pré-condição: `cargo test` — 179 testes, zero violations.

Face ao v2, uma correcção: `CompactNode` usa `.name()` em vez de
`format!("{:?}", kind)` para normalizar `SyntaxKind`. Ambas as
crates têm o método `.name() -> &'static str` que retorna o nome
canónico minúsculo ("text", "markup", "heading", etc.) — sem
dependência do formato de `Debug` de cada crate.

Isto elimina a única fonte de falsos negativos cosméticos que
restava no v2.

---

## Estrutura da crate de paridade

```
lab/parity/
  Cargo.toml
  src/
    main.rs           ← runner interactivo com diff para ficheiros
    compact.rs        ← CompactNode DTO partilhado entre bin e tests
  tests/
    parse_parity.rs   ← testes com pretty_assertions
  corpus/
    markup/
    math/
    code/
```

---

## Tarefa 1 — Cargo.toml

**Criar**: `lab/parity/Cargo.toml`

```toml
[package]
name    = "typst-parity"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "parity-runner"
path = "src/main.rs"

[dependencies]
typst-syntax = { path = "../typst-original/crates/typst-syntax" }
typst-core   = { path = "../../01_core" }

[dev-dependencies]
typst-syntax      = { path = "../typst-original/crates/typst-syntax" }
typst-core        = { path = "../../01_core" }
pretty_assertions = "1"
walkdir           = "2"
```

---

## Tarefa 2 — CompactNode com normalização via .name()

**Criar**: `lab/parity/src/compact.rs`

```rust
//! DTO de comparação neutro para testes de paridade de parsing.
//!
//! Elimina spans estruturalmente. Normaliza SyntaxKind via .name()
//! (nome canónico minúsculo) para independência do formato Debug.

/// Representação neutra de um SyntaxNode para comparação de paridade.
#[derive(Debug, PartialEq)]
pub enum CompactNode {
    /// Nó folha: (kind_name canónico, texto exacto do token)
    Leaf(String, String),
    /// Nó interior: (kind_name canónico, filhos)
    Branch(String, Vec<CompactNode>),
    /// Nó de erro: (mensagem de erro, texto original)
    Error(String, String),
}

/// Converte SyntaxNode do parser ORIGINAL para CompactNode.
///
/// Usa `.kind().name()` para normalização canónica de SyntaxKind.
pub fn compact_original(node: &typst_syntax::SyntaxNode) -> CompactNode {
    use typst_syntax::SyntaxKind;

    if node.kind() == SyntaxKind::Error {
        let msg = node.errors()
            .first()
            .map(|e| e.message.to_string())
            .unwrap_or_default();
        return CompactNode::Error(msg, node.text().to_string());
    }

    let children: Vec<_> = node.children()
        .map(compact_original)
        .collect();

    let kind_name = node.kind().name().to_string();

    if children.is_empty() {
        CompactNode::Leaf(kind_name, node.text().to_string())
    } else {
        CompactNode::Branch(kind_name, children)
    }
}

/// Converte SyntaxNode do parser CRISTALINO para CompactNode.
///
/// Usa `.kind().name()` — mesmo método, mesmo output canónico.
pub fn compact_cristalino(
    node: &typst_core::entities::syntax_node::SyntaxNode,
) -> CompactNode {
    use typst_core::entities::syntax_kind::SyntaxKind;

    if node.kind() == SyntaxKind::Error {
        let msg = node.errors()
            .first()
            .map(|e| e.message.to_string())
            .unwrap_or_default();
        return CompactNode::Error(msg, node.text().to_string());
    }

    let children: Vec<_> = node.children()
        .map(compact_cristalino)
        .collect();

    let kind_name = node.kind().name().to_string();

    if children.is_empty() {
        CompactNode::Leaf(kind_name, node.text().to_string())
    } else {
        CompactNode::Branch(kind_name, children)
    }
}
```

---

## Tarefa 3 — Runner interactivo

**Criar**: `lab/parity/src/main.rs`

```rust
mod compact;
use compact::{compact_cristalino, compact_original};

fn main() {
    let arg = std::env::args().nth(1)
        .unwrap_or_else(|| "Hello *world*".to_string());

    // Aceita path de ficheiro ou string inline
    let input = if std::path::Path::new(&arg).exists() {
        std::fs::read_to_string(&arg)
            .unwrap_or_else(|e| { eprintln!("Erro ao ler {}: {}", arg, e); std::process::exit(1); })
    } else {
        arg.replace("\\n", "\n")  // permitir \n na linha de comandos
    };

    let orig  = typst_syntax::parse(&input);
    let crist = typst_core::rules::parse::parse(&input);

    let co = compact_original(&orig);
    let cc = compact_cristalino(&crist);

    if co == cc {
        println!("✓ Paridade confirmada ({} bytes)", input.len());
        return;
    }

    // Gravar para diff visual
    let expected = format!("{:#?}", co);
    let actual   = format!("{:#?}", cc);

    std::fs::write("/tmp/parity_expected.txt", &expected).unwrap();
    std::fs::write("/tmp/parity_actual.txt",   &actual).unwrap();

    eprintln!("✗ Divergência detectada");
    eprintln!();
    eprintln!("  Inspecionar com:");
    eprintln!("    delta /tmp/parity_expected.txt /tmp/parity_actual.txt");
    eprintln!("    diff  /tmp/parity_expected.txt /tmp/parity_actual.txt | head -40");
    eprintln!("    code --diff /tmp/parity_expected.txt /tmp/parity_actual.txt");
    eprintln!();

    // Mostrar primeira divergência inline
    for (i, (a, b)) in expected.lines().zip(actual.lines()).enumerate() {
        if a != b {
            eprintln!("  Primeira divergência na linha {}:", i + 1);
            eprintln!("    expected: {}", a.trim());
            eprintln!("    actual:   {}", b.trim());
            break;
        }
    }

    std::process::exit(1);
}
```

---

## Tarefa 4 — Testes de paridade

**Criar**: `lab/parity/tests/parse_parity.rs`

```rust
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
```

---

## Tarefa 5 — Corpus mínimo obrigatório

**Criar** os ficheiros em `lab/parity/corpus/`:

```
markup/empty.typ       — (vazio)
markup/plain.typ       — Hello, world!
markup/heading.typ     — = H1\n== H2
markup/strong.typ      — *bold*
markup/error.typ       — #{{{broken
markup/spaces.typ      — a  b   c
markup/parbreak.typ    — Para 1\n\nPara 2
math/simple.typ        — $x^2$
math/block.typ         — $ sum_(i=0)^n i $
code/let.typ           — #let x = 1
code/set.typ           — #set text(size: 12pt)
```

---

## Execução

```bash
# Testes de paridade — fora do workspace cristalino
cargo test --manifest-path lab/parity/Cargo.toml 2>&1 | tee lab/parity/report.txt

# Runner interactivo
cargo run --manifest-path lab/parity/Cargo.toml -- "Hello *world*"
cargo run --manifest-path lab/parity/Cargo.toml -- "a\n\nb"
cargo run --manifest-path lab/parity/Cargo.toml -- lab/parity/corpus/markup/error.typ

# Diff visual se houver divergência
delta /tmp/parity_expected.txt /tmp/parity_actual.txt

# Workspace cristalino não deve regredir
cargo test && crystalline-lint .
```

---

## Regra de decisão para divergências

| Tipo | Diagnóstico | Acção |
|------|-------------|-------|
| Divergência de `kind_name` | `.name()` retorna strings diferentes | Verificar se `SyntaxKind::name()` cristalino é idêntico ao original |
| Divergência de texto de folha | Token diferente no mesmo nó | Divergência real no lexer — identificar input mínimo |
| Divergência de estrutura de filhos | Agrupamento diferente | Divergência real no parser — identificar input mínimo |
| Recuperação de erros diferente | `Error` em posição diferente | Imitar o oráculo — fidelidade ao original tem precedência |

Em qualquer caso: **imitar o original**. O oráculo é lei nesta fase.

---

## Ao terminar, reportar

- Número de testes: passaram / falharam
- Se houve divergências e de que tipo (kind, texto, estrutura, erros)
- Inputs mínimos que reproduzem divergências, se alguma
- Número total de testes do workspace cristalino (não deve regredir)

# Passo 9 — Testes de paridade de parsing

## Contexto

Pré-condição: `cargo test` — 179 testes, zero violations.

O pipeline cristalino tem `parse()`, `parse_code()` e `parse_math()`
em L1. O objectivo é verificar que produzem output estruturalmente
idêntico ao `lab/typst-original` para os mesmos inputs.

**Método**: crate de paridade em `lab/` que corre os dois parsers
sobre o mesmo corpus e compara com `spanless_eq`. Spans são
intencionalmente ignorados — dependem de `FileId` e numeração
interna que difere entre implementações.

---

## Estrutura da crate de paridade

```
lab/
  parity/
    Cargo.toml
    src/
      main.rs       ← runner com relatório
    tests/
      parse_parity.rs
    corpus/
      markup/       ← ficheiros .typ de teste
      math/
      code/
```

`lab/parity` não é membro do workspace cristalino — compila
separadamente para evitar contaminar L1–L4 com dependências do
original.

---

## Tarefa 1 — Criar corpus de teste

**Criar**: `lab/parity/corpus/`

Ficheiros `.typ` que cobrem os casos importantes do parser:

```
markup/
  empty.typ              — ficheiro vazio
  plain_text.typ         — "Hello, world!"
  emphasis.typ           — "Hello *world* and _emphasis_"
  heading.typ            — "= Heading 1\n== Heading 2"
  list.typ               — "- item 1\n- item 2"
  enum.typ               — "+ first\n+ second"
  link.typ               — "https://typst.org"
  escape.typ             — "\\# escaped hash"
  raw_inline.typ         — "`code`"
  raw_block.typ          — "```rust\nfn main() {}\n```"
  label_ref.typ          — "= Section <sec>\nSee @sec"
  strong_nested.typ      — "*bold _and italic_*"
  error_recovery.typ     — "#{{{broken" (parser nunca falha)

math/
  simple.typ             — "$x^2 + 1$"
  fraction.typ           — "$a/b$"
  attach.typ             — "$x_1^2$"
  delimited.typ          — "$[a + b]$"
  equation_block.typ     — "$ sum_(i=0)^n i $"

code/
  let_binding.typ        — "#let x = 1"
  func_call.typ          — "#f(x, y)"
  if_else.typ            — "#if x { y } else { z }"
  for_loop.typ           — "#for i in range(10) { i }"
  closure.typ            — "#let f = (x) => x + 1"
  import.typ             — "#import \"file.typ\": a, b"
  set_rule.typ           — "#set text(size: 12pt)"
  show_rule.typ          — "#show heading: it => it.body"
```

---

## Tarefa 2 — Cargo.toml da crate de paridade

**Criar**: `lab/parity/Cargo.toml`

```toml
[package]
name    = "typst-parity"
version = "0.1.0"
edition = "2021"

# NÃO é membro do workspace cristalino
# Compilar separadamente: cd lab/parity && cargo test

[dev-dependencies]
# Parser original — via path relativo ao lab/
typst-syntax = { path = "../typst-original/crates/typst-syntax" }

# Parser cristalino — via path relativo à raiz
typst-core = { path = "../../01_core" }
```

**Nota**: se `typst-syntax` original tiver dependências que conflituam
com o workspace cristalino, compilar com `--manifest-path` explícito:

```bash
cargo test --manifest-path lab/parity/Cargo.toml
```

---

## Tarefa 3 — Testes de paridade

**Criar**: `lab/parity/tests/parse_parity.rs`

```rust
//! Testes de paridade: parser cristalino vs typst-syntax original.
//!
//! Compara SyntaxNode com spanless_eq — spans são intencionalmente
//! ignorados (dependem de FileId e numeração interna).

use std::path::Path;

// Parser original
use typst_syntax as original;

// Parser cristalino
use typst_core::rules::parse as cristalino;
use typst_core::entities::syntax_node::SyntaxNode as CristalSyntaxNode;

/// Converte SyntaxNode do original para representação comparável.
/// Usamos a representação de debug como proxy para spanless_eq
/// entre duas crates diferentes.
fn debug_tree_original(node: &original::SyntaxNode) -> String {
    format!("{:#?}", node)
        // Remover spans da representação — variam entre implementações
        .lines()
        .filter(|l| !l.contains("span:") && !l.contains("Span"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn debug_tree_cristalino(node: &CristalSyntaxNode) -> String {
    format!("{:#?}", node)
        .lines()
        .filter(|l| !l.contains("span:") && !l.contains("Span"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn assert_paridade(input: &str) {
    let orig  = original::parse(input);
    let crist = cristalino::parse(input);

    let orig_tree  = debug_tree_original(&orig);
    let crist_tree = debug_tree_cristalino(&crist);

    assert_eq!(
        orig_tree, crist_tree,
        "\nParidade falhou para input: {:?}\n\nOriginal:\n{}\n\nCristalino:\n{}",
        input, orig_tree, crist_tree
    );
}

fn assert_paridade_math(input: &str) {
    let orig  = original::parse_math(input);
    let crist = cristalino::parse_math(input);
    let orig_tree  = debug_tree_original(&orig);
    let crist_tree = debug_tree_cristalino(&crist);
    assert_eq!(orig_tree, crist_tree,
        "\nParidade math falhou para: {:?}", input);
}

fn assert_paridade_code(input: &str) {
    let orig  = original::parse_code(input);
    let crist = cristalino::parse_code(input);
    let orig_tree  = debug_tree_original(&orig);
    let crist_tree = debug_tree_cristalino(&crist);
    assert_eq!(orig_tree, crist_tree,
        "\nParidade code falhou para: {:?}", input);
}

// --- Markup ---

#[test]
fn paridade_vazio() { assert_paridade(""); }

#[test]
fn paridade_texto_simples() { assert_paridade("Hello, world!"); }

#[test]
fn paridade_enfase() { assert_paridade("Hello *world* and _emphasis_"); }

#[test]
fn paridade_heading() { assert_paridade("= Heading 1\n== Heading 2"); }

#[test]
fn paridade_lista() { assert_paridade("- item 1\n- item 2"); }

#[test]
fn paridade_enum() { assert_paridade("+ first\n+ second"); }

#[test]
fn paridade_link() { assert_paridade("https://typst.org"); }

#[test]
fn paridade_escape() { assert_paridade("\\# escaped hash"); }

#[test]
fn paridade_raw_inline() { assert_paridade("`code`"); }

#[test]
fn paridade_raw_block() { assert_paridade("```rust\nfn main() {}\n```"); }

#[test]
fn paridade_strong_nested() { assert_paridade("*bold _and italic_*"); }

#[test]
fn paridade_error_recovery() {
    // parse() nunca falha — erros viram nós de erro
    assert_paridade("#{{{broken");
}

// --- Math ---

#[test]
fn paridade_math_simples() { assert_paridade_math("x^2 + 1"); }

#[test]
fn paridade_math_fraccao() { assert_paridade_math("a/b"); }

#[test]
fn paridade_math_attach() { assert_paridade_math("x_1^2"); }

#[test]
fn paridade_math_equation() { assert_paridade("$x^2 + 1$"); }

// --- Code ---

#[test]
fn paridade_let_binding() { assert_paridade_code("let x = 1"); }

#[test]
fn paridade_func_call() { assert_paridade("#f(x, y)"); }

#[test]
fn paridade_if_else() { assert_paridade_code("if x { y } else { z }"); }

#[test]
fn paridade_closure() { assert_paridade_code("let f = (x) => x + 1"); }

#[test]
fn paridade_set_rule() { assert_paridade("#set text(size: 12pt)"); }

#[test]
fn paridade_show_rule() {
    assert_paridade("#show heading: it => it.body");
}

// --- Corpus de ficheiros ---

fn corpus_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("corpus")
}

#[test]
fn paridade_corpus_completo() {
    let mut falhas = Vec::new();
    for entry in walkdir::WalkDir::new(corpus_dir())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |x| x == "typ"))
    {
        let input = std::fs::read_to_string(entry.path()).unwrap();
        let orig  = original::parse(&input);
        let crist = cristalino::parse(&input);

        if debug_tree_original(&orig) != debug_tree_cristalino(&crist) {
            falhas.push(entry.path().display().to_string());
        }
    }

    if !falhas.is_empty() {
        panic!("Paridade falhou para {} ficheiros:\n{}",
            falhas.len(), falhas.join("\n"));
    }
}
```

Para o teste de corpus com `walkdir`, adicionar ao `Cargo.toml`:
```toml
[dev-dependencies]
walkdir = "2"
```

---

## Tarefa 4 — Runner com diff legível

**Criar**: `lab/parity/src/main.rs`

```rust
//! Runner de paridade com diff legível.
//! Uso: cargo run --manifest-path lab/parity/Cargo.toml -- [input]

fn main() {
    let input = std::env::args().nth(1)
        .unwrap_or_else(|| "Hello *world*".to_string());

    let orig  = typst_syntax::parse(&input);
    let crist = typst_core::rules::parse::parse(&input);

    println!("=== Input ===\n{:?}\n", input);
    println!("=== Original ===\n{:#?}\n", orig);
    println!("=== Cristalino ===\n{:#?}\n", crist);

    // spanless_eq entre as duas crates não é directamente possível
    // (tipos distintos) — usar debug como proxy
    let orig_s  = format!("{:#?}", orig);
    let crist_s = format!("{:#?}", crist);

    if orig_s == crist_s {
        println!("✓ Paridade confirmada");
    } else {
        // Mostrar primeira divergência
        for (i, (a, b)) in orig_s.lines().zip(crist_s.lines()).enumerate() {
            if a != b {
                println!("✗ Divergência na linha {}:", i + 1);
                println!("  Original:   {}", a);
                println!("  Cristalino: {}", b);
                break;
            }
        }
        std::process::exit(1);
    }
}
```

---

## Execução

```bash
# Testes de paridade
cd lab/parity
cargo test 2>&1 | tee paridade-report.txt

# Runner interactivo para diagnóstico
cargo run -- "= Heading\n\nHello *world*"
cargo run -- "#let x = 1 + 2"
cargo run -- '$x^2 + 1$'

# Verificar que workspace cristalino não foi afectado
cd ../..
cargo test
crystalline-lint .
```

---

## Interpretação dos resultados

**Zero falhas**: paridade confirmada — o parser cristalino é
estruturalmente idêntico ao original para o corpus testado.

**Falhas de span**: se a divergência for apenas em linhas com
`span:` ou `Span` — o filtro no `debug_tree_*` deve eliminá-las.
Se não eliminar, ajustar o filtro.

**Falhas estruturais**: divergência real na árvore — indica uma
diferença na migração de `parser.rs` ou `lexer.rs`. Reportar:
- O input que falha
- A linha de divergência
- Se é no `kind()`, no texto, ou na estrutura de filhos

---

## Ao terminar, reportar

- Número de testes de paridade que passaram / falharam
- Se houve divergências estruturais (não de span)
- Quais inputs do corpus revelaram diferenças, se algum
- Número total de testes do workspace cristalino (não deve regredir)

Esta informação confirma ou refuta que a migração de parser.rs
e lexer.rs é fiel ao original — base para avançar com eval().

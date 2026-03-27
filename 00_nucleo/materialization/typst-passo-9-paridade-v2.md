# Passo 9 — Testes de paridade de parsing (v2)

## Contexto

Pré-condição: `cargo test` — 179 testes, zero violations.

A comparação por `format!("{:?}")` com filtro de texto é frágil —
ordem de campos, escaping de strings, e formato de `Arc` podem gerar
divergências cosméticas sem erro lógico. A abordagem correcta é um
DTO de comparação neutro (`CompactNode`) que elimina spans
estruturalmente e é independente do formato de `Debug` de cada crate.

---

## Estrutura da crate de paridade

```
lab/parity/
  Cargo.toml
  src/
    main.rs          ← runner interactivo com diff para ficheiros
  tests/
    parse_parity.rs  ← testes de paridade com CompactNode
  corpus/
    markup/          ← ficheiros .typ de teste
    math/
    code/
```

`lab/parity` não é membro do workspace cristalino. Compilar com:
```bash
cargo test --manifest-path lab/parity/Cargo.toml
```

---

## Tarefa 1 — Cargo.toml da crate de paridade

**Criar**: `lab/parity/Cargo.toml`

```toml
[package]
name    = "typst-parity"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "parity-runner"
path = "src/main.rs"

[dev-dependencies]
# Parser original
typst-syntax = { path = "../typst-original/crates/typst-syntax" }
# Parser cristalino
typst-core   = { path = "../../01_core" }
# Diffs legíveis nos testes
pretty_assertions = "1"
# Corpus de ficheiros
walkdir = "2"

[dependencies]
# Para o runner binário
typst-syntax = { path = "../typst-original/crates/typst-syntax" }
typst-core   = { path = "../../01_core" }
```

---

## Tarefa 2 — CompactNode DTO

**Criar**: `lab/parity/tests/compact.rs`

O DTO elimina spans, endereços de memória, e detalhes de
implementação. Apenas `SyntaxKind` (pelo nome) e texto de folhas.

```rust
// lab/parity/tests/compact.rs

/// Representação neutra de um SyntaxNode para comparação de paridade.
/// - Spans eliminados estruturalmente (não via filtro de texto)
/// - Arc/Rc internos invisíveis
/// - SyntaxKind representado pelo nome canónico (&'static str)
#[derive(Debug, PartialEq)]
pub enum CompactNode {
    /// Nó folha: (kind_name, text)
    Leaf(String, String),
    /// Nó interior: (kind_name, children)
    Branch(String, Vec<CompactNode>),
    /// Nó de erro: (message, text)
    Error(String, String),
}

/// Converte SyntaxNode do parser ORIGINAL para CompactNode.
pub fn compact_original(node: &typst_syntax::SyntaxNode) -> CompactNode {
    use typst_syntax::SyntaxKind;

    if node.kind() == SyntaxKind::Error {
        let msg = node.errors()
            .first()
            .map(|e| e.message.to_string())
            .unwrap_or_default();
        return CompactNode::Error(msg, node.text().to_string());
    }

    let children: Vec<_> = node.children().map(compact_original).collect();

    if children.is_empty() {
        CompactNode::Leaf(
            format!("{:?}", node.kind()),
            node.text().to_string(),
        )
    } else {
        CompactNode::Branch(
            format!("{:?}", node.kind()),
            children,
        )
    }
}

/// Converte SyntaxNode do parser CRISTALINO para CompactNode.
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

    let children: Vec<_> = node.children().map(compact_cristalino).collect();

    if children.is_empty() {
        CompactNode::Leaf(
            format!("{:?}", node.kind()),
            node.text().to_string(),
        )
    } else {
        CompactNode::Branch(
            format!("{:?}", node.kind()),
            children,
        )
    }
}
```

---

## Tarefa 3 — Testes de paridade

**Criar**: `lab/parity/tests/parse_parity.rs`

```rust
mod compact;
use compact::{compact_cristalino, compact_original};
use pretty_assertions::assert_eq;

fn assert_paridade(input: &str) {
    let orig  = typst_syntax::parse(input);
    let crist = typst_core::rules::parse::parse(input);
    assert_eq!(
        compact_original(&orig),
        compact_cristalino(&crist),
        "input: {:?}", input
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

// ── Markup ──────────────────────────────────────────────────────────────────

#[test] fn vazio()           { assert_paridade(""); }
#[test] fn texto_simples()   { assert_paridade("Hello, world!"); }
#[test] fn enfase()          { assert_paridade("Hello *world* and _emphasis_"); }
#[test] fn heading()         { assert_paridade("= Heading 1\n== Heading 2"); }
#[test] fn lista()           { assert_paridade("- item 1\n- item 2"); }
#[test] fn enum_items()      { assert_paridade("+ first\n+ second"); }
#[test] fn link()            { assert_paridade("https://typst.org"); }
#[test] fn escape()          { assert_paridade("\\# escaped"); }
#[test] fn raw_inline()      { assert_paridade("`code`"); }
#[test] fn raw_block()       { assert_paridade("```rust\nfn main() {}\n```"); }
#[test] fn strong_nested()   { assert_paridade("*bold _and italic_*"); }
#[test] fn label_ref()       { assert_paridade("= Section <sec>\nSee @sec"); }
#[test] fn parbreak()        { assert_paridade("Para 1\n\nPara 2"); }

// Trivia — espaços consecutivos (caso sensível identificado no diagnóstico)
#[test] fn espacos_consecutivos() { assert_paridade("a  b   c"); }
#[test] fn tab_e_espaco()         { assert_paridade("a\t b"); }
#[test] fn newline_simples()      { assert_paridade("a\nb"); }

// Recuperação de erros — o cristalino deve produzir o mesmo nó Error
// no mesmo local da hierarquia que o original
#[test] fn error_recovery_brace()  { assert_paridade("#{{{broken"); }
#[test] fn error_recovery_hash()   { assert_paridade("#"); }
#[test] fn error_recovery_dollar() { assert_paridade("$"); }

// ── Math ────────────────────────────────────────────────────────────────────

#[test] fn math_simples()   { assert_paridade_math("x^2 + 1"); }
#[test] fn math_fraccao()   { assert_paridade_math("a/b"); }
#[test] fn math_attach()    { assert_paridade_math("x_1^2"); }
#[test] fn math_equation()  { assert_paridade("$x^2 + 1$"); }
#[test] fn math_block()     { assert_paridade("$ sum_(i=0)^n i $"); }
#[test] fn math_delimited() { assert_paridade_math("[a + b]"); }

// ── Code ─────────────────────────────────────────────────────────────────────

#[test] fn let_binding() { assert_paridade_code("let x = 1"); }
#[test] fn func_call()   { assert_paridade("#f(x, y)"); }
#[test] fn if_else()     { assert_paridade_code("if x { y } else { z }"); }
#[test] fn closure()     { assert_paridade_code("let f = (x) => x + 1"); }
#[test] fn set_rule()    { assert_paridade("#set text(size: 12pt)"); }
#[test] fn show_rule()   { assert_paridade("#show heading: it => it.body"); }
#[test] fn for_loop()    { assert_paridade_code("for i in range(10) { i }"); }
#[test] fn import()      { assert_paridade("#import \"f.typ\": a, b"); }

// ── Corpus de ficheiros ──────────────────────────────────────────────────────

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
        let path = entry.path();
        let input = std::fs::read_to_string(path).unwrap();

        let orig  = typst_syntax::parse(&input);
        let crist = typst_core::rules::parse::parse(&input);

        let co = compact_original(&orig);
        let cc = compact_cristalino(&crist);

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

## Tarefa 4 — Runner interactivo com diff para ficheiros

**Criar**: `lab/parity/src/main.rs`

```rust
use std::io::Write;

mod compact {
    // re-usar o mesmo compact.rs via include! ou mover para src/
    // Para simplicidade, duplicar aqui ou usar um módulo partilhado
}

fn main() {
    let input = std::env::args().nth(1)
        .unwrap_or_else(|| "Hello *world*".to_string());

    // Suporte a ficheiro como argumento
    let input = if std::path::Path::new(&input).exists() {
        std::fs::read_to_string(&input).unwrap()
    } else {
        input
    };

    let orig  = typst_syntax::parse(&input);
    let crist = typst_core::rules::parse::parse(&input);

    let co = compact::compact_original(&orig);
    let cc = compact::compact_cristalino(&crist);

    if co == cc {
        println!("✓ Paridade confirmada para {} bytes de input", input.len());
        return;
    }

    // Gravar ficheiros para diff visual externo
    let mut f_expected = std::fs::File::create("/tmp/parity_expected.txt").unwrap();
    let mut f_actual   = std::fs::File::create("/tmp/parity_actual.txt").unwrap();
    write!(f_expected, "{:#?}", co).unwrap();
    write!(f_actual,   "{:#?}", cc).unwrap();

    eprintln!("✗ Divergência detectada");
    eprintln!("  expected → /tmp/parity_expected.txt");
    eprintln!("  actual   → /tmp/parity_actual.txt");
    eprintln!();
    eprintln!("  diff visual: delta /tmp/parity_expected.txt /tmp/parity_actual.txt");
    eprintln!("  ou: code --diff /tmp/parity_expected.txt /tmp/parity_actual.txt");

    std::process::exit(1);
}
```

---

## Tarefa 5 — Corpus de ficheiros

**Criar**: `lab/parity/corpus/` com os ficheiros `.typ` do corpus.
Ver lista completa no Passo 9 v1 — incluir os casos de trivia
e recuperação de erros que são os mais sensíveis.

Ficheiros mínimos obrigatórios:
```
corpus/markup/empty.typ          — (ficheiro vazio)
corpus/markup/plain.typ          — Hello, world!
corpus/markup/heading.typ        — = H1\n== H2
corpus/markup/strong.typ         — *bold*
corpus/markup/error.typ          — #{{{broken
corpus/markup/spaces.typ         — a  b   c  (espaços consecutivos)
corpus/math/simple.typ           — $x^2$
corpus/math/block.typ            — $ sum_(i=0)^n i $
corpus/code/let.typ              — #let x = 1
corpus/code/set.typ              — #set text(size: 12pt)
```

---

## Execução

```bash
# Testes de paridade
cargo test --manifest-path lab/parity/Cargo.toml 2>&1 | tee lab/parity/report.txt

# Runner interactivo
cargo run --manifest-path lab/parity/Cargo.toml -- "Hello *world*"
cargo run --manifest-path lab/parity/Cargo.toml -- lab/parity/corpus/markup/error.typ

# Diff visual se houver divergência
delta /tmp/parity_expected.txt /tmp/parity_actual.txt

# Workspace cristalino não deve regredir
cargo test
crystalline-lint .
```

---

## Regra de decisão para divergências

Se `pretty_assertions` mostrar divergência:

1. **Divergência de span**: impossível com `CompactNode` — spans
   foram eliminados estruturalmente. Se aparecer, é bug no DTO.

2. **Divergência cosmética** (nomes de variante diferentes):
   verificar se `SyntaxKind` cristalino usa o mesmo nome no `Debug`
   que o original. Ex: `SyntaxKind::Text` vs `Text`.

3. **Divergência estrutural real** (filhos diferentes, kinds
   diferentes): divergência genuína na migração. A regra é sempre
   **imitar o original** — fidelidade ao oráculo tem precedência
   sobre "pureza" da árvore.

4. **Divergência em recuperação de erros**: o mais provável de
   falhar. Se o cristalino produz `Error` noutro nó da hierarquia
   que o original, identificar o input mínimo que reproduce e
   reportar com o diff.

---

## Ao terminar, reportar

- Número de testes de paridade: passaram / falharam
- Se houve divergências estruturais (não cosméticas)
- Quais inputs do corpus falharam, se algum
- Se recuperação de erros foi fiel ao original
- Número total de testes do workspace cristalino (não deve regredir)

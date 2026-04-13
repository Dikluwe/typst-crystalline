# Prompt L0 — `parse` (parser e lexer do Typst)
Hash do Código: 6e8f311a

**Camada**: L1
**Ficheiros**:
  - `01_core/src/rules/parse.rs` — `parse()`, `parse_code()`, `parse_math()`, `Parser`
  - `01_core/src/rules/lexer/mod.rs` — `Lexer`
  - `01_core/src/entities/syntax_mode.rs` — `SyntaxMode`
  - `01_core/src/entities/operators.rs` — `UnOp`, `BinOp`, `Assoc`
**ADRs**: 0006–0015

---

## Interface pública

```rust
/// Analisa texto Typst como markup de topo.
/// Função pura: sem I/O, sem estado, sem efeitos.
pub fn parse(text: &str) -> SyntaxNode;

/// Analisa texto Typst como código de topo.
pub fn parse_code(text: &str) -> SyntaxNode;

/// Analisa texto Typst como math de topo.
pub fn parse_math(text: &str) -> SyntaxNode;
```

`Lexer` é `pub(super)` — detalhe de implementação do módulo `rules`.

---

## Dependências internas autorizadas

| Módulo | Uso |
|--------|-----|
| `crate::rules::lexer::scanner::Scanner` | Motor de travessia de string (ADR-0014) |
| `crate::entities::math_class::default_math_class` | Overrides de classe math (ADR-0009, 0011) |
| `crate::entities::syntax_mode::SyntaxMode` | Modo de parsing (Markup/Math/Code) |
| `crate::entities::operators::{UnOp, BinOp, Assoc}` | Operadores para parsing de expressões |
| `crate::entities::syntax_set::{SyntaxSet, syntax_set!}` | Conjuntos de SyntaxKind |
| `crate::utils::{timing_scope!, defer}` | No-op timing + RAII cleanup (ADR-0006, 0008) |
| `std::collections::{HashMap, HashSet}` | Substituição de rustc_hash (ADR-0007) |

---

## Dependências externas autorizadas

| Crate | Uso | ADR |
|-------|-----|-----|
| `unicode_ident` | `is_xid_start`, `is_xid_continue` — identificadores UAX #31 | 0010 |
| `unicode_math_class` | `MathClass` — classificação TR25 | 0011 |
| `unicode_script` | `Script`, `UnicodeScript` — detecção de script | 0012 |
| `unicode_segmentation` | `UnicodeSegmentation` — grapheme clusters | 0013 |

---

## Proibido em L1

| O quê | Substituto | ADR |
|-------|-----------|-----|
| `ecow::{EcoString, eco_format!}` | `String`/`SyntaxText` + `format!` | 0015 |
| `rustc_hash::{FxHashMap, FxHashSet}` | `std::collections::{HashMap, HashSet}` | 0007 |
| `typst_timing::TimingScope` | `timing_scope!("nome")` | 0006 |
| `typst_utils::defer` | `crate::utils::defer` | 0008 |
| `typst_utils::default_math_class` | `crate::entities::math_class::default_math_class` | 0009 |
| `unscanny::Scanner` | `crate::rules::lexer::scanner::Scanner` | 0014 |

---

## Tabela de substituições EcoString (ADR-0015)

| Padrão original | Substituição |
|----------------|-------------|
| `EcoString` em assinatura `impl Into<EcoString>` | `impl Into<SyntaxText>` |
| `EcoString` como variável local temporária | `String` |
| `FxHashSet<EcoString>` | `HashSet<String>` |
| `eco_format!(...)` | `format!(...)` |
| `let text: EcoString = nodes.drain(..).map(SyntaxNode::into_text).collect()` | loop acumulador em `String` + `SyntaxText::from(s)` |

---

## Critérios de verificação

**parse() — texto simples**
- `parse("Hello, world!").kind() == SyntaxKind::Markup`
- `!parse("Hello, world!").erroneous()`

**parse() — vazio**
- `parse("").kind() == SyntaxKind::Markup`
- `parse("").len() == 0`
- `!parse("").erroneous()`

**parse() — nunca falha**
- `parse("#{{{broken").kind() == SyntaxKind::Markup`
- `parse("#{{{broken").erroneous() == true`
- `!parse("#{{{broken").errors().is_empty()`

**parse() — equação**
- `parse("$x^2 + 1$")` tem filho com `kind() == SyntaxKind::Equation`

**parse() — código**
- `parse("#let x = 1")` tem filho com `kind() == SyntaxKind::LetBinding`

**parse_math() — básico**
- `parse_math("x^2").kind() == SyntaxKind::Math`
- `!parse_math("x^2").erroneous()`

**parse_code() — básico**
- `parse_code("let x = 1").kind() == SyntaxKind::Code`
- `!parse_code("let x = 1").erroneous()`

---

## Notas

- `parse()` é função pura — sem I/O, sem estado global, determinística
- `timing_scope!("parse")` é um no-op (ADR-0006); pontos de religação
  em `01_core/DEBT.md`
- Os pontos de timing originais eram: `"parse"`, `"parse code"`, `"parse math"`
- Reparsing incremental (`reparse_*`) não é exposto na interface pública de L1
- `ast::Expr::Ident`/`ast::Expr::Str` substituídos por `node.kind()` directo
  nos contextos de duplicate-detection (ADR-0015 aplica-se apenas ao texto)

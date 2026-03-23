# Passo 4 — parse() e lexer (v2)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/0001-estrategia-migracao.md`
- `00_nucleo/adr/0006-typst-timing.md` → timing_scope! no-op em utils.rs
- `00_nucleo/adr/0007-rustc-hash.md` → FxHashMap → std::HashMap (executar aqui)
- `00_nucleo/adr/0008-defer-inline.md` → defer! em utils.rs (já executado)
- `00_nucleo/adr/0009-math-class.md` → default_math_class em entities/ (já executado)
- `00_nucleo/adr/0010-unicode-ident.md` → autorizado
- `00_nucleo/adr/0011-unicode-math-class.md` → autorizado
- `00_nucleo/adr/0012-unicode-script.md` → autorizado
- `00_nucleo/adr/0013-unicode-segmentation.md` → autorizado
- `00_nucleo/adr/0014-unscanny.md` → Scanner em rules/lexer/scanner.rs (já executado)
- `00_nucleo/adr/0015-ecow.md` → EcoString → String/SyntaxText (executar aqui)

Estrutura já existente em L1 que este passo usa directamente:

```
01_core/src/
  entities/
    math_class.rs    ← default_math_class, MathClass (ADR-0009, 0011)
    syntax_node.rs   ← SyntaxNode, SyntaxError, SyntaxText (ADR-0004)
    syntax_kind.rs   ← SyntaxKind
    syntax_set.rs    ← SyntaxSet, constantes STMT/MATH_EXPR/etc.
    span.rs          ← Span, Spanned
    file_id.rs       ← FileId
  rules/
    lexer/
      scanner.rs     ← Scanner inlinado (ADR-0014) — interface idêntica ao unscanny original
  utils.rs           ← timing_scope!, defer!
  DEBT.md            ← 3 pontos de timing a religar no Passo 10
```

Pré-condição: `cargo test -p typst-core` — 105 testes, zero violations.

---

## Diagnósticos necessários (reduzidos)

As dependências externas já foram todas decididas nas ADRs 0006–0015.
Os diagnósticos aqui servem apenas para confirmar assinaturas e
identificar usos residuais de tipos já resolvidos.

```bash
# Confirmar assinaturas das 3 funções de entrada
grep -n "^pub fn parse" \
  lab/typst-original/crates/typst-syntax/src/parser.rs

# Confirmar que parse() não tem estado global (V13)
grep -n "^static\|OnceLock\|LazyLock\|Mutex" \
  lab/typst-original/crates/typst-syntax/src/parser.rs \
  lab/typst-original/crates/typst-syntax/src/lexer.rs

# Contar ocorrências de EcoString/eco_format para estimar esforço de ADR-0015
grep -c "EcoString\|eco_format" \
  lab/typst-original/crates/typst-syntax/src/parser.rs \
  lab/typst-original/crates/typst-syntax/src/lexer.rs

# Confirmar que FxHashMap/FxHashSet está apenas no parser (ADR-0007)
grep -n "FxHashMap\|FxHashSet" \
  lab/typst-original/crates/typst-syntax/src/parser.rs
```

Reportar o output. Se aparecer qualquer externo novo não coberto
pelas ADRs 0006–0015 — parar e reportar antes de continuar.

---

## Tarefa 1 — Prompt L0 para parse.md

**Criar ou actualizar**: `00_nucleo/prompts/rules/parse.md`

O prompt deve documentar:

```
Interface pública:
  pub fn parse(text: &str) -> SyntaxNode
  pub fn parse_code(text: &str) -> SyntaxNode
  pub fn parse_math(text: &str) -> SyntaxNode

Dependências internas autorizadas (não externas):
  crate::rules::lexer::scanner::Scanner  (ADR-0014)
  crate::entities::math_class::*         (ADR-0009, 0011)
  crate::utils::{timing_scope!, defer!}  (ADR-0006, 0008)
  std::collections::HashMap/HashSet      (ADR-0007)

Dependências externas autorizadas:
  unicode_ident       (ADR-0010)
  unicode_math_class  (ADR-0011)
  unicode_script      (ADR-0012)
  unicode_segmentation (ADR-0013)

Proibido:
  ecow (ADR-0015) — usar String/SyntaxText
  rustc_hash (ADR-0007) — usar std::collections
  typst_timing (ADR-0006) — usar timing_scope!()
  typst_utils (ADR-0008, 0009) — inline ou em entities/
  unscanny (ADR-0014) — usar crate::rules::lexer::scanner::Scanner
```

Critérios de verificação (incluir no prompt):
```
Dado parse("Hello, world!")
Quando kind() for chamado no resultado
Então SyntaxKind::Markup, erroneous() == false

Dado parse("")
Então SyntaxKind::Markup, len() == 0, erroneous() == false

Dado parse("#{{{broken")
Então SyntaxKind::Markup, erroneous() == true, errors().len() > 0

Dado parse("$x^2 + 1$")
Então filho com kind() == SyntaxKind::Equation existe

Dado parse("#let x = 1")
Então filho com kind() == SyntaxKind::LetBinding existe

Dado parse_math("x^2")
Então SyntaxKind::Math, erroneous() == false

Dado parse_code("let x = 1")
Então SyntaxKind::Code, erroneous() == false
```

---

## Tarefa 2 — Migrar lexer.rs

**Origem**: `lab/typst-original/crates/typst-syntax/src/lexer.rs`
**Destino**: `01_core/src/rules/lexer/mod.rs`
            (ou `01_core/src/rules/lexer/lexer.rs` re-exportado em mod.rs)

### Substituições obrigatórias (ADRs 0006–0015)

| Original | Substituição | ADR |
|----------|-------------|-----|
| `use unscanny::Scanner` | `use super::scanner::Scanner` | 0014 |
| `use ecow::{EcoString, eco_format}` | remover — ver abaixo | 0015 |
| `use typst_utils::default_math_class` | `use crate::entities::math_class::default_math_class` | 0009 |
| `use unicode_math_class::MathClass` | manter (autorizado) | 0011 |
| `use unicode_script::{Script, UnicodeScript}` | manter (autorizado) | 0012 |
| `use unicode_segmentation::UnicodeSegmentation` | manter (autorizado) | 0013 |

**Substituições de EcoString no lexer (ADR-0015):**

```rust
// Padrão 1 — token text (maioria dos casos)
// Original:
let text: EcoString = self.scanner.slice().into();
// Substituição:
let text: SyntaxText = self.scanner.slice().into();  // From<&str> já existe

// Padrão 2 — mensagem de erro
// Original:
eco_format!("invalid escape: {}", c)
// Substituição:
format!("invalid escape: {}", c)  // aceite por SyntaxError::new via From<String>
```

**Verificar**: se `EcoString` for clonada dentro de um loop de
reparsing — usar `SyntaxText` (clone O(1) via Arc). Para todos os
outros contextos — `String`/`format!` é correcto.

Header obrigatório em `mod.rs` do lexer:
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-23
```

---

## Tarefa 3 — Migrar parser.rs

**Origem**: `lab/typst-original/crates/typst-syntax/src/parser.rs`
**Destino**: `01_core/src/rules/parse.rs`

### Substituições obrigatórias

| Original | Substituição | ADR |
|----------|-------------|-----|
| `use ecow::{EcoString, eco_format}` | remover — usar `format!` e `SyntaxText` | 0015 |
| `use rustc_hash::{FxHashMap, FxHashSet}` | `use std::collections::{HashMap, HashSet}` | 0007 |
| `use typst_timing::TimingScope` | `use crate::utils::timing_scope` | 0006 |
| `use typst_utils::defer` | `use crate::utils::defer` | 0008 |
| `TimingScope::new("parse", None)` | `timing_scope!("parse")` | 0006 |

### Verificação de V14 após migração

Correr `crystalline-lint .` após a migração. Se V14 disparar para
qualquer import não coberto pelas ADRs 0006–0015 — parar e reportar.
Não adicionar a `[l1_allowed_external]` sem ADR.

---

## Tarefa 4 — Actualizar lib.rs e mod.rs

```rust
// 01_core/src/lib.rs — adicionar
pub mod rules;

// 01_core/src/rules/mod.rs — já existe com pub mod lexer;
// adicionar:
pub mod parse;
```

---

## Tarefa 5 — Testes de paridade

Os testes de paridade são a parte mais importante deste passo.
`parse()` deve produzir output **estruturalmente idêntico** ao
original para o mesmo input (usar `spanless_eq` — spans dependem
de `FileId` que não existe neste contexto).

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::syntax_kind::SyntaxKind;

    #[test]
    fn texto_simples() {
        let node = parse("Hello, world!");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        assert!(!node.erroneous());
    }

    #[test]
    fn texto_vazio() {
        let node = parse("");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        assert!(!node.erroneous());
        assert_eq!(node.len(), 0);
    }

    #[test]
    fn parse_nunca_falha() {
        let node = parse("#{{{broken");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        assert!(node.erroneous());
        assert!(!node.errors().is_empty());
    }

    #[test]
    fn expressao_matematica() {
        let node = parse("$x^2 + 1$");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        let eq = node.children().find(|n| n.kind() == SyntaxKind::Equation);
        assert!(eq.is_some());
    }

    #[test]
    fn codigo_typst() {
        let node = parse("#let x = 1");
        let binding = node.children()
            .find(|n| n.kind() == SyntaxKind::LetBinding);
        assert!(binding.is_some());
    }

    #[test]
    fn parse_math_basico() {
        let node = parse_math("x^2");
        assert_eq!(node.kind(), SyntaxKind::Math);
        assert!(!node.erroneous());
    }

    #[test]
    fn parse_code_basico() {
        let node = parse_code("let x = 1");
        assert_eq!(node.kind(), SyntaxKind::Code);
        assert!(!node.erroneous());
    }
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critério de conclusão:
- `parse("Hello")` retorna `SyntaxNode` com `kind() == SyntaxKind::Markup` e zero erros
- Zero violations no linter
- ADR-0007 executada (FxHashMap → HashMap)
- ADR-0015 aplicada (zero EcoString em L1)

---

## Ao terminar, reportar

- Número total de substituições de EcoString aplicadas
- Se algum EcoString foi mantido como SyntaxText por estar em loop de reparsing
- Número total de testes
- Se V14 disparou para algum externo não coberto pelas ADRs

Esta informação vai para o Passo 5 (Source real + AST tipada).

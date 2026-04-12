# Prompt L0 — `rules/math/mod` — Agregador do Subsistema Matemático

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/math/mod.rs`
**ADRs relevantes**: ADR-0011 (delegação de MathClass), ADR-0019 (ttf-parser → L3)

---

## Contexto e Objetivo

Módulo raiz do subsistema matemático do compilador Cristalino. Actua como
*namespace* público e **ponto de entrada único** para todos os componentes de
processamento de equações. Não contém lógica de negócio — a sua função é
exclusivamente organizar a visibilidade dos submódulos.

## Estrutura dos Submódulos

| Módulo | Ficheiro | Responsabilidade |
|--------|----------|-----------------|
| `layout` | `layout.rs` | `MathLayouter` — converte `Content` matemático em `Vec<FrameItem>` |
| `symbols` | `symbols.rs` | Mapeamento de identificadores e shorthands para Unicode + classificação tipográfica |

## Regras de Arquitetura

- **Sem lógica própria**: este `mod.rs` apenas declara `pub mod layout` e
  `pub mod symbols`. Qualquer lógica que "não sabe onde ir" **não pertence
  aqui** — deve ser alocada num dos submódulos específicos.
- **Fronteira L1**: nenhuma dependência de L3. O subsistema matemático é
  puro — recebe `&dyn FontMetrics` (trait de L1) por injeção de dependências.
- **Reexportação controlada**: outros módulos de L1 importam de
  `rules::math::layout::MathLayouter` ou `rules::math::symbols::*`
  directamente — não existe re-export em `mod.rs`.

## Interface Pública (via submódulos)

```rust
// Via rules::math::layout
pub struct MathLayouter<'a, M: FontMetrics>
pub fn layout_equation(&self, body: &Content, style: &TextStyle) -> Vec<FrameItem>

// Via rules::math::symbols  
pub fn ident_to_unicode(name: &str) -> Option<&'static str>
pub fn shorthand_to_unicode(text: &str) -> Option<&'static str>
pub fn is_math_function(name: &str) -> bool
pub fn is_single_letter_var(name: &str) -> bool
pub fn is_large_operator(c: char) -> bool
pub fn is_limit_function(s: &str) -> bool
```

## Critérios de Verificação

```
// Estrutura do módulo
mod.rs contém apenas: pub mod layout; pub mod symbols;
mod.rs NÃO contém: use, fn, struct, impl, extern crate

// Importabilidade
use crate::rules::math::layout::MathLayouter;  // compila
use crate::rules::math::symbols::ident_to_unicode;  // compila

// Ausência de dependências externas
grep "ttf_parser" 01_core/src/rules/math/ = vazio
grep "ttf-parser" 01_core/Cargo.toml = não encontrado
```

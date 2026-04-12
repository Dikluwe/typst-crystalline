# Prompt L0 — `rules/mod` — Agregador do Motor de Regras

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/mod.rs`
**ADRs relevantes**: ADR-0017 (adiamento de eval completo)

---

## Contexto e Objetivo

Ponto de entrada único (*facade*) do domínio lógico de processamento — a camada
que transforma texto fonte em valores avaliados. Declara e organiza os seis
submódulos que compõem o pipeline de compilação L1.

Este ficheiro **não contém lógica** — é um mapa de dependências que comunica
a arquitectura do compilador de forma instantânea.

---

## Estrutura dos Submódulos

| Módulo | Ficheiro | Responsabilidade | Camada pré-prompt |
|--------|----------|-----------------|------------------|
| `eval` | `eval.rs` | Motor de avaliação da AST → `Module` | `prompts/rules/eval.md` |
| `layout` | `layout.rs` | Conversor `Content` → `Frame` (layout de páginas) | `prompts/rules/layout.md` |
| `lexer` | `lexer/mod.rs` + `scanner.rs` | Tokenizador — texto → `SyntaxKind` stream | `prompts/rules/lexer/mod.md` |
| `math` | `math/mod.rs` | Subsistema matemático (MathLayouter + symbols) | `prompts/rules/math/mod.md` |
| `parse` | `parse.rs` | Parser — tokens → AST (`SyntaxNode`) | `prompts/rules/parse.md` |
| `scopes` | `scopes.rs` | Gestão de escopos aninhados de bindings | `prompts/rules/scopes.md` |
| `stdlib` | `stdlib.rs` | Funções nativas intrínsecas da linguagem | `prompts/rules/stdlib.md` |

---

## Regras de Arquitetura

- **Visibilidade controlada**: todos os módulos são `pub mod` — a API pública
  de cada um é gerida internamente com `pub`/`pub(crate)`/privado.
- **Sem re-exports**: o consumidor (L4 wiring ou outros módulos L1) importa
  directamente de `crate::rules::eval::eval`, `crate::rules::layout::Layouter`,
  etc. Não há `pub use` aqui.
- **Isolamento garantido**: nenhum módulo em `rules/` pode importar de
  `03_infra` — violação detectada pelo linter (regra V5).
- **Dependências entre submódulos**:
  - `eval.rs` usa `scopes.rs` (gestão de escopos)
  - `layout.rs` usa `math/layout.rs` (via `MathLayouter`)
  - `math/layout.rs` usa `math/symbols.rs`
  - `parse.rs` usa `lexer/mod.rs` (tokenização)

---

## Interface (via submódulos)

```rust
// Pipeline principal
pub fn eval(world, source) -> SourceResult<Module>   // rules::eval
pub fn parse(text: &str) -> SyntaxNode               // rules::parse
pub struct Layouter<'a, M>                            // rules::layout
pub struct MathLayouter<'a, M: FontMetrics>           // rules::math::layout
pub struct Scopes<'a>                                 // rules::scopes

// Stdlib
pub fn native_type(args: &[Value]) -> SourceResult<Value>
pub fn make_calc_module() -> Value
// ... demais funções nativas
```

---

## Critérios de Verificação

```
// Estrutura do módulo
rules/mod.rs contém APENAS declarações pub mod
Sem: use, fn, struct, impl, const, static

// Importações válidas do exterior
use crate::rules::eval::eval;             // ok
use crate::rules::layout::Layouter;       // ok
use crate::rules::math::layout::MathLayouter; // ok
use crate::rules::stdlib::make_calc_module;   // ok

// Isolamento
grep "03_infra" 01_core/src/rules/ = vazio   // L1 nunca toca L3
```

# ⚖️ ADR-0009: `default_math_class` → `01_core/entities/math_class.rs`

**Status**: `PROPOSTO`
**Data**: 2026-03-23

---

## Contexto

O diagnóstico do Passo 4 revelou que `parser.rs` e `lexer.rs` usam
`typst_utils::default_math_class` — uma função que mapeia caracteres
Unicode para classes matemáticas (`MathClass`) definidas pela
especificação Unicode Math.

`MathClass` é um enum que classifica símbolos matemáticos:
`Normal`, `Alphabetic`, `Binary`, `Closing`, `Fence`, `Large`,
`Opening`, `Punctuation`, `Relation`, `Space`, `Unary`, `Vary`,
`Special`.

Esta classificação determina o comportamento tipográfico de cada
símbolo no modo math do Typst: espaçamento, prioridade de operador,
associatividade. É uma regra do domínio tipográfico — não
infraestrutura.

A classificação vive em `typst_utils` por conveniência histórica,
mas pertence conceptualmente ao domínio do parser matemático.
Manter em `typst_utils` significa que L1 depende de uma crate de
utilitários genéricos para aceder a uma regra de domínio própria.

---

## Decisão

`default_math_class` e o enum `MathClass` são movidos para
`01_core/entities/math_class.rs`.

A tabela de lookup (`char → MathClass`) e o enum são propriedade
de L1. `typst_utils` deixa de ser dependência de `typst-core`
(após ADR-0008 tratar `defer!`).

Estrutura do módulo:

```rust
// 01_core/src/entities/math_class.rs
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/math-class.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-23

/// Classificação Unicode de símbolos matemáticos.
///
/// Define o comportamento tipográfico de cada símbolo em modo math:
/// espaçamento automático, prioridade de operador, associatividade.
///
/// Especificação: https://www.unicode.org/reports/tr25/
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MathClass {
    Normal,
    Alphabetic,
    Binary,
    Closing,
    Fence,
    Large,
    Opening,
    Punctuation,
    Relation,
    Space,
    Unary,
    Vary,
    Special,
}

/// Retorna a classe math padrão para um caractere Unicode.
///
/// Movido de `typst_utils::default_math_class` — ADR-0009.
/// A tabela segue a especificação Unicode Math (TR25).
pub fn default_math_class(c: char) -> Option<MathClass> {
    // tabela copiada de typst_utils — origem: Apache-2.0
    // ...
}
```

Um prompt L0 é criado para este módulo:
`00_nucleo/prompts/entities/math-class.md`

---

## Política de sincronização com upstream

`default_math_class` é uma tabela de dados derivada do standard
Unicode Math (TR25). O Typst actualiza esta tabela quando o standard
evolui. Como L1 agora é dono da tabela, a sincronização com upstream
é responsabilidade do processo de migração.

Mecanismo: quando `lab/typst-original` for actualizado (testes de
paridade), o diff em `typst_utils/src/lib.rs` é inspeccionado. Se
`default_math_class` mudou, a tabela em `entities/math_class.rs` é
actualizada e o prompt revisado.

Isto é um custo de manutenção explícito e aceite — a alternativa
(manter em `typst_utils`) é pior arquitecturalmente.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/entities/math-class.md` | Criar — novo módulo de entidade |
| `00_nucleo/prompts/rules/parse.md` | Actualizar import: `typst_utils::default_math_class` → `crate::entities::math_class::default_math_class` |
| `00_nucleo/prompts/entities/mod.md` | Adicionar `math_class` à lista de módulos |

---

## Consequências

**Positivas**: `typst_utils` removida de `01_core/Cargo.toml`
(combinado com ADR-0008); classificação de símbolos math é domínio
de L1 com prompt e testes próprios; mudanças na tabela são visíveis
no histórico do prompt.

**Negativas**: Custo de sincronização manual com upstream para
actualizações da tabela Unicode Math.

**Neutras**: `MathClass` passa a ser o tipo canónico em L1 — o
`unicode_math_class::MathClass` que o lexer usa via `unicode_math_class`
crate (ADR-0011) é distinto e serve propósito diferente (lookup
exaustiva de todos os caracteres Unicode; `default_math_class` é
um subset com defaults tipográficos do Typst).

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Autorizar `typst_utils` em `[l1_allowed_external]` | Sem cópia; sincronização automática | Crate de utilitários genéricos em L1; `defer!` também já foi inlinado (ADR-0008) |
| Mover para L3 com lookup injectável | L1 puro sem tabelas | Classificação math é regra de domínio — pertence a L1 |

---

## Referências

- Diagnóstico Passo 4 — ocorrências de `default_math_class` em parser.rs e lexer.rs
- ADR-0008 — remoção do outro uso de `typst_utils` (`defer!`)
- Unicode Math TR25: https://www.unicode.org/reports/tr25/
- `typst_utils`: https://github.com/typst/typst/tree/main/crates/typst-utils

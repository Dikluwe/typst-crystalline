# Prompt L0 — entities/parity
Hash do Código: ce1c056c

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/parity.rs`
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0037
(coesão por domínio), ADR-0061 (Layout Fase X roadmap; primeira
aplicação no Passo 156E)

## Contexto

`Parity` é um enum simples `Even`/`Odd` usado como atributo
`to` em `Content::Pagebreak`. Materializado no Passo 156E
(sub-passo 3 da Fase 1 Layout, per ADR-0061) como suporte
estrutural para `pagebreak(to: "even")` / `pagebreak(to:
"odd")`.

Réplica simplificada de
`lab/typst-original/crates/typst-library/src/layout/page.rs::Parity`,
reduzida ao essencial para o consumer cristalino actual.

Vanilla expõe `Auto` via `Smart<Parity>` (i.e. `Smart::Auto`
quando `to` ausente, `Smart::Custom(Parity::Even)` quando
especificado). Cristalino simplifica para `Option<Parity>`:
`None` == Auto (sem ajuste); `Some(parity)` == ajuste forçado.
Decisão pragmática per ADR-0054 graded.

Diagnóstico prévio: ausente — entrada nova adicionada em P156E
(modelo "tudo-num-passo" análogo ao `entities/parity.md`
vanilla).

## Interface pública

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    Even,
    Odd,
}

impl Parity {
    pub fn matches(self, page_number: usize) -> bool;
}
```

## Semântica

- `matches(page_number)`: retorna `true` se `page_number`
  (1-indexed) bate a paridade. Página 1 é ímpar; 2 é par; etc.
  Modulo 2 directo (`page_number % 2 == 0` para Even).
- **`Copy`** derivado: trivial; sem heap.
- **`Eq`** derivado: comparação exacta.

## Invariantes

- Apenas duas variantes — sem `Auto` interno (representação
  externa via `Option<Parity>`).
- Sem mutação após construção (variant pura).
- `matches` é função pura sem side-effects.

## Consumers actuais

- `Content::Pagebreak { weak: bool, to: Option<Parity> }`
  (Passo 156E).

## Consumers planeados

- Possível uso futuro em `header_ascent`/`footer_descent`
  per-página quando refino Page rico (Fase 3 Layout per
  ADR-0061) introduzir cabeçalhos/rodapés diferentes em páginas
  pares vs ímpares (e.g. layout de livro).

## Sobre paridade

Vanilla usa `Smart<Parity>` em todo o lado para representar
"automático ou específico". Cristalino usa `Option<T>` como
convenção idiomática Rust (`None` = Auto). Sem perda
funcional; ganho em clareza idiomática.

`Side` enum (Left/Top/Right/Bottom) e `Binding` enum
(Left/Right) são tipos similares-em-espírito mas distintos —
usados em `PageElem` vanilla e adicionáveis em refino futuro
per ADR-0061 §6.3 sem necessidade de unificação.

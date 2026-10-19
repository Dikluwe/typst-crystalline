# Relatório — typst-passo-841: `layout::em` — `Sub` de `Length` ausente (#31) + braços da mesma família (#36, #37)

**Data:** 2026-07-22
**Executor:** Kimi Code (agente principal — prompt lido de `00_nucleo/materialization/typst-passo-841.md`).
**Proveniência das medições:** commit HEAD `68181efbe` (P840) + alterações deste passo na working tree (`01_core/src/engine/eval/operators.rs`, `00_nucleo/prompts/engine/eval/ops.md`). Medição "antes" com o binário release de P840; "depois" com o binário rebuildado pós-implementação.
**Baseline da suíte (antes):** `cargo test -p typst-core` → **4567 passed; 0 failed**; 2 ignored (medido em P840).

**Nota de âmbito (decisão do executor, permitida pelo prompt §Passo 1.2):** os achados #36 (`Angle - Angle`) e #37 (`fr + fr`) são da mesma família de #31 — `operators.rs` com lista incompleta de braços aritméticos — e foram implementados **neste mesmo passo** (uma revisão só de `operators.rs`). O âmbito de #36/#37 dentro de P842 (`layout` define) fica assim coberto: P842 não os reimplementa, apenas regista esta cobertura. Registado também no L0 `engine/eval/ops.md`.

---

## Medição antes (saída literal, `#repr(expr)`)

```text
2em - 5em   => cris(1): erro: cannot apply Sub to length and length | van(0): '-3em'
10pt - 3pt  => cris(1): idem                                       | van(0): '7pt'
1cm - 5mm   => cris(1): idem                                       | van(0): '14.17pt'
90deg - 45deg => cris(1): erro: cannot apply Sub to angle and angle | van(0): '45deg'
1fr + 2fr   => cris(1): erro: cannot apply Add to fraction and fraction | van(0): '3fr'
6pt + 1em - 2pt => cris(1): erro (composto)                        | van(0): '4pt + 1em'
(2em + 3pt  => cris(0): '3pt + 2em' | van(0): '3pt + 2em' — já em paridade, controlo)
```

## Código identificado

- Cristalino: `01_core/src/engine/eval/operators.rs` — `eval_binary_op` com `Add` para `Length` (`:382`) mas sem `Sub`; sem braços para `Angle` e `Fraction` (caíam no erro genérico da fronteira, `:532`).
- Vanilla: `lab/typst-original/crates/typst-library/src/foundations/ops.rs:196` (`Length(a - b)`), `:194` (`Angle(a - b)`), `:127` (`Fraction(a + b)`).

## Testes primeiro (RED confirmado)

3 testes novos no módulo `#[cfg(test)]` de `operators.rs` — `p841_sub_length` (2em−5em = −3em; 10pt−3pt = 7pt), `p841_sub_angle` (90deg−45deg = 45deg), `p841_add_fraction` (1fr+2fr = 3fr). Antes da implementação: **3 failed** (`0 passed; 3 failed`).

## Diff

`01_core/src/engine/eval/operators.rs` — 3 braços novos em `eval_binary_op`:
- `(Sub, Length, Length) => Length(a - b)` (a estrutura `Length` já implementa `Sub` em `entities/layout_types.rs:882` — componentes abs/em separadas);
- `(Sub, Angle, Angle) => Angle::rad(a.to_rad() - b.to_rad())`;
- `(Add, Fraction, Fraction) => Fraction(a + b)`.

L0 `00_nucleo/prompts/engine/eval/ops.md` — nova secção "Braços aritméticos P841" + registo retroativo dos braços `Neg` de P832 (#59: Angle/Ratio/Fraction; `Duration` pendente de decisão de escopo, ver relatório de P832). Hash refeito (`a6f9df18`).

## Medição depois (saída literal, binário release rebuildado)

```text
2em - 5em   => cris(0): '-3em'      | van(0): '-3em'
10pt - 3pt  => cris(0): '7pt'       | van(0): '7pt'
1cm - 5mm   => cris(0): '14.17pt'   | van(0): '14.17pt'
90deg - 45deg => cris(0): '45deg'   | van(0): '45deg'
1fr + 2fr   => cris(0): '3fr'       | van(0): '3fr'
6pt + 1em - 2pt => cris(0): '4pt + 1em' | van(0): '4pt + 1em'
```

## Validação

- `cargo test -p typst-core` → **4570 passed; 0 failed**; 2 ignored. Cálculo: 4567 (baseline) + 3 testes novos = 4570. ✔
- `crystalline-lint .` → **exit 0, zero violations**.

# Relatório P186A — Diagnóstico Equation locatable

**Data**: 2026-05-03
**Magnitude**: S (diagnóstico puro)
**Pré-condição**: P185 série fechada; ADR-0068 ACEITE; tests
workspace 1.783 verdes; zero violations.

---

## §1 Escopo

P186A é o passo de diagnóstico-primeiro que precede a
implementação P186B+. Replica registo de
P181A/P182A/P183A/P184A/P185A.

P186A produziu 2 artefactos documentais e zero código:

- `00_nucleo/diagnosticos/diagnostico-equation-locatable-passo-186a.md` (8 secções).
- `00_nucleo/materialization/typst-passo-186a-relatorio.md` (este ficheiro, 14 secções).

Sem ADR nova. Sem DEBT novo. Sem código tocado.

---

## §2 Inputs verificados empiricamente (8 grep/read)

| # | Input | Resultado |
|---|-------|-----------|
| 1 | Forma do `Content::Equation` | `{ body: Box<Content>, block: bool }` (`content.rs:84-87`) |
| 2 | `ElementPayload` variants | 9 variantes; **Equation ausente** (`element_payload.rs:33-105`) |
| 3 | `ElementKind` variants | 8 variantes; **Equation ausente** (`element_kind.rs:16-37`) |
| 4 | `is_locatable(Content::Equation)` | `false` (`locatable.rs:60`) |
| 5 | `extract_payload(Content::Equation)` | catch-all `_ => None` (`extract_payload.rs:83`) |
| 6 | `from_tags` arm Equation | ausente |
| 7 | Walk legacy Equation | `if block && is_numbering_active("equation") → step_flat` (`introspect.rs:377-382`) |
| 8 | Vanilla typst | `Counter::of(EquationElem::ELEM)` chave única (`lab/.../math/equation.rs:234`) |

Crítico descoberto: **C2 consumer já tem `is_numbered`
override-com-fallback** (`equation.rs:25-33`); comentário
inline confirma que *introspector path nunca activa para
equation* até `Content::SetEquationNumbering` materializar.

---

## §3 Decisões cláusulas 1–6 (resumo)

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Forma `ElementPayload::Equation` | **Opção B**: `{ block: bool, counter_update: CounterUpdate }` (paralelo Figure P184B) |
| 2 | Sub-store alvo | **Opção 1**: `CounterRegistry` (reuso) |
| 3 | Convenção de chave | **Opção A**: `"equation"` simples (paridade legacy + vanilla) |
| 4 | Auto-init | **Não necessário** (`apply_at` já defensivo via `or_insert`) |
| 5 | Forma migração C2 (P188) | substitution-with-fallback `flat_counter_at("equation", current_location).or_else(legacy)` |
| 6 | Critério de fecho P186 | **Opção 2**: variant + locatable + extract_payload + from_tags + tests E2E |

---

## §4 Plano de sub-passos B–F (sem condicionais)

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | `ElementPayload::Equation` + `ElementKind::Equation` + L0s | S |
| `.C` | `is_locatable(Equation) = true` + L0 | trivial |
| `.D` | `extract_payload` arm | S |
| `.E` | `from_tags` arm com gate `block && state.numbering_active:equation` | S |
| `.F` | Tests E2E + relatório consolidado | S |

---

## §5 Magnitude agregada

**P186 série = S puro** (4×S + 1×trivial).

Diferente de P185 (M agregado por P185C arquitectural). P186
é replicação de padrão P181/P182C/P184B em 4 sítios
uniformes.

---

## §6 Dependências de outros passos

### §6.1 — Pré-requisitos (cumpridos em P185)

- `flat_counter_at(key, location)` no trait `Introspector`
  (P185B).
- `current_location: Option<Location>` no Layouter (P185C).
- Sincronização Locator validada (P185D).
- ADR-0068 ACEITE (P185E).

### §6.2 — Dependentes (P186 desbloqueia)

- **P188 (C2 migration)** — usa
  `flat_counter_at("equation", current_location)`
  populated por P186E.

### §6.3 — Independente

- **P187 (C1 migration)** — depende de P185 mas não de
  P186. Pode prosseguir em paralelo.

---

## §7 ADR avaliação

**Sem ADR criada.** Promoção a locatable é refino dentro de
ADR-0026 (Content enum fechado). Replicação de padrão
P181F (Bibliography) + P178 (Outline) + P182C
(SetHeadingNumbering). Decisão atómica fica registada no
diagnóstico §2.

Caso futuro (extensão a outros math elements como
`MathDelimited`, `MathMatrix`) precisaria de ADR — fora de
escopo P186.

---

## §8 DEBT avaliação

**Sem DEBT novo.**

DEBT M4-residual cumulativo (cobre apenas C1 + C2) **não
fecha em P186** — fecha após P187 + P188.

Condição pré-existente herdada (não criada por P186):
`Content::SetEquationNumbering` ausente → introspector path
dormente até materializar. Documentação inline em
`equation.rs:25-29` + L0 layout + diagnóstico §1.9.

---

## §9 Restrições honradas

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Sem reservas de identificadores**.
- **Não modifica `is_locatable`** (P186C).
- **Não adiciona variant a `ElementPayload`** (P186B).
- **Não adiciona arm em `extract_payload`** (P186D).
- **Não adiciona arm em `from_tags`** (P186E).
- **Não migra C2** (P188).
- **Sem inflação retórica**.
- **Sem cláusulas condicionais nos sub-passos**.

---

## §10 Verificações

- ✅ `cargo check --workspace` passa (sem código tocado).
- ✅ `cargo test --workspace` passa: **1.783** inalterado vs
  P185.
- ✅ `crystalline-lint .` zero violations.
- ✅ Diagnóstico produzido (8 secções).
- ✅ Relatório produzido (este ficheiro).
- ✅ Sem ADR nova.
- ✅ Sem DEBT novo.

---

## §11 Achados não-triviais

### §11.1 — `Content::Equation` é minimal

Apenas `{ body, block }`. Sem `numbering: Option<...>` que
Figure tem. Numeração de equação é controlada externamente
via state → counter.

### §11.2 — `Content::SetEquationNumbering` é o elo perdido

A infra `numbering_active:equation` no StateRegistry é
populada apenas por `Content::StateUpdate` directo (raro)
ou por `Content::SetEquationNumbering` (que não existe).

Em produção actual, intr.state para `"numbering_active:equation"`
é sempre `None`. Logo a infra P186 fica **dormente** em
produção até equation-set-rule materializar (passo dedicado
fora da série P186-P188).

Este é design intencional documentado em
`equation.rs:25-29` e L0 layout — não é regressão.

### §11.3 — `ElementKind::Equation` exige adição

8 variantes existentes não cobrem Equation. P186B precisa
adicionar a variante junto com `ElementPayload::Equation`.

### §11.4 — Vanilla simplifica chave

`Counter::of(EquationElem::ELEM)` em vanilla é counter
único (sem sub-kind). Confirma que cláusula 3 deve usar
`"equation"` simples.

### §11.5 — Gate em `from_tags` arm é não-trivial

Diferente de Figure (que increment unconditional em P184B
porque consumer usa figure_progress + idx),
Equation precisa de gate `block && state-active` para
preservar paridade com legacy. Caso contrário, valores em
block-equation locations divergiriam quando há equations
inline ou block-não-numeradas.

Em produção (sem state populado), gate nunca dispara → counter
nunca populado → fallback legacy carrega. Caminho dormente
mas correcto por construção.

---

## §12 Snapshot pós-P186A

- **Tests workspace**: 1.783 (inalterado).
- **Trait `Introspector`**: 18 métodos (inalterado).
- **Layouter**: `locator` + `current_location` (inalterado).
- **ADR-0068**: ACEITE (P185E).
- **DEBT M4-residual**: cobre C1 + C2 (inalterado; fecha
  com P187 + P188).
- **48 passos executados**.
- **Padrão diagnóstico-primeiro**: 11ª aplicação
  (P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A).

---

## §13 Próximo passo

**P186B** — adicionar variant `ElementPayload::Equation` +
`ElementKind::Equation`:

- Editar `01_core/src/entities/element_payload.rs` —
  variant nova.
- Editar `01_core/src/entities/element_kind.rs` — variante
  nova.
- Actualizar L0s `entities/element_payload.md` +
  `entities/element_kind.md`.
- Tests unit dos enums.

Magnitude: S puro. Sem cláusulas condicionais.

---

## §14 Conclusão

P186A fechou 6 cláusulas com decisão literal e plano em 5
sub-passos sem condicionais. Magnitude S agregada confirmada
para a série P186 inteira. ADR avaliada e dispensada
(replicação de padrão). DEBT avaliado e dispensado (herda
condição pré-existente, não cria nova).

Padrão diagnóstico-primeiro mantido — 11/11 acertaram a
magnitude planeada ±1 nível.

P186B–F materializa promoção concreta de `Content::Equation`
a locatable, desbloqueando eixo 2 do bloqueio P183C.
Combinado com P185 (ADR-0068 ACEITE = eixo 1 desbloqueado),
fica caminho livre para P188 migrar C2.

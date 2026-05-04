# Diagnóstico — Walk puro M5 (Passo P189A)

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico)
**ADR vinculada**: nenhuma (replicação de padrão estabelecido).
**Pré-condição**: M4-residual fechado funcionalmente (P188B);
tests workspace 1.808; zero violations.

---

## §1 Validação do estado actual + regra dos 2 eixos

### §1.1 — Inventário empírico de mutações em walk

`grep` empírico em `01_core/src/rules/introspect.rs` (linhas
< 600, walk fn body):

| Arm | Linha | Mutação | Field |
|-----|-------|---------|-------|
| `Heading` | 349 | `state.step_hierarchical("heading", *level)` | `hierarchical` |
| `Heading` | 352 | `state.auto_label_counter += 1` | `auto_label_counter` |
| `Heading` | 367 | `state.resolved_labels.insert(auto_label, ...)` | `resolved_labels` |
| `Heading` | 372 | `state.headings_for_toc.push((auto_label, frozen, level))` | `headings_for_toc` |
| `Equation` | 379 | `state.step_flat("equation")` (gated `block && active`) | `flat` |
| `Figure` | 391-396 | `state.local_figure_counters` step + `state.figure_numbers.push` | `local_figure_counters`, `figure_numbers` |
| `Labelled` | 435 | `state.figure_label_numbers.insert(label, n)` | `figure_label_numbers` |
| `Labelled` | 451 | `state.resolved_labels.insert(label, text)` | `resolved_labels` |
| `SetHeadingNumbering` | 456 | `state.numbering_active.insert("heading", *active)` | `numbering_active` |
| `CounterUpdate` | 462-464 | `state.step_hierarchical` ou `step_flat` | `hierarchical`, `flat` |
| `CounterUpdate` (Update) | ? | `state.update_flat(key, val)` | `flat` |
| `Outline` | 611 | `state.has_outline = true` | `has_outline` |
| `Bibliography` | — | (puro pós-P181H ✅) | — |
| `Cite` | — | (puro ✅) | — |

Total: **6 arms com mutações** (Heading, Equation, Figure,
Labelled, SetHeadingNumbering, CounterUpdate, Outline).

### §1.2 — Aplicação da regra dos 2 eixos

Para cada mutação:
- **Eixo 1**: consumer downstream precisa do valor durante
  walk (mutável) ou snapshot final (TagIntrospector)?
- **Eixo 2**: sub-store correspondente é populado em
  produção?

| Arm.field | Eixo 1 | Eixo 2 | Conclusão |
|-----------|--------|--------|-----------|
| Heading.hierarchical | **walk** (Labelled lê para resolver) | counter populado por from_tags P170 ✅ | bloqueado por Labelled |
| Heading.auto_label_counter | walk (cada Heading lê para gerar `auto-toc-N` único) | sem sub-store | **excepção** |
| Heading.resolved_labels | walk (Layouter Ref-arm lê) | **sem sub-store** | **excepção (Reserva 2)** |
| Heading.headings_for_toc | walk (outline.rs consome) | sem sub-store (lacuna #3) | **excepção** |
| Equation.flat | walk (Labelled lê para `Equação ({n})`) | counter populado por from_tags P186E (gate dormente) | **excepção (Reserva 1)** |
| Figure.local_figure_counters | walk (interno ao arm) | redundante — substituível por sub-store | bloqueado por Labelled |
| Figure.figure_numbers | walk (Labelled lê para `Figura {n}`) | populado por from_tags P184B ✅ | bloqueado por Labelled |
| Labelled.figure_label_numbers | walk (`layout_ref` lê) | populado por from_tags P168 ✅ | C4 fechado (P184D); migrável **se Labelled puro** |
| Labelled.resolved_labels | walk (Ref-arm lê) | **sem sub-store** | **excepção (Reserva 2)** |
| SetHeadingNumbering.numbering_active | walk (Heading arm lê) | populado por from_tags StateUpdate P182C ✅ | bloqueado se Heading lê durante walk; pode migrar se Heading.hierarchical migrar |
| CounterUpdate.step_* | walk (escopo CounterDisplay) | counter populated por from_tags? **a verificar** | depende cobertura `from_tags` |
| Outline.has_outline | **snapshot final** (caller verifica para iniciar fixpoint TOC) | `query_by_kind(Outline) ✅ | **migrável trivialmente** |

### §1.3 — Cadeia de dependências bloqueia migração universal

Descoberta crítica empírica: a maioria das mutações é
gated por **Labelled**, que precisa de ler counters
durante walk para popular `resolved_labels`. Cadeia:

1. `Heading` walk muta `state.hierarchical`.
2. `Labelled { target: Heading, label }` lê
   `state.format_hierarchical("heading")` durante walk →
   produz `"Secção 1.2"`.
3. `state.resolved_labels.insert(label, "Secção 1.2")`.
4. Layouter `Content::Ref { target: label }` lê
   `state.resolved_labels[label]` para renderizar texto.

Idem para Equation e Figure: Labelled lê
`state.figure_numbers` ou `state.get_flat("equation")`
durante walk → popula `resolved_labels`.

**Consequência**: tornar walk puro para Heading.hierarchical
ou Figure.figure_numbers exige primeiro tornar Labelled
puro. Labelled puro exige sub-store para `resolved_labels`
(que não existe — **Reserva 2 alargada**).

### §1.4 — Reserva 1 confirmada empiricamente

`grep -rn "SetEquationNumbering" 01_core/src/`: zero hits
em produção. Apenas referências em comentários (per
P186A §11.2 e P188 §5).

Walk arm Equation linha 377-382 muta
`state.step_flat("equation")` gated por `state.is_numbering_active("equation")`.
Sem `Content::SetEquationNumbering`, `numbering_active["equation"]`
é populado externamente em testes (`numbering_active.insert("equation", true)`).

**Reserva 1 confirma-se**: Equation arm não pode ser puro
sem `SetEquationNumbering` materializar primeiro.

### §1.5 — Reserva 2 alargada — toca todos os Heading/Labelled arms

P183E não corrido. C4 (resolved label TOC) lê legacy em
runtime. Mas a análise §1.3 mostra que **a Reserva 2 é
mais alargada** do que C4 isolado:

- Heading walk arm popula `resolved_labels` (auto-toc).
- Labelled walk arm popula `resolved_labels` (explicit
  labels).
- Layouter Ref-arm lê `state.resolved_labels`.

Migrar qualquer destes exige sub-store para
`resolved_labels` E migração do consumer (C4). Trabalho
não-trivial.

### §1.6 — Conclusão honesta sobre escopo M5

**Apenas 1 arm é trivialmente migrável**:
`Outline.has_outline` → `query_by_kind(Outline)` (P178 já
forneceu sub-store; consumer outline.rs pode adoptar).

Restantes 11+ mutações precisam de:
- Reserva 1 (`SetEquationNumbering` materializado) — 1
  arm.
- Reserva 2 alargada (sub-store `resolved_labels` + C4
  migration) — 5+ arms.
- Sub-store `headings_for_toc` (lacuna #3 não fechada) — 1
  arm.
- Auditoria de `from_tags` arm CounterUpdate — pode estar
  pendente.

---

## §2 Decisões cláusulas 1–7

### §2.1 — Cláusula 1: lista exacta de arms não-puros

**Decisão fixada**: lista empírica em §1.1. Total 6 arms
(Heading, Equation, Figure, Labelled, SetHeadingNumbering,
CounterUpdate, Outline) com 11+ mutações distintas.

### §2.2 — Cláusula 2: estratégia por arm

| Arm | Opção | Justificação |
|-----|-------|--------------|
| `Outline.has_outline` | **α** (já é locatable; remover mutação directa) | snapshot final; sub-store funcional via P178 |
| `Heading.*` (4 mutações) | **δ** (excepção — Reserva 2 alargada) | depende sub-store `resolved_labels` + C4 migration |
| `Equation.flat` | **δ** (excepção — Reserva 1) | depende `SetEquationNumbering` |
| `Figure.*` | **δ** (excepção — Reserva 2 chained) | Labelled lê `figure_numbers` durante walk |
| `Labelled.*` | **δ** (excepção — Reserva 2 alargada) | depende sub-store `resolved_labels` + C4 |
| `SetHeadingNumbering.numbering_active` | **δ** (excepção — chained) | Heading arm lê durante walk; só pode migrar se Heading migrar |
| `CounterUpdate.*` | **a auditar empiricamente em P189B `.A`** | depende cobertura `from_tags` |

**Conclusão**: apenas 1 arm migrável (Outline). 5+ arms
ficam excepcionados.

### §2.3 — Cláusula 3: sub-store por field

| Field legacy | Sub-store / mecanismo | Status para M5 |
|-------------|----------------------|----------------|
| `numbering_active` | `StateRegistry` (P182C) | activo ✅ — mas walk muta paralelamente |
| `figure_numbers`, `figure_label_numbers` | `CounterRegistry` (P184B) + `figure_label_numbers` map | activo ✅ |
| `flat["equation"]` | `CounterRegistry` (P186E) | dormente em produção (Reserva 1) |
| `bib_entries`, `bib_numbers` | `BibStore` (P181) | activo ✅ |
| `resolved_labels` | **sem sub-store** | **bloqueador M5 universal** |
| `headings_for_toc` | sem sub-store | lacuna #3 não fechada |
| `auto_label_counter` | sem sub-store; counter local ao walk | refactor possível |
| `has_outline` | `query_by_kind(Outline)` (P178) | activo ✅ |
| `local_figure_counters` | redundante com `figure_numbers` | refactor trivial |

### §2.4 — Cláusula 4: backward compat durante transição

**Decisão fixada**: **Opção A modificada** — `from_tags`
popula sub-store; legacy permanece campo de
`CounterStateLegacy` (consumers M4-fallback ainda lêem em
caso de Introspector vazio).

Para arm Outline (único migrável):
- `from_tags` arm Outline popula `kind_index[Outline]` ✅.
- Walk arm Outline deixa de mutar `state.has_outline`.
- Consumer (`outline.rs::layout_outline` ou caller que
  inicia fixpoint TOC) passa a consultar
  `intr.kind_index.contains_key(&ElementKind::Outline)`.
- Field `state.has_outline` em `CounterStateLegacy` fica
  morto (cleanup orgânico em M6).

### §2.5 — Cláusula 5: `Content::Styled`

**Confirmado empiricamente** (linha 608):
```rust
Content::Styled(body, _) => walk(body, state, locator, tags, None),
```

**Puro por design**. Não muta state. Sem trabalho.

### §2.6 — Cláusula 6: excepções declaradas

**5 excepções identificadas**:

#### Excepção 1 — `Equation.step_flat` (Reserva 1)

- Justificação: `Content::SetEquationNumbering` ausente
  (P186A §11.2; P188 §5).
- Documentação obrigatória (4 pontos):
  1. Comentário inline em `introspect.rs:377-382`.
  2. Secção em L0 `rules/introspect.md`.
  3. Test sentinela paridade walk legacy ↔ Layouter
     `(N)` rendering.
  4. Secção em P189 consolidado.
- Fechamento: quando `SetEquationNumbering` materializar.

#### Excepção 2 — `Heading.*` + `Labelled.resolved_labels` (Reserva 2 alargada)

- Justificação: sub-store para `resolved_labels` ausente;
  C4 não migrado (P183E não corrido). Cadeia bloqueia
  todas as mutações Heading/Labelled relacionadas com
  resolved_labels.
- Documentação obrigatória (4 pontos):
  1. Comentário inline em `introspect.rs:348-376`,
     `407-455`.
  2. Secção em L0.
  3. Test sentinela paridade.
  4. Secção em P189 consolidado.
- Fechamento: quando sub-store `resolved_labels` for
  criado E C4 migrado.

#### Excepção 3 — `Figure.figure_numbers` + `Labelled.figure_label_numbers` (Reserva 2 chained)

- Justificação: walk arm Labelled lê `state.figure_numbers`
  durante walk para popular `figure_label_numbers`.
  `figure_label_numbers` é populado paralelamente por
  from_tags arm Figure (P184B is_counted), mas walk arm
  Labelled ainda muta directamente.
- Sub-store existe (P168/P184B) — mas migração exige
  ordem (Figure pure precisa Labelled pure precisa
  resolved_labels sub-store).
- Documentação idêntica a Excepção 2.

#### Excepção 4 — `Heading.headings_for_toc`

- Justificação: sem sub-store (lacuna #3 não fechada).
  TOC consome este field directamente.
- Pode fechar como passo independente (criar sub-store
  + migrar consumer).

#### Excepção 5 — `SetHeadingNumbering.numbering_active`

- Justificação: walk arm Heading lê `state.is_numbering_active("heading")`
  durante walk para resolver auto-toc text. Removendo
  mutação aqui rompe a cadeia.
- Migrável **após** Heading walk arm migrar.

### §2.7 — Cláusula 7: critério de fecho M5

**Decisão fixada**: **Opção 3** — `grep` zero matches em
arms migráveis + tests E2E paridade.

Critério literal:
```bash
grep -E "state\.\w+\s*[=.]" 01_core/src/rules/introspect.rs |
  awk -F: '$2 < 600' |
  grep -v "^#" |
  grep -vE "(// .*excepção|state\.numbering_active.*equation|state\.numbering_active.*heading|state\.flat.*equation|state\.resolved_labels|state\.headings_for_toc|state\.figure_label_numbers|state\.figure_numbers|state\.local_figure_counters|state\.auto_label_counter|state\.is_numbering_active|state\.format_hierarchical|state\.get_flat)"
```

(Excluindo as 5 excepções declaradas.)

Tests E2E:
- Outline walk arm puro: `Content::Outline` no doc → walk
  emite Tag → `from_tags` populates `kind_index[Outline]`
  → `state.has_outline` permanece `false` em walk; consumer
  lê `intr.kind_index.contains_key(...)` e funciona
  identicamente.

---

## §3 Plano de sub-passos

**Magnitude consolidada P189**: triviais para excepções
+ S para migração Outline.

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Migrar `Outline.has_outline` arm: walk pure + consumer adapta a `query_by_kind(Outline)` + L0 + test paridade | S |
| `.C` | Documentar 5 excepções declaradas (4 pontos cada) + tests sentinela paridade walk legacy ↔ output observable + actualizar nota DEBT M5-residual | S |
| `.D` | Relatório consolidado P189 com §"Excepções M5" e snapshot de arms restantes | S |

3 sub-passos. Total agregado ~50 LOC produção (apenas
Outline migration) + ~150 LOC tests + ~200 LOC documentação.

**M5 fecha com 1 arm migrado e 5 excepções declaradas.**
P189 não fecha M5 universalmente — fecha **a primeira
peça** (Outline) e **documenta honestamente** o que falta
e porquê.

---

## §4 Magnitude consolidada

P189 série: **S agregado**. 3 sub-passos triviais a S
cada.

Diferente de P186 (S agregado em 6 sub-passos para
infraestrutura) e P187/P188 (S em 1 sub-passo agregado).
P189 cobre 1 migração + documentação extensiva de
excepções.

---

## §5 ADR avaliação

**Sem ADR criada.** Justificação:
- Padrão estabelecido (P162/P165/P181E) replicado para
  Outline.
- Excepções são honestidade documental, não decisão
  arquitectural.
- Cadeia de dependências (Reserva 2 alargada) é facto
  empírico, não decisão.

Eventualmente, ADR pode formalizar "M5 é incremental,
não universal" — mas decisão fica para passo subsequente.

---

## §6 DEBT avaliação

### Cenário B (replica M4-residual)

**Sem DEBT formal aberto**. Nota preventiva no relatório
consolidado P189:

> DEBT M5-residual cobre 5 excepções declaradas:
> 1. Equation walk arm — depende `SetEquationNumbering`.
> 2. Heading walk arm + auto-toc — depende sub-store
>    `resolved_labels` + C4 migration.
> 3. Labelled walk arm + resolved_labels — depende
>    Excepção 2.
> 4. Heading.headings_for_toc — depende sub-store TOC
>    (lacuna #3 fechar).
> 5. SetHeadingNumbering chained — depende Excepção 2
>    fechar primeiro.
>
> Quando excepções fecharem, walk fica universalmente
> puro; M5 fecha; segue M6 (eliminação `CounterStateLegacy`).

---

## §7 Excepções declaradas

5 excepções listadas em §2.6. Documentação obrigatória em
4 pontos para cada (per Q6) — materializada em P189B `.C`
(documentação massa) e `.D` (relatório consolidado).

Resumo:

| # | Arm | Reserva | Fechamento |
|---|-----|---------|------------|
| 1 | `Equation.step_flat` | 1 | `SetEquationNumbering` materializado |
| 2 | `Heading.*` (4 mutações) + `Labelled.resolved_labels` | 2 alargada | sub-store `resolved_labels` + C4 |
| 3 | `Figure.figure_numbers` + `Labelled.figure_label_numbers` | 2 chained | Excepção 2 fechada primeiro |
| 4 | `Heading.headings_for_toc` | sub-store TOC | lacuna #3 fechar |
| 5 | `SetHeadingNumbering.numbering_active` | Excepção 2 chained | Excepção 2 fechada primeiro |

`Content::Styled`: puro por design, sem excepção
necessária.

`CounterUpdate`: a auditar empiricamente em P189B `.A`
(decisão pendente).

---

## §8 Próximo sub-passo

**P189B** — migrar Outline arm + documentar 5 excepções +
tests sentinela:

- Editar `01_core/src/rules/introspect.rs:611`:
  - Substituir `state.has_outline = true` por `// has_outline
    via query_by_kind(Outline) — walk puro M5`.
  - (Mutação removida; `from_tags` arm Outline P178 já
    popula `kind_index[Outline]`.)
- Editar consumer:
  - Localizar quem lê `state.has_outline` em
    `01_core/src/rules/layout/mod.rs` (per P181H linha
    1423: `if !initial_state.has_outline { ... }`).
  - Substituir por consulta Introspector ou equivalente.
- Adicionar 5 comentários de excepção inline + L0 +
  tests sentinela.
- Actualizar nota DEBT M5-residual.

Magnitude: S. Sem cláusulas condicionais nas excepções
(já decididas).

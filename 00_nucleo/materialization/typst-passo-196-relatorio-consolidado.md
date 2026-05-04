# Relatório Consolidado — Série P196

**Data**: 2026-05-03
**Sub-passos**: A ✅ B ✅ C ✅
**Magnitude agregada**: S diagnóstico (P196A) + M materialização (P196B) + S documental (P196C)
**Estado**: Série fechada
**Pattern arquitectural protagonista**: ADR-0069 — segunda aplicação concreta

---

## §1 Resumo executivo

Série P196 materializa a **segunda aplicação concreta** do
pattern ADR-0069 (post-recursion tag emission), aplicada ao
walk arm `Content::Heading` para resolver E2 — a maior das 6
excepções M5 declaradas em P189A (4 mutações encadeadas).

**Resultado estrutural**:
- E2 fecha 3 das 4 mutações estruturalmente via emissão de
  `Tag::Labelled` auto-toc pós-recursão.
- 1 mutação residual (`state.headings_for_toc.push`) declarada
  formalmente como **E2-residuo**, bloqueada por lacuna #3
  (sub-store `intr.headings_for_toc` ausente).
- Caminho Introspector torna-se **universal para resolved labels**
  — auto-toc (P196B) + explicit (P195D) + figure-ref (P168) — em
  todos os 3 casos consumer C4 P194B recebe `Some(text)` do
  Introspector path; fallback legacy persiste mas raramente
  disparado em produção.

**Resultado funcional**: output observable em produção
inalterado (write paralelo M5 preserva paridade legacy↔Introspector).

**Pattern ADR-0069 consolidado com 2 variantes operacionais**:
- **P195D variant** — target não-locatable (`Labelled`):
  snapshot+find_map para reuso de Location do recurse.
- **P196B variant** — content locatable (`Heading`):
  `emitted_loc` directo do walk top.

---

## §2 Sub-passos materializados

| Sub | Magnitude planeada | Magnitude real | Δ tests | L0 tocado |
|-----|-------------------|----------------|---------|-----------|
| **P196A** (diagnóstico) | S puro | S | 0 | 0 |
| **P196B** (walk arm) | M | M | +5 | 1 (`introspect.md` hash `3bc33823`) |
| **P196C** (encerramento) | S puro | S | 0 | 0 |

**Totais série**:
- 3 sub-passos.
- +5 testes E2E novos (todos em `introspect.rs::tests::*` ramo P196B).
- 5 tests existentes adaptados (4 tags em vez de 2 por Heading).
- 1 L0 tocado.
- 1 helper privado novo (`compute_heading_auto_toc`).
- 0 ADRs novas (ADR-0069 já ACEITE em P195E).
- 0 sub-stores novos.
- 0 variants novas em `ElementPayload` (Opção 1: reuso).

---

## §3 Decisões arquitecturais

7 cláusulas P196A fechadas + decisão de helper (P196B):

1. **Cláusula P196A §11.1 — `emitted_loc` directo**: como
   Heading é locatable (`extract_payload(Heading)` retorna
   `Some(ElementPayload::Heading{…})`), o walk top emite
   `Tag::Start` na entrada e guarda `Some(loc)` em
   `emitted_loc`. A arm pode então reusar essa variável
   directamente sem snapshot+find_map (variant P195D
   só era necessário porque `Labelled` é não-locatable).

2. **Cláusula P196A §11.2 — `ElementPayload::Labelled`
   cobre auto-toc semanticamente**: auto-label sintetizada
   (`auto-toc-N`) tem mesma estrutura semântica que label
   explícita — apontador para texto resolvido. Reuso da
   variant existente sem novo enum case (Opção 1
   confirmada).

3. **Cláusula P196A §11.3 — lacuna #3 mantida**:
   `headings_for_toc` requer sub-store dedicado
   (`Vec<(Label, Content, usize)>` ou variant em Tag). Não
   resolvida em P196 — declarada como E2-residuo formal.

4. **Cláusula P196A §11.4 — mutações 1+2 são write paralelo
   necessário**: `state.step_hierarchical("heading", level)`
   e `state.auto_label_counter += 1` continuam activas
   porque `compute_heading_auto_toc` lê `state.format_hierarchical`
   e precisa do counter atualizado, e o helper consome
   `auto_label_counter` para sintetizar `Label("auto-toc-N")`.
   Não migram para Tag — fecham em M6 quando legacy for
   removido.

5. **Cláusula P196A §11.5 — Tag auto-toc partilha Location
   com Tag Heading**: reuso explícito de `emitted_loc`
   garante sincronização-por-construção ADR-0068. Walk
   Locator e Layouter Locator não avançam para Labelled
   sintética em ambos os lados — apenas a variant
   `ElementPayload::Labelled` é "anexada" à mesma
   Location do Heading.

6. **Cláusula P196A §11.6 — helper privado para reuso
   entre legacy e Tag**: `compute_heading_auto_toc(state, n)`
   isola lógica de computação para que mutação legacy
   (`state.resolved_labels.insert`) e populate Tag
   (`ElementPayload::Labelled { resolved_text, … }`)
   consumam mesma fonte. Reduz duplicação e garante
   paridade write paralelo.

7. **Cláusula P196A §11.7 — Tag::End com hash=0**: pattern
   ADR-0069 fixa `Tag::End(loc, 0)` para Labelled emitida
   pós-recursão (não há `Content` distinto a hashar — a
   Tag é meta-informação anexa). Filter em testes
   (`hash != 0`) para distinguir End real do End
   sintético.

**Decisão P196B §3 — helper retorna concrete `(Label,
String)` em vez de `Option<…>`**: paridade legacy preserva
insert de `auto_label → ""` quando numbering inactivo
(presença é informativa, não conteúdo). Helper não devolve
`Option<String>` — `String::new()` quando inactivo. Tag
emitida com `resolved_text: Some("")` para preservar
insert no sub-store.

---

## §4 Achados não-triviais durante execução

### Achado A1 — `emitted_loc` directo simplifica dramaticamente face a P195D

P195D usou `let tags_len_before = tags.len();` antes da
recursão, depois `find_map` no range novo para identificar
a primeira Location emitida pelo `target`. P196B é mais
simples: como Heading é locatable, walk top já emitiu
`Tag::Start(loc, …)` antes de entrar no match arm, então
`emitted_loc: Option<Location>` (Location é Copy) está
disponível directamente. Variant P196B preferível sempre
que content for locatable.

### Achado A2 — `ElementPayload::Labelled` cobre auto-toc semanticamente

Sem novo variant em `ElementPayload`. Auto-label sintetizada
(`Label("auto-toc-N")`) é tratada idêntica a label explícita
do ponto de vista do consumer C4 — `intr.resolved_label_for`
retorna `Some(text)` sem distinção.

### Achado A3 — lacuna #3 mantida; E2-residuo documentado

Sub-store `intr.headings_for_toc` ausente em P196B. Outline
consumer (`outline.rs:24`) lê directamente de
`state.headings_for_toc` (legacy). Migração precisa de:
(a) decidir estrutura do sub-store (Vec ou variant em Tag);
(b) `from_tags` arm popular o sub-store;
(c) consumer migrar.

Decisão P196A: **fora do escopo P196**. Passo dedicado
paralelo. E2-residuo declarada formalmente.

### Achado A4 — mutações 1+2 são write paralelo necessário

`state.step_hierarchical` e `state.auto_label_counter += 1`
não migram para Tag. Razões:
- helper consome `auto_label_counter` para sintetizar
  `Label("auto-toc-N")` — counter precisa estar avançado
  no momento da chamada;
- `compute_heading_auto_toc` lê `state.format_hierarchical`
  para computar prefix — requer state mutado;
- ambos fecham orgânicamente em M6 (eliminação
  `CounterStateLegacy`).

### Achado A5 — Tag auto-toc partilha Location com Tag Heading

`emitted_loc` reusada em ambos pares Tag::Start/End do
Heading. Sequência:
1. `Tag::Start(loc, Heading)` — walk top.
2. `Tag::Start(loc, Labelled auto-toc)` — arm pós-recursão.
3. `Tag::End(loc, 0)` — arm pós-recursão.
4. `Tag::End(loc, hash_content(heading))` — walk bottom.

Bracketing válido (verificado por `bracketing_valido_em_sequencia_plana` adaptado).

### Achado A6 — helper retorna concrete `(Label, String)` em vez de `Option`

Paridade legacy preserva insert de `auto_label → ""` quando
numbering inactivo. Helper retorna `(Label, String)` directo
(`String::new()` quando inactivo) — Tag emitida com
`resolved_text: Some("")` para preservar insert no
sub-store via `from_tags`.

### Achado A7 — 5 tests existentes adaptados via padrão pragmático auditor #1

Tests assumiam 2 tags por Heading. P196B adiciona 2 tags
(par Labelled auto-toc) → total 4 por Heading. Adaptação
via update do count esperado e pattern matching dos tags;
sem alterar fixture do conteúdo. Padrão pragmático auditor
#1: ajustar fixture é trivial; sem necessidade de gate
substancial.

### Achado A8 — sequência exacta de tags registada para Heading e Heading-com-Figure

Documentada em L0 §"Walk arm Heading migrado (P196B,
ADR-0069)" — referência futura para P197/P198 quando
aplicarem pattern ADR-0069 a Figure/SetHeadingNumbering/CounterUpdate.

---

## §5 Estado dormente vs activo

### Activo em produção (P196B materializa)

- **Auto-toc Heading via Introspector path**: consumer C4
  (`references.rs:53-67`) recebe `Some(text)` para `auto-toc-N`
  via `intr.resolved_label_for(label)` — primeira branch da
  expressão `or_else`. Caminho Introspector activo.
- **`intr.resolved_labels` populated universalmente**:
  - Auto-toc (P196B): sintetizada via walk arm Heading.
  - Explicit (P195D): label do user via walk arm Labelled.
  - Figure-ref (P168 + P195D combinados): label sobre
    Figure capturado via walk arm Labelled.
- **Inversão observable parcial completa para resolved
  labels**: 3 dos 3 casos típicos cobertos. Fallback
  legacy raramente disparado em produção (apenas casos
  edge: target não-locatable em Labelled wrapper sem Tag::Start
  no range).

### Dormente / continua legacy

- **E2-residuo activo** — `state.headings_for_toc.push((auto_label,
  frozen_body, level))` no walk arm Heading (linha 452):
  - Sub-store `intr.headings_for_toc` ausente (lacuna #3).
  - Consumer `outline.rs:24` lê directamente do legacy.
  - Sentinela em test `walk_e2_residuo_headings_for_toc_via_legacy`
    confirma 3 entries para 3 headings.
  - Fecha em passo dedicado abrir sub-store
    `intr.headings_for_toc`.

- **Mutações 1, 2, 3 de E2 activas como write paralelo M5**:
  - `state.step_hierarchical("heading", level)` (linha 438).
  - `state.auto_label_counter += 1` (linha 440).
  - `state.resolved_labels.insert(auto_label.clone(), resolved_text.clone())`
    (linha 445).
  - Cleanup orgânico em M6 quando `CounterStateLegacy` for
    eliminado e consumers migrarem totalmente para Introspector.

- **E1, E3, E5, E6 inalteradas** — outras 4 excepções M5
  continuam activas com pré-requisitos próprios:
  - E1 (Equation): `Content::SetEquationNumbering` ausente
    (Reserva 1).
  - E3 (Figure): cadeia com E2-residuo (Labelled lê
    `figure_numbers` durante walk).
  - E5 (SetHeadingNumbering): cadeia com E2/E3 (Heading
    arm lê `is_numbering_active` durante walk).
  - E6 (CounterUpdate): cadeia com E2/E3.

---

## §6 Estado final M9 e M5

### Marco M9 (Introspector capabilities)

| Métrica | P195E | P196B | Δ |
|---------|-------|-------|---|
| Variants `ElementPayload` | 11 | 11 | 0 |
| Variants `ElementKind` | 9 | 9 | 0 |
| Métodos trait `Introspector` | 19 | 19 | 0 |
| Sub-stores `TagIntrospector` | 8 | 8 | 0 |
| Tests workspace | 1.838 | 1.843 | +5 |

M9 estável: 11/11 elementos materializáveis cobertos
(inalterado vs P195B per Opção 1 — reuso da variant
`Labelled`).

### Marco M5 (walk-puro progressão)

| Arm | Estado pré-P196 | Estado pós-P196 |
|-----|-----------------|-----------------|
| Outline | migrado (P189B) | migrado |
| Bibliography | migrado (P181H) | migrado |
| Labelled | migrado estruturalmente (P195D) | migrado estruturalmente |
| **Heading** | activa (E2, 4 mutações) | **migrado parcialmente** (E2 → E2-residuo, 1 mutação) |
| Figure | activa (E3) | activa (E3) |
| Equation | activa (E1) | activa (E1) |
| SetHeadingNumbering | activa (E5) | activa (E5) |
| CounterUpdate | activa (E6) | activa (E6) |

**Excepções M5 activas após P196**: 4 + 1 residuo
(E1, E2-residuo, E3, E5, E6).

---

## §7 Estado final lacunas

| # | Lacuna | Pré-P196 | Pós-P196 |
|---|--------|----------|----------|
| #1 | Figure kind=None ↔ Introspector | activa | activa (sem trabalho em P196) |
| #2 | reservada (não atribuída) | — | — |
| #3 | `headings_for_toc` sub-store ausente | activa | **activa, agora bloqueia E2-residuo** (formalizada) |
| #4 | reservada | — | — |
| #5 | `formatted_counter` Introspector | resolvida (P170) | resolvida |

Lacuna #3 ganha proeminência: passa de "limitação ambient"
para "bloqueador formal de E2-residuo". Passo dedicado
paralelo abrir sub-store é critical-path para fechamento M5
universal.

---

## §8 Pendências cumulativas + DEBT M5-residual

### Pendências série P196

- ✅ A — diagnóstico empírico walk arm Heading.
- ✅ B — walk arm migrado + helper + L0 + 5 tests.
- ✅ C — auditoria + relatório consolidado + nota DEBT.

### DEBT M5-residual — estado actualizado

> **Antes P196**: 5 excepções activas (E1, E2, E3, E5, E6); 2 pré-requisitos M5-residual restantes.
>
> **Após P196B**: **4 excepções activas + 1 residuo**:
> - E1 — Reserva 1 (`Content::SetEquationNumbering` ausente).
> - **E2-residuo** — `headings_for_toc.push` (lacuna #3 bloqueia fechamento total; sub-store ausente).
> - E3 — Figure walk arm.
> - E5 — SetHeadingNumbering walk arm.
> - E6 — CounterUpdate walk arm.
>
> **2 pré-requisitos restantes** (inalterado vs P195):
> - Sub-store `intr.headings_for_toc` (lacuna #3). **Fecha E2-residuo**.
> - `Content::SetEquationNumbering`. **Fecha E1**.
>
> **3 das 4 mutações de E2 estruturalmente fechadas**.
> Caminho Introspector universal para resolved labels
> (auto-toc + explicit + figure-ref). Mutação legacy
> preservada como fallback durante janela compat M5;
> cleanup orgânico em M6.

**Cenário B continua** (sem DEBT formal aberto). Notas
preventivas só.

---

## §9 Próximos passos sugeridos

### Imediato (próxima série)

- **P197A — diagnóstico walk arm Figure** (E3 fecha):
  magnitude S esperada (3ª aplicação reduz incerteza).
  Figure é locatable → variant P196B aplicável (`emitted_loc`
  directo). Pattern ADR-0069 3ª aplicação concreta.

### Encadeamento M5 universal

- **P198 — walk arm SetHeadingNumbering + CounterUpdate**
  (E5 + E6 fecham): magnitude M agregada esperada. Ambos
  são locatable parcialmente (StateUpdate emitida via
  `extract_payload`).
- **Passo dedicado abrir sub-store `intr.headings_for_toc`**
  (fora série P196/P197/P198): fecha **E2-residuo**.
  Decisão pendente sobre estrutura (Vec vs variant em Tag).
- **Materialização `Content::SetEquationNumbering`** (passo
  dedicado, paralelo a P197/P198): fecha E1.

### Após sequência completa

- Walk torna-se universalmente puro.
- M5 fecha.
- Segue M6 (P200 ou P190): eliminação de
  `CounterStateLegacy`. Passo agregado removendo:
  - Mutações legacy walk paralelas em todos os arms.
  - Wrapper `introspect()` (passa a ser
    `introspect_with_introspector`).
  - Field-by-field consumers em Layouter.

---

## §10 Linhagem

- **Pattern arquitectural**: ADR-0069 (PROPOSTO em P195B,
  ACEITE em P195E).
- **Variantes operacionais consolidadas em série P196**:
  - P195D variant — target não-locatable: snapshot+find_map.
  - **P196B variant** — content locatable: `emitted_loc` directo.
- **Helper análogo**: `compute_labelled` (P195D) ↔
  `compute_heading_auto_toc` (P196B).
- **Sub-store consumido**: `intr.resolved_labels` (P193B
  abriu; P194B consumer migrou; P195D + P196B populated
  via Tag::Labelled).
- **Consumer C4**: `references.rs:53-67` substitution-with-fallback
  (P194B). Recebe Some via Introspector path para
  auto-toc + explicit + figure-ref pós-P196B.
- **L0 tocado**: `00_nucleo/prompts/rules/introspect.md`
  hash `3bc33823`.
- **Código tocado**: `01_core/src/rules/introspect.rs`
  hash `73489ae5`.
- **Padrão diagnóstico-primeiro**: 18ª aplicação consecutiva
  (P196A diagnóstico antes de P196B materialização).

---

## §11 Métricas finais

- **Sub-passos**: 3 (A diagnóstico + B materialização + C encerramento).
- **LOC produção**: ~30 (helper) + ~10 (walk arm) = ~40.
- **LOC teste**: ~150 (5 tests E2E novos) + ~50 (5 adaptações).
- **LOC L0**: ~80 (secção nova "Walk arm Heading migrado P196B" + actualização tabela Excepções).
- **LOC relatórios**: ~250 (P196A diagnóstico + P196A relatório + P196B relatório + P196 consolidado).
- **Variants ElementPayload novas**: 0.
- **Sub-stores novos**: 0.
- **ADRs novas**: 0.
- **Excepções M5 fechadas**: 1 (E2 → E2-residuo, 3/4 mutações).
- **Tests netos adicionados**: +5.
- **Hashes desactualizados**: 1 → 0 (corrigido por `--fix-hashes`).

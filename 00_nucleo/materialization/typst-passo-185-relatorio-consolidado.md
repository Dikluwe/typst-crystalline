# Relatório consolidado — Série P185

**Período**: 2026-05-03 (P185A–E executados no mesmo dia)
**Magnitude agregada**: M (dominada por P185C; outros são S)
**Estado**: ✅ Série fechada (A ✅ B ✅ C ✅ D ✅ E ✅)
**ADR vinculada**: ADR-0068 — `ACEITE` em P185E.

---

## §1 Resumo executivo

A série P185 materializou o **mecanismo M3 da ADR-0068**:
Layouter location-aware via `Locator` dedicado determinístico
e field `current_location: Option<Location>`. Resolve eixo 1
da regra dos 2 eixos (snapshot-during-walk) que bloqueava C1
heading prefix (P183B) e C2 equation counter (P183C).

Decisão arquitectural materializada: ao contrário de M2
(parâmetro propagado) que cascataria mudanças de assinatura
em ~30 métodos do Layouter, M3 isola toda a mudança a 2
fields no struct + 1 helper privado de gating. Sincronização
com walk de introspect garantida **por construção** via
determinismo do `Locator` (provado em P161) + invariante
`is_locatable ↔ extract_payload.is_some()` (estabelecido em
P164).

Custo real:
- P185B: +180 LOC (trait methods + impls + 10 tests).
- P185C: +37 LOC em `mod.rs` (2 fields, helper, gating).
- P185D: +192 LOC tests E2E.

Δ tests cumulativo: **+14** (1769 → 1783) com **zero
regressões**.

ADR-0068 transitou `PROPOSTO` → `ACEITE` após validação
empírica em P185D (4 tests confirmam sincronização literal
em pipeline real).

**Output observable em produção inalterado** — P185 é
infraestrutural; consumers C1/C2 migram em P187/P188.

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | L0s tocados | ADRs |
|-------|---------------------|-----------------|---------|-------------|------|
| **P185A** | M (diagnóstico) | M | 0 | nenhum (cria diagnóstico + ADR) | ADR-0068 PROPOSTO |
| **P185B** | S | S | **+10** | `entities/introspector.md` | — |
| **P185C** | M (genuíno) | M | 0 | `rules/layout.md` | — |
| **P185D** | S | S | **+4** | nenhum (só tests) | — |
| **P185E** | S (documental) | S | 0 | nenhum (só ADR + relatório) | ADR-0068 ACEITE |
| **Total** | — | — | **+14** | 2 L0 produção | 1 ADR transitada |

Detalhe Δ tests: 1769 (baseline P184F) → 1779 (P185B) → 1779
(P185C, Δ 0 esperado para infra sem consumer) → 1783 (P185D)
→ 1783 (P185E sem código).

---

## §3 Decisões arquiteturais

### ADR-0068 transitou PROPOSTO → ACEITE

Justificação literal registada no `Histórico`:
- 4 tests E2E em `mod p185d_locator_sync` passam.
- Sequências de Locations Layouter ↔ walk coincidem.
- Custo real P185C 37 LOC vs estimativa 30 LOC: 1.23× (dentro
  ±50% per critério §3).

### Cláusulas P185A fechadas (6/6)

P185A documentou 6 cláusulas pendentes; P185B–E fecharam
todas:

| # | Cláusula | Decisão | Sub-passo |
|---|----------|---------|-----------|
| 1 | Inicialização do Locator | Opção A — `Locator::new()` em `Layouter::new` | P185C |
| 2 | Gating em `layout_content` | Opção α — atómico no topo via helper | P185C |
| 3 | Save/restore para scoping | Opção 2 — monotónico (sem save/restore); aligna com walk | P185C |
| 4 | Mecanismo de instrumentação para tests | Opção B — iteração manual em tests sem hook em produção | P185D |
| 5 | Tipo de `current_location` | `Option<Location>` (não sentinel `from_raw(0)`) | P185C; validado em P185D test `.D` |
| 6 | Trait methods location-aware adicionais | 2 novos: `is_numbering_active_at` + `flat_counter_at` | P185B |

### Divergência face ao pseudocódigo da ADR (cláusula gate trivial)

ADR-0068 sugeriu save/restore de `prev_loc` em `layout_content`.
P185C executou Opção 2 (avanço monotónico) por simetria
literal com walk de introspect. Documentado em "Notas
pós-aceitação" da ADR. Test E2E P185D `.B` confirmou
sincronização nesta forma.

---

## §4 Achados não-triviais durante execução

### P185B — test re-update com assert de divergência explícito

`is_numbering_active_at_re_update_reflecte_location_consultada`
inclui assert explícito de divergência face a
`is_numbering_active` (snapshot final): após
`init(Bool(true), loc(10))` + `update(Bool(false), loc(20))`,
o test confirma:
- `is_numbering_active_at(key, loc(15))` = true (snapshot
  pré-update)
- `is_numbering_active_at(key, loc(25))` = false (snapshot
  pós-update)
- `is_numbering_active(key)` = false (snapshot final)

Sem este assert de contraste, o test seria semánticamente
equivalente a um test de P171 e o valor diferencial
(motivação central da ADR-0068) perderia-se. Pattern a
replicar em futuros tests location-aware (P187/P188).

### P185C — `Option<Location>` em vez de sentinel `Location::from_raw(0)`

`Location` não tem `Default` derivado e `from_raw` é
`pub(crate)`. Sentinel `Location::from_raw(0)` cria
ambiguidade: é também a Location real produzida pelo primeiro
`Locator::next()`. Solução: `current_location: Option<Location>`,
`None` antes do primeiro locatable, `Some(loc)` após. P185D
test `.D` valida empiricamente.

### P185D — instrumentação Opção B sem hook em produção

Captura da sequência de `current_location` do Layouter foi
implementada via **iteração manual de `Vec<Content>`** com
`layout_content` chamado por part:

```rust
for part in parts {
    layouter.layout_content(part);
    if is_locatable(part) {
        locs.push(layouter.current_location.expect("..."));
    }
}
```

Funciona porque tests no mesmo crate L1 acedem `pub(super)`
via `use super::*`. Cada part top-level locatable tem
`body: Empty` ou conteúdo não-locatable, garantindo que
`current_location` estabiliza no valor da part após a chamada
(recursão interna não altera). Opção C (expor método público
no Layouter) ficou disponível como cláusula de escape mas
não foi necessária.

### P185D — agregação via `kind_index.values()` para capturar Locations do walk

`walk` em `introspect.rs:321` é privado; `Tag` não é exposto.
Em vez disso, helper agrega `intr.kind_index.values()` e
ordena por `Location::as_u128()`:

```rust
fn collect_walk_locations(intr) -> Vec<Location> {
    let mut all: Vec<Location> = intr.kind_index
        .values().flatten().copied().collect();
    all.sort_by_key(|l| l.as_u128());
    all
}
```

Recupera ordem global do walk porque `Locator` é
monotonicamente crescente (provado em
`locator.rs:counter_e_monotonico_crescente`). Sem
necessidade de tocar `walk` ou expor tags.

---

## §5 Estado final M9 e M5/M4

### M9 (counter-feature) — inalterado: 11/11

P185 não introduz feature M9 nova. Os métodos
`is_numbering_active_at` + `flat_counter_at` são **extensão
location-aware** de `is_numbering_active` (P182B) e
`formatted_counter` (P170) — não slot novo. Slot 11 livre
mantém-se.

### M5/M4 (read-site migration) — inalterado: 6/12

P185 não migra read-sites. C1 + C2 ainda bloqueados, mas
agora pela razão **certa** (consumer pendente em P187/P188),
não pela razão **estrutural** (infra ausente).

### Trait `Introspector` — 18 métodos (era 16)

| Categoria | Antes P185 | Após P185 |
|-----------|------------|-----------|
| Total métodos | 16 | **18** |
| Location-aware | 4 (`state_value`, `formatted_counter_at`, `figure_number_at_index`, …) | **6** (+`is_numbering_active_at`, +`flat_counter_at`) |

### Layouter — 2 fields novos

```rust
pub(super) locator:           Locator,
pub(super) current_location:  Option<Location>,
```

Ambos `pub(super)` — acessíveis a tests no crate L1 e a
consumers internos do Layouter. Sem mudança de API pública.

---

## §6 Estado final lacunas

Inalterado em P185. As 7 lacunas catalogadas até P184F
permanecem na mesma situação (algumas resolvidas, outras
pendentes). P185 não foi sobre lacunas — foi sobre
infraestrutura para desbloquear consumers.

---

## §7 Pendências cumulativas

### Activas

- **P183B (C1 heading prefix)** — depende de P187, blueprint
  literalizado no test P185D `.E`
  (`pipeline_e2e_is_numbering_active_at_via_current_location`).
- **P183C (C2 equation counter)** — depende de P186
  (Equation locatable) + P188 (consumer migration).
- **4 sites M4-fora-de-escopo** (TOC, fixpoint side-channels,
  resolved labels) — fora de escopo P185.

### Resolvidas estruturalmente

- **Pendência P182E §5.2** — eixo 1 da regra dos 2 eixos:
  resolvida ao nível **infraestrutural** em P185.
  Resolução **funcional** completa fica para P187 (C1) +
  P188 (C2) que materializam consumers.

---

## §8 Próximos passos sugeridos

1. **P186 — Equation locatable**: pré-requisito para C2.
   Promove `Content::Equation` em `is_locatable`,
   `extract_payload`, `from_tags`. Magnitude S.

2. **P187 — Migrar C1 (heading prefix)**: substitui
   `self.counter.format_hierarchical("heading")` em
   `mod.rs:310` por `self.introspector.formatted_counter_at("heading",
   self.current_location.unwrap())`. Padrão substitution-with-fallback
   per ADR-0061. Blueprint literal no test P185D `.E`.
   Magnitude S–M.

3. **P188 — Migrar C2 (equation counter)**: depende P186 +
   P185. Substitui `state.get_flat("equation")` em
   `equation.rs:97` por `self.introspector.flat_counter_at("equation",
   self.current_location.unwrap())`. Magnitude S–M.

4. **P189+ — Após P186+P187+P188**: M4-residual fechado;
   DEBT M4-residual fecha; segue M5 com novos read-sites
   ou M9 slot 11 (próxima feature counter).

---

## §9 Conclusão

P185 fechou limpamente a cinco sub-passos com magnitudes
correctamente estimadas (S+S+M+S+S). A primeira ADR ACEITE
da fase M4-residual está em livro (ADR-0068), e fornece
template para mecanismos location-aware futuros (ADR-0067
attribute-grammar pode generalizar `current_location` para
attributes herdados adicionais como cor/dir/lang).

Achados centrais:
- **Sincronização-por-construção** (mecanismo M3) funciona
  empiricamente — não foi necessário recuar para M2 (parâmetro
  propagado, magnitude M-L) ou M1 (Locator partilhado, com
  acoplamento desnecessário).
- **Custo M3 alinhou com estimativa**: ~30 LOC estimados, 37
  LOC reais. Multiplicador 1.23×; sem escalada de magnitude.
- **Tests E2E sem hook em produção** — instrumentação Opção
  B (iteração manual de parts) provou-se suficiente. P185D
  fica como precedente de "validação arquitectural via
  pipeline real sem alterar produção".

A série P185 termina como **primeira série completa
M-magnitude desde P165** (M3 introspection trait). Padrão
diagnóstico-primeiro aplicado pela 10ª vez
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A) — todos
os 10 acertaram a magnitude planeada ±1 nível.

**48 passos executados** após P185E. Próximo passo: P186A
(diagnóstico promoção `Content::Equation` a locatable).

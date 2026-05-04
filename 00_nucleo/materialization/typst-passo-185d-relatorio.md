# Relatório P185D — Tests E2E sincronização Locator

**Data**: 2026-05-03
**Magnitude**: S puro (executada como S — apenas tests, sem
toque em código de produção)
**Pré-condição**: P185C concluído ✅ (Layouter ganhou `locator`
+ `current_location` + gating em `layout_content`).

---

## Resumo

4 tests E2E adicionados em `01_core/src/rules/layout/tests.rs`,
agrupados em `mod p185d_locator_sync`. Validam empiricamente a
sincronização-por-construção entre Locator do Layouter (P185C)
e Locator do walk de introspect — invariante central da
ADR-0068 mecanismo M3.

Mecanismo de instrumentação **Opção B** (per `.A`): teste
itera parts manualmente, chamando `layout_content` por cada
um, capturando `current_location` após cada arm locatável.
**Sem alteração de produção.**

Os 4 tests:

1. **`sincronizacao_locator_layouter_iguala_walk_introspect`**
   (caso central) — Heading + Figure + Cite. Walk emite 3
   tags; Layouter avança 3 vezes. Sequências comparadas
   elemento por elemento.
2. **`gating_locator_apenas_em_locatables`** — mistura
   [Heading, Text, Figure, Equation, Cite]. Walk emite 3 tags
   (skip Text + Equation); Layouter idem. Confirma gating
   uniforme.
3. **`current_location_none_antes_de_primeiro_locatable`** —
   `Layouter::new()` tem `current_location = None`; Text +
   Space não actualizam; Heading actualiza para
   `Some(loc(0))`. Valida decisão de tipo `Option<Location>`
   (P185C §"Tipo do field").
4. **`pipeline_e2e_is_numbering_active_at_via_current_location`**
   — pipeline completo: SetHeadingNumbering(true) + 3 Headings.
   Para cada heading processado, valida que
   `intr.is_numbering_active_at("numbering_active:heading",
   layouter.current_location.unwrap())` retorna `true`. **É o
   blueprint literal que P187 vai usar para migrar C1.**

Sincronização empiricamente confirmada — ADR-0068 candidata
a transitar PROPOSTO → ACEITE em P185E.

---

## Confirmação `.F` (8/8)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` passa (Δ vs P185C 1779: +4) | ✅ 1783 verdes |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | Tests `p185d_*` passam isoladamente | ✅ 4/4 verdes |
| 5 | Tests existentes não regridem | ✅ Δ é puramente novo |
| 6 | Output observable em produção inalterado | ✅ P185D não toca produção |
| 7 | Snapshot tests ADR-0033 verdes | ✅ incluídos em workspace |
| 8 | Linter passa final | ✅ |

---

## Δ tests vs baseline

- Baseline P185C: **1779** verdes
- Após P185D: **1783** verdes
- Δ: **+4** (limite superior do range esperado [+3, +4]).

Todos os 4 tests do mod `p185d_locator_sync` passam à primeira.

---

## Hashes finais

**Zero edits ao L0** — P185D é exclusivamente tests, conforme
restrição. Hashes registados em P185C mantidos:
- L0 layout: `20d03fe5`
- Código layout: `2b8010ce`
- L0 introspector: `070a390f`
- Código introspector: `0938d161`

`crystalline-lint .` confirma 0 drift warnings.

---

## Decisões de execução notáveis

### Mecanismo de instrumentação — Opção B (sem mudança de produção)

A captura da sequência de `current_location` do Layouter foi
implementada via **iteração manual de `Vec<Content>`** com
`layout_content` chamado por part:

```rust
fn collect_layout_locations(parts: &[Content]) -> Vec<Location> {
    let mut layouter = Layouter::new(FixedMetrics, NullImageSizer, 12.0);
    let mut locs = Vec::new();
    for part in parts {
        layouter.layout_content(part);
        if is_locatable(part) {
            locs.push(layouter.current_location.expect("..."));
        }
    }
    locs
}
```

Funciona porque:
- Tests no mesmo crate L1 acedem `pub(super)` via
  `use super::*` (per pattern existente de `layout/tests.rs`).
- Cada part top-level locatable tem `body: Empty` ou
  conteúdo não-locatable, garantindo que `current_location`
  estabiliza no valor da part após a chamada (recursão
  interna não altera).

Opção C (expor método público no Layouter) ficou disponível
como cláusula de escape mas não foi necessária — Opção B
viável directamente.

### Captura de Locations do walk via `kind_index`

`walk` em `introspect.rs:321` é privado; tags não são
expostas. Em vez disso, helper `collect_walk_locations`
agrega `intr.kind_index.values()` e ordena por
`Location::as_u128()`:

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
`locator.rs:counter_e_monotonico_crescente`). Sem necessidade
de tocar `walk` ou expor tags.

### `Content::Equation` confirmado não-locatable empiricamente

Test `.C` inclui `Content::Equation { body: Empty, block: false }`.
Verificação directa: walk emite **3** tags (não 4), Layouter
avança **3** vezes (não 4) — Equation não dispara gating.
Confirma cobertura do `is_locatable` (P185A §3.5; promoção a
locatable fica para P186).

### `current_location` antes do primeiro locatable é `None`

Test `.D` valida directamente:
- `Layouter::new()` → `current_location == None`.
- `layout_content(Text)` → `current_location == None`.
- `layout_content(Space)` → `current_location == None`.
- `layout_content(Heading)` → `current_location == Some(loc(0))`.

Decisão de tipo `Option<Location>` (P185C §"Tipo do field")
empiricamente justificada — `Location::from_raw(0)` como
sentinel teria criado ambiguidade indistinguível com a
Location real do primeiro locatable.

### Sem cláusula gate substancial disparada

- Sequências A (walk) e B (Layouter) coincidem ✓
  sincronização-por-construção empiricamente correcta.
- Mecanismo de instrumentação Opção B viável ✓ — não foi
  necessário recuar para P185C e expor hook.
- Tests existentes não regridem ✓ — P185D é aditivo.

---

## Estado actual

- **P185 série**: A ✅ B ✅ C ✅ D ✅ | E pendente.
- **Sincronização ADR-0068 validada empiricamente** — 4 tests
  passam. ADR-0068 candidata a ACEITE em P185E.
- **Layouter location-aware validado** — pronto para consumer
  migration em P187 (C1) e P188 (C2).
- **Trait `Introspector`**: 18 métodos (inalterado).
- **Tests workspace**: 1779 → 1783 (+4).
- **47 passos executados** (após P185C).
- **M5/M4 progresso**: 6/12 read-sites migrados (inalterado).
- **DEBT M4-residual**: cobre apenas C1 + C2 (inalterado).

---

## Pendências cumulativas

Inalteradas em P185D:

- P183B (C1 heading prefix migration) — depende de P187,
  blueprint literalizado em test `.E` deste passo.
- P183C (C2 equation counter migration) — depende de P188.
- 4 sites M4-fora-de-escopo — fora de escopo P185.

---

## Próximo passo

**P185E** — encerramento da série P185:

1. Transição **ADR-0068 PROPOSTO → ACEITE** (validação
   empírica de P185D habilita).
2. Relatório consolidado da série (P185A + B + C + D + E).
3. Actualização de cross-refs em ADRs e prompts L0
   relacionados.

Após P185E, série P185 fecha. Os passos P187 (C1) e P188 (C2)
podem prosseguir, agora com:
- Trait methods location-aware disponíveis (P185B).
- Layouter com `current_location` populado (P185C).
- Sincronização-por-construção validada (P185D, este passo).
- Decisão arquitectural ratificada (ADR-0068 ACEITE em P185E).

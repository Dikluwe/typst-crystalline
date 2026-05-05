# Diagnóstico — Walk pipeline redesign (P191A)

**Data**: 2026-05-05
**Contexto**: P190F §3 identificou barreira arquitectural — walk fn não tem acesso a Introspector durante execução.
**Magnitude diagnóstico**: S-M.
**Estado empírico**: confirmado por leitura directa.

---

## §1 Validação estado actual

### §1.1 Workspace baseline

- Tests: 1.855 verdes.
- Linter: zero violations.
- M5 universal completo (P200B).
- M6 série P190 em pausa após P190F.

### §1.2 Pipeline walk actual

`introspect.rs:78` — `introspect_with_introspector`:

```rust
pub fn introspect_with_introspector(
    content: &Content,
    engine:  Option<&mut Engine<'_>>,
    ctx:     Option<&mut EvalContext>,
) -> (CounterStateLegacy, TagIntrospector) {
    let mut state = CounterStateLegacy::new();
    let mut locator = Locator::new();
    let mut tags: Vec<Tag> = Vec::new();
    walk(content, &mut state, &mut locator, &mut tags, None);
    let introspector = self::from_tags::from_tags(&tags, engine, ctx);
    (state, introspector)
}
```

**Sequência**: `walk` → `from_tags` → return. Introspector construído POST-walk.

### §1.3 Walk fn signature

`introspect.rs:460`:
```rust
fn walk(
    content:           &Content,
    state:             &mut CounterStateLegacy,
    locator:           &mut Locator,
    tags:              &mut Vec<Tag>,
    label_from_parent: Option<&Label>,
);
```

**Sem acesso a Introspector** — barreira P190F.

### §1.4 Inventário 4 helpers

| Helper | Localização | Reads de state durante walk | Categoria |
|--------|-------------|------------------------------|-----------|
| `compute_labelled` (P195D) | introspect.rs:326 | `state.flat`, `state.figure_numbers`, `state.lang` | walk-reader (state-dependent payload) |
| `compute_heading_auto_toc` (P196B) | introspect.rs:386 | `state.is_numbering_active`, `state.format_hierarchical` | walk-reader |
| `compute_figure` (P197B) | introspect.rs:419 | `state.local_figure_counters` | walk-internal (sem Introspector equivalente) |
| `compute_heading_for_toc` (P200B) | introspect.rs:452 | `state.auto_label_counter` | walk-internal (counter local) |

**2 helpers walk-readers** (state legacy → Introspector necessário).
**2 helpers walk-internal** (locais ao walk; sem dependência Introspector).

### §1.5 Walk arm gates

| Walk arm | Gate | Read |
|----------|------|------|
| Equation (introspect.rs:579) | `state.is_numbering_active("equation")` | `numbering_active` |

Outros walk arms não têm gates state-dependent.

### §1.6 `from_tags` arms inventário

`from_tags.rs:39` — `from_tags(&tags, engine, ctx) -> TagIntrospector`. Match sobre 12 ElementPayload variants:

| ElementPayload | Sub-store(s) populated |
|----------------|------------------------|
| Heading | counters.apply_hierarchical_at, kind_index |
| Figure | counters.apply_at + kind_index + figure_label_numbers |
| Citation | kind_index |
| Metadata | metadata + kind_index |
| State | state.init + kind_index |
| Outline | kind_index |
| Bibliography | bib_store + kind_index |
| StateUpdate | state.update + kind_index (auto-init) |
| Equation | counters.apply_at gated + kind_index |
| Labelled | resolved_labels + figure_label_numbers |
| CounterUpdate | counters.apply_at/apply_hierarchical_at + kind_index |
| HeadingForToc | headings_for_toc |

**from_tags = construtor único de Introspector**. Logic complexa por arm.

### §1.7 L0 alvos

- `rules/introspect.md` (signature change).
- Possivelmente novo L0 dedicado ao mecanismo (depende cláusula 1).

---

## §2 4 opções arquiteturais

### Opção A — Walk recebe `&mut TagIntrospector`

**Mecanismo**: Walk fn signature ganha `intr: &mut TagIntrospector` parameter. Walk arms replicam from_tags logic incrementalmente — populate Introspector durante walk em vez de POST-walk.

**Pipeline pós-A**:
```rust
let mut intr = TagIntrospector::empty();
walk(content, &mut state, &mut intr, &mut locator, &mut tags, None);
// from_tags becomes redundant — already populated.
```

**Walk arm Heading exemplo**:
```rust
Content::Heading { level, body } => {
    state.step_hierarchical("heading", *level as usize);  // legacy ainda
    if let Some(loc) = emitted_loc {
        intr.counters.apply_hierarchical_at("heading", *level as usize, loc);  // NEW
        intr.kind_index.entry(ElementKind::Heading).or_default().push(loc);    // NEW
    }
    // Helper agora pode queryar intr:
    let resolved_text = compute_heading_auto_toc(intr, loc, *level as usize);
    ...
}
```

**Pros**:
- Single pass.
- Helpers podem queryar Introspector durante walk.
- Estrutura cognitivamente simples.

**Cons**:
- Duplica from_tags logic em walk arms (~60 LOC redundância inicial).
- from_tags pode ser eliminado depois (cleanup posterior).

**Magnitude**: M+ (signature change + 12 arm logic replicated + 1 helper migration).

### Opção B — Two-pass walk

**Mecanismo**: 1ª pass actual; build Introspector; 2ª pass com Introspector access; fill-in payloads.

**Pros**: Separação clara entre Tag emission e payload resolution.

**Cons**:
- 2 walks = double cost.
- State mutations applied twice ou skipped (precisa flag).
- Complex pipeline.

**Magnitude**: L (refactor maior).

### Opção C — Eliminate helpers; embed inline

**Mecanismo**: 4 helpers eliminados; lógica embedded em walk arms ou movida para from_tags.

**Pros**: Sem helpers para migrar.

**Cons**:
- Perde pattern ADR-0069 stylesheet (4 helpers consolidados em 7 séries).
- Walk arm Labelled bloated.
- Refactor cross-cutting.

**Magnitude**: M (eliminação) + L (refactor walks).

### Opção D — Deferred resolution Layouter-side

**Mecanismo**: Tags emitidas com payloads parciais. Resolution lazy at Layouter render time via Introspector queries.

**Pros**: Alinhado com vanilla typst lazy semantics.

**Cons**:
- Mudança semântica de Tag::Labelled (resolved_text passa a ser placeholder).
- Layouter precisa de novo mecanismo de resolution.
- Multiple consumers afetados.

**Magnitude**: M+ (semantic change cross-cutting).

---

## §3 Decisões cláusula 1–9

### Cláusula 1 — Mecanismo: **Opção A**

**Decisão**: **Opção A — walk recebe `&mut TagIntrospector`**.

**Justificação literal**:
- **Magnitude**: M+ — achievable em P191B único sub-passo.
- **Compatibilidade ADR-0069**: 5 variantes operacionais preservadas (helpers continuam a existir, agora queryam Introspector em vez de state).
- **Compatibilidade `from_tags`**: from_tags pode tornar-se no-op em P191B, eliminado em P191C ou P190I.
- **Alinhamento vanilla**: vanilla typst tem state mutável durante walk (similar). Não força lazy semantics como Opção D.
- **Reversibilidade**: trivial — restaurar signature anterior.

Opções B/C/D descartadas:
- **B descartada**: 2-pass tem custo prohibitivo + complexidade de state mutations.
- **C descartada**: perde 7 séries de pattern ADR-0069 stylesheet (P195D + P196B + P197B + P198B + P198C + P199B + P200B).
- **D descartada**: mudança semântica cross-cutting demasiada para passo M+.

### Cláusula 2 — Helpers a manter vs eliminar

| Helper | Decisão | Justificação |
|--------|---------|---------------|
| `compute_labelled` (P195D) | **Migrar** — passa a queryar `intr` via param | Pattern ADR-0069 preserved |
| `compute_heading_auto_toc` (P196B) | **Migrar** — idem | Pattern ADR-0069 preserved |
| `compute_figure` (P197B) | **Manter** — walk-internal; sem dependência Introspector | Reads `local_figure_counters` (state walk-only) |
| `compute_heading_for_toc` (P200B) | **Manter** — walk-internal; reads `auto_label_counter` (walk-only) | Idem |

### Cláusula 3 — Walk arm Equation gate

**Decisão**: migrar para `intr.is_numbering_active_at("numbering_active:equation", emitted_loc)` location-aware (P185B).

Pre-condição: walk fn tem `&mut TagIntrospector` (Opção A); StateRegistry populated incrementalmente quando walk processa SetEquationNumbering.

### Cláusula 4 — Compatibilidade ADR-0069 stylesheet

5 variantes operacionais ADR-0069 preservadas:
- P195D variante (não-locatable + snapshot+find_map): **inalterada** — usa Tags emit + helper.
- P196B variante (locatable + body): **inalterada** — usa emitted_loc + helper.
- Cenário α (P197B Figure, P198B SetHeadingNumbering): **inalterado**.
- Cenário α por construção (P199B SetEquationNumbering): **inalterado**.
- Cenário β-promote (P198C CounterUpdate): **inalterado**.

Pattern stylesheet stays. Helpers signature alterada mas pattern identity preservada.

### Cláusula 5 — `from_tags` compatibilidade

**Decisão**: from_tags eliminado em P191B (substituído por mutação directa de `intr` durante walk em cada arm).

Estado projectado pós-P191:
- `from_tags::from_tags()` removido ou reduzido a no-op.
- `introspect_with_introspector` simplificado:
  ```rust
  let mut intr = TagIntrospector::empty();
  walk(content, &mut state, &mut intr, &mut locator, &mut tags, None);
  (state, intr)
  ```
- Tags emit preservado (para outros consumers eventualmente).

### Cláusula 6 — Pre-condições populate timing

Para `intr.is_numbering_active_at("X", loc)` retornar valor correcto durante walk:
- `intr.state` deve ser populated antes da query.
- Walk arm SetX emits Tag::StateUpdate; nova logic chama `intr.state.update(...)` directamente.
- Walk continues processing; quando arm Equation gate query intr.is_numbering_active_at, valor já lá.

Para `intr.formatted_counter_at("heading", loc)`:
- `intr.counters.apply_hierarchical_at(...)` em walk arm Heading.
- Helper `compute_heading_auto_toc` query intr.counters depois.

Ordem natural preservada — populate antes de query, sequencial dentro do walk.

### Cláusula 7 — Estratégia migração

**Decisão**: P191B implementa mecanismo + migra **1 helper como prova de conceito** (`compute_heading_auto_toc`). Restantes 1 helper (`compute_labelled`) + walk arm Equation gate migrados em P191C ou P190G/H (após retomar série).

**Razão**: validar mecanismo Opção A com 1 helper antes de cascade.

### Cláusula 8 — Tests

- Tests existentes preservados via padrão pragmático auditor #1.
- Adaptação esperada: tests que dependem de from_tags directo podem precisar adaptação.
- Tests novos: validação de mecanismo Opção A (1-2 tests sentinela).

### Cláusula 9 — Critério fecho P191

P191 série fecha quando:
- Mecanismo Opção A implementado: walk fn signature tem `&mut TagIntrospector`.
- 12 walk arms replicam from_tags logic.
- 1 helper migrado (compute_heading_auto_toc) — prova de conceito.
- Walk arm Equation gate migrado.
- from_tags eliminado ou reduzido a no-op.
- Tests workspace verdes.
- ADR-0071 ACEITE.
- Pre-condição arquitectural cumprida para retomar P190G.

---

## §4 Plano de sub-passos sem condicionais

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| **P191B** | Implementar Opção A: walk fn signature `&mut TagIntrospector`; replicar 12 from_tags arms em walk arms; migrar 1 helper `compute_heading_auto_toc` como validação; migrar walk arm Equation gate; eliminar/no-op from_tags; tests novos + adaptados; L0 actualizada | M+ |
| **P191C** | Migrar 2º helper `compute_labelled`; cleanup; relatório consolidado P191; ADR-0071 ACEITE; lembrete formal retomar P190G | S-M |

**Total agregado**: M+ a L (depende de cleanup em P191C).

---

## §5 ADR-0071 PROPOSTO

**Título**: "Walk pipeline com Introspector acessível durante execução".

**Contexto**: P190F identificou barreira — walk fn não tem acesso a Introspector. Helpers compute_* leem state legacy. Eliminação de fields walk-readable de CounterStateLegacy bloqueada.

**Decisão**: **Opção A** — walk fn signature ganha `&mut TagIntrospector`. Walk arms replicam from_tags logic incrementalmente. Pattern ADR-0069 stylesheet preserved.

**Estado**: PROPOSTO em P191A. ACEITE em P191C após validação empírica.

**Detalhes em ADR-0071** (a criar como ficheiro separado).

---

## §6 DEBT + lembrete formal

P191 não fecha F1 (que requer P190I). Mas fecha barreira P190F §3.

**Lembrete formal CRÍTICO**: P190 série tem 3 sub-passos restantes (G, H, I) + cleanup 4 defers acumulados. Após P191 fechar, retomar P190G.

**Ficheiro dedicado para lembrete**: `00_nucleo/m1-lacunas-captura.md` ou novo `00_nucleo/p190-pause-resume-tracker.md`.

---

## §7 Compatibilidade com pattern ADR-0069 — análise

| Variante | Mecanismo actual | Impacto Opção A |
|----------|------------------|------------------|
| P195D (não-locatable + snapshot+find_map) | Walk arm Labelled chama compute_labelled(target, state) | Helper signature change — recebe intr+loc; pattern preserved |
| P196B (locatable + body) | Walk arm Heading chama compute_heading_auto_toc(state, n) | Idem |
| Cenário α (P197B, P198B) | Walk arm é declaração formal | Inalterado |
| Cenário α por construção (P199B) | Walk arm muta state.numbering_active | Walk arm passa a chamar intr.state.update directamente; mutação legacy preservada (write paralelo) ou eliminada |
| Cenário β-promote (P198C) | Walk arm muta state via apply_at-like | Idem |

**Conclusão**: pattern ADR-0069 stylesheet preserved. Apenas signatures mudam para passar Introspector.

---

## §8 Pre-condições para implementação P191B

1. Walk arms emit Tag::Start ANTES de chamar helpers (current behavior já garante).
2. `emitted_loc: Option<Location>` disponível em scope da arm (current).
3. TagIntrospector tem methods públicos (ou pub(crate)) suficientes para apply_at/etc.
4. `from_tags::from_tags` não dependente de logic externa que walk não pode replicar.

Verificação 4 empírica: from_tags depende de:
- ElementInfo (label_from_parent + payload).
- Tag struct.
- Engine + EvalContext (apenas para StateUpdate::Func).

Tudo replicável em walk.

---

## §9 Próximo sub-passo (P191B com escopo concreto)

**P191B — Implementar Opção A + 1 helper validation**:

1. Walk fn signature change:
   ```rust
   fn walk(
       content:           &Content,
       state:             &mut CounterStateLegacy,
       intr:              &mut TagIntrospector,  // NEW
       locator:           &mut Locator,
       tags:              &mut Vec<Tag>,
       label_from_parent: Option<&Label>,
   );
   ```

2. ~20 recursive walk calls actualizados (mecânico).

3. `introspect_with_introspector` simplificado.

4. 12 walk arms replicam from_tags logic:
   - Heading: `intr.counters.apply_hierarchical_at(...)` + `intr.kind_index.entry(...)`.
   - Figure: `intr.counters.apply_at(...)` + `kind_index`.
   - SetHeadingNumbering: `intr.state.update(...)`.
   - Equation, etc.

5. Migrar `compute_heading_auto_toc`:
   ```rust
   fn compute_heading_auto_toc<I: Introspector>(intr: &I, loc: Location, n: usize) -> (Label, String) {
       let auto_label = Label(format!("auto-toc-{}", n));
       let resolved_text = if intr.is_numbering_active_at("numbering_active:heading", loc) {
           intr.formatted_counter_at("heading", loc)
               .map(|prefix| format!("Secção {}", prefix))
               .unwrap_or_default()
       } else {
           String::new()
       };
       (auto_label, resolved_text)
   }
   ```

6. Migrar walk arm Equation gate:
   ```rust
   if *block && emitted_loc.map(|loc| intr.is_numbering_active_at("numbering_active:equation", loc)).unwrap_or(false) {
       state.step_flat("equation");  // legacy preserved até P190 retomar
       intr.counters.apply_at("equation", Step, emitted_loc.unwrap());
   }
   ```

7. `from_tags::from_tags` reduzido a no-op (ou eliminado se possível):
   ```rust
   pub fn from_tags(_tags: &[Tag], _engine: ..., _ctx: ...) -> TagIntrospector {
       // P191B: from_tags no-op — Introspector populated durante walk (Opção A).
       TagIntrospector::empty()
   }
   ```
   Actual: pode preservar populate logic como fallback durante migration window.

8. Tests workspace verdes (Δ esperado: 0 ou marginal — population happens at different time but result identical).

**Magnitude**: M+ (~80-150 LOC produção + adaptações).

**Critério de fecho P191B**: tests workspace 1.855 verdes; lint zero violations; mechanism funcional; helper migrado; ADR-0071 ainda PROPOSTO (ACEITE em P191C).

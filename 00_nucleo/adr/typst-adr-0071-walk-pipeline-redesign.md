# ADR-0071 — Walk pipeline com Introspector acessível durante execução

**Estado**: **ACEITE** (P191C — validação empírica completa após 2 helpers migrados + walk gate migrado).
**Data**: 2026-05-05 (PROPOSTA P191A); 2026-05-05 (ACEITE P191C).
**Pre-condição**: barreira arquitectural identificada em P190F §3.

---

## Contexto

Após **27 séries de execução** (P181-P200 + P190A-F), barreira arquitectural identificada em P190F:

> **Walk fn não tem acesso a `Introspector`** — Introspector é construído POST-walk via `from_tags::from_tags(&tags)`. Helpers chamados durante walk (`compute_labelled` P195D, `compute_heading_auto_toc` P196B) não podem queryar Introspector. Walk arm Equation gate (`if state.is_numbering_active("equation")`) também não pode.

**5 fields têm walk readers** (per P190F §3):
- `state.flat` — `compute_labelled` Equation arm.
- `state.hierarchical` — `compute_heading_auto_toc` (`format_hierarchical`).
- `state.figure_numbers` — `compute_labelled` Figure arm.
- `state.lang` — `compute_labelled` Figure supplement.
- `state.numbering_active` — `compute_heading_auto_toc` + walk arm Equation.

Eliminação completa de `CounterStateLegacy` (M6 final, P190I) **bloqueada** por estas dependências walk-side. Pattern stylesheet ADR-0070 ("eliminação write paralelo M5") atinge limite — cleanup incremental insuficiente.

**P190 série em pausa** após P190F (3 sub-passos restantes G/H/I + 4 defers acumulados). Retomar requer redesign walk pipeline.

---

## Decisão

**Opção A — Walk fn signature ganha `intr: &mut TagIntrospector` parameter**:

```rust
fn walk(
    content:           &Content,
    state:             &mut CounterStateLegacy,
    intr:              &mut TagIntrospector,  // NEW (ADR-0071)
    locator:           &mut Locator,
    tags:              &mut Vec<Tag>,
    label_from_parent: Option<&Label>,
);
```

Walk arms replicam `from_tags` logic incrementalmente — populate Introspector durante walk em vez de POST-walk.

**Pipeline pós-ADR-0071**:
```rust
let mut intr = TagIntrospector::empty();
walk(content, &mut state, &mut intr, &mut locator, &mut tags, None);
// from_tags eliminado — intr já populated.
(state, intr)
```

**Walk arm Heading exemplo**:
```rust
Content::Heading { level, body } => {
    state.step_hierarchical("heading", *level as usize);  // legacy preserved
    if let Some(loc) = emitted_loc {
        intr.counters.apply_hierarchical_at("heading", *level as usize, loc);
        intr.kind_index.entry(ElementKind::Heading).or_default().push(loc);
    }
    state.auto_label_counter += 1;
    let (auto_label, resolved_text) = compute_heading_auto_toc(intr, emitted_loc.unwrap(), state.auto_label_counter);
    state.resolved_labels.insert(auto_label.clone(), resolved_text.clone());
    ...
}
```

**Helpers signature change**:
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

**Pattern stylesheet ADR-0069 preserved** — 5 variantes operacionais inalteradas conceptualmente; apenas helpers signature alterada (params: state → intr+loc).

---

## Consequências

### Positivas

- **F1 desbloqueado** — após P190G/H/I retomarem.
- **F3 parcialmente desbloqueado** — Layouter pode eliminar campo `counter` em P190I.
- **4 defers resolvíveis** (`lang`, `numbering_active`, `flat`, `hierarchical`) em P190G/H.
- **`from_tags` eliminado** — pipeline simplificado.
- **2 helpers migrados** para Introspector path location-aware.
- **Walk arm Equation gate migrado** para `is_numbering_active_at`.

### Neutras

- **Pattern ADR-0069 preserved** — 5 variantes + 7 aplicações intactas conceptualmente.
- **Tags emit preservado** — para consumers eventuais (memoização comemo M8, etc.).
- **2 helpers walk-internal** (`compute_figure`, `compute_heading_for_toc`) inalterados.

### Riscos

- **Risco moderado**: redesign do walk pipeline é trabalho cross-modular sem precedente directo.
  - **Mitigação**: P191B implementa mecanismo + 1 helper como prova de conceito; P191C migra 2º helper apenas após validação.

- **Risco signature change cascading**: ~20 recursive walk calls precisam de update.
  - **Mitigação**: mudanças mecânicas; cargo check garante completude.

- **Risco de duplicação from_tags logic em walk arms**: ~12 arms × 5 LOC = ~60 LOC duplicação inicial; from_tags eliminado depois.
  - **Mitigação**: redução LOC líquida positiva após cleanup (-from_tags +walk arm logic).

---

## Alternativas avaliadas

### Opção B — Two-pass walk (descartada)

**Mecanismo**: 1ª pass + build intr + 2ª pass com intr access.

**Razões para descartar**:
- 2 walks = double cost (performance).
- State mutations applied twice ou skipped — complex.
- Pipeline cognitivamente pesado.

### Opção C — Eliminate helpers (descartada)

**Mecanismo**: 4 helpers eliminados; logic embedded inline.

**Razões para descartar**:
- Perde 7 séries de pattern ADR-0069 stylesheet.
- Walk arms bloated.
- Refactor cross-cutting maior que Opção A.

### Opção D — Deferred resolution Layouter-side (descartada)

**Mecanismo**: Tags com payloads parciais; resolution lazy at Layouter render.

**Razões para descartar**:
- Mudança semântica de Tag::Labelled (resolved_text passa a ser placeholder).
- Layouter precisa de novo mecanismo de resolution.
- Multiple consumers afetados.
- Mudança cross-cutting demasiada para passo M+.

---

## Estado de execução

**P191A (este diagnóstico)** — completo:
- 4 opções avaliadas; Opção A escolhida.
- 9 cláusulas fechadas.
- Plano 2 sub-passos B-C.
- ADR-0071 PROPOSTO (este ficheiro).

**P191B (executado)** — implementação:
- Walk fn signature change ✅ (`+intr: &mut TagIntrospector`).
- 25 recursive call sites mecânicos ✅.
- 12 ElementPayload variants populated via `populate_intr_from_tag_start`
  helper centralizado ✅.
- 1 helper migrado (`compute_heading_auto_toc`) — prova de conceito ✅.
- Walk arm Equation gate migrado ✅.
- `from_tags::from_tags` eliminado (Opção α) ✅; `apply_state_funcs`
  slim post-pass preservada para Func eval em fixpoint.
- Tests workspace verdes (1832 → 1832; Δ marginal) ✅.
- LOC líquido -699 ✅.

**P191C (executado)** — encerramento:
- 2º helper migrado (`compute_labelled`) ✅ — signature
  `<I: Introspector>(intr, location, target, lang)`.
- 4 arms de `compute_labelled` migrados (Heading, Equation, Figure,
  catch-all) ✅.
- Caller (walk arm Labelled) adapta com snapshot+find_map preservado
  per pattern P195D variante não-locatable ✅.
- Cláusula `lang` resolvida via Opção β (parameter passing) ✅.
- 1 cláusula gate substancial empírica resolvida: populate_intr Figure
  arm gated por `is_counted` (alinha com legacy
  `state.figure_numbers`) ✅.
- Relatório consolidado P191 ✅.
- ADR-0071 transitada PROPOSTO → ACEITE ✅.
- Tracker P190 retomar actualizado ✅.

**P191 série fechada** — pre-condição arquitectural cumprida para retomar P190G.

### Validação empírica completa

| Aspecto | P191B | P191C | Verificado |
|---------|-------|-------|------------|
| Walk fn signature | ✅ | — | cargo check passa |
| 25 recursive call sites | ✅ | — | grep + cargo check |
| 12 ElementPayload variants populated | ✅ | — | populate_intr_from_tag_start completo |
| `from_tags::from_tags` eliminado | ✅ | — | Opção α; 969 LOC removidos |
| `compute_heading_auto_toc` migrado | ✅ | — | helper signature genérica |
| Walk arm Equation gate migrado | ✅ | — | location-aware |
| `compute_labelled` migrado | — | ✅ | helper signature genérica |
| `compute_labelled` 4 arms via Introspector | — | ✅ | sentinela + tests E2E |
| Pattern ADR-0069 stylesheet preservado | ✅ | ✅ | 5 variantes operacionais |
| Tests verdes Δ marginal | ✅ | ✅ | 1832 → 1832 |
| Cláusulas gate substanciais resolvidas | 5 | 1 | empírica |

---

## Cross-references

- **ADR-0068** — Location-aware Layouter (PROPOSTO em P185A; ACEITE depois). Análogo arquitectural — também mecanismo novo via signature change.
- **ADR-0069** — Post-recursion Tag emission. Pattern stylesheet preserved em ADR-0071.
- **ADR-0070** — Eliminação CounterStateLegacy (PROPOSTO em P190A). Recíproco a ADR-0069. Bloqueada parcialmente até ADR-0071 fechar (P191).
- **P190F §3** — barreira arquitectural identificada.
- **P190F §10** — recomendação P191A diagnóstico (precedente).
- **F1** (`auditoria-fresh-projecto.md`) — `CounterStateLegacy` 16 fields heterogéneos. Fecha após ADR-0071 + retomar P190G/H/I.

---

## Padrão estabelecido

ADR-0071 estabelece **pattern stylesheet "Introspector accessible during walk"** complementar a ADR-0069 + ADR-0070:

| Pattern | ADR | Direção |
|---------|-----|---------|
| Adição write paralelo | ADR-0069 | Legacy + Introspector path em paralelo |
| Eliminação write paralelo | ADR-0070 | Eliminar legacy quando Introspector path estável |
| **Walk pipeline com intr accessible** | **ADR-0071** | **Walk fn signature ganha intr; helpers queryam intr** |

Aplicação futura: helpers e walk arm gates similares (em outros domínios) podem migrar seguindo padrão ADR-0071.

**Sequência de pattern formation no projecto**:
- ADR-0069 (P195B PROPOSTO; P195E ACEITE) — adição write paralelo.
- ADR-0070 (P190A PROPOSTO; P190I ACEITE [esperado]) — eliminação write paralelo.
- **ADR-0071 (P191A PROPOSTO; P191C ACEITE [esperado])** — walk pipeline com intr accessible.

3 ADRs sequenciais formando ciclo arquitectural completo de M5/M6.

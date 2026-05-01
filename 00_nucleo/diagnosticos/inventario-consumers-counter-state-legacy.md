# Inventário de consumers `CounterStateLegacy`

Diagnóstico produzido em P167 (M5 sub-passo 1, 2026-04-30) — **sem código de produção tocado**. Output documental para informar P168 (primeira migração real).

---

## Leitores por consumer

Identificados por grep sobre `01_core/src/rules/` e `03_infra/src/`. Excluídos: tests internos do tipo (`01_core/src/entities/counter_state_legacy.rs`), tests do introspect (`01_core/src/rules/introspect.rs`).

### Consumer 1: `Layouter::layout` + métodos do Layouter — `01_core/src/rules/layout/mod.rs`

Bloco central. Reads e writes extensivos. Inclui injecção de `initial_state` no início do método público `layout()`.

| Field/método lido | Localização | Categoria |
|-------------------|-------------|-----------|
| `initial_state.has_outline` | mod.rs:1332 | production |
| `initial_state.numbering_active` | mod.rs:1339, 1366 | production |
| `initial_state.resolved_labels` | mod.rs:1334, 1364 | production |
| `initial_state.headings_for_toc` | mod.rs:1335, 1365 | production |
| `initial_state.bib_entries` | mod.rs:1341 | production |
| `initial_state.bib_numbers` | mod.rs:1343 | production |
| `self.counter.figure_numbers.get(...)` | mod.rs:408 | production |
| `self.counter.bib_entries.iter().find(...)` | mod.rs:565 | production |
| `self.counter.bib_numbers.get(key)` | mod.rs:571 | production |
| `self.counter.format_hierarchical("heading")` | mod.rs:283 | production |
| `self.counter.is_numbering_active("heading")` | mod.rs:282 | production |
| `self.counter.step_hierarchical("heading", level)` | mod.rs:274 | production (mutator) |
| `self.counter.label_pages` | mod.rs:1034 | production (write) |

### Consumer 2: `layout_ref` + `layout_labelled` — `01_core/src/rules/layout/references.rs`

Resolve referências cruzadas (sintaxe `@label`).

| Field/método lido | Localização | Categoria |
|-------------------|-------------|-----------|
| `layouter.counter.figure_label_numbers.get(target)` | references.rs:35 | production |
| `layouter.counter.resolved_labels.get(target)` | references.rs:39 | production |
| `layouter.counter.label_pages.insert(...)` | references.rs:28 | production (mutator/write) |

### Consumer 3: `layout_outline` — `01_core/src/rules/layout/outline.rs`

Renderiza Tabela de Conteúdos.

| Field/método lido | Localização | Categoria |
|-------------------|-------------|-----------|
| `layouter.counter.headings_for_toc.clone()` | outline.rs:24 | production |
| `layouter.counter.known_page_numbers.get(&label)` | outline.rs:35 | production |
| `layouter.counter.is_readonly = true/false` | outline.rs:55, 58 | production (write) |

### Consumer 4: `counter_helpers` — `01_core/src/rules/layout/counters.rs`

Walk arm `Content::CounterUpdate` e helper `display_value`.

| Field/método lido | Localização | Categoria |
|-------------------|-------------|-----------|
| `counter.step_hierarchical("heading", 1)` | counters.rs:25 | production (mutator) |
| `counter.step_flat(key)` | counters.rs:27 | production (mutator) |
| `counter.update_flat(key, *val)` | counters.rs:31 | production (mutator) |
| `counter.numbering_active.insert("heading", active)` | counters.rs:12 | production (write) |
| `counter.display_value(kind)` | counters.rs:39 | production |

### Consumer 5: `layout_equation` — `01_core/src/rules/layout/equation.rs`

Numeração de equações.

| Field/método lido | Localização | Categoria |
|-------------------|-------------|-----------|
| `self.counter.is_numbering_active("equation")` | equation.rs:24 | production |
| `self.counter.step_flat("equation")` | equation.rs:26 | production (mutator) |
| `self.counter.get_flat("equation")` | equation.rs:88 | production |

Nota: numeração de equação **só é activada em testes** (`layout/tests.rs:899` insere `numbering_active["equation"] = true`). Em production esta arma sempre vê `is_numbered = false`.

### Consumer 6 (test-only): `integration_tests` — `03_infra/src/integration_tests.rs`

| Field/método lido | Localização | Categoria |
|-------------------|-------------|-----------|
| `state.figure_numbers.get("image")` | integration_tests.rs:1116 | test (assertion) |

---

## Mapeamento por field

Categorias:
- **Directo**: equivalente 1:1 sem perda de informação.
- **Parcial**: equivalente cobre subset; falta info ou exige computação no consumer.
- **Lacuna**: sem equivalente em `TagIntrospector` actual.

| Field/método legacy | Equivalente Introspector | Categoria | Notas |
|---------------------|--------------------------|-----------|-------|
| `resolved_labels.get(label)` | `query_by_label(label)` + computação de texto formatado | **Lacuna** | Introspector não tem texto formatado pré-computado. Texto "Secção 1.1" vem do walk arm `Content::Labelled` que combina format_hierarchical + supplement. |
| `figure_label_numbers.get(label)` | `query_by_label(label)` + posição em `kind_index[Figure]` | **Parcial** | Derivável: location → index 1-based. Caveat: legacy só inclui figuras com `numbering+caption`; Introspector inclui todas. Lacuna #1 de `m1-lacunas-captura.md` aplica-se. |
| `headings_for_toc.iter()` (Vec<(Label, Content, level)>) | `kind_index[Heading]` + `query_by_label` | **Lacuna** | Introspector não armazena Content body (lacuna #3 de `m1-lacunas-captura.md`). |
| `known_page_numbers.get(label)` | (não aplicável) | **Layout-output** | Page numbers vêm da renderização (fixpoint). Não derivável de introspecção. |
| `figure_numbers.get(kind)` | `counters.value("figure")` | **Parcial** | Counter é flat por kind; legacy resolve `kind=None → "image"` mas Introspector preserva literal. Lacuna #1. |
| `bib_entries.iter().find(...)` | (sem equivalente) | **Lacuna** | Adiada para M9 quando `MetadataStore` for adicionado. |
| `bib_numbers.get(key)` | (sem equivalente) | **Lacuna** | Idem; M9. |
| `label_pages` | (não aplicável) | **Layout-output** | Page numbers populados por `layout_labelled` durante renderização. |
| `is_readonly` | (não aplicável) | **Layout-control** | Flag interna do Layouter para protecção contra reentry no TOC. Nada que ver com introspecção. |
| `is_numbering_active(key)` | (sem equivalente) | **Lacuna** | Walk arm `Content::SetHeadingNumbering` não populariza Introspector. Mecanismo equivalente seria adicionar bool a Introspector ou mover lógica. |
| `format_hierarchical(key)` | `counters.value(key)` formatado | **Parcial** | CounterRegistry é flat (1 nível); legacy retorna "1.2.3" hierárquico. Lacuna nova: hierarquia em counters. |
| `step_hierarchical/step_flat/update_flat` | (sem equivalente — mutação) | **Lacuna** | Introspector é read-only; Layouter precisa de mutação durante renderização. Migração exige re-arquitectura. |
| `display_value(kind)` | `counters.value(kind)` formatado como string | **Parcial** | Mesma limitação que format_hierarchical. |
| `auto_label_counter` | (sem equivalente) | **Layout-internal** | Gerador de IDs para auto-labels da TOC; não exposto a Introspector. |
| `local_figure_counters` | (interno do walk) | **Layout-internal** | Auxiliar interno; Introspector consome o resultado via counters. |
| `numbering_active` (HashMap) | (sem equivalente directo) | **Lacuna** | Idem `is_numbering_active`. |
| `has_outline` | (sem equivalente) | **Lacuna** | Walk arm `Content::Outline` não populariza Introspector. Mecanismo seria adicionar `has_outline: bool` ao Introspector ou usar `query_by_kind(Outline)` se Outline fosse payload-yielder. Não é em M1. |
| `state.lang` | (sem equivalente) | **Lacuna** | Lang context para supplement resolution. Não materializado em Introspector. |
| `figure_label_numbers` (escrita) | (não aplicável) | **Layout-output** | Populado pelo walk arm `Labelled` quando target é figure numerada — escreve no state. Migração exige re-arquitectura. |

### Lacunas novas adicionadas em P167

Comparando com `m1-lacunas-captura.md` (3 divergências documentadas em P163), P167 confirma 4 lacunas adicionais:

- **Lacuna #4 (P167)**: `is_numbering_active` / `numbering_active` — Introspector não captura SetHeadingNumbering nem reflecte estado de numeração. M9 ou passo dedicado.
- **Lacuna #5 (P167)**: `format_hierarchical` / hierarquia em counters — `CounterRegistry` é flat; vanilla e legacy suportam hierarquia "1.2.3". M9 quando `CounterKey` enum for adicionado.
- **Lacuna #6 (P167)**: `bib_entries` / `bib_numbers` — M9 quando `MetadataStore` ou variant Bibliography for materializada.
- **Lacuna #7 (P167)**: `has_outline` / detecção de Outline — Introspector não rastreia. M9 ou passo dedicado.

Estas serão adicionadas a `m1-lacunas-captura.md` em pendência.

---

## Migrabilidade por consumer

Aplicando regras `.C` per spec.

| Consumer | Fields críticos lidos | Migrabilidade | Razão |
|----------|----------------------|---------------|-------|
| Layouter::layout (mod.rs) | resolved_labels, figure_numbers, bib_entries, bib_numbers, has_outline, format_hierarchical, is_numbering_active, step_*, numbering_active | **Bloqueada** | Múltiplas lacunas (#1, #4, #5, #6, #7) + mutação extensiva. Re-arquitectura. |
| layout_ref (references.rs) | figure_label_numbers, resolved_labels | **Parcial** | Caso figura-ref derivável via Introspector + computação (caveat lacuna #1). Caso section-ref bloqueado por lacuna texto formatado. |
| layout_labelled (references.rs) | (escrita label_pages) | **N/A** | Apenas write de page number — não consome Introspector. |
| layout_outline (outline.rs) | headings_for_toc, known_page_numbers | **Bloqueada** | Lacuna #3 (Content body em headings_for_toc); known_page_numbers é layout-output. |
| counter_helpers (counters.rs) | (mutators) | **Bloqueada** | Padrão mutação não suportado em Introspector. |
| layout_equation (equation.rs) | is_numbering_active("equation"), step_flat, get_flat | **Bloqueada** | Equation não é payload kind (M9). |
| integration_tests (test) | figure_numbers.get("image") | **Parcial** | Pode migrar para `introspector.counters.value("figure")` mas com caveat lacuna #1. |

**Resumo**: 0 consumers Total, 2 Parcial (layout_ref figure-case + integration_tests), 5 Bloqueados.

---

## Escolha para P168

**Consumer escolhido**: `layout_ref` em `01_core/src/rules/layout/references.rs` — **subset apenas para o caso de figura-ref**.

**Localização**: `references.rs:35` — `if let Some(&fig_num) = layouter.counter.figure_label_numbers.get(target) { ... }`.

**Justificação numérica**:
1. **Migrabilidade Total**: zero consumers. Cláusula gate trivial aplicada — Parcial é admitido com caveats documentados.
2. **Entre os Parciais**: `layout_ref` figure-case > `integration_tests` (production beats test).
3. **Menor número de call-sites**: 1 (apenas `references.rs:35` para figure-ref). `Layouter::layout` tem 13+; bloqueado por lacunas múltiplas.
4. **Menor superfície downstream**: layout_ref é função folha. Não exporta nada nem é consumida por outros sítios além do walk arm `Content::Ref` em `layout/mod.rs`.
5. **Validação real**: production code, exercitado por integration_tests que fazem refs a figuras.

**Fields a migrar**: apenas `figure_label_numbers.get(target)` para o caso figure-ref. **Não migrar**:
- `resolved_labels.get(target)` (lacuna texto formatado).
- `label_pages.insert(...)` em `layout_labelled` (layout output).

**Mapeamento concreto**:

```rust
// Antes (legacy):
if let Some(&fig_num) = layouter.counter.figure_label_numbers.get(target) {
    layouter.layout_content(&Content::text(format!("Figura {}", fig_num)));
    return;
}

// Depois (M5 P168):
if let Some(loc) = layouter.introspector.query_by_label(target) {
    // Verificar que loc corresponde a um Figure
    let figures: Vec<Location> = layouter.introspector.query_by_kind(ElementKind::Figure);
    if let Some(idx_zero_based) = figures.iter().position(|l| *l == loc) {
        // Filtro adicional para "figuras numeradas+captioned" — caveat:
        // Introspector inclui TODAS as figures; legacy figure_label_numbers
        // só conta as numeradas+captioned. Em P168, o caller deve filtrar
        // ou a from_tags deve aprender a omitir figuras não numeradas.
        let fig_num = idx_zero_based + 1;
        layouter.layout_content(&Content::text(format!("Figura {}", fig_num)));
        return;
    }
}
// Fallback continua: resolved_labels (legacy)
```

**Riscos identificados**:
1. **Lacuna #1 (figure.kind None vs Some)**: figura com `kind=None` continua a apontar para mesma chave de counter; mas indexação via `kind_index[Figure]` agrupa todas. Para paridade observable, `figure_label_numbers` precisaria do mesmo filtro: figura numerada+captioned.
2. **Adicionar field `introspector: TagIntrospector` ao `Layouter`**: requer toque em `Layouter::new` (layout/mod.rs:144) e em `pub fn layout()` (mod.rs:1325). Não destrutivo — `Default` ou `TagIntrospector::empty()` para tests existentes.
3. **Filtro figura-numerada-captioned**: P168 pode resolver via dois caminhos:
   - **A**: `from_tags` aprende a omitir figuras não numeradas/captioned (modifica P165 `from_tags`). Limpa mas re-toca M3.
   - **B**: P168 filtra no momento da query (verifica via `Content::Figure` no consumer, mas Introspector não tem o Content). **Não viável** sem ElementPayload mais rico.
   - **C**: Adicionar a `ElementPayload::Figure` um campo `is_counted: bool`. Tocaria L0 (P165 element_payload) + extract_payload + from_tags. Solução mais limpa.

   Decisão **adiada para P168**: caminho **C** parece preferível por ser local e explícito, mas é decisão para P168.

**Tamanho estimado de P168**: **M**.

Componentes:
- Adicionar field `introspector: TagIntrospector` a `Layouter` (em `layout/mod.rs`).
- Modificar `pub fn layout()` para receber introspector (alterando assinatura — quebra os ~22 call-sites em production e tests) OU receber via parameter no `pub fn layout_with_introspector()` (mais um wrapper-pattern, parallel a `introspect_with_introspector`). Decisão local em P168.
- Modificar `references.rs::layout_ref` para usar `layouter.introspector` na arm figure.
- Resolver filtro figura-numerada via caminho A/B/C escolhido.
- Tests novos: verificar que figure-ref via Introspector produz "Figura N" igual ao legacy.

---

## Notas finais

P167 confirma o que P165 deixou implícito: o **Introspector M3 está deliberadamente minimal** e a primeira migração real (P168) já força extensão (via ElementPayload mais rico OU via from_tags filtrando OU via Layouter filtrando com helper).

Sem migração de **Total** disponível em M5. Todas as migrações reais serão Parcial-com-caveat até M9 expandir o Introspector substancialmente.

Lista cumulativa de pendências passada para `m1-lacunas-captura.md`:
- #1, #2, #3 (originais P163).
- #4 (P167): `is_numbering_active` / `numbering_active`.
- #5 (P167): `format_hierarchical` / hierarquia em counters.
- #6 (P167): `bib_entries` / `bib_numbers`.
- #7 (P167): `has_outline`.

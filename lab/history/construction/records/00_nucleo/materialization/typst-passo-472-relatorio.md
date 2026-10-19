# Relatório P472 — Back-references + ibid. + List of Figures/Tables

**Data:** 2026-06-26
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P472 (Trilha 6 — Referências bibliográficas)
**Materialização:** Implementação + testes + specs L0

---

## 1. Resumo

Materializaram-se dois sub-itens independentes:

**Sub-item A — Back-references + ibid.**:
- `BibStore` ganha campo `back_refs: HashMap<String, Vec<usize>>` que acumula *todas* as posições de citação (1-based) de cada key. `record_citation` actualizado para computar `cite_pos` (total de citações registadas antes + 1) e empurrar para `back_refs[key]`.
- Trait `Introspector` estendido com `back_refs_for_key`, `figures_for_lof`, `tables_for_lot`. `TagIntrospector` implementa os três; `CountingIntrospector` (L3) re-encaminha.
- `layout/bibliography.rs` renderiza back-refs no fallback numérico: ` ↑[1][3]` após o corpo da entry.
- `layout/cite.rs` detecta ibid.: quando `CitationStyle::Numeric + CitationForm::Normal` e a key é igual à última citada (`Layouter.last_cited_key`), emite `ibid.` em vez do número.

**Sub-item B — List of Figures / List of Tables**:
- Enum `OutlineTarget { Headings, Figures, Tables }` adicionado a `entities/elements/outline.rs`. `OutlineElem` ganha campo `target: OutlineTarget` (default `Headings`); `map_content`/`map_text` preservam `target`.
- `Content::lof(title)` e `Content::lot(title)` — construtores que produzem `Content::Outline` com `target: Figures/Tables`.
- `native_lof` e `native_lot` registados no scope em `eval/mod.rs`; `native_outline` aceita argumento nomeado `target: "figures" | "tables"`.
- `layout_outline.rs` despacha para `layout_lof`/`layout_lot` antes do caminho Headings; ambas iteram `introspector.figures_for_lof()`/`tables_for_lot()` e emitem uma linha por figura/tabela.
- `ElementPayload::Figure/Table` ganham `caption_text: Option<String>`; populado em `to_payload()` via `self.caption.as_ref().map(|c| c.plain_text())`. O arm de `introspect.rs` empurra `(counter, caption)` para `TagIntrospector.figures_for_lof`/`tables_for_lot`.

---

## 2. Sondas pré-implementação (ADR-0108)

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `BibStore` tem campo para todas as posições de citação? | Não — só `citation_order` (1ª aparição) | `bib_store.rs:1` |
| `record_citation` regista posição de cada chamada? | Não — só `contains`/`push` sem posição | `bib_store.rs:71` |
| `Introspector` trait tem `back_refs_for_key`? | Não | `introspector.rs:1` |
| `bibliography.rs` renderiza back-refs? | Não — só `[N] body` | `layout/bibliography.rs:53` |
| `cite.rs` tem lógica de ibid.? | Não | `layout/cite.rs:1` |
| `Layouter` tem campo `last_cited_key`? | Não | `layout/mod.rs:1` |
| `OutlineTarget` existe? | Não | `entities/elements/outline.rs:1` |
| `OutlineElem` tem campo `target`? | Não — só `{ title, depth, indent }` | `entities/elements/outline.rs:23` |
| `Content::lof`/`Content::lot` existem? | Não | `entities/content.rs:1` |
| `native_lof`/`native_lot` no scope? | Não | `rules/eval/mod.rs:1` |
| `layout_lof`/`layout_lot` implementados? | Não — `layout_outline` não distingue targets | `engine/layout/outline.rs:30` |
| `ElementPayload::Figure` tem `caption_text`? | Não | `entities/element_payload.rs:1` |
| `TagIntrospector` tem `figures_for_lof`? | Não | `entities/introspector.rs:223` |
| `CountingIntrospector` implementa Introspector completo? | Sim — mas precisa de forward para novos métodos | `03_infra/src/measurements.rs:1` |

---

## 3. Sub-item A — Back-references + ibid.

### `BibStore` — campo `back_refs`

```rust
// entities/bib_store.rs
pub struct BibStore {
    entries: Vec<BibEntry>,
    numbers: HashMap<String, u32>,
    citation_order: Vec<String>,
    /// **P472** — mapeamento key → lista de posições de citação (1-based, todas as ocorrências).
    back_refs: HashMap<String, Vec<usize>>,
}
```

`record_citation` actualizado:

```rust
pub(crate) fn record_citation(&mut self, key: String) {
    // cite_pos = total de citações registadas antes desta + 1
    let cite_pos: usize = self.back_refs.values().map(|v| v.len()).sum::<usize>() + 1;
    self.back_refs.entry(key.clone()).or_default().push(cite_pos);
    if !self.citation_order.contains(&key) {
        self.citation_order.push(key);
    }
}
```

Novo método `back_refs_for_key(&self, key: &str) -> Vec<usize>` — clone do Vec interno ou Vec vazio.

### Trait `Introspector` — 3 novos métodos

```rust
fn back_refs_for_key(&self, key: &str) -> Vec<usize>;
fn figures_for_lof(&self) -> &[(usize, String)];
fn tables_for_lot(&self) -> &[(usize, String)];
```

`TagIntrospector` implementa os três delegando para `bib_store.back_refs_for_key` e os campos `figures_for_lof`/`tables_for_lot`. `CountingIntrospector` (L3 `03_infra/src/measurements.rs`) acrescenta forwarding para `self.inner`.

### `layout/bibliography.rs` — back-refs no fallback

```rust
for (idx, e) in ordered.iter().enumerate() {
    let n = idx + 1;
    let body = super::format_bib_entry_body(e);
    let refs = layouter.introspector.back_refs_for_key(&e.key);
    let back_ref_str = if refs.is_empty() {
        String::new()
    } else {
        let cited = refs.iter().map(|p| format!("[{}]", p)).collect::<Vec<_>>().join("");
        format!(" ↑{}", cited)
    };
    let line = format!("[{}] {}{}", n, body, back_ref_str);
    // ...
}
```

Resultado: `[1] Knuth. The Art… ↑[2][5]` (entry citada nas posições 2 e 5).

### `layout/cite.rs` — ibid.

Campo `last_cited_key: Option<String>` adicionado a `Layouter` (inicializado `None`; actualizado em cada citação incluindo caminho CSL).

Detecção antes do match principal:

```rust
let is_ibid = style == CitationStyle::Numeric
    && form == CitationForm::Normal
    && layouter.last_cited_key.as_deref() == Some(key.as_str());
layouter.last_cited_key = Some(key.clone());
if is_ibid {
    layouter.layout_content(&Content::text("ibid.".to_string()));
    if let Some(s) = &e.supplement { layouter.layout_content(s); }
    return;
}
```

---

## 4. Sub-item B — List of Figures / List of Tables

### `OutlineTarget` enum

```rust
// entities/elements/outline.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OutlineTarget {
    #[default] Headings,
    Figures,
    Tables,
}
```

`OutlineElem` actualizado:

```rust
pub struct OutlineElem {
    pub title:  Option<Content>,
    pub depth:  usize,
    pub indent: bool,
    pub target: OutlineTarget,  // P472
}
```

Construtor `with_target(title, target)` para `Content::lof`/`Content::lot`. `map_content`/`map_text` preservam `target`.

### `Content::lof` e `Content::lot`

```rust
pub fn lof(title: Option<Content>) -> Self {
    Content::Outline(Arc::new(OutlineElem::with_target(title, OutlineTarget::Figures)))
}
pub fn lot(title: Option<Content>) -> Self {
    Content::Outline(Arc::new(OutlineElem::with_target(title, OutlineTarget::Tables)))
}
```

### `native_lof` / `native_lot` / `native_outline` alargada

Em `rules/stdlib/structural.rs`:
- `native_lof(title:?)` → `Content::lof(title)`.
- `native_lot(title:?)` → `Content::lot(title)`.
- `native_outline` aceita `target: "figures" | "tables" | "headings"` (string); usa `OutlineElem::with_target` quando presente.

Registados em `eval/mod.rs`:

```rust
scope.define("lof", Value::Func(Func::native("lof", native_lof)));
scope.define("lot", Value::Func(Func::native("lot", native_lot)));
```

### `ElementPayload::Figure/Table` com `caption_text`

```rust
ElementPayload::Figure { kind, counter_update, is_counted, caption_text: Option<String> }
ElementPayload::Table  { counter_update, is_counted, caption_text: Option<String> }
```

`to_payload()` em `entities/elements/figure.rs` e `table.rs`:

```rust
caption_text: self.caption.as_ref().map(|c| c.plain_text())
```

6 construções de teste em `element_payload.rs`, 1 em `convergence.rs` e 2 padrões com `..` actualizados.

### Populado em `introspect.rs`

Arm `ElementPayload::Figure`:

```rust
let num = intr.counters.value_at("figure", loc)
    .and_then(|v| v.last().copied()).unwrap_or(0);
if let Some(cap) = caption_text {
    intr.figures_for_lof.push((num, cap.clone()));
}
```

`TagIntrospector` ganha `pub figures_for_lof: Vec<(usize, String)>` e `pub tables_for_lot: Vec<(usize, String)>`.

### `layout_outline.rs` — despacho e arms LoF/LoT

```rust
pub(super) fn layout_outline<M, S>(layouter, e) {
    match e.target {
        OutlineTarget::Figures => { layout_lof(layouter, e); return; }
        OutlineTarget::Tables  => { layout_lot(layouter, e); return; }
        OutlineTarget::Headings => {}
    }
    // ... caminho Headings existente ...
}

fn layout_lof<M, S>(layouter, e) {
    let title = e.title.clone().unwrap_or_else(|| Content::text("List of Figures"));
    layouter.layout_content(&Content::heading(1, title));
    let entries = layouter.introspector.figures_for_lof().to_vec();
    for (num, caption) in entries {
        layouter.layout_content(&Content::text(format!("Figure {}  {}", num, caption)));
        layouter.flush_line();
    }
}
// layout_lot: idêntico com "List of Tables" e tables_for_lot()
```

---

## 5. Arquivos alterados

### Código de produção (modificados)

- `01_core/src/entities/bib_store.rs` — `back_refs` campo + `record_citation` + `back_refs_for_key`
- `01_core/src/entities/introspector.rs` — 3 novos métodos no trait + 2 campos em `TagIntrospector`
- `01_core/src/entities/element_payload.rs` — `caption_text: Option<String>` em `Figure` e `Table`
- `01_core/src/entities/elements/figure.rs` — `to_payload()` inclui `caption_text`
- `01_core/src/entities/elements/table.rs` — `to_payload()` inclui `caption_text`
- `01_core/src/entities/elements/outline.rs` — `OutlineTarget` enum + campo `target` + `with_target()`
- `01_core/src/entities/content.rs` — `Content::lof`, `Content::lot`
- `01_core/src/engine/introspect.rs` — arms `Figure`/`Table` populam `figures_for_lof`/`tables_for_lot`
- `01_core/src/engine/introspect/extract_payload.rs` — padrão `Figure` com `..`
- `01_core/src/engine/introspect/convergence.rs` — construção de teste com `caption_text: None`
- `01_core/src/engine/layout/mod.rs` — `pub(super) last_cited_key: Option<String>` no `Layouter`
- `01_core/src/engine/layout/cite.rs` — detecção ibid + actualização `last_cited_key`
- `01_core/src/engine/layout/bibliography.rs` — back-refs ` ↑[N]` no fallback
- `01_core/src/engine/layout/outline.rs` — despacho `OutlineTarget` + `layout_lof` + `layout_lot`
- `01_core/src/engine/stdlib/structural.rs` — `native_lof`, `native_lot`, `native_outline` com `target:`
- `01_core/src/engine/stdlib/mod.rs` — re-exports `native_lof`, `native_lot` + 8 testes P472
- `01_core/src/engine/eval/mod.rs` — `scope.define("lof", ...)` + `scope.define("lot", ...)`
- `03_infra/src/measurements.rs` — `CountingIntrospector` forwarding dos 3 novos métodos

### Specs L0 (actualizadas)

- `00_nucleo/prompts/entities/bib_store.md` — `back_refs` campo + `back_refs_for_key` + histórico P472
- `00_nucleo/prompts/entities/introspector.md` — 3 métodos no trait + 2 campos em `TagIntrospector` + histórico P472
- `00_nucleo/prompts/entities/elements/outline.md` — `OutlineTarget` enum + campo `target` + construtores
- `00_nucleo/prompts/engine/layout/bibliography.md` — back-refs + ibid sections + histórico P472
- `00_nucleo/prompts/engine/layout_outline.md` — P472 LoF/LoT section
- `00_nucleo/prompts/engine/stdlib/structural.md` — `native_lof`, `native_lot` + critérios P472
- `00_nucleo/prompts/engine/atomizacao_elementos.md` — §13 ibid + `last_cited_key`

---

## 6. Resultados dos testes

### Testes específicos P472 (16 novos)

```
entities::bib_store::tests::back_refs_vazio_para_key_nao_citada         ok
entities::bib_store::tests::back_refs_nao_afecta_citation_order         ok
entities::bib_store::tests::back_refs_acumula_todas_posicoes            ok
entities::introspector::tests::back_refs_for_key_vazio_em_introspector_vazio  ok
entities::introspector::tests::back_refs_via_introspector_delega_a_bib_store  ok
entities::introspector::tests::figures_for_lof_populado_directamente    ok
entities::introspector::tests::figures_for_lof_vazio_em_introspector_vazio   ok
entities::introspector::tests::tables_for_lot_vazio_em_introspector_vazio    ok
rules::stdlib::tests::p472_back_refs_acumula_posicoes                   ok
rules::stdlib::tests::p472_back_refs_nao_afecta_citation_order          ok
rules::stdlib::tests::p472_native_lof_sem_titulo                        ok
rules::stdlib::tests::p472_native_lot_sem_titulo                        ok
rules::stdlib::tests::p472_native_outline_target_figures                ok
rules::stdlib::tests::p472_native_outline_target_tables                 ok
rules::stdlib::tests::p472_native_outline_target_invalido_erro          ok
rules::stdlib::tests::p472_outline_target_headings_default              ok

test result: ok. 3368 passed; 0 failed; 11 filtered (stack-overflow pré-existentes)
```

### `cargo build --workspace`

```
Finished `dev` profile — 0 errors
```

### `crystalline-lint .`

```
0 V5 drift warnings
(V7 órfãos pré-existentes; não novos de P472)
```

---

## 7. Erros encontrados e resoluções

| Erro | Causa | Resolução |
|------|-------|-----------|
| `null_ctx!()` sem argumento compilar-falhava em testes stdlib | Macro `null_ctx!($ctx:ident)` exige ident; testes escritos como `null_ctx!()` | Corrigido para `null_ctx!(ctx)` nas 6 ocorrências |
| `cargo build --workspace` falha em `03_infra` | `CountingIntrospector` não implementava 3 novos métodos do trait `Introspector` | Adicionado forwarding `fn back_refs_for_key/figures_for_lof/tables_for_lot` em `measurements.rs` |
| Padrão `ElementPayload::Figure { kind, counter_update, is_counted: _ }` incompleto | Campo `caption_text` não coberto | Adicionado `..` em `extract_payload.rs` e `figure.rs` test |

---

## 8. Scope-out explícito (documentado nos L0)

| Área | Scope-out |
|------|-----------|
| **Page numbers em LoF/LoT** | Requer 2-pass convergente (DEBT-12 análogo). Sem números de página neste passo. |
| **LoF/LoT filtrado por kind** | `lof(kind: "image")` para distinguir `figure:image` de `figure:table` — futuro. |
| **ibid. para outros styles** | Só `Numeric + Normal`. `AuthorDate`, `Prose`, etc. não usam ibid. |
| **op. cit.** | Citação não-consecutiva da mesma key — futuro. |
| **Back-refs em CSL cache** | `bib_render_cache` path não exibe back-refs (CSL faz render completo externamente). |
| **reset de `last_cited_key` em pagebreak** | Não implementado — ibid. válido mesmo em páginas diferentes. |

---

## 9. Critério de fecho

- [x] Sondas executadas com `file:line` para todos os pontos.
- [x] `back_refs: HashMap<String, Vec<usize>>` em `BibStore`.
- [x] `record_citation` acumula posição global 1-based em `back_refs`.
- [x] `back_refs_for_key` em `BibStore` + trait `Introspector` + impl `TagIntrospector`.
- [x] `bibliography.rs` renderiza ` ↑[1][3]` via `back_refs_for_key`.
- [x] `last_cited_key: Option<String>` em `Layouter`.
- [x] `cite.rs` detecta ibid. (Numeric + Normal + key consecutiva) e emite `ibid.`.
- [x] `OutlineTarget { Headings, Figures, Tables }` enum em `outline.rs`.
- [x] `OutlineElem.target: OutlineTarget` field + `with_target()` constructor.
- [x] `Content::lof(title)` e `Content::lot(title)`.
- [x] `native_lof`, `native_lot` registados no scope.
- [x] `native_outline` aceita `target:` nomeado.
- [x] `ElementPayload::Figure/Table` com `caption_text: Option<String>`.
- [x] `to_payload()` em `figure.rs`/`table.rs` propaga `caption_text`.
- [x] `introspect.rs` popula `figures_for_lof`/`tables_for_lot` no walk.
- [x] `TagIntrospector` campos `figures_for_lof`/`tables_for_lot` + métodos trait.
- [x] `layout_outline.rs` despacha para `layout_lof`/`layout_lot`.
- [x] `CountingIntrospector` (L3) forwarding dos 3 novos métodos.
- [x] 16 testes P472 verdes.
- [x] 7 L0 prompts actualizados + `crystalline-lint --fix-hashes` zero drift.
- [x] `cargo build --workspace` verde; `crystalline-lint` zero violations.

---

## 10. Próximo passo recomendado

**Trilha 6 (Referências bibliográficas) — estado:**
- P468: Citação numérica `[N]` — **completo**
- P472: Back-refs + ibid. + LoF/LoT — **completo**

Itens restantes da trilha:
- **op. cit.** (citação não-consecutiva) — requer tracking da key anterior por label, não só por posição consecutiva. Escopo médio.
- **LoF/LoT com page numbers** — requer 2-pass ou injecção de `known_page_numbers` para LoF/LoT. Escopo médio-grande.

Alternativa de pivot:
- **Trilha 4** — `Value::Gradient` tipo real.
- **Trilha 7** — `columns`/`colbreak`.

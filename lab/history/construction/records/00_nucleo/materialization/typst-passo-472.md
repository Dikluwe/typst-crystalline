---

# P472 — Trilha 6 Fase 2: back-references + List of Figures / List of Tables

> **Passo:** 472
> **Data:** 2026-06-26
> **Foco:** (1) Back-references bibliográficas ("ver [3]") e `ibid`/`op. cit.`; (2) List of Figures (`lof`) e List of Tables (`lot`).
> **Trilha:** 6 — Bibliografia Fase 2 (desbloqueada por Trilha 1 + Trilha 2).
> **Tipo:** Materialização.
> **Tamanho:** M (~40 min total; dois sub-itens com infra partilhada).
> **ADR-0117 Cláusula 4:** Reaproveita `BibStore.citation_order` (P468), `figure_supplement_for_lang` (P158B/P470), `CounterRegistry` (P451), `Introspector` (P468). Não propõe estrutura em elementos existentes sem verificar P468/P469.

---

## Contexto

O roteiro (Trilha 6, `roteiro-conclusao-typst-cristalino-atualizado.md`) lista como pendentes:

1. **Back-references** ("ver [3]") — cada entrada bibliográfica deveria saber em que páginas/números foi citada. Scope-out de P468.
2. **`ibid` / `op. cit.`** — formas abreviadas de citação repetida. Scope-out de P468.
3. **List of Figures / List of Tables** — `lof()` e `lot()` análogos ao `outline()` (P457), mas para figuras e tabelas.

Trilha 6 está em 1/5 completo (P468 fechou estilos numéricos). Este passo fecha mais 2 itens: back-references + LoF/LoT.

**Dependências desbloqueadas:**
- `BibStore.citation_order` + `citation_number_for_key` (P468) — infra para saber a ordem de citação de cada entrada.
- `figure_supplement_for_lang` (P158B, ligado em P470) — prefixo de caption localizado.
- `CounterRegistry` com chaves `"figure"` e `"table"` (P451/P459) — números de figuras e tabelas.
- `Introspector` com `flat_counter_at` (P468) — acesso a contadores por posição.
- `outline()` (P457) como precedente arquitectural para `lof()`/`lot()`.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `BibStore` tem `citation_order: Vec<String>`? | Sim — P468 | ✅ |
| `citation_number_for_key` existe no `Introspector`? | Sim — P468 | ✅ |
| `Introspector` tem campo de page/location por citação? | Verificar — P468 não materializou locations | 🟡 |
| `Content::Outline` existe (P457)? | Sim | ✅ |
| `outline()` usa `CounterRegistry`? | Verificar `engine/layout/outline.rs` ou `stdlib/structural.rs` | 🟡 |
| `Introspector` tem acesso a figuras/tabelas por posição? | Verificar — `figure_label_numbers`, `resolved_labels` | 🟡 |
| `ibid` requer saber qual foi a última citação? | Sim — precisa de `last_cited_key: Option<String>` no Introspector ou no layout | 🟡 |
| Bloqueadores técnicos? | Nenhum conhecido | ✅ |

**Sondas 🟡 com `grep`/`file:line` antes de escrever código.**

---

## Sub-item A — Back-references + `ibid`/`op. cit.`

### A.1 — Contexto vanilla

O Typst vanilla suporta back-references implícitas na bibliografia: cada entrada `[1]` lista as páginas onde foi citada (`cited on p. 3, 7`). `ibid` e `op. cit.` são formas abreviadas para citações repetidas consecutivas ou não-consecutivas.

No cristalino, o subset minimal cobre:

- **Back-references** — a lista de números de citação onde cada entrada foi referenciada (ex.: `[1]` foi citada na posição 1 e 3 → a entrada mostra `[1, 3]` ou apenas o primeiro número). Sem números de página (requer layout 2-pass com page tracking — scope-out).
- **`ibid`** — quando a mesma entrada é citada consecutivamente, a segunda citação usa `ibid` em vez de `[N]`. Opcional (activado por configuração).
- **`op. cit.`** — quando uma entrada já citada é citada de novo (não consecutivamente). Opcional.

### A.2 — Mapa inverso de citações

**Ficheiro:** `entities/bib_store.rs`

Adicionar `back_refs: HashMap<String, Vec<usize>>` — para cada chave de entrada, a lista de posições de citação (1-based, em ordem de aparição):

```rust
pub struct BibStore {
    // ... existentes ...
    pub citation_order: Vec<String>,    // P468
    pub back_refs: HashMap<String, Vec<usize>>,  // NOVO P472
}
```

`record_citation(key)` atualizado: além de registar `citation_order`, regista a posição em `back_refs`:

```rust
pub fn record_citation(&mut self, key: &str) {
    let pos = if let Some(p) = self.citation_order.iter().position(|k| k == key) {
        p + 1  // 1-based, posição de PRIMEIRA aparição
    } else {
        self.citation_order.push(key.to_string());
        self.citation_order.len()  // nova entrada
    };
    // Posição de CITAÇÃO (contagem de record_citation calls):
    let cite_pos = self.back_refs.values().map(|v| v.len()).sum::<usize>() + 1;
    self.back_refs.entry(key.to_string()).or_default().push(cite_pos);
}
```

**Nota:** `back_refs` regista a contagem de chamadas a `record_citation` (posição de citação no documento), não a posição da entrada na `citation_order`. Uma entrada citada nas posições 1, 3, 5 tem `back_refs["key"] = [1, 3, 5]`.

### A.3 — `back_refs_for_key` no `Introspector`

**Ficheiro:** `entities/introspector.rs`

Adicionar método ao trait `Introspector`:

```rust
pub trait Introspector {
    // ... existentes ...
    fn back_refs_for_key(&self, key: &str) -> Vec<usize>;
}
```

`TagIntrospector` delega para `BibStore.back_refs`.

### A.4 — Render de back-references na bibliografia

**Ficheiro:** `engine/layout/bibliography.rs`

Após o corpo de cada entrada bibliográfica, adicionar os números de citação:

```rust
// Ao renderizar cada entrada:
let refs = introspector.back_refs_for_key(&entry.key);
if !refs.is_empty() {
    let refs_str: String = refs.iter()
        .map(|n| format!("[{n}]"))
        .collect::<Vec<_>>()
        .join(", ");
    // Renderizar: "[author info] — cited as: [1], [3]"
    // OU apenas indicar os números de citação (subset minimal)
}
```

**Decisão de subset:** renderizar apenas os números de citação (ex.: `↑[1][3]` ou `cited: [1][3]`), não os números de página. Divergência declarada face ao vanilla.

### A.5 — `ibid`

**Ficheiro:** `engine/layout/cite.rs` (ou onde `Content::Cite` é renderizado no layout)

Adicionar campo `last_cited_key: Option<String>` ao `Layouter` (ou ao `Introspector` se multi-pass).

```rust
// No arm Content::Cite { key, style, .. }:
let is_ibid = self.last_cited_key.as_deref() == Some(key.as_str());
self.last_cited_key = Some(key.clone());

let rendered = if is_ibid {
    "ibid.".into()
} else {
    // render normal [N]
    format!("[{}]", citation_number)
};
```

**Scope-out:** `ibid` com page override (`ibid., p. 5`). `op. cit.` (citação não-consecutiva abreviada) — scope-out para passo seguinte.

---

## Sub-item B — List of Figures (`lof`) e List of Tables (`lot`)

### B.1 — Contexto vanilla e precedente `outline()`

O vanilla suporta:

```typst
#outline(target: figure.where(kind: image))  // List of Figures
#outline(target: figure.where(kind: table))  // List of Tables
```

O cristalino tem `outline()` (P457) que gera TOC de headings. `lof()`/`lot()` são análogos mas listam figuras/tabelas com seus números e captions.

### B.2 — Decisão de implementação

Duas opções:

- **Opção A:** Generalizar `outline()` com argumento `target: Str` (`"figure"`, `"table"`).
- **Opção B:** Funções separadas `native_lof()` e `native_lot()`.

**Decisão:** Opção A é mais próxima do vanilla e reutiliza a infra de `outline()`. Verificar sonda se `Content::Outline` tem campo `target` ou se é headings-only.

### B.3 — `Content::Outline` alargado (se sonda confirmar headings-only)

**Ficheiro:** `entities/elements/outline.rs` (ou `entities/content.rs`)

```rust
pub enum OutlineTarget {
    Headings,           // comportamento existente P457
    Figures,            // NOVO — lista figuras com número e caption
    Tables,             // NOVO — lista tabelas com número e caption
}

// Em OutlineElem ou Content::Outline:
pub target: OutlineTarget,  // NOVO; default = Headings
```

Retrocompatibilidade: `native_outline()` sem `target:` = `OutlineTarget::Headings`.

### B.4 — `native_outline` alargada

**Ficheiro:** `rules/stdlib/structural.rs`

```
native_outline(target:?)  // "headings" | "figures" | "tables" | none
```

Alternativa: `native_lof()` e `native_lot()` como aliases.

### B.5 — Layout de LoF/LoT

**Ficheiro:** `engine/layout/outline.rs` (ou arm no `layout/mod.rs`)

O layout de `OutlineTarget::Figures` itera as figuras registadas no `Introspector` (por `citation_order` de figuras, ou por `flat_counter_at("figure", ...)`) e para cada uma renderiza:

```
Figure 1  Título da figura . . . . . . . 3
Figure 2  Outra figura . . . . . . . . . 7
```

Sem números de página reais (scope-out — requer 2-pass). Renderizar apenas número e caption.

**Fonte de dados:** O `Introspector` tem acesso a `figure_label_numbers` e `resolved_labels` (P460/P462). Verificar se caption está acessível.

**Decisão de subset:** LoF/LoT sem page numbers, apenas número + caption. Divergência declarada.

### B.6 — Acesso a captions de figuras no `Introspector`

Se o `Introspector` não tiver acesso às captions, adicionar `figure_captions: HashMap<String, String>` ao `BibStore` ou estrutura paralela, populado no walk de introspecção quando `Content::Figure` é encontrado.

```rust
// Em introspect.rs, walk de Content::Figure:
if let Some(caption) = figure.caption {
    introspector.register_figure_caption(label_key, caption.plain_text());
}
```

---

## Tests

### Sub-item A

- **L1:** `BibStore::record_citation` actualiza `back_refs` corretamente para citações repetidas.
- **L1:** `back_refs_for_key` devolve vec vazio para key não citada.
- **L1:** Citações múltiplas da mesma entry → `back_refs` acumula posições.
- **L2:** `ibid.` renderizado quando mesma key citada consecutivamente.
- **L2:** Segunda citação de key diferente → não é `ibid.`.
- **L2:** Layout de bibliografia inclui back-refs (`[1][3]` ou similar) após corpo da entrada.
- **L3 (E2E):** Documento com 2 entradas BibTeX; A citada nas posições 1 e 3; B citada na posição 2; back-refs na bibliografia mostram A→`[1,3]`, B→`[2]`.

### Sub-item B

- **L1:** `OutlineTarget::Figures` distinto de `OutlineTarget::Headings`.
- **L2:** `native_outline(target: "figures")` produz `Content::Outline` com target correto.
- **L2:** `native_lof()` (se existir) equivalente a `native_outline(target: "figures")`.
- **L2:** Layout de `OutlineTarget::Figures` itera figuras registadas.
- **L2:** Layout de `OutlineTarget::Tables` itera tabelas registadas.
- **L3 (E2E):** Documento com 2 figuras e `#lof()` — lista mostra "Figure 1  Caption 1" e "Figure 2  Caption 2".
- **L3 (E2E):** `#lot()` análogo para tabelas.

---

## Spec L0

### Actualizados

- `entities/bib_store.md` — campo `back_refs`, `record_citation` actualizado, `back_refs_for_key`.
- `entities/introspector.md` — método `back_refs_for_key`.
- `engine/layout/bibliography.md` — render de back-refs na entrada bibliográfica.
- `engine/layout/cite.md` — `ibid.` logic, `last_cited_key` no Layouter.
- `entities/elements/outline.md` (ou `entities/content.md`) — `OutlineTarget` enum, campo `target`.
- `rules/stdlib/structural.md` — `native_outline` alargada com `target:`, `native_lof`, `native_lot`.
- `engine/layout/outline.md` (ou `layout/mod.md`) — arms `Figures`/`Tables`.

---

## Scope-out explícito

- **Números de página nas back-references** — requer 2-pass com page tracking. Scope-out.
- **`op. cit.`** (citação não-consecutiva abreviada) — requer histórico de todas as chaves citadas. Futuro.
- **`ibid.` com page override** (`ibid., p. 5`) — scope-out.
- **Page numbers em LoF/LoT** — scope-out; apenas número + caption.
- **`lof`/`lot` filtrado por `kind`** (`figure.where(kind: image)`) — requer `Selector::Where` (Trilha 3). Scope-out.
- **Múltiplas bibliografias com back-refs independentes** — P420 scope-out; continua.
- **`ibid.` em estilos não-numéricos** (author-date) — subset numérico apenas.

---

## Critério de fecho

- [ ] Sondas: `BibStore.back_refs`, `last_cited_key`, `Content::Outline.target`, captions no Introspector — todos com `file:line`.
- [ ] `BibStore` tem `back_refs: HashMap<String, Vec<usize>>`.
- [ ] `record_citation` actualiza `back_refs`.
- [ ] `Introspector` trait tem `back_refs_for_key`.
- [ ] Layout de bibliografia renderiza back-refs.
- [ ] `last_cited_key` no Layouter; arm de `Cite` detecta `ibid.`.
- [ ] `OutlineTarget` enum com `Headings`, `Figures`, `Tables`.
- [ ] `Content::Outline` tem campo `target: OutlineTarget`.
- [ ] `native_outline(target:?)` aceita `"headings"`, `"figures"`, `"tables"`.
- [ ] `native_lof()` e `native_lot()` registados no scope.
- [ ] Layout de `OutlineTarget::Figures` itera figuras; idem para `Tables`.
- [ ] 14+ testes verdes (7 sub-A + 7 sub-B).
- [ ] Spec L0 actualizada (7 ficheiros).
- [ ] Divergências de paridade declaradas (back-refs sem page numbers; LoF/LoT sem page numbers).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 6: 3/5 completo** (estilos numéricos P468 + back-refs/ibid P472 + LoF/LoT P472).

---

## Próximo passo (Trilha 6 termina ou pivot)

Itens restantes de Trilha 6 após P472:

- **P473** — `op. cit.` + múltiplas bibliografias num documento (M, ~30 min) — Trilha 6.
- **P473** — Pivot para Trilha 4: `Value::Gradient` tipo real (M, ~35 min).
- **P473** — Pivot para Trilha 7: `columns`/`colbreak` (L, ~60+ min).
- **P473** — Trilha 8: verificar se `pad`/`corners`/`sides` têm trabalho real pendente (sonda XS antes de propor).

---

## Estado pós-P471 (para referência)

| Passo | Descrição | Estado | Trilha |
|-------|-----------|--------|--------|
| P465 | `repr()` completo | ✅ FECHADO | 8 |
| P466 | Métodos array/dict/str | ✅ FECHADO | 8 |
| P467 | Sonda `Selector::Where` | ✅ FECHADO | 3 |
| P468 | Estilos numéricos `[1]`, `[2]` | ✅ FECHADO | 6 |
| P469 | `Value::Relative` (`Rel<Length>`) | ✅ FECHADO | 8 |
| P470 | Marcadores list/enum + i18n caption | ✅ FECHADO | 8 |
| P471 | `Symbol` + `highlight` params + `sub`/`super` size | ✅ FECHADO | 8 |
| **P472** | Back-refs + ibid + LoF/LoT | 🔄 EM PREPARAÇÃO | 6 |

**Trilha 1: COMPLETA.**
**Trilha 2: COMPLETA.**
**Trilha 3: 1/3 completo.**
**Trilha 6: 1/5 completo** (pré-P472).
**Trilha 8: 5/8 completo.**
**Inventário de débitos: LIMPO.**

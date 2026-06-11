# Relatório P323 — Lote 8 (largura: Ref, Outline, Columns, Quote)

**Pré-condição**: Lote 7 (P322) fechado — lint 0, suíte verde. ✅ Verificado.
**Commit**: `Passo 323 — lote 8` (lote único; sem caronas neste passo).

---

## Lote 8 — largura crescente (4 variantes)

**Composição (confirmada no checkpoint): 4 variantes**, ordem por largura
crescente: `Ref`(25) · `Outline`(26) · `Columns`(29) · `Quote`(32) =
**~112 sites**. **1 locatável** (`Outline`); 3 não-locatáveis.

### Forma de cada `…Elem`

| Variante | Campos | Locatável | `plain_text` | `is_empty` | `map_*` | Hash |
|----------|--------|-----------|--------------|------------|---------|------|
| `RefElem` | `target: Label` | não | `@{target.0}` | default `false` | terminal (leaf) | **derive** (`Label: Eq+Hash`) |
| `OutlineElem` | **unit** | **SIM** | `""` | default `false` | terminal (leaf) | **derive** (unit) |
| `ColumnsElem` | `count`, `gutter?`, `body` | não | `body.plain_text()` | `body.is_empty()` | **recurse** `body` | **manual** (`Length`/f64) |
| `QuoteElem` | `body`, `attribution?`, `block`, `quotes` | não | custom (aspas + `— attrib`) | `body.is_empty()` | **recurse** `body`+`attribution` | **derive** (`Content: Hash`) |

### `Outline` — variante unit (decisão (a) do dono)

- **Não é inédito**: `DividerElem;` (piloto P316) e `LinebreakElem;` (Lote 5)
  já são structs unit — mesmo padrão. Verificado antes de gravar.
- **Migração uniforme** (opção (a)): `Content::Outline` →
  `Content::Outline(Arc<OutlineElem>)`. Custo `Arc<ZST>` negligível (~1/doc).
- **Locatável** — absorve `element_kind`/`to_payload` no trait (precedente
  Heading/Lote 6). `extract_payload.rs` passou a `Content::Outline(e) =>
  e.to_payload()`; `locatable.rs` a `Content::Outline(_) => true`. O consumo
  por `ElementPayload::Outline` (`introspect.rs:645`/`from_tags`) é
  **inalterado** (matcheia o payload, não o `Content`).
- **L0 anota**: campos do vanilla (`title`/`depth`/`indent`/…) pendentes de
  cobertura aterram **no módulo** sem tocar o hub — a vantagem estrutural do
  modelo D. `Outline` é elemento de utilizador (fora da classe DEBT-58).

### Achados content-preserving

- **Construtor `Content::reference`** (não `r#ref`): o raw identifier seria
  fricção em cada call-site (colisão com a keyword `ref`). Comentário de 1
  linha no módulo regista a colisão.
- **`|`-combinado barato**: `Ref` e `Outline` partilham o arm terminal de
  `map_content`/`map_text` com outros leaves. Como são terminais (sem binding),
  ficaram **no mesmo arm** (`{ .. }`/bare → `(_)`) — **sem split** (contraste
  com Lotes 5/6, que tinham binding de campos).
- **`Ref` em padrão de teste**: o transformador de construção converteu por
  engano um `matches!` (posição de padrão) — corrigido manualmente para
  `Content::Ref(e) if e.target.0 == …`. Nenhuma asserção alterada.

### Hub (`content.rs`) — 6 matches → dispatch

- `enum`: `Ref {…}`/`Outline`/`Columns {…}`/`Quote {…}` →
  `Ref(Arc<RefElem>)` … `Quote(Arc<QuoteElem>)`.
- `plain_text`, `is_empty`, `map_content`, `map_text`, `PartialEq`: braços de
  cada variante → **1 linha** de despacho ao trait. (`get_field`: nenhuma das
  4 tinha arm — default `None`.)
- Construtores: `columns` actualizado; `reference`/`outline`/`quote`
  adicionados.

### Sites externos (produção)

`introspect.rs` (walk + materialize_time), `introspect/{extract_payload,
locatable}.rs`, `eval/{mod,closures}.rs`, `layout/{mod,outline}.rs`,
`stdlib/{layout,structural}.rs`: blocos terminais → `(_)`; destruturações →
`(e)` + `e.campo`; construções → `Content::{reference,outline,columns,quote}`.

---

## Verificação

- **`cargo build`**: limpo (lib 0 erros).
- **Suíte**: `RUST_MIN_STACK=33554432 cargo test --workspace` →
  **typst-core 2615 passed** (era 2596; **+19** líquido, com os 4 módulos
  novos a somar testes unitários), 0 failed. Restantes crates verdes.
- **`crystalline-lint --fix-hashes .`**: 0 drift; **`crystalline-lint .`**:
  **✓ No violations found**.

## Contabilidade (modelo actualizado)

- Migradas **43 → 47**; restantes element-shaped **~19 → ~15**.
- Conta de fecho: 47 + 4 `Set*` + 11 (DEBT-58) + 15 restantes = **77** ✓.
- Próximo gatilho: lotes 9+ saem dos `~15` por largura (`SmartQuote`28 a
  `Figure`89, com o bloco grid/table cell `~193 sites` como lote próprio); ao
  esgotar os elegíveis, dispara a triagem do DEBT-58 e a decisão F.

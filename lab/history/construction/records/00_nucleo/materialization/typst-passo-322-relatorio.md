# Relatório P322 — Lote 7 (largura: Raw, Align, Image, Hide, Repeat)

**Pré-condição**: Lote 6 (P321) fechado — lint 0, suíte verde. ✅ Verificado.
**Commit**: `Passo 322 — lote 7` (lote único; sem caronas neste passo).

---

## Lote 7 — largura crescente (5 variantes)

**Composição (confirmada no checkpoint): 5 variantes**, ordem por largura
crescente: `Raw`(9) · `Align`(11) · `Image`(17) · `Hide`(18) · `Repeat`(18) =
**~73 sites**. Lote **não-locatável padrão** — todas no default `element_kind`/
`to_payload` (`None`); nenhum braço de `extract_payload`/`locatable` tocado.

### Forma de cada `…Elem`

| Variante | Campos | `plain_text` | `is_empty` | `map_*` | Hash |
|----------|--------|--------------|------------|---------|------|
| `RawElem` | `text: EcoString`, `lang: Option<EcoString>`, `block: bool` | `text.to_string()` | default `false` | terminal (leaf) | derive |
| `ImageElem` | `path: String`, `data: PtrEqArc<Vec<u8>>`, `width/height: Option<Box<Value>>` | `""` | default `false` | terminal (leaf) | **manual** (`Value`, `PtrEqArc`) |
| `AlignElem` | `alignment: Align2D`, `body: Content` | `body.plain_text()` | default `false` (não delega) | **recurse** `body`, preserva `alignment` | **manual** (`Align2D` sem `Hash`) |
| `HideElem` | `body: Content` | `""` | `body.is_empty()` | **recurse** `body` | derive |
| `RepeatElem` | `body: Content`, `gap: Option<Length>`, `justify: bool` | `body.plain_text()` | `body.is_empty()` | **recurse** `body`, preserva `gap`/`justify` | **manual** (`Length`/f64) |

- **Raw/Image** são leaves (`map_content`/`map_text` terminais — devolvem `self`).
- **Align/Hide/Repeat** são contentores de um único `body`: recursam em **ambos**
  `map_content` e `map_text`, reconstruindo `Arc<…Elem>` com os campos
  cosméticos preservados.
- **`Align::is_empty` não delega** a `body.is_empty()` — mantém o default
  `false` (paridade: um `align` vazio ainda ocupa o eixo). `Hide`/`Repeat`
  **delegam** (são transparentes ao conteúdo).

### Achado content-preserving — `.into()` redundante (E0283)

Os construtores novos tomam `impl Into<…>`:
`raw(text: impl Into<EcoString>, …)` e `image(path: impl Into<String>, …)`.
Os call-sites de teste vinham do modelo struct-literal (`text: "…".into()`,
`path: "…".into()` — necessário quando o campo era `EcoString`/`String`).
Com o construtor genérico, o `.into()` no argumento torna-se **ambíguo**
(`cannot satisfy _: Into<EcoString>` / `Into<String>` — E0283): o compilador
não consegue inferir o tipo intermédio. **Correcção**: remover o `.into()`
(passar `&str` directo ao parâmetro genérico). **Nenhuma asserção alterada** —
só a sintaxe de construção. Registado como nota no modelo.

### Hub (`content.rs`) — 6 matches → dispatch

- `enum`: `Raw {…}`/`Align {…}`/`Image {…}`/`Hide {…}`/`Repeat {…}` →
  `Raw(Arc<RawElem>)` … `Repeat(Arc<RepeatElem>)`.
- `plain_text`, `is_empty`, `map_content`, `map_text`, `get_field`, `PartialEq`:
  braços de cada variante → **1 linha** de despacho ao trait.
- Construtores: `raw`/`hide`/`repeat` actualizados; `align`/`image` adicionados
  (antes inline struct-literal nos callers).

### Sites externos (produção)

`introspect.rs`, `introspect/locatable.rs`, `layout/mod.rs`,
`stdlib/{layout,structural,figure_image}.rs`: blocos terminais → `(_)`;
destruturações → `(e)` + `e.campo`; construções → `Content::{raw,align,image,
hide,repeat}(…)`. `figure_image.rs::infer_kind_from_body` agora matcheia
`Content::Image(_)`/`Content::Raw(_)`.

---

## Verificação

- **`cargo build`**: limpo (lib 0 erros).
- **Suíte**: `RUST_MIN_STACK=33554432 cargo test --workspace` →
  **typst-core 2596 passed** (era 2583; **+13** testes unitários novos nos 5
  módulos), 0 failed. Restantes crates verdes.
- **`crystalline-lint --fix-hashes .`**: 0 drift; **`crystalline-lint .`**:
  **✓ No violations found**.

## Contabilidade (modelo actualizado)

- Migradas **38 → 43**; restantes element-shaped **~24 → ~19**.
- Conta de fecho: 43 + 4 `Set*` + 11 (DEBT-58) + 19 restantes = **77** ✓.
- Próximo gatilho: lotes 8+ saem dos `~19` por largura (`Quote`20 a `Figure`89,
  com o bloco grid/table cell `~193 sites` como lote próprio); ao esgotar os
  elegíveis, dispara a triagem do DEBT-58 e a decisão F.

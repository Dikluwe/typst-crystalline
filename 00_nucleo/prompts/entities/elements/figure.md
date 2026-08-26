# Prompt L0 — `entities/elements/figure` — `FigureElem`
Hash do Código: 2caa9204

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/figure.rs`
**Origem**: modelo D (ADR-0105), **Lote 13 P328** (o último element-shaped).
Trait e glossário (§A.0): ver `entities/elements/_comum.md`. Contentor — `map_*` recursam em
`body` **e** `caption` (precedente `Quote` L8; **simétrico**, sem assimetria).

> **Fronteira: LOCATÁVEL** (M1, junto de Heading/Cite). Absorve o braço de
> `extract_payload` no trait (precedente Heading/Lote 6): `element_kind` →
> `ElementKind::Figure`, `to_payload` → `ElementPayload::Figure { kind,
> counter_update: Step, is_counted }`. O consumo por `ElementPayload::Figure`
> (`from_tags`/walk) é **inalterado** (matcheia o payload, não o `Content`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FigureElem {
    pub body:      Content,             // era Box<Content>
    pub caption:   Option<Content>,     // era Option<Box<Content>>
    pub kind:      Option<String>,
    pub numbering: Option<String>,
}
```

`Content::Figure { body, caption, kind, numbering }` →
`Content::Figure(Arc<FigureElem>)`. Construtor ergonómico (cobre os 4 campos
— logo o transformador usa `Content::figure(…)`, não Arc-wrap):
`Content::figure(body, caption, kind, numbering)`. **Deriva `Hash`** (`Content`
tem `impl Hash` manual; `Option<String>` deriva).

## `impl Element for FigureElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | **custom** (`content.rs:1706`): junta `body.plain_text()` e `caption.plain_text()` com espaço; cada um omitido se vazio |
| `is_empty` | **override**: `self.body.is_empty() && self.caption.as_ref().is_none_or(\|c\| c.is_empty())` (`content.rs:1573`) |
| `map_content` | **recursivo** em `body` e `caption`, preserva `kind`/`numbering` (`content.rs:1996`) |
| `map_text` | **recursivo** em `body` e `caption`, preserva `kind`/`numbering` (`content.rs:2206`) — **simétrico** com `map_content` |
| `get_field` | default `None` |
| `element_kind` | `Some(ElementKind::Figure)` |
| `to_payload` | `Some(ElementPayload::Figure { kind: self.kind.clone(), counter_update: CounterUpdate::Step, is_counted: self.numbering.is_some() && self.caption.is_some() })` (absorve `extract_payload.rs:24`) |

> **Fonte de paridade e correcção de enquadramento (P1031).**
>
> **Caption é opcional** — literal, `crates/typst-library/src/model/figure.rs:203-204`
> (vanilla ratificado `e0e8ca4d`): `/// The figure's caption.` sobre
> `pub caption: Option<Packed<FigureCaption>>`. Publicado em
> `typst.app/docs/reference/model/figure/#parameters-caption`. ✅ confere.
>
> **Numeração automática** — literal, `figure.rs:296-299`:
> `#[default(Some(NumberingPattern::from_str("1").unwrap().into()))] pub numbering:
> Option<Numbering>`, doc *"How to number the figure. Accepts a numbering pattern or
> function taking a single number."* Confirma que a numeração é automática **e que tem
> default**, o que o cristalino não aplica — achado escalado registado em
> `compiler/layout_figure.md` §P1031 (ACHADO 1).
>
> **`is_counted: numbering.is_some() && caption.is_some()` não é o gate da linguagem.** O
> vanilla conta em `figure.rs:437-445`, `impl Count for Packed<FigureElem>`:
>
> ```rust
> fn update(&self) -> Option<CounterUpdate> {
>     // If the figure is numbered, step the counter by one.
>     self.numbering().is_some().then(|| CounterUpdate::Step(NonZeroUsize::ONE))
> }
> ```
>
> — só `numbering.is_some()`; **a caption não entra**. Uma figura sem caption consome
> número na mesma.
>
> **Medição (2026-08-13)**, documento com `#set text(lang: "en")` + `#set figure(numbering:
> "1")` + duas figuras sem caption + duas com caption (`C1`, `C2`):
>
> | Binário | Saída |
> |---|---|
> | Vanilla `/usr/local/bin/typst` (`typst 0.15.1 (e0e8ca4d)`) | `Figure 3: C1` / `Figure 4: C2` |
> | Cristalino `target/release/typst` (fonte em HEAD `4f64e4e69`) | `Figure 3: C1` / `Figure 4: C2` |
>
> **Byte-idêntico**: as duas figuras sem caption consomem os números 1 e 2 nos dois
> binários. Logo o observável de linguagem **está em paridade**; o que diverge é a
> expressão interna — o número vem do índice
> (`Introspector::figure_number_at_index`, ver `compiler/layout_figure.md`), não deste
> `is_counted`. A conjunção `&& caption.is_some()` fica registada como **mecânica interna
> com enquadramento enganador**, não como regra da linguagem. Não é achado de código (o
> observável coincide); é achado de redacção, fechado aqui.

## `eq`

`#[derive(PartialEq)]` compara os 4 campos (paridade `content.rs:1837`).

## Interação `figure_image.rs`

`infer_kind_from_body` (tocado no Lote 7) matcheia o **body** (`Content::Image(_)`/
`Content::Raw(_)`) — **inalterado** (matcheia o conteúdo, não `Figure`). Só a
**construção** em `native_figure` (`figure_image.rs:92`) muda para
`Content::figure(…)`.

# Prompt L0 — `compiler/layout/link` — Layout de `Content::Link`
Hash do Código: 03639cf2

**Camada**: L1 · **Alvo**: `01_core/src/compiler/layout/link.rs`
**Origem**: P422 (S) — atomização do layout de hiperligações; **P424** — bbox.

---

## Decisão arquitetural (ADR-0109 forma B)

- `entities/elements/link.rs` — `LinkElem` struct puro (`url`, `body`).
- `compiler/layout/link.rs` — free function `layout_link(layouter, &LinkElem)` que
  renderiza o body, calcula a bounding-box acumulada e envolve os itens
  resultantes em `FrameItem::Link`.
- `entities/layout_types.rs` — `FrameItem::Link { target, items, pos, size }`
  com `LinkTarget::Url(EcoString) | LinkTarget::Destination(Label)`.

## Algoritmo

1. Renderizar `body` normalmente através do `Layouter`.
2. Coletar os `FrameItem`s produzidos (tanto em `current_line` quanto em
   `current_items`, caso tenha havido flush).
3. Calcular `pos`/`size` da bbox acumulada dos items:
   - `Text`: medir avanço via `FontMetrics::advance` e altura via
     `FontMetrics::vertical_metrics`.
   - `Glyph`: `x_advance` × `size`.
   - `Shape`/`Image`: `width` × `height`.
   - `Line`: bounding box entre `start` e `end`.
   - `Group`: aproximar por `pos` + `inner_width`/`inner_height`.
   - `Link` aninhado: reutilizar `pos`/`size` já calculados.
4. Emitir um único `FrameItem::Link { target, items, pos, size }` no frame atual.
   Para `Content::Link`, `target = LinkTarget::Url(url)`.

## Scope-out

- `QuadPoints` para áreas clicáveis multi-linha.

---

## P1031 — fonte de paridade e achado escalado

Doc comments `#[elem]`/campos do vanilla ratificado (`e0e8ca4d`),
`crates/typst-library/src/model/link.rs`, publicados em
`typst.app/docs/reference/model/link/`:

- **Propósito** — `link.rs:24`: *"Links to a URL or a location in the document."*
- **Sintaxe automática** — `link.rs:41-49`: *"= Syntax — This function also has dedicated
  syntax: Text that starts with `http://` or `https://` is automatically turned into a
  link. To avoid automatic creation of a link, you can put the text in a string. […]"*
- **Destinos aceites** — `link.rs:175-195`, campo `#[required] pub dest: LinkTarget`:
  *"To link to another part of the document, `dest` can take one of three forms: - A label
  attached to an element. […] - A location (typically retrieved from `here`, `locate` or
  `query`). - A dictionary with a `page` key of type integer and `x` and `y` coordinates of
  type length. Pages are counted from one, and the coordinates are relative to the page's
  top left corner."* O tipo é `pub enum LinkTarget { Dest(Destination), Label(Label) }`
  (`link.rs:259-262`).

> **Aparência por defeito — o "scope-out" anterior era, afinal, paridade.**
>
> A lista de scope-outs deste L0 incluía *"Cor azul / sublinhado — aguarda
> `FrameItem::Decoration`"*, o que sugeria uma lacuna. A documentação diz o contrário
> (`link.rs:26-27`): *"By default, links do not look any different from normal text.
> However, you can easily apply a style of your choice with a show rule."* Não pintar o
> link é o comportamento correcto da linguagem; a entrada foi removida da lista.

> **ACHADO ESCALADO — o passo 4 ("`Content::Link` mapeia sempre para URL") descreve uma
> restrição do cristalino, não a linguagem.**
>
> Medição directa (2026-08-13; vanilla `/usr/local/bin/typst` = `typst 0.15.1 (e0e8ca4d)`;
> cristalino `target/release/typst` da fonte em HEAD `4f64e4e69`, árvore só com edições em
> `00_nucleo/prompts/**`). Documento: `= Intro <intro>` + `#link(<intro>)[Ir para intro]` +
> `#link("https://example.com")[Site]`.
>
> | Binário | Resultado |
> |---|---|
> | Vanilla | compila; texto `Intro` / `Ir para intro Site` |
> | Cristalino | **erro**: `link() espera URL como string, recebeu label` |
>
> Das três formas de destino documentadas (label, location, dicionário `page`/`x`/`y`),
> **nenhuma** é aceite; só a string de URL. `LinkTarget::Destination(Label)` já existe no
> lado do `FrameItem` (§"Decisão arquitetural"), pelo que a lacuna está na aceitação em
> eval, não na representação. É superfície de linguagem (morfologia do argumento,
> ADR-0107) → paridade, não divergência permitida. Mudança de contrato público + de
> comportamento por defeito → gate ADR-0127 e passo próprio. **Não implementado aqui.**

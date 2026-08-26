# Prompt L0 — `entities/tiling` — padrão de azulejos (Tiling)
Hash do Código: 8fb9a293

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/tiling.rs`
**Origem**: Passo 395 — modelagem de tipos (M); abertura do portão ADR-0017.
**ADRs**: ADR-0017 (enum fechado; variant novo exige tipo migrado), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (graded scope-out).

---

## 1. Contexto

O vanilla expõe `tiling(...)` como construtor de padrão de preenchimento repetido em grelha. A morfologia linguagem é:

> **Fonte de paridade (P1031)** — **citação literal** do doc comment `#[ty]` do vanilla
> ratificado (`e0e8ca4d`), `crates/typst-library/src/visualize/tiling.rs:15-24`, publicado
> em `typst.app/docs/reference/visualize/tiling/`:
>
> *"A repeating tiling fill. Typst supports the most common type of tilings, where a pattern
> is repeated in a grid-like fashion, covering the entire area of an element that is filled
> or stroked. The pattern is defined by a tile `size` and a body defining the content of
> each cell. You can also add horizontal or vertical `spacing` between the cells of the
> tiling. The `offset` and `angle` determine the placement of the tiling."*
>
> **Precisão trazida pela citação**: o `tiling` preenche **ou traceja** (*"filled or
> stroked"*) e o corpo de cada célula é **conteúdo arbitrário** — não é um construtor "para
> gradientes e imagens" como a redacção anterior dizia. Corrigido acima. O construtor tem
> cinco parâmetros documentados: `size`, `spacing`, `offset`, `angle` e `relative`; o
> exemplo do L0 nomeia dois (`size`, `relative`) — os outros três são **lacuna
> documentada**.
>
> **Sobre `relative`** — `tiling.rs:36-41`: *"Tilings are also supported on text, but only
> when setting `relative` to either `{auto}` (the default value) or `{"parent"}`."* Confirma
> que `auto` é o default; note-se que o exemplo acima usa `relative: "self"`, que é um valor
> válido mas **não** o default e **não** suportado sobre texto.

```typst
#let t = tiling(image("pat.png"), size: auto, relative: "self")
#box(fill: t)
```

Este passo **modela apenas o tipo** `Tiling` em L1; o construtor `tiling()` é P396.

## 2. Estrutura

```rust
pub struct Tiling {
    pub body: TilingBody,
    pub size: Option<Size>,
    pub relative: TilingRelative,
    pub spacing: Option<Size>,
}

pub enum TilingBody {
    Image(ImageElem),
    Gradient(Gradient),
    Color(Color),
}

pub enum TilingRelative {
    Self,
    Parent,
}
```

- `body`: corpo do padrão — imagem, gradiente ou cor sólida.
- `size`: tamanho da célula do padrão; `None` ↔ `auto` (bounds do body).
- `relative`: `"self"` (default) ou `"parent"`.
- `spacing`: gap entre repetições; `None` ↔ zero.

## 3. Decisões

- `TilingBody::Gradient` é **placeholder** — o tipo `Gradient` existe (P262) mas o consumer real de Tiling+Gradient é scope-out ADR-0054. O variant existe para paridade futura.
- `TilingBody::Image` usa `ImageElem` já existente em L1.
- `TilingBody::Color` é o fallback simples e único consumer inicial.
- Derives: `Debug`, `Clone`, `PartialEq`.
- Constructor: `Tiling::new(body: TilingBody) -> Self` com defaults (`size: None`, `relative: Self`, `spacing: None`).

## 4. Integração Paint / Fill

`Tiling` integra no enum `Paint` de `entities/paint.rs` como `Paint::Tiling(Tiling)`, lado a lado com `Solid` e `Gradient`. O fallback de cor (`Paint::to_color`) resolve:

- `TilingBody::Color(c)` → `c`.
- `TilingBody::Gradient(g)` → `g.first_stop_color()`.
- `TilingBody::Image(_)` → cor preta (scope-out graded — pattern fill de imagem ainda não renderizado).

## 5. Scope-out

- `tiling()` função nativa — P396.
- Render PDF de pattern fill — scope-out ADR-0054 graded.
- `TilingBody::Gradient` consumer real — scope-out.
- `TilingBody::Image` consumer real — scope-out.

## 6. Testes

- `tiling_new_color` — construção com cor.
- `tiling_new_image` — construção com `ImageElem` mock.
- `tiling_equality` — PartialEq.
- `tiling_clone` — Arc clone O(1) por body image compartilhado.
- `tiling_relative_default` — `Self` default.
- `tiling_size_none` — auto default.
- `paint_tiling_to_color` — fallback Color via `Paint::to_color`.


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — identidade pública do tipo `tiling`

O nome global `tiling` é o valor `Value::Type(Type::Tiling)`, chamável
pelo dispatcher estático e delegado ao construtor L1 já existente. Assim,
`type(tiling) == type` e `repr(type(tiling)) == "type"`, enquanto valores
construídos continuam com `type_name() == "tiling"`. A representação interna
da entidade e suas operações não mudam.

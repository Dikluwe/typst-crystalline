# Prompt L0 — `entities/tiling` — padrão de azulejos (Tiling)
Hash do Código: 42d13202

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-placement.toml sha256:9c88c9c3bde8b005623fa70b5213400655a9a741b1a140b6b76c7225fc16edb4

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
os probes públicos em `lab/results/construction/diagnostics/superficie-linguagem-p1140*`
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

## P1245 — contrato declarativo aprovado para conteúdo arbitrário

A decisão abaixo é vigente e não introduz `ResolvedTiling` público. Os limites
de stroke e targets não cobertos permanecem explicitamente fora do contrato.

### Medição que precede a decisão

O modelo anterior em `01_core/src/entities/tiling.rs:22-31,61-77` conserva
somente corpo fechado Image/Gradient/Color, size, spacing e relative
self/parent. O vanilla ratificado `a51e02804`, em
`crates/typst-library/src/visualize/tiling.rs:59-76,99-185,278-376`, aceita
conteúdo arbitrário, materializa-o em frame na fase de layout, resolve
`size:auto`, conserva spacing/offset/angle/relative e gira a grade em torno
do ponto offset (ordem esclarecida no núcleo tiling-placement). A obrigação é a semântica e morfologia da linguagem, não armazenar um
`Frame` vanilla nem copiar sua mecânica Rust (ADR-0107).

### Contrato público proposto

- `Tiling` permanece entidade declarativa pura L1 e transporta o corpo como
  `Content` fechado conforme ADR-0026; quando a recursão física exigir
  indireção, ela usa `Arc<Content>` ou representação equivalente já permitida,
  sem `dyn`, vtable ou estado global.
- O contrato público conserva `size`, `spacing`, `offset`, `angle` e
  `relative`. `offset` aceita os componentes relativos/absolutos necessários à
  linguagem; `angle` é finito. Defaults: size auto, spacing zero, offset zero,
  angle zero e relative auto.
- `relative:auto` preserva a decisão contextual da linguagem: self para shapes
  e parent para texto. `self` e `parent` continuam seleções explícitas.
- A entidade não contém frame resolvido, não lê recursos, não executa layout e
  não conhece exporters. Não existe novo tipo público `ResolvedTiling`.
- A materialização resolvida é detalhe privado da fase de layout existente e
  só pode alcançar targets pelo contrato normal do pipeline.
- Fallback para primeira cor/preto pode continuar como diagnóstico `Unknown`,
  mas nunca prova paridade do tiling nem substitui a morfologia repetida.

### Compatibilidade

Construções antigas de Color/Image continuam representáveis. Os defaults
anteriores só são preservados quando equivalentes ao observável vanilla; em
particular, `relative:auto` não pode ser colapsado fisicamente em `self` antes
de conhecer o contexto de layout.


## Origem da rotação de tiling — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-transformacao-2026-10-14/before/results.json.gz`
conserva fontes/owners, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado, diffstat, UTC e binários. Referência pinada
9f2b6e8715237cb086899a42873660fe744622e8. Para angle90 e offset(3pt,5pt),
o SVG vanilla emite matrix(0 1 -1 0 3 5); o Tekt emite (0 1 -1 0 -5 3).
No pin, visualize/tiling.rs:173-174 descreve rotação em torno do offset;
:375-377 concatena translate(offset) com rotate(angle), e
layout/transform.rs:334-343 calcula T*R. O texto histórico "offset antes de
angle" foi interpretado incorretamente como rotação do próprio offset.
Semântica geométrica é obrigação de linguagem; ordem de chamadas Rust não é.

before/render-comparison.json.gz discrimina targets: em quarter sem offset,
Tekt PDF e SVG coincidem exatamente com vanilla SVG, enquanto vanilla PDF
difere em 414 pixels >16. Não copiar essa diferença do PDF como requisito.
Em quarter-offset, os SVGs diferem em 1764 pixels >16. Rasterizações usam
canvas inteiro 240×192, branco, librsvg e Poppler, sem recortes/tolerâncias
criadas para aprovar. A evidência não certifica parent composto nem todo PDF.

### Responsabilidade deste owner

Preservar todos os campos, tipos, valores, constructor e defaults da entidade
Tiling. A documentação de angle declara rotação horária em torno do offset,
sem implicar que o vetor offset é girado. O núcleo pinado contém a obrigação
compartilhada. Não introduzir frame resolvido ou novo tipo público; nenhum
método ganha comportamento de layout. A ambiguidade do texto de P1245 fica
retificada, sem alteração de contrato Rust ou fase.

Correção interna/de paridade ADR-0127 em fluxo contínuo: L0 primeiro,
RED→GREEN e resselo restrito. Nenhuma mudança pública ou nova fase.

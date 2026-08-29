# Prompt L0 — `stdlib/tiling` — constructor `tiling(...)`
Hash do Código: e641265e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/visualize.rs`
**Origem**: Passo 396 — materialização do constructor `tiling()` (M); depende P395.
**ADRs**: ADR-0017 (portão aberto por P395), ADR-0107 (paridade linguagem), ADR-0054 (graded scope-out).

---

## 1. Contexto

P395 modelou `Value::Tiling`, `TilingBody`, `TilingRelative` e `Paint::Tiling`. Este passo
materializa o constructor user-facing `tiling(...)` e activa o consumer layout (fallback
Color) para provar que o tipo funciona no pipeline.

## 2. Função nativa

`native_tiling(ctx, args, world, current_file)`:

- `body` (positional obrigatório):
  - `Value::Color(c)` → `TilingBody::Color(c)`.
  - `Value::Content(Content::Image(img))` → `TilingBody::Image(*img)`.
  - `Value::Str(path)` → resolve via `world.read_bytes(current_file, path)` e constrói
    `ImageElem` (graded — se não houver world, retorna erro claro).
  - `Value::Tiling(t)` → identidade (devolve o mesmo `Value::Tiling`).
  - `Value::Gradient(_)` → erro `"gradient em tiling não suportado — scope-out ADR-0054"`.
  - outro → erro de tipo.
- `size` (named, opcional):
  - `Value::Length(l)` → `Size::uniform(Pt(l.abs.to_pt()))`.
  - `Value::Array([w, h])` com dois `Length` → `Size { width, height }`.
  - `Value::None` | `Value::Auto` → `None`.
  - outro → erro.
- `spacing` (named, opcional): mesmo formato que `size`.
- `relative` (named, opcional):
  - `"self"` → `TilingRelative::Itself`.
  - `"parent"` → `TilingRelative::Parent`.
  - outro → erro.

Devolve `Value::Tiling(Arc::new(tiling))`.

## 3. Helpers

- `extract_size(value, fn_name, field) -> SourceResult<Option<Size>>`.
- `parse_relative(value) -> SourceResult<TilingRelative>`.

## 4. Registo

Registar em `make_stdlib`:

```rust
scope.define("tiling", Value::Func(Func::native("tiling", native_tiling)));
```

## 5. Consumer layout

`Paint::Tiling` já existe em `entities/paint.rs` com `to_color()` fallback. Verificar que
consumers de `Paint` em layout/export não dão panic — usam `to_color()` ou match
exhaustivo. Não adicionar pattern fill real (scope-out ADR-0054).

## 6. Scope-out

- Pattern fill PDF real — ADR-0054 graded (XL futuro).
- `Gradient` como body de `tiling` — rejeitado com erro.
- Semântica real de `relative: "parent"` — armazenado, mas consumer igual a `"self"`.

## 7. Testes

- `tiling(red)` → `Value::Tiling` com `TilingBody::Color`.
- `tiling(red, size: 50pt)` → `size` uniforme.
- `tiling(red, size: (50pt, 30pt))` → `Size` diferenciado.
- `tiling(red, relative: "parent")` → `TilingRelative::Parent`.
- `tiling(tiling(red))` → identidade.
- `tiling(123)` → erro de tipo.
- `tiling(gradient.linear(...))` → erro ADR-0054.
- `#rect(fill: tiling(red))` → layout aceita e emite shape com fallback Color.


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — `tiling` como tipo chamável

O binding global `tiling` passa a `Value::Type(Type::Tiling)`. A chamada
delega ao mesmo `native_tiling`, preservando corpo, size, spacing, relative e
resultado. Nenhuma lógica de render ou entidade muda neste lote.

## P1245 — construtor declarativo completo proposto

**Gate ADR-0127 (histórico):** o dono confirmou a mudança pública no P1245.
O contrato observável foi pré-selado e materializado no P1254. Esta decisão é
vigente; qualquer ampliação para stroke ou outro target exige evidência própria
e não pode ser inferida do sucesso focal de fill SVG.

### Medição que precede a decisão

O construtor anterior em `01_core/src/compiler/stdlib/visualize.rs:27-113`
aceita Color/Image/path, rejeita Gradient e não analisa offset ou angle. O
vanilla ratificado `a51e02804`, em
`crates/typst-library/src/visualize/tiling.rs:99-185,250-315`, recebe Content,
valida size/spacing/offset/angle e deixa a produção do frame dependente do
layout. Esta superfície é linguagem pública (ADR-0107).

### Contrato do construtor

- O argumento posicional obrigatório é `Content` arbitrário. Valores já
  convertíveis a Content seguem a conversão pública normal; não se cria uma
  whitelist paralela Color/Image/Gradient no construtor.
- Named args: `size`, `spacing`, `offset`, `angle` e `relative`, com defaults
  auto, zero, zero, zero e auto, respetivamente.
- `size:auto` permanece não resolvido durante eval. `spacing` deve ser finito e
  obedecer às restrições absolutas da linguagem. `offset` deve ser finito,
  rejeitar componente font-relative proibido e preservar percentuais para
  resolução contra o pitch. `angle` deve ser finito.
- `relative` aceita auto, self e parent. O construtor não decide o auto sem
  contexto de layout.
- `tiling(t) == t` permanece identidade quando não há overrides incompatíveis.
- Eval apenas valida e constrói a entidade declarativa. Não materializa frame,
  não rasteriza, não resolve `size:auto` e não importa lógica de exporter.
- Erros são observáveis e devem manter distinção entre tipo inválido, valor não
  finito e unidade proibida; a redação exata só é obrigação quando medida como
  superfície pública.

### Fronteira de fase

A fase de layout existente é a única autoridade para materializar o Content da
célula, derivar size auto, resolver percentuais e aplicar offset antes de angle.
Qualquer estrutura resolvida é privada dessa fase. Exporters recebem somente a
saída normal do layout e não repetem avaliação ou layout do corpo.

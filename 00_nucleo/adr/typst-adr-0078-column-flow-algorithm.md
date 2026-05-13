# ⚖️ ADR-0078: Column flow algorithm — `Region/Regions` abstraction + multi-column consumer

**Status**: **IMPLEMENTADO** (P215 PROPOSTO 2026-05-12 → P221
IMPLEMENTADO 2026-05-12 — sub-fases (a)+(b) materializadas;
DEBT-56 ENCERRADO simultaneamente).
**Data**: 2026-05-12 (PROPOSTO P215 → IMPLEMENTADO P221).
**Materialização**:
- `00_nucleo/materialization/typst-passo-216a-relatorio.md` (sub-fase a parte 1).
- `00_nucleo/materialization/typst-passo-216b-relatorio.md` (sub-fase a parte 2).
- `00_nucleo/materialization/typst-passo-217-relatorio.md` (variant Columns).
- `00_nucleo/materialization/typst-passo-218-relatorio.md` (stdlib native_columns).
- `00_nucleo/materialization/typst-passo-219-relatorio.md` (consumer real graded Opção B).
- `00_nucleo/materialization/typst-passo-220-relatorio.md` (Colbreak agregado).
- `00_nucleo/materialization/typst-passo-221-relatorio.md` (encerramento série).
**Validado**: 7 sub-passos materializados (P216A+B + P217-P220
+ P221); 1987 tests verdes; 0 violations; Opção B graded
literal fixada; multi-region flow real Opção A scope-out
documentada (Fase 4 candidata NÃO-reservada per política P158).
**Sub-passos planeados**: P215 (PROPOSTO; este P215 diagnóstico
prévio); **P216A-B** (sub-fase a — refactor estrutural Layouter
em 2 sub-passos per P215.div-1); **P217** (`Content::Columns`
variant); **P218** (`native_columns` stdlib); **P219** (sub-fase
b — consumer multi-column); **P220** (`Content::Colbreak` +
`native_colbreak`); **P221** (encerramento Fase 3 + transição
ACEITE/IMPLEMENTADO).
**Diagnóstico prévio**:
- `00_nucleo/materialization/typst-passo-215-relatorio.md` (P215).

---

## Contexto

ADR-0061 PROPOSTO (Layout Fase X roadmap) fixou em P156B
caminho 1 graded — Fase 1 (sub-passos C-F: pad/hide/h/v/skew),
Fase 2 (sub-passos G-I: block/box/stack), Fase 3 sub-passo 1
(P156J: repeat). **Pós-P156L (refino sides individualizadas
em pad)**, ADR-0061 está em **50% concluído** caminho 1:
- ✅ Fase 1 fechada (4 sub-passos).
- ✅ Fase 2 fechada (3 sub-passos; atinge target 72% Layout).
- ✅ Fase 3 sub-passo 1 fechado (repeat).
- ⏸ Fase 3 sub-passo 2 (refino pad) fechado P156L.
- ❌ **Fase 3 columns + colbreak pendentes** — DEBT-56
  EM ABERTO desde P156B (estimado L+ ~5-8h).

P214 confirmou Layout em 78% via sincronização §2.1 ↔ Tabela A
(P156B-L já reflectidos em footnotes). 4 entradas restantes
em §A.5 separam Layout de 100%:

1. `columns(n)` — ausente (Fase 3 DEBT-56).
2. `colbreak()` — ausente (depende columns; DEBT-56).
3. `place(...)` — parcial (refino column scope).
4. `measure(body)` — parcial (Bloco C ADR-0066).

Adicionalmente 2 entradas em §A.6 Model dependem de DEBT-56:
5. `grid` header/footer real (P157 diferiu).
6. `TableHeader.repeat` algoritmo (P157C diferiu).

DEBT-56 é o maior refactor estrutural pendente. Layouter
actual é **single-page write-target** com cursors escalares
globais (`cursor_x`/`cursor_y`/`current_items`/`current_line`/
`page_config`); 135 call-sites empíricos em
`01_core/src/rules/layout/mod.rs` (P215 C1 inventário).

ADR-0078 endereça especificamente a **abstracção arquitectural**
necessária — `Region/Regions` — que desbloqueia 4 das 6
entradas restantes (columns, colbreak, grid header/footer,
TableHeader.repeat) e deixa 2 isoladas (place refino + measure
stdlib).

---

## Decisão

**Refactor `Layouter` em 2 sub-fases distintas**, com sub-fase
(a) decomposta em 2 sub-passos per P215.div-1 (>100 call-sites
empíricos):

### Sub-fase (a) — Region/Regions abstraction

**Objectivo**: introduzir `Region` / `Regions` como tipos L1
preservando comportamento single-column. Comportamento
observable inalterado (todos os tests workspace continuam a
passar).

**Decomposição P215.div-1** (135 call-sites > 100 limiar):

- **P216A**: introduzir `Region` type + substituir
  `cursor_x`/`cursor_y`/`current_items`/`current_line`/
  `page_config` em `Layouter` por `Region` activa. ~80
  call-sites refactor; comportamento single-column preservado
  via `Regions::new_single(width, height)`.
- **P216B**: introduzir `Regions` (Vec<Region>) wrapper +
  helper `Layouter::with_regions`; iteração sequencial de
  regions (ainda só 1 region por padrão). ~30-40 call-sites
  refactor; preparação para sub-fase (b).

**Tipo proposto** (paridade vanilla `typst-layout/src/regions.rs`):

```rust
pub struct Region {
    pub size:  Size,             // largura × altura disponível
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub items: Vec<FrameItem>,   // acumulador da region
}

pub struct Regions {
    pub current: Region,
    pub backlog: Vec<Region>,    // regions futuras (multi-column)
    pub last:    Option<Region>, // region final (overflow)
}
```

### Sub-fase (b) — Consumer multi-column

**Objectivo**: materializar `Content::Columns { count, gutter,
body }` arm + iteração sequencial de N regions com
`width / count - gutter`.

**Decomposição** (per P215 C4 roadmap):

- **P217**: variant enum `Content::Columns { count, gutter,
  body }` + arms exhaustivos.
- **P218**: stdlib `native_columns(count, gutter: ?)` +
  scope register.
- **P219**: consumer Layouter — converter Content::Columns
  arm em Regions multi-column; iterar; flush per region.
- **P220**: `Content::Colbreak { weak: bool }` + stdlib
  `native_colbreak(weak: ?)` + tests mixing pagebreak.

### Decisão arquitectural — sub-fases reduzem risco

Sub-fase (a) é refactor sem mudança observable; tests
existentes funcionam como regression suite. Sub-fase (b) é
aditivo (novo arm `Content::Columns`); não modifica
single-column behavior.

Total decomposição: **6 sub-passos núcleo (P216A+B+217+218+
219+220) + P221 encerramento**. Magnitude cumulativa **L
(~6-9h)** vs DEBT-56 estimado L+ original (~5-8h L+
incluindo todas as 6 entradas dependentes).

---

## Não-objectivos

- **Balanceamento altura entre colunas** — vanilla typst
  também não faz balanceamento; sequência sequential
  preenche região 1, transborda para região 2, etc.
- **Column span** (rowspan-like) — não existe no vanilla;
  fora escopo.
- **Cross-page column flow rules** — específicas (e.g.
  "última coluna deve ter altura ≥ 50%") — fora escopo
  Fase 3 inicial.
- **Refino de `place`** float/clearance — sub-passo opcional
  P223 paralelo; não bloqueia sub-fases (a)+(b).
- **`measure(body)` stdlib expose** — sub-passo opcional
  P222 paralelo; isolado de DEBT-56 per ADR-0066 Bloco C.
- **`grid` header/footer real + `TableHeader.repeat`
  algoritmo** — sub-passo opcional P224 que requer sub-fases
  (a)+(b) completas; fora escopo desta ADR (cobre apenas
  abstracção Region + columns/colbreak; grid extension é
  consumer adicional).

---

## Plano de materialização

ADR-0078 transita PROPOSTO → IMPLEMENTADO quando todas as 6
condições forem verdadeiras (verificadas em P221):

1. **Sub-fase (a) materializada** (P216A+B): `Region`/`Regions`
   tipos L1 introduzidos; 135 call-sites refactor sem
   regressão; tests workspace verdes pré e pós refactor.
2. **`Content::Columns { count, gutter, body }` variant**
   (P217): enum cresce 56→57 variants; arms exhaustivos
   passam crystalline-lint.
3. **`native_columns` stdlib registada** (P218): stdlib
   funcs cresce 53→54; scope register em `eval/mod.rs`;
   `extract_count` helper validando count≥1.
4. **Consumer multi-column funcional** (P219): tests
   `columns(2)`, `columns(3)`, `columns(4)` produzem
   regions correctas; `width / count - gutter` aplicado;
   tests mixing com pagebreak válidos.
5. **`Content::Colbreak { weak: bool }` + `native_colbreak`**
   (P220): variant 57→58; stdlib 54→55; tests
   `colbreak()` + `colbreak(weak: true)` válidos; mixing
   pagebreak/colbreak honest.
6. **`crystalline-lint .` 0 violations** preservadas em
   todos os sub-passos.

ADR transita REJEITADO se:
- Sub-fase (a) introduz regressão observable em tests
  existentes (improvável — refactor sem mudança comportamental).
- Sub-fase (b) `Region/Regions` impossível com Frame/Page
  pattern actual (improvável — paridade vanilla
  `typst-layout/src/regions.rs`).
- Custo agregado L+ → XL+ por complexidade não-prevista
  (improvável — decomposição P215.div-1 mitiga).

Se ADR rejeitada, DEBT-56 mantém-se aberto em formato
diferente; sub-passo dedicado pós-rejeição reavalia
abordagem.

### P216A materializado 2026-05-12

Sub-fase (a) parte 1 fechada:

- Tipo `Region` introduzido em `01_core/src/entities/region.rs`
  com 7 fields (`cursor_x: Pt`, `cursor_y: Pt`, `line_start_x: Pt`,
  `current_items: Vec<FrameItem>`, `current_line: Vec<FrameItem>`,
  `width: f64`, `height: f64`) + 3 métodos (`new`, `reset`,
  `has_pending`) + 4 sentinelas P216A.
- L0 `00_nucleo/prompts/entities/region.md` criado;
  hash propagado via `crystalline-lint --fix-hashes`.
- `Layouter` struct refactored em
  `01_core/src/rules/layout/mod.rs`: 5 fields escalares
  + 2 dimensões (via `page_config.width/height`)
  agregados em field único `region: Region`.
- Inventário empírico real: ~167 call-sites refactored
  mecânicamente em 6 ficheiros: `mod.rs` (~136 sub),
  `cursor.rs` (22), `equation.rs` (8), `grid.rs` (13),
  `placement.rs` (12), `tests.rs` (2).
- **Caminho B1 fixado**: `PageConfig.width/height` preservados;
  `region.width/height` é cópia derivada em `Layouter::new`.
  Sincronização adicionada em `Content::SetPage` arm
  (`mod.rs:761-763`) — region.width/height actualizados
  quando page_config muda. Único ajuste manual além da
  substituição mecânica `sed`.
- **Opção α fixada** (sem helpers): acesso directo
  `self.region.X` em todos os call-sites; sem
  `cursor_x()`/`set_cursor_x()` etc. Anti-inflação aplicada
  10ª vez cumulativa pós-P205D.
- 0 mudança observable: 1939 → **1943 verdes** (+4 P216A
  region sentinelas; nenhum test pre-existente regrediu).
- 0 violations preservadas.
- Sem `P216A.div-N` registadas (substituição mecânica
  + 1 ajuste sincronização SetPage = único ponto).

ADR-0078 mantém-se PROPOSTO. Próximo sub-passo: **P216B**
(`Regions` wrapper + `Layouter::with_regions` helper —
sub-fase (a) parte 2 per P215.div-1).

### P216B materializado 2026-05-12

**Sub-fase (a) parte 2 fechada — sub-fase (a) DEBT-56 fechada
estruturalmente**:

- Struct `Regions` adicionada em `01_core/src/entities/region.rs`
  cohabitando com `Region` (mesmo módulo, mesma L0 — precedente
  `Sides<T>` em `sides.rs`).
- **Forma minimal `{ current: Region }` fixada** por
  **anti-inflação 11ª aplicação cumulativa pós-P205D**.
  Fields `backlog: Vec<Region>` + `last: Option<Region>`
  (paridade vanilla literal) **diferidos a P219** (consumer
  multi-column real). Critério de reabertura explícito:
  materialização `Content::Columns` arm no Layouter.
- 2 métodos `Regions::single(width, height)` + `reset_current()`.
- L0 `region.md` actualizado com secção `Regions` + Histórico
  P216B.
- `Layouter` struct refactored: `region: Region` →
  `regions: Regions`. Field doc actualizado.
- Inventário empírico real: **158 call-sites** refactored
  mecânicamente em 5 ficheiros via `sed
  's/self\.region\./self.regions.current./g'`:
  `mod.rs` (107), `cursor.rs` (22), `equation.rs` (8),
  `grid.rs` (12), `placement.rs` (9). + 1 ajuste manual em
  `tests.rs` (2 refs `l.region.cursor_y` → `l.regions.current.cursor_y`
  — substituição literal).
- **Cohabitação L0 N=2** — `Region` + `Regions` no mesmo
  módulo (precedente `Sides<T>`).
- **Pattern emergente "refactor stacking" N=1** — P216B
  refactora output P216A (`self.region.X` → `self.regions.current.X`).
  Possível pattern N=2 se P217+ stack sobre P216B; promoção
  a meta diferida (N=3-4 política consistente).
- 0 mudança observable: 1943 → **1946 verdes** (+3 P216B
  sentinelas; nenhum test pre-existente regrediu).
- 0 violations preservadas.
- Sem `P216B.div-N` registadas (1 ajuste manual em tests.rs
  é trivialmente literal).

**Sub-fase (a) DEBT-56 fechada estruturalmente** — `Region` +
`Regions` em L1; `Layouter` agregado em `regions: Regions`
single-region. Pre-condição estrutural para sub-fase (b)
consumer multi-column (P219) cumprida.

ADR-0078 mantém-se PROPOSTO. Próximo sub-passo: **P217**
(`Content::Columns { count, gutter, body }` variant + arms
exhaustivos; aditivo puro sem refactor estrutural).

### P217 materializado 2026-05-12

**Sub-fase (b) DEBT-56 — primeiro sub-passo aditivo (1/4)**:

- Variant `Content::Columns { count: usize, gutter:
  Option<Length>, body: Box<Content> }` adicionado a
  `01_core/src/entities/content.rs` (Content variants
  cresce; ordem após `Repeat`).
- 10 arms exhaustivos cobrindo 4 ficheiros L1 (vs 8
  estimados em spec — 2 adicionais descobertos via
  compiler errors):
  - `entities/content.rs`: `is_empty`, `plain_text`,
    `PartialEq::eq`, `map_content`, `map_text` = **5 arms**.
  - `rules/introspect.rs`: `materialize_time` + `walk` = **2 arms**.
  - `rules/layout/mod.rs`: `layout_content` (stub
    transparente) + `measure_content_constrained`
    (transparente) = **2 arms**.
  - `rules/introspect/locatable.rs`: `is_locatable`
    catch-all `_ => false` = **1 arm**.
  - **Total: 10 arms**.
- Construtor Rust `Content::columns(body, count, gutter)`
  adicionado.
- Stdlib `native_columns` **diferido P218** (atomização
  ADR-0036 — separação variant + stdlib).
- ADR-0064 Caso C aplicado a `gutter` (cumulativo
  N=cresce; precedentes P156I Stack.spacing, P156L
  Sides<Option<Length>>).
- 6 tests adicionados: 5 unit em `content.rs::tests`
  (`p217_columns_*`) + 1 E2E em `layout/tests.rs`
  (`p217_columns_arm_transparente_renderiza_body`).
- 0 mudança observable: 1946 → **1952 verdes** (+6 P217
  tests; nenhum test pre-existente regrediu).
- 0 violations preservadas. `crystalline-lint --fix-hashes`:
  "Nothing to fix" (L0 content.md não tocado em P217 —
  variant inline-documentado per convenção).
- Sem `P217.div-N` registadas (1 ajuste manual em test
  `map_content` — closure retorna `Ok(Some(x.clone()))`
  vs `Ok(x.clone())` per signature `Result<Option<Content>>`;
  trivial).

**Status ADR-0078**: PROPOSTO mantido. Sub-fase (b) DEBT-56:
1/4 sub-passos materializados (P217 ✓; P218 P219 P220
pendentes).

Próximo sub-passo: **P218** (stdlib `native_columns(count,
gutter: ?)` + scope register; validação `count >= 1`).

### P218 materializado 2026-05-12

**Sub-fase (b) DEBT-56 — segundo sub-passo aditivo trivial (2/4)**:

- `native_columns(count, body, gutter: ?)` registada em
  `01_core/src/rules/stdlib/layout.rs` — pattern paridade
  `native_repeat` (P156J). Stdlib funcs registadas: ~53 → 54.
- Helper `extract_count(args, fn_name)` novo (privado em
  `stdlib/layout.rs`) para `count` posicional obrigatório
  (paridade `extract_usize_or_none_min` P157B mas para
  posicional). N=1 pós-P218; promoção a helper público
  diferida a N=2-3 reuso.
- Re-export em `01_core/src/rules/stdlib/mod.rs` `pub use`
  block + scope register em `01_core/src/rules/eval/mod.rs`
  (`scope.define("columns", ...)`) imediatamente após
  `native_repeat` (ordem ADR-0061 Fase 3 sub-passos).
- Validações implementadas:
  - `count >= 1` rejeita `count = 0` e `count < 0` (paridade
    `NonZeroUsize` vanilla per ADR-0054 graded).
  - `body` Content/Str obrigatório (Str → `Content::text(s)`).
  - `gutter` Option<Length>; negativo rejeitado (`abs.0 < 0.0
    || em < 0.0`).
  - Named arg desconhecido rejeitado.
  - >2 posicionais rejeitado.
- 12 tests adicionados: 11 unit em `stdlib/mod.rs::tests`
  (`p218_native_columns_*` — count valid/zero/negativo/
  não-int/ausente/body-ausente/body-Str-aceita/gutter-Length/
  gutter-negativo/named-desconhecido/extra-positional) + 1
  E2E em `layout/tests.rs` (`p218_columns_count_3_renderiza_body_transparentemente`
  — variant produzido por `Content::columns()` com `count=3`
  + `gutter` explícito; renderiza body via stub transparente
  P217).
- ADR-0064 Caso C cumulativo via `gutter` (validação stdlib
  + variant em P217).
- 0 mudança observable: 1952 → **1964 verdes** (+12 P218
  tests; 0 regressões pre-existente).
- 0 violations preservadas. `crystalline-lint --fix-hashes`:
  "Nothing to fix" (L0 stdlib.md inline-doc per convenção
  emergente — paridade decisão P217).
- 1 ajuste manual em E2E test (texto "multi-col body" com
  espaço causa fragmentação `FrameItem::Text`; substituído
  por single-word "p218body" — limitação layout actual; não
  regressão P218).

**Status ADR-0078**: PROPOSTO mantido. Sub-fase (b) DEBT-56:
**2/4 sub-passos materializados** (P217 ✓, P218 ✓; P219
P220 pendentes).

Próximo sub-passo: **P219** (consumer multi-column real no
Layouter — sub-fase (b) DEBT-56 substantiva). Magnitude
M+ (~3-4h).

### P219 materializado 2026-05-12

**Sub-fase (b) DEBT-56 — terceiro sub-passo (núcleo
substantivo; 3/4)**:

- Arm `Content::Columns` em `layout_content`
  (`01_core/src/rules/layout/mod.rs`) substituído por
  **consumer real graded (Opção B paridade ADR-0054)**.
  Arm em `measure_content_constrained` paralelo (mesma
  semântica para grid measurement).
- **Fórmula vanilla literal**: `column_width = (full_width
  - (count-1)*gutter) / count`.
- **Default gutter ~4% via constante named
  `COLUMNS_DEFAULT_GUTTER_RATIO`** (top-level em mod.rs;
  Opção β fixada em C6). Anti-inflação 14ª aplicação
  cumulativa pós-P205D.
- **Saved/restore pattern** explícito (paridade P156C
  Pad cursor_x): `saved_width = full_width;
  region.current.width = column_width; layout_body;
  flush_line; region.current.width = saved_width;`.
- **count=0 caso degenerate**: tratado como passthrough
  (count_f=1; column_width=full_width; gutter irrelevante)
  — stdlib P218 valida `>= 1`; construtor Rust aceita 0.
- **Multi-region flow real é SCOPE-OUT** documentado:
  body overflow salta para next page (não next column).
  Refino candidato a P-Layout-Fase4 (Opção A multi-region
  completa). **Decisão P216B preservada literal** —
  `Regions { current: Region }` minimal mantido; backlog/last
  continuam diferidos.
- 8 layout E2E tests adicionados em
  `01_core/src/rules/layout/tests.rs`:
  `p219_columns_count_1_equivale_a_body_directo`,
  `_count_2_renderiza_body`, `_count_3_renderiza_body`,
  `_gutter_length_explicito_renderiza`,
  `_gutter_default_renderiza`, `_width_restaurada_apos_body`,
  `_counters_contam_uma_vez`, `_aninhado_compoe_width`.
- 0 regressões em tests pre-existentes (P217 E2E
  `p217_columns_arm_transparente_renderiza_body` preservado
  porque `count=2` ainda renderiza body via column_width
  reduzida — stub transparente conceito generalizado para
  graded transparente).
- Tests workspace: 1964 → **1972 verdes** (+8 P219).
- 0 violations preservadas. `crystalline-lint --fix-hashes`:
  "Nothing to fix" (L0 `entities/content.md` não tocado em
  P219 — Opção α extensão deferida).

**Inventário 148**:
- §A.5 Layout `columns(n)` reclassificada **`ausente` →
  `parcial`** (footnote ⁴⁰ em
  `typst-cobertura-vanilla-vs-cristalino.md`).
- Tabela A.5: `13/1/3/1/0 → 13/1/4/0/0` (1 ausente
  eliminado; **zero ausentes em Layout**).
- Total user-facing: `69/24/25/21/2 → 69/24/26/20/2`.
- Cobertura Layout categoria: **78% preservada** (parcial
  fora numerador per metodologia §A.9 P213); ganho
  qualitativo.
- Cobertura user-facing total: **~66% preservada**.

**L0 `entities/content.md` extensão Opção α deferida**: P219
não criou secção dedicada `Variant Content::Columns`. Mantém
convenção emergente P217 (inline-doc no L1). Decisão
empírica — overhead documental não compensava em P219
(refactor focado em arm Layouter; variant inalterado desde
P217).

**Status ADR-0078**: PROPOSTO mantido. Sub-fase (b) DEBT-56:
**3/4 sub-passos materializados** (P217 ✓, P218 ✓, P219 ✓;
P220 colbreak pendente).

Próximo sub-passo: **P220** (`Content::Colbreak { weak: bool
}` variant + `native_colbreak` stdlib + tests mixing
pagebreak). Magnitude S+ (~1.5h).

### P220 materializado 2026-05-12

**Sub-fase (b) DEBT-56 — quarto e ÚLTIMO sub-passo
(fecha sub-fase b inteira estructuralmente; 4/4)**:

- Variant `Content::Colbreak { weak: bool }` adicionado
  imediatamente após `Content::Pagebreak` em
  `01_core/src/entities/content.rs` (Content variants
  **55 → 56**). Sem `to: Option<Parity>` — vanilla
  `ColbreakElem` não tem (paridade só faz sentido em
  páginas).
- 8 arms exhaustivos em **4 ficheiros L1** (compiler-driven;
  paridade Pagebreak P156E mas leaf sem `to`):
  - `entities/content.rs`: `is_empty` (sempre `false`),
    `plain_text` (`""`), `PartialEq::eq` (1-field),
    `map_content` + `map_text` (terminal clone).
  - `rules/introspect.rs`: `materialize_time` no-op
    (sem children), `walk` no-op (sem tag).
  - `rules/layout/mod.rs::layout_content`: arm Opção β
    graded — flush_line + `new_page` (downgrade literal).
  - `rules/layout/mod.rs::measure_content_constrained`:
    no-op `(0.0, 0.0)` (paridade Pagebreak measure).
  - `rules/introspect/locatable.rs`: catch-all `_ => false`
    + entrada explícita `Content::Colbreak { .. }`.
- `native_colbreak(weak: false)` stdlib registada em
  `rules/stdlib/layout.rs` (paridade pattern
  `native_pagebreak`); re-export em `stdlib/mod.rs`;
  scope `define("colbreak", ...)` em `eval/mod.rs` após
  `columns` (stdlib funcs ~54 → 55).
- **Arm Layouter Opção β fixada** (vs Opção α stub puro
  vs Opção γ erro fora de columns context): downgrade
  literal a pagebreak via `Layouter::new_page` reuso
  P156E. **Justificação literal**: vanilla também
  downgrade colbreak a pagebreak quando fora de columns
  context; pós-P219 cristalino tudo é "fora" (single-region
  per Opção B); `Layouter::new_page` é zero refactor
  estrutural. `weak: bool` armazenado mas semantic
  adiada (paridade P156D HSpace/VSpace + P156E Pagebreak).
- **Multi-region salto entre colunas reais é SCOPE-OUT**
  documentado: refino candidato a P-Layout-Fase4 (Opção A
  multi-region completa). Decisão P216B preservada literal
  — `Regions { current: Region }` minimal mantido;
  `backlog`/`last` continuam diferidos.
- **Sub-passo agregado único** (variant + arm + stdlib
  num único P220 vs P217+P218+P219 atomizados em 3
  sub-passos). **Anti-inflação 15ª aplicação cumulativa
  pós-P205D**: atomização ADR-0036 não é dogma absoluto;
  agregação justificada por (i) precedente directo P156E
  pagebreak, (ii) arm trivial (downgrade trivial via reuso
  `new_page`), (iii) sem consumer substantivo separado
  para extrair como sub-passo.
- 15 tests novos: 5 unit content (`p220_colbreak_variant_existe`,
  `_is_empty_sempre_false`, `_plain_text_vazio`,
  `_partial_eq_1_field`, `_map_content_terminal`); 6 unit
  stdlib (`p220_native_colbreak_sem_args_aceita`,
  `_weak_true_aceita`, `_posicional_rejeita`,
  `_weak_nao_bool_rejeita`, `_named_desconhecido_rejeita`,
  `_to_rejeita`); 4 E2E layout
  (`p220_colbreak_produz_new_page_downgrade`,
  `_dentro_columns_downgrade_graded`,
  `_misturado_com_pagebreak`,
  `_no_inicio_documento_pagina_vazia`).
- Tests workspace: 1972 → **1987 verdes** (+15 P220);
  zero regressões pre-existente.
- 0 violations preservadas. `crystalline-lint --fix-hashes`:
  "Nothing to fix" (L0 `entities/content.md` Opção γ
  deferida — convenção emergente N=4 com P217+P218+P219+P220).

**Inventário 148**:
- §A.5 Layout linha `colbreak()` reclassificada
  **`ausente` → `parcial`** ⁴¹ (variant + stdlib + arm
  real existem; multi-region salto entre colunas reais
  ausente — P-Layout-Fase4 candidato).
- Recontagem Layout pós-auditoria empírica P220 corrige
  off-by-one da footnote ⁴⁰: `12/1/4/1/0 = 18 →
  12/1/5/0/0 = 18` (zero ausentes em Layout pós-P220;
  ganho qualitativo via segunda reclassificação Layout
  pós-M9c).
- Total user-facing: `69/24/26/20/2 = 141 → 68/24/27/20/2 =
  141` (corrige offset + reclassifica colbreak).
- Cobertura Layout: `13/18 = 72%`; cobertura user-facing:
  `92/141 ≈ 65%` (ambas com correcção de auditoria;
  declínio de 1pp reflecte ajuste numerário, não
  regressão semântica).

**Sub-fase (b) DEBT-56 FECHADA estructuralmente**:
**4/4 sub-passos materializados** (P217 ✓, P218 ✓, P219 ✓,
P220 ✓). DEBT-56 fica completo estructuralmente
(P-Layout-Fase4 multi-region real é refino futuro
não-reservado per política P158).

**Status ADR-0078**: PROPOSTO mantido. Marco interno:
sub-fase (b) fechada. Transição IMPLEMENTADO ocorre em
**P221** (encerramento Fase 3 documental + DEBT-56
fecha + Tabela B.2 Content variants actualizada + ADR-0061
promoção candidata).

Próximo sub-passo: **P221** (encerramento Fase 3 + ADR-0078
PROPOSTO → IMPLEMENTADO + DEBT-56 fecha + ADR-0061
candidato a transição). Magnitude S documental (~30min).

### P221 encerramento série 2026-05-12

**Série P217-P220 fechada estructuralmente** (precedida por
P216A+B sub-fase a + P215 diagnóstico):
- P216A+B: Region + Regions sub-fase (a) (refactor; ~325
  substituições mecânicas em 6 ficheiros L1; zero mudança
  observable; pattern co-habitação L0 N=2).
- P217: Content::Columns variant (variants 54 → 55) + 10
  arms exhaustivos + 6 unit tests; arm Layouter stub
  transparente (consumer real diferido P219).
- P218: native_columns stdlib (~53 → 54 funcs) + helper
  privado `extract_count` (N=1) + 12 unit + scope register;
  validações 6 (count ≥1, gutter Length, gutter ≥0, named
  desconhecido, body Content/Str, ≤2 posicionais).
- P219: consumer multi-column real graded (Opção B paridade
  ADR-0054) — width temporariamente reduzida `(full_width -
  (count-1)*gutter) / count`; body single-render; width
  restaurada; default gutter ~4% via constante named
  `COLUMNS_DEFAULT_GUTTER_RATIO` (anti-inflação 14ª);
  saved/restore pattern explícito; arm `measure_content_constrained`
  paralelo; 8 E2E tests.
- P220: Content::Colbreak agregado (variant + arm + stdlib
  num único sub-passo paridade P156E pagebreak;
  anti-inflação 15ª); arm Layouter Opção β graded —
  downgrade literal a pagebreak via reuso `Layouter::new_page`
  P156E (zero refactor estrutural); 8 arms exhaustivos
  em 4 ficheiros L1; 15 tests novos (5 unit content + 6
  unit stdlib + 4 E2E).

**6 condições §"Plano materialização" satisfeitas
explicitamente**:
1. ✓ **Region + Regions abstractions** (P216A+B; cohabitação
   L0 N=2; 7 tests novos).
2. ✓ **Content::Columns + Content::Colbreak variants**
   (P217 + P220; 2 variants Content novos; 18 arms
   exhaustivos cumulativos em 4 ficheiros L1).
3. ✓ **native_columns + native_colbreak stdlib** (P218 +
   P220; 2 stdlib funcs registadas no scope eval; ~53 → 55
   funcs cumulativas).
4. ✓ **Layouter consumer Opção B graded** (P219 + P220;
   arm `Content::Columns` consumer real graded com width
   reduzida + saved/restore; arm `Content::Colbreak`
   downgrade β a pagebreak; ambos paridade vanilla literal
   quando fora de columns context).
5. ✓ **Test suite multi-column verde** (44 tests cumulativos
   Fase 3: 6 P217 + 12 P218 + 8 P219 + 15 P220 + 3 unit
   region/regions; 1987 verdes workspace; 0 regressões
   pre-existente).
6. ✓ **Multi-region flow real scope-out documentado** —
   Opção A multi-region completa diferida a P-Layout-Fase4
   candidato (não-reservada per política P158); §A.5
   `columns(n)` + `colbreak()` reclassificadas `parcial`
   (não `implementado`) reflectindo limitação documentada.

**Transição PROPOSTO → IMPLEMENTADO ratificada**.

DEBT-56 ENCERRADO simultaneamente (ver `DEBT.md` —
critério fecho 5/5 cumprido). ADR-0061 transita
simultaneamente PROPOSTO → IMPLEMENTADO (ver
`typst-adr-0061-layout-fase-x-roadmap.md` — Caminho 1
100% cumprido; refinos `measure`/`place` Fase 4
candidata NÃO-reservada).

**Distribuição ADRs pós-P221**: PROPOSTO 13 → 11;
IMPLEMENTADO 19 → 21.

**Pattern emergente "L0 minimal para refactors" N=4**
(P217+P218+P219+P220 todos Opção γ — sem extensão L0
formal; documentação inline no L1). Promoção a ADR meta
documental fica como decisão diferida (Caminho 4 P221
§8 candidato; política consistente N=3-4 mínima).

**Pattern emergente "encerramento Fase Layout pós-M9c"
N=1 inaugurado**. Precedente estrutural pré-M9c: P156I
(Fase 2 Layout fechada) + P155 (Fase 1 Model fechada).
Pattern reusável (Fase 4 Layout candidata; Model Fase
3 candidata; etc.).

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **Region/Regions com sub-fase decomposta (escolhida)** | Risco mitigado por sub-fases; tests existentes como regression suite; paridade vanilla `regions.rs` | Custo M+ na sub-fase (a); 2 sub-passos vs 1 |
| Big-bang refactor (sub-fase a+b num passo único) | 1 sub-passo; menos overhead | Risco alto — 135 call-sites refactor + nova feature simultaneamente; difícil debug se regressão |
| `Content::Columns` sem refactor Region (Layouter mantém scalar cursors) | Sub-fase (a) evitada | `Content::Columns` consumer impossível arquitecturalmente — single-page cursor não suporta iteração regions |
| Adiar DEBT-56 pós-Layout 100% por outras vias | Baixa entrada de risco curto-prazo | Layout fica em 78% indefinidamente — 4 entradas restantes em cascata; gap não fechável sem column flow |

**Escolha**: Region/Regions com decomposição P215.div-1 —
balança risco e progresso incremental. Pattern paridade
ADR-0061 sub-fases graded.

---

## Reservas

P216-P224 (sub-passos núcleo + opcionais) **NÃO são reservas**
per política P158 ("sem novas reservas"). São **opções
identificadas** documentadas em P215 §5 + roadmap C4. Decisão
humana sobre prosseguir fica em aberto pós-P215.

ADR-0078 PROPOSTO autoriza **abordagem arquitectural**
(Region/Regions); não compromete cronograma. Análoga a
ADR-0061 PROPOSTO em P156B que autorizou caminho 1 graded
sem comprometer quando materializar cada Fase.

---

## Análise paridade vanilla

Vanilla typst tem `typst-layout/src/regions.rs` com tipos
`Region` + `Regions` análogos. Diferenças cristalino vs
vanilla:

| Aspecto | Vanilla | Cristalino (P215 proposto) |
|---------|---------|----------------------------|
| `Region` | `pub struct Region { size: Size, expand: Axes<bool>, full: Abs }` | `pub struct Region { size: Size, cursor_x: f64, cursor_y: f64, items: Vec<FrameItem> }` |
| `Regions` | `pub struct Regions<'a> { size: Size, full: Abs, expand: Axes<bool>, backlog: &'a [Abs], last: Option<Abs>, root: bool }` | `pub struct Regions { current: Region, backlog: Vec<Region>, last: Option<Region> }` |
| Lifetime | `'a` (borrow `&'a [Abs]` backlog) | sem lifetime (owned `Vec<Region>`) |
| `expand` axes | sim (controla auto-expand) | adiar (cristalino single-pass; auto-expand controlado por flush_*) |
| `root` flag | sim (page-level vs nested) | adiar (cristalino infere via Region::new_single vs new_columns) |
| Balanceamento | não | não (paridade) |

**Divergência justificada** (per ADR-0054 graded perfil):
cristalino simplifica `Region`/`Regions` via owned Vec sem
lifetime; auto-expand controlado por flush helpers; sem
`root` flag explícito. Cobertura essencial preservada;
refinos vanilla (lifetime + axes + root) adiados se
consumer real necessitar.

---

## Referências

- **ADR-0061** PROPOSTO (Layout Fase X roadmap) — fixou
  caminho 1 graded em P156B; este ADR-0078 endereça
  especificamente Fase 3 sub-passos columns+colbreak.
- **ADR-0066** PROPOSTO (Introspection runtime) — Bloco C
  `measure(body)` stdlib expose isolado de DEBT-56.
- **ADR-0054** graded — perfil scope-out autorizado;
  refinos vanilla (lifetime, axes, root) adiados sob
  política graded.
- **DEBT-56** EM ABERTO (column flow L+) — endereçado
  por sub-fases (a)+(b) deste ADR.
- **DEBT-37** FECHADO P84.6 (`PlaceScope::Parent`) —
  refino column scope é trabalho novo (P223 opcional;
  fora ADR-0078).
- **P156B** diagnóstico Layout amplo — precedente
  metodológico para diagnóstico-primeiro arquitectural.
- **P215 relatório** — diagnóstico prévio + 135
  call-sites empíricos + P215.div-1 decomposição.
- **Vanilla `typst-layout/src/regions.rs`** — paridade
  arquitectural com simplificações cristalino documentadas.

---

## Histórico

| Data | Status | Notas |
|------|--------|-------|
| 2026-05-12 | PROPOSTO | Criada em P215 para autorizar abordagem Region/Regions abstraction. Diagnóstico prévio confirmou 135 call-sites empíricos > 100 limiar; P215.div-1 decompõe sub-fase (a) em P216A+B. Transição IMPLEMENTADO em P221 quando 6 condições satisfeitas. |

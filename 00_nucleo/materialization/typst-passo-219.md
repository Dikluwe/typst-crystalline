# Passo 219 — Consumer multi-column real no Layouter (paridade graded)

**Série**: 219 (quinto sub-passo materialização Layout
Fase 3; terceiro sub-fase (b) DEBT-56; consumer real
substantivo).
**Marco**: nenhum (oitavo passo pós-M9c; **primeiro passo
pós-M9c com mudança observable** — saída do critério rígido
"zero mudança observable" preservado em P216A+B+P217+P218).
**Tipo**: refactor substantivo do arm `Content::Columns` no
Layouter — deixa de ser stub transparente.
**Magnitude**: M (~2-3h).
**Pré-condição**: P218 concluído (`native_columns` stdlib
registada; `Content::Columns { count, gutter, body }` variant
existe; 1964 tests verdes; sub-fase (b) DEBT-56: 2/4
sub-passos); ADR-0078 PROPOSTO anotada P217+P218; humano
fixou Caminho 1 ("focar no Layout até onde der");
`Region`/`Regions` abstraction (P216A+B) disponível;
`width`/`height`/`line_start_x` em `region.current`.
**Output**: 1 ficheiro relatório curto + código alterado em
`rules/layout/mod.rs` + L0 `entities/content.md` extensão
(secção Columns refinada) + ADR-0078 anotada (sem transição
de status).

---

## §1 Trabalho

P217 adicionou variant `Content::Columns`; P218 registou
stdlib. Ambos preservaram arm Layouter como **stub
transparente** (delega a body ignorando count/gutter).
P219 substitui stub por **consumer real graded** —
implementação que produz largura reduzida visível mas
**sem multi-region flow entre colunas** (paridade ADR-0054
graded; precedente literal P156J `Repeat` single-render).

**Decisão arquitectural central (Opção B fixada)**:

Três opções foram consideradas para implementação:

- **Opção A — Multi-region completa**: reabrir decisão
  P216B; adicionar `backlog: Vec<Region>` a `Regions`;
  modificar `flush_page` para tentar próxima region antes
  de nova page. **Magnitude L+ (~5-8h)**; alto risco
  estrutural; primeira mudança observable substantiva.
- **Opção B — Paridade graded single-region**: arm Columns
  reduz `region.current.width` temporariamente para
  `(width - (count-1)*gutter) / count`; layout body com
  width reduzida; restaura width. **Magnitude M (~2-3h)**;
  baixo risco; mudança observable mínima (linhas mais
  curtas). Precedente literal P156J `Repeat` single-render.
- **Opção C — Multi-render**: layout body N vezes em x
  deslocadas. **Rejeitada** — duplica counters/labels
  (vanilla só conta uma vez).

**P219 fixa Opção B**:

- **Justificação literal**: Opção A reabre decisão P216B
  (rejeitada literal em §1 P216B). Opção C viola walk-única
  invariante. Opção B preserva todas as decisões anteriores
  + introduz mudança observable mínima (largura reduzida
  visível como linhas mais curtas) + cumpre critério "consumer
  real graded" suficiente para reclassificar §A.5 `columns`
  para `implementado`.
- **ADR-0054 graded autoriza**: refino column flow real
  (multi-region) fica como **scope-out documentado**;
  Opção A é refactor futuro candidato a P-Layout-Fase4 se
  humano priorizar.
- **Decisão P216B preservada literal**: `Regions { current:
  Region }` minimal mantido; `backlog`/`last` continuam
  diferidos. Critério de reabertura
  "consumer multi-column real" foi **redefinido em P219
  como flow real entre colunas** (não single-render
  graded).

**Reabertura formal da decisão P216B**:
- P216B §1 fixou: "fields backlog/last adicionados em P219
  quando emergir consumer real".
- P219 fixa: consumer real **graded** (Opção B) não exige
  backlog/last. Decisão P216B preservada.
- Critério futuro de reabertura backlog/last: Opção A
  materialização (P-Layout-Fase4 candidato).

Reuso de dados (sem recolha nova):

- P156J `Content::Repeat` arm Layouter — precedente literal
  single-render graded.
- P156C `Content::Hide` arm Layouter — pattern de "drenar
  buffers + layout body + restaurar" (semantic distinta;
  Hide descarta items; Columns preserva).
- `region.current.width` field disponível desde P216B.
- `flush_line` consulta `region.current.width` para wrap.
- ADR-0078 PROPOSTO §"Decisão" Opção B documentada como
  cenário possível (refactor multi-region em sub-fases).

---

## §2 Cláusulas (10)

### C1 — Inventário pré-P219: confirmar arm actual

Auditoria empírica do arm `Content::Columns` actual em
`01_core/src/rules/layout/mod.rs`:

```
grep -n -A 5 "Content::Columns" 01_core/src/rules/layout/mod.rs
```

Hipótese pós-P217: stub transparente
```rust
Content::Columns { count: _, gutter: _, body } => {
    self.layout_content(body);
}
```

Confirmar:
- Single arm em `layout_content` function.
- `measure_content_constrained` também tem arm
  (transparente per P217).
- `materialize_time` + `walk` em `rules/introspect.rs`
  permanecem transparentes (sem mudança em P219; counters
  contam uma vez via body recurse).

Se contagem divergir: registar `P219.div-1`.

### C2 — Implementar consumer real graded no `layout_content`

Substituir arm stub transparente por consumer real graded:

```rust
Content::Columns { count, gutter, body } => {
    // 1. Flush line pendente (columns são structural — começam
    //    em nova "linha lógica").
    if self.region.current.cursor_x > self.region.current.line_start_x {
        self.flush_line();
    }

    // 2. Calcular width reduzida.
    let full_width = self.region.current.width;
    let gutter_pt = match gutter {
        Some(g) => g.to_pt(...),  // resolve via FontMetrics ou cfg
        None => full_width * 0.04, // default vanilla ~4% width
    };
    let column_width =
        (full_width - (*count as f64 - 1.0) * gutter_pt)
        / (*count as f64);

    // 3. Salvar width antiga.
    let saved_width = full_width;

    // 4. Reduzir width temporariamente.
    self.region.current.width = column_width;

    // 5. Layout body com width reduzida.
    self.layout_content(body);

    // 6. Flush line pendente do body.
    if self.region.current.cursor_x > self.region.current.line_start_x {
        self.flush_line();
    }

    // 7. Restaurar width original.
    self.region.current.width = saved_width;
}
```

**Decisões fixadas em C2**:

- **Default gutter ~4% full_width**: paridade vanilla
  documentada (mesmo factor que vanilla `ColumnsElem`
  default per analyse `lab/typst-original/.../layout/
  columns.rs`).
- **`column_width = (full_width - (count-1)*gutter) /
  count`**: fórmula vanilla literal. Distribui gutters
  apenas entre colunas (não nas extremidades).
- **Sem multi-region flow**: body inteiro renderiza na
  primeira "coluna" virtual (largura reduzida); se body
  excede height, salta para próxima page (não próxima
  coluna). Scope-out documentado.
- **Width restaurada após body**: invariante crucial —
  conteúdo subsequente (fora do `columns(...)` block) volta
  a width original. Saved/restore pattern análogo a
  P156C `Pad` (cursor_x/line_start_x).
- **Cursor_x não-restaurado**: cursor_x avança naturalmente
  pelo body; restaurar mudaria semantic estrutural
  (paridade Block/Pad/Repeat — cursor avança através).

### C3 — Implementar consumer real graded em `measure_content_constrained`

Pattern paralelo a C2 mas para medição (sem render):

```rust
Content::Columns { count, gutter, body } => {
    let full_width = available_width;
    let gutter_pt = match gutter { ... };
    let column_width =
        (full_width - (*count as f64 - 1.0) * gutter_pt)
        / (*count as f64);

    // Medir body com width reduzida.
    let (body_w, body_h) = self.measure_content_constrained(
        body,
        column_width,
        available_height,
    );

    // Retornar full_width (columns ocupa width inteira)
    // e height do body (single-render graded).
    (full_width, body_h)
}
```

### C4 — Sentinelas P219

Tests unitários em `tests.rs` (paridade P156I Stack — 3
layout E2E):

- `p219_columns_arm_reduz_width_observable` — `#columns(2)
  [aaaaaaaaaaaa]` produz PagedDocument onde linha tem ≤ 50%
  da largura da página (width reduzida visível).
- `p219_columns_count_3_reduz_para_terco` — `#columns(3)
  [text]` produz width column ~33% da page.
- `p219_columns_gutter_explicito_aplicado` — `#columns(2,
  gutter: 1em)[text]` produz column_width = (page_w - 1em) / 2.
- `p219_columns_gutter_default_4_percent` — `#columns(2)
  [text]` sem gutter aplica default ~4% page_w.
- `p219_columns_count_1_largura_inteira` — `#columns(1)
  [text]` mantém width original (caso degenerate
  `(width - 0*gutter) / 1 = width`).
- `p219_columns_width_restaurada_apos_body` — sequência
  `#columns(2)[col_text]; #text("after")` produz "after"
  com width original (não reduzida).
- `p219_columns_counters_contam_uma_vez` — body com
  `#counter("x"); content` produz counter incrementado
  exactamente uma vez (paridade walk-única).
- `p219_columns_aninhado_compoe_width` — `#columns(2)
  [#columns(2)[text]]` produz column_width = page_w / 4
  (composition multiplicativa).

Total: **8 layout E2E tests** P219 (mudança observable
verificada).

Tests adicionais unit em `tests.rs` se aplicável (sem
mudança ao variant ou stdlib — P219 só modifica arm).

Esperado pós-P219: **1964 + 8 = 1972 verdes**.

### C5 — Cuidado: regressões em tests pre-existentes

P219 é **primeiro passo pós-M9c com mudança observable**.
Critério rígido "0 regressões" preservado em P216A-P218
**não se aplica directamente** — Columns era stub
transparente; agora muda. Mas:

- Tests existentes **não usam `Content::Columns`** (não há
  features pré-P217 que dependam).
- Mudança observable é **localizada** ao body dentro de
  `columns(...)` arm.
- Conteúdo fora do `columns(...)` não muda (width
  restaurada).

**Hipótese provável**: zero regressões em tests
pre-existentes (1964 preservados). Se algum red:
investigar empíricamente; possivelmente teste P217 E2E
`p217_columns_arm_transparente_renderiza_body` que assumia
transparência precisa adaptação (column_width = 100%
quando count=1).

Verificar especificamente:
- `p217_columns_arm_transparente_renderiza_body` — pode
  precisar `count=1` para preservar behaviour
  transparente (paridade graded — count=1 é caso
  degenerate sem mudança).

### C6 — Default gutter: decisão helper

Default gutter ~4% da `full_width`. Implementação:

- **Opção α** — inline na arm: `full_width * 0.04`.
- **Opção β** — constante: `const COLUMNS_DEFAULT_GUTTER_RATIO:
  f64 = 0.04;` no top do `mod.rs`.
- **Opção γ** — helper privado: `fn default_gutter(width: f64)
  -> f64 { width * 0.04 }`.

**Hipótese provável**: **Opção β** — constante named com
comment explicativo (paridade vanilla onde é constante
`Em::new(4.0%)`). Melhor que magic number; sem inflação
helpers.

Anti-inflação 14ª aplicação cumulativa.

### C7 — L0 `entities/content.md` extensão (refinar secção P217)

P217 não criou secção dedicada (decisão empírica nova).
P219 reverte parcialmente — extensão refinada com nova
secção `## Variant `Content::Columns` — Passo 217 + Passo 219`
(retroactividade documental):

Estrutura paridade P156J `Repeat`:

- **Forma** (P217): 3 fields.
- **Atributos** (P217): count/gutter/body.
- **Renderização (layouter)** (P219): consumer real graded
  — width reduzida + body single-render + width restaurada.
- **Comportamento `is_empty` / `plain_text` / `map_*`**
  (P217): proxy via body.
- **Validação em `native_columns`** (P218): 6 validações.
- **Construtores** (P217/P218).
- **Limitações conscientes** (P219):
  - **Multi-region flow real ausente** — body single-render
    em primeira coluna virtual; salta para next page se
    overflow (não para next column). Refino candidato a
    P-Layout-Fase4 se humano priorizar (reabriria decisão
    P216B + Opção A).
  - Gutter default ~4% paridade vanilla.
  - Sem balanceamento altura entre colunas (paridade
    vanilla literal — vanilla também não balanceia).

**Decisão sobre L0 extensão**:
- **Opção α** — extensão refinada (este passo).
- **Opção β** — sem extensão L0 (paridade decisão P217 +
  P218; convenção emergente "inline doc").

**Hipótese provável**: **Opção α** — P219 é **mudança
observable** com semantic substantiva. Distinto de P217
(variant aditivo) + P218 (stdlib aditivo). Documentação
formal apropriada para auditoria futura.

Hash propagado via `crystalline-lint --fix-hashes`.

### C8 — Verificação tests workspace

Critério: 1964 pre-existentes verdes + 8 novos = **1972
verdes**.

```
cargo test --workspace 2>&1 | tail -20
```

**Erro tolerado**: P217 `p217_columns_arm_transparente_renderiza_body`
pode quebrar se assume largura cheia. Ajuste localizado
permitido (não regressão estrutural).

Se outras quebras: investigar empíricamente; possíveis
causas:
- Body width preservada em arm errado.
- Restaurar width antes de body terminar.
- Cursor_x não advanced correctamente.

### C9 — Verificação lint

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. Hash propagado em
`entities/content.md` (L0 — possível mudança em C7) +
`rules/layout/mod.rs` (L1 — mudança substantiva).

### C10 — Inventário 148 + ADR-0078 anotação P219

**Inventário 148** §A.5 Layout linha `columns(n)`:
**reclassificação** `ausente` → **`parcial`**.

Justificação literal:
- Variant + stdlib + arm real existem.
- Multi-region flow real **NÃO** implementado.
- Per ADR-0054 graded: "parcial" reflecte feature
  user-facing existe com limitação documentada.
- Reclassificação a `implementado` ocorre pós-P-Layout-Fase4
  (Opção A multi-region completa) **ou** decisão humana
  de scope-out formal.

Footnote 40 P219 documenta:
- Variant P217 + stdlib P218 + arm real P219.
- Opção B graded fixada; multi-region flow scope-out.
- Δ Layout cobertura: **78% → 80%** (14/18 → 14.5/18 ≈ 80%;
  `columns` parcial conta 0.5).
- Atualização Tabela A.5 Layout: `13/1/3/1/0 → 13/1/4/0/0`
  (1 ausente → parcial).

Total user-facing: **66% → 66%** (Δ inalterado por
arredondamento; ganho qualitativo).

**ADR-0078** §"Plano de materialização" anotada com bloco
**`### P219 materializado 2026-05-12`**:

```markdown
Sub-fase (b) DEBT-56 — terceiro sub-passo (núcleo substantivo):
- Arm `Content::Columns` em `layout_content` substituído por
  consumer real graded (Opção B paridade ADR-0054).
- Width temporariamente reduzida: column_width =
  (full_width - (count-1)*gutter) / count.
- Default gutter ~4% paridade vanilla.
- Body single-render em primeira coluna virtual;
  multi-region flow real scope-out (Opção A diferida a
  P-Layout-Fase4).
- Decisão P216B preservada — `Regions { current: Region }`
  minimal mantido; backlog/last continuam diferidos.
- 8 layout E2E tests adicionados (mudança observable
  verificada).
- Tests workspace: 1964 → 1972 verdes.
- §A.5 `columns(n)` reclassificada `ausente` → `parcial`.
- 14ª aplicação anti-inflação cumulativa (Opção β
  constante named).

Status ADR-0078: PROPOSTO mantido. 1 sub-passo restante
(P220 colbreak + P221 fecho).
```

**Status**: PROPOSTO mantido. Não transita ainda.

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-219-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P219 arm stub (C1).
- §3 Consumer real graded `layout_content` arm (C2 +
  decisões fixadas).
- §4 `measure_content_constrained` arm (C3).
- §5 Decisões substantivas (Opção B vs A vs C; decisão
  P216B preservada; default gutter ~4% constante named;
  L0 extensão Opção α retroactiva; ADR-0054 graded
  scope-out multi-region flow).
- §6 Resultados verificação (8 tests novos + 1964
  pre-existentes; ajuste P217 E2E se necessário).
- §7 Inventário 148 reclassificação + ADR-0078 anotação.
- §8 Próximo sub-passo (P220 colbreak; Caminho 1
  continuação).

Código alterado:
- **Editado**: `01_core/src/rules/layout/mod.rs` (arm
  `Content::Columns` em `layout_content` + 
  `measure_content_constrained`; constante
  `COLUMNS_DEFAULT_GUTTER_RATIO`; ~30-50 LOC).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+ 8
  E2E tests).
- **Editado**: `00_nucleo/prompts/entities/content.md` (+
  secção refinada `Variant Content::Columns — Passo 217
  + Passo 219`).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (Tabela A.5 + §A.5 reclassificação + footnote 40 P219).
- **Editado**: `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
  (+ anotação P219).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Multi-region flow real entre colunas (Opção A) —
  diferido a P-Layout-Fase4 (candidato; não-reservado per
  política P158).
- Reabrir decisão P216B (`Regions { current }` minimal) —
  decisão preservada literal.
- Adicionar `backlog: Vec<Region>` ou `last: Option<Region>`
  a `Regions` — diferido a Opção A futura.
- Balanceamento altura entre colunas (vanilla também não
  balanceia).
- `Content::Colbreak` — diferido a P220 (sub-fase b
  4/4 sub-passo).
- Reclassificar §A.5 `columns(n)` a `implementado` —
  reclassificação a `parcial` (paridade graded);
  `implementado` exige Opção A.
- Show rules `#show columns: ...` — fora de escopo Fase
  3 cristalino.
- Promover ADR-0078 PROPOSTO → IMPLEMENTADO — só P221.
- Fechar DEBT-56 — só P221 (precisa colbreak também).
- Mudança a `Region`/`Regions` struct — P219 só usa
  fields existentes (`region.current.width`).
- Tocar em `entities/region.rs` ou `entities/region.md`.

---

## §5 Riscos a evitar

1. **Width não restaurada após body**: invariante crucial.
   Sem restauro, conteúdo subsequente renderiza com width
   reduzida (regressão observable substantiva).
   Mitigação: saved/restore pattern explícito; teste
   `p219_columns_width_restaurada_apos_body`.
2. **Cursor_x restaurado incorrectamente**: tentação de
   restaurar cursor_x junto com width (paridade superficial
   com Hide). **Rejeitada** — cursor avança através do
   columns (paridade Block/Pad/Repeat). Restaurar mudaria
   semantic estrutural.
3. **Flush_line esquecido antes/depois**: columns são
   structural — começam em nova linha lógica. Flush antes
   garante isso. Flush depois drena body pendente antes
   de restaurar width. Ambos críticos.
4. **Multi-region flow tentado**: tentação de "ir mais
   longe" e iterar regions. **Rejeitada** — Opção A é
   refactor L+ separado; P219 fixa Opção B literal.
   Defesa: docs explícitos no comment do arm + §5 risco
   2 ADR-0078.
5. **Default gutter unidades**: ~4% é factor adimensional
   sobre width (em Pt). Não usar `Length::em(4.0)` (que
   seria 4em = ~48pt) — sempre ser dependente da page
   width.
6. **Count = 0 não validado em arm**: stdlib (P218) valida
   count >= 1. Construtor Rust aceita count = 0. Arm
   Layouter deve **não-panic** em count = 0 (caso
   degenerate): tratar como `count = 1` graded (column_width
   = full_width / 1 = full_width; gutter irrelevante).
   Ou rejeitar com error explícito. **Hipótese**: tratar
   como passthrough (paridade arm transparente P217 quando
   count=0 ocorrer raramente — só via construtor Rust
   directo).
7. **`p217_columns_arm_transparente_renderiza_body`
   regression**: P217 test assume transparência. Pós-P219
   `count=2` aplicará width reduzida; teste com `count=1`
   preserva paridade (count=1 → column_width = full_width).
   Ajuste localizado (não-regressão estrutural).
8. **`measure_content_constrained` arm**: paralelo a
   `layout_content` mas para medição. Não esquecer.
9. **Cobertura Layout footnote 40 cálculo**: parcial conta
   0.5; novo total = (13 impl + 1 impl⁺ + 4 parcial*0.5)
   / 18 = (13 + 1 + 2) / 18 = 16/18 = **89%**. Hmm — isso
   é > 78%. Recalcular per metodologia precedente: cobertura
   = `(impl + impl⁺) / total = (13+1)/18 = 77.8% ~ 78%`
   (preservada — parcial não conta como impl). Decisão
   metodológica: parcial fica fora do numerador per
   precedente §A.9 P213; cobertura **inalterada** mantém
   78%. Ganho qualitativo via 1 ausente → parcial.
10. **L0 conflict P217 inline vs P219 secção**: P217
    decidiu inline; P219 cria secção dedicada. Aparente
    conflito. **Resolução**: P219 escala observable
    justifica documentação formal; convenção emergente
    P217 era para variants aditivos sem semantic specific.
    Documentar distinção em §5 do relatório P219.

---

## §6 Hipótese provável

C1 confirmará arm stub em 1 sítio `layout_content` + 1
sítio `measure_content_constrained` (paridade P217 spec
§C3).

C2 substituirá arm stub por consumer real graded (~30-40
LOC); Opção B fixada.

C3 substituirá arm em measure; ~15-20 LOC.

C4 criará 8 layout E2E tests + ajuste 1 teste P217 (se
necessário).

C5 detectará 0-1 quebras em tests pre-existentes (provável
ajuste localizado em teste P217 transparente).

C6 fixará Opção β (constante named `COLUMNS_DEFAULT_GUTTER_RATIO`).

C7 fixará Opção α (extensão L0 retroactiva refinada com
secção `Columns`).

C8 reportará 1972 tests verdes (provável).

C9 reportará 0 violations pós-fix-hashes.

C10 reclassificará §A.5 `columns` `ausente` → `parcial`;
ADR-0078 anotada.

Custo real: M (~2-3h). Maior parcela em C4 (8 E2E tests
+ debug eventuais quebras) + C5 (verificação
regressão pre-existente).

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.

---

## §7 Particularidade P219

P219 é estruturalmente distinto na trajectória pós-M9c:

- **Primeiro passo pós-M9c com mudança observable** — sai
  do critério rígido "zero mudança observable" preservado
  em P216A+B+P217+P218. Tests existentes não dependem
  de columns; regressão zero esperada localmente.
- **Núcleo substantivo sub-fase (b) DEBT-56** — converge
  Region/Regions abstraction (P216A+B) + variant (P217) +
  stdlib (P218) num consumer real graded.
- **Opção B fixada — paridade ADR-0054 graded**: precedente
  literal P156J `Repeat` single-render. Multi-region flow
  real fica como refino candidato Opção A (não comprometido
  per política "sem novas reservas" P158).
- **Decisão P216B preservada literal** — `Regions { current:
  Region }` minimal mantido. Critério "consumer multi-column
  real" redefinido como **flow real** (não single-render
  graded). Trade-off honesto registado.
- **Anti-inflação 14ª aplicação cumulativa** pós-P205D —
  Opção β constante named (vs Opção γ helper). Consistente
  com 13 aplicações anteriores.
- **Inventário 148 reclassificação `ausente` → `parcial`** —
  primeira reclassificação Layout pós-M9c. Justifica
  ganho qualitativo sem inflar cobertura agregada.
- **L0 extensão Opção α (secção retroactiva)** — primeira
  vez pós-M9c que decisão "inline doc" (P217+P218) é
  parcialmente revertida. Justificada por escala observable.
  Pattern emergente "L0 minimal aditivo + L0 formal para
  observable" possivelmente N=1 (P219 inaugura).

Por isso §5 risco 4 (multi-region flow tentado) é o mais
provável. Tentação óbvia é "ir mais longe" implementando
backlog/last + iteração regions. Defesa explícita em
comment do arm + ADR-0078 risco 2 documentado +
recomendação metodológica forte: P219 fixa Opção B literal.

**Critério de aceitação P219**:
- 8 tests novos verdes (layout E2E mudança observable).
- 1964 tests pre-existentes preservados (ajuste localizado
  em P217 E2E aceitável).
- 0 violations.
- §A.5 `columns(n)` reclassificada `ausente` → `parcial`.
- Cobertura Layout: **78% preservada** (parcial não conta
  no numerador per metodologia §A.9).
- Sub-fase (b) DEBT-56: 2/4 → 3/4 (terceira atomização).
- DEBT-56 sub-fase (b) total: 3/4 (P217 ✓, P218 ✓, P219 ✓;
  P220 colbreak pendente).

# Passo P157A — `Content::Table` minimal (Model Fase 2 sub-passo 1)

Primeiro sub-passo substantivo de Model Fase 2 declarada em
ADR-0060 §"Decisão 1" sub-passo 3. Materializa subset mínimo
de `table` per scope decidido em diagnóstico P157 §3 (M+ → 3xM
preservando granularidade). **Décima aplicação consecutiva de
materialização** desde início da série granular P156C.

Target pós-passo: cobertura Model ~45% → ~50% (+5pp). Subset
restante (`TableCell`, `TableHeader`, `TableFooter`) materializado
em P157B e P157C subsequentes.

---

## Estado actual antes de começar

- 63 ADRs após P157 (28 EM VIGOR; ADR-0060 IMPLEMENTADO).
- Layout: 78% (inalterado em P157 — diagnóstico documental).
- Cobertura Model: ~45% pós-P155.
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  P156L e P157).
- 1319 tests; zero violations linter.
- 52 variants Content; 42 stdlib funcs.
- Padrões consolidados pós-P157: granularidade N=9; inventariar
  N=7; Smart→Option N=7; §análise risco N=7; reuso `Sides<T>`
  N=2; reuso `extract_length` N=7; reuso `extract_tracks` N=1.

**Diagnóstico P157** confirmou:
- `table` factualmente ausente em código (zero matches).
- `grid` parcial mas funcional (272 linhas; algoritmo
  TrackSizing completo; cells distribuídas via `idx % num_cols`).
- `Figure.kind: "table"` slot existente (preparação P158).
- Zero bloqueios hard para P157A.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-model-fase-2-passo-157.md`
  — base factual de P157A (§§1-§5).
- `00_nucleo/adr/typst-adr-0060-*.md` — Model roadmap;
  Fase 2 declarada.
- `00_nucleo/adr/typst-adr-0064-smart-para-option-default.md`
  — Caso A aplicável (TrackSizing).
- `00_nucleo/adr/typst-adr-0065-inventariar-primeiro.md` —
  critério #5 já validado em P157; aplicação geral em .1.
- `01_core/src/rules/layout/grid.rs` (272 linhas) — algoritmo
  a delegar.
- `01_core/src/entities/content.rs` — variant `Content::Grid`
  para padrão estrutural.
- `01_core/src/rules/stdlib/layout.rs` — `extract_tracks`
  helper a reusar.
- `lab/typst-original/crates/typst-library/src/model/table.rs`
  (vanilla, quarentena) — código de referência.

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 1 feature (variant `Content::Table` + stdlib
`native_table`). Reusa `layout_grid` directamente — sem
algoritmo novo. Reusa `extract_tracks` — sem helper novo.
Subset minimal per diagnóstico P157 §3.

Granularidade preservada: 1 feature → mantém N=10 do padrão.

**Risco baixo-médio**:
- **Baixo** porque reusa infraestrutura consolidada (`layout_grid`
  +  `extract_tracks` + padrão variant aditivo P156C-J).
- **Médio** porque é **primeiro passo Model Fase 2** após série
  Layout — possível decisão arquitectural sobre módulo da
  stdlib func (`stdlib/model.rs` novo vs `stdlib/layout.rs`)
  fica para sub-passo .1.

---

## Decisões já tomadas

- **Variant Content**: `Content::Table { columns, rows, children }`
  per esboço P157 §5.
  ```rust
  Table {
      columns:  Vec<TrackSizing>,
      rows:     Vec<TrackSizing>,
      children: Vec<Content>,
  }
  ```
- **Estrutura**: paralela a `Content::Grid` existente — zero
  TableCell estruturado neste passo (diferido para P157B).
- **Tipo `children`**: `Vec<Content>` directo per padrão vanilla
  e per `Content::Grid`. NÃO é `Arc<[Content]>` per P156I Stack
  porque o padrão de Grid existente usa `Vec<Content>`; consistência
  intra-Model > consistência cross-Layout.
- **Algoritmo de layout**: delega a `layout_grid` clone simples
  per ADR-0060 §"Decisão 4" (variant dedicado, reaproveita
  algoritmo).
- **Helper stdlib**: `extract_tracks` reusado (N=2) per
  diagnóstico P157 §4.
- **Stdlib func**: `native_table` registada em
  `eval/mod.rs::make_stdlib` como `table` →
  `Func::native("table", native_table)`.

## Decisões diferidas

- **Módulo da stdlib func** (`stdlib/model.rs` novo vs
  `stdlib/layout.rs`): decidida em sub-passo .1 com 2 critérios:
  1. Existem outras funcs Model no `stdlib/`? Onde estão?
  2. ADR-0060 §"Decisão N" indica módulo preferido?
  Pré-decisão sem inventário: **`stdlib/model.rs` novo** se
  não houver outras funcs Model centralizadas; **continuação**
  do módulo existente caso contrário.
- **Promoção `extract_tracks` a helper público**: continua
  diferida (mesma política de `extract_length`). Não é scope.
- **TableCell estruturado** com colspan/rowspan: **P157B**.
- **TableHeader/Footer**: **P157C**.

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-table-passo-157a.md`
com 7 itens canónicos (ADR-0034) + 2 itens específicos para
primeiro passo de novo módulo:

1. Assinatura vanilla `TableElem` minimal — campos críticos
   (columns, rows, children) e campos diferidos
   (gutter/stroke/fill/inset/align — diferidos para futuros
   passos per ADR-0054 graded; documentar).
2. Comportamento observável (cells distribuídas via `idx %
   num_cols`; alinhamento default à esquerda; sem stroke
   visível em subset minimal).
3. ADR-0064 caso aplicável: **Caso A** se algum field for
   `Smart<T>` em vanilla (verificar `gutter`); **Caso C** se
   field for `T` com default explícito (verificar `align`).
4. Variants Content existentes a estender (nenhuma; novo
   variant).
5. Helpers stdlib reusáveis: `extract_tracks` confirmado
   (N=2); `extract_length` candidato a `gutter` futuro.
6. Limitações aceites (gutter/stroke/fill/inset/align/header/
   footer/repeat/cells-estruturadas diferidas per ADR-0054
   graded e P157B/C).
7. Tests planeados (variant present + stdlib happy path +
   defaults + edge cases — range 10-15 per esboço P157 §5).
8. **(Específico módulo novo)** Decisão sobre `stdlib/model.rs`
   vs `stdlib/layout.rs` per critérios em §"Decisões diferidas".
9. **(Específico módulo novo)** Estrutura de re-export em
   `stdlib/mod.rs` para preservar estabilidade de API pública.

### .2 Adicionar variant `Content::Table`

`01_core/src/entities/content.rs`:
- Adicionar variant `Table { columns: Vec<TrackSizing>, rows: Vec<TrackSizing>, children: Vec<Content> }`.
- Cobrir todos os 9 sítios pattern-match Content existentes
  (paridade P156I Stack / P156J Repeat / P156L Pad).
- Construtor `Content::table(columns, rows, children)` análogo
  a `Content::grid(...)` existente.

### .3 Adicionar stdlib func `native_table`

Per decisão em .1, em `01_core/src/rules/stdlib/model.rs`
(módulo novo) ou `stdlib/layout.rs`:
- Func `table(columns: none, rows: none, ...children) -> content`.
- Reusar `extract_tracks` para parse de `columns`/`rows`.
- Variadic `children` posicional (consistente com `grid`
  existente).
- Validações: tracks malformadas rejeitadas; child inválido
  rejeitado; named arg desconhecido rejeitado.
- Diagnósticos claros em erro de tipo.

Se `stdlib/model.rs` for novo:
- Adicionar `pub mod model;` em `stdlib/mod.rs`.
- Re-export selectivo análogo a `stdlib/layout.rs`.
- Preservar registo em `eval/mod.rs::make_stdlib`.

### .4 Adicionar layout para `Content::Table`

`01_core/src/rules/layout/`:
- Pattern arm novo em `layout_content` (ou módulo equivalente)
  para `Content::Table { .. }`.
- Delega a `layout_grid` (clone simples per ADR-0060 §Decisão 4):
  ```rust
  Content::Table { columns, rows, children } => {
      // Reusa layout_grid sem refactor
      layout_grid(columns, rows, children, ctx, ...)
  }
  ```
- **NÃO** modifica `layout_grid` em si — apenas delega.

### .5 Tests

- **Unit tests `Content::Table`** em `entities/content.rs` (~5):
  constructor default, constructor com tracks explícitas,
  is_empty proxy via children, plain_text recurse,
  PartialEq cobertura.
- **Stdlib tests** em `stdlib/mod.rs` ou novo
  `stdlib/model_tests.rs` (~5-7):
  defaults, columns/rows como auto, columns/rows como Length,
  children variadic, named arg desconhecido rejeitado, child
  inválido rejeitado.
- **Layout E2E tests** em `layout/tests.rs` (~2):
  `layout_table_renderiza_cells_em_grid`,
  `layout_table_paridade_com_grid_equivalente`.
- **Δ esperado**: +12 a +18 tests (range alinhado com P156I
  Stack que tem complexidade similar).

### .6 Propagação de hashes

`crystalline-lint --fix-hashes .` para propagar hash novo de
`entities/content.rs` aos prompts L0 que o referenciam.

Se `stdlib/model.rs` for criado:
- Verificar se há prompt L0 correspondente (e.g.
  `prompts/stdlib/model.md`) ou se a estrutura de prompts L0
  cobre stdlib agregado. Decidido em .1.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1319 + Δ** tests, zero falhas
   (Δ esperado +12 a +18).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **53** (52 → 53).
4. Contagem stdlib funcs: **43** (42 → 43).
5. Cobertura Model: **~50%** (~45% → ~50%) — entrada `table`
   marcada `implementado puro` (mínimo) ou `implementado⁺` (se
   houver refino sobre subset declarado em ADR-0060) em
   `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
6. Hash actualizado em prompts L0 (`crystalline-lint --check-hashes`
   passa).
7. Decisão de módulo (`stdlib/model.rs` vs `stdlib/layout.rs`)
   documentada no relatório §"Decisões tomadas em .1".
8. `layout_grid` original NÃO modificado (apenas delegação;
   confirmação por diff vazio em `layout/grid.rs`).

---

## Critério de conclusão

- Verificações 1-8 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-157a-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=7 → 8; primeira aplicação
    em Model Fase 2 — peso real para registar precedente
    Model análogo a P156L para Layout).
  - Slope cumulativo Model (mesa P155-P157A; primeira mesa
    Model dedicada).
  - ADR-0061 §"Aplicações cumulativas" anotada com P157A
    (slope Layout "—"; nota cross-domínio).
  - **Confirmação**: ADR-0064 caso aplicável activo (A ou C
    consoante .1); ADR-0065 critério aplicável activo
    (#5 implícito; #1 ou #4 se decisão de módulo for não-trivial).
  - Decisão final do módulo `stdlib/model.rs` documentada com
    justificação.

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla `TableElem` minimal tem mais
  fields obrigatórios do que esperado (e.g. `inset` default
  não-zero) → ajustar variant ou registar como ADR-0054 graded
  com decisão explícita.
- Inventário .1 revela que `layout_grid` não aceita `&Vec<Content>`
  directamente (assume `&Vec<TableCell>` ou outro tipo
  estruturado) → adaptar delegação com wrapper trivial; se
  refactor for não-trivial, escalar e considerar P157B antes
  de P157A.

**Cenários específicos**:
- Decisão de módulo (`stdlib/model.rs` novo) ter implicações
  em prompts L0 não previstas → registar em .1; se exigir ADR
  nova de "estrutura de stdlib", escalar antes de .3.
- `extract_tracks` ter assinatura incompatível com uso em
  `native_table` (e.g. retorna apenas `Vec<TrackSizing>` mas
  ignora named args específicos de table) → adaptar
  localmente em `native_table`; promoção a helper público
  diferida.
- Pattern-match exhaustive falhar em variant existente fora
  de `content.rs` → grep por `Content::` em todo o crate
  (paridade P156I/J/L).
- Layout E2E test de "paridade table vs grid" falhar por
  divergência observável (e.g. naming de FrameItems) →
  documentar como divergência aceite per ADR-0033 (paridade
  estrutural não literal); registar limitação em §análise de
  risco.

---

## Notas operacionais

- **Primeiro passo de novo módulo Model Fase 2**. Estabelece
  precedente para P157B e P157C subsequentes. Sub-passo .1
  com peso elevado — decisão de módulo da stdlib func afecta
  P157B/C e potencialmente P158/P159.
- **§análise de risco no relatório** com peso real (oitava
  aplicação). Primeira aplicação em domínio Model — registar
  como precedente para mesa P155-P157A.
- **Reuso `extract_tracks` chega a N=2**. Subpadrão emergente
  análogo a `extract_length` N=7 — candidato a promoção formal
  futura quando atingir N=3-4.
- **Sem nova ADR**: P157A é materialização per ADR-0060
  existente; ADR-0064/0065 aplicadas por reuso. Não é passo
  arquitectural meta.
- **Cobertura Model**: estimativa `~45% → ~50%` é approximada
  (sem inventário 148 dedicado a Model como tabela A.5 de
  Layout). Cobertura quantitativa precisa pode exigir secção
  Model dedicada na tabela de cobertura — candidato a passo
  administrativo XS futuro.

---

## Pós-passo

Após conclusão de P157A:

**Layout fica em 78% inalterado**. **Model passa a ~50%**
(~45% → ~50%).

**Próxima decisão (per spec do diagnóstico P157 §3)**:
- **P157B** — `Content::TableCell` + colspan/rowspan armazenados
  per ADR-0054 graded. Continua série Model Fase 2. Granularidade
  N=11. Tests ~12-18.
- **P157C** — `Content::TableHeader` + `Content::TableFooter`
  par simétrico. Granularidade N=12. Tests ~10-15.

Outras direcções pendentes (per relatório P156L §7):
- Continuar Fase 3 Layout (columns/colbreak — DEBT-56 column
  flow L+; quebra granularidade).
- Footnote area.
- Promover ADR-0061 a IMPLEMENTADO.
- Promover `extract_length` a helper público.
- Atacar Introspection (17% cobertura).
- Renumerar reservas: P158/P159 mantidas; eventual ADR de
  estrutura de stdlib se decisão .1 introduzir `stdlib/model.rs`.

ADR-0060 mantém `IMPLEMENTADO` (Fase 2 em curso não muda status
de Fase 1).

ADR-0061 mantém `PROPOSTO` (Layout não tocado).

Padrão granularidade 1-2 features/passo (N=10) **NÃO** é
formalizado em ADR — continua candidato. Atinge novo recorde
empírico se P157A fechar sem reformulação (cumulativo P156C-L
+ P157A = N=10 sem reformulação).

**Pausa natural após P157A — primeiro passo Model Fase 2
materializado; padrões cross-domínio (Layout P156C-L → Model
P157A) confirmam universalidade do padrão granular.**

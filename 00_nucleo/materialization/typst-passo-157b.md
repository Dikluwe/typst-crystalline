# Passo P157B — `Content::TableCell` + colspan/rowspan armazenados (Model Fase 2 sub-passo 2)

Segundo sub-passo substantivo de Model Fase 2 declarada em
ADR-0060 §"Decisão 1" sub-passo 3. Materializa `TableCell`
estruturado com `x`/`y`/`colspan`/`rowspan` armazenados per
ADR-0054 graded — placement algorítmico diferido (DEBT-34e).
**Décima primeira aplicação consecutiva de materialização**
desde início da série granular P156C.

**Primeira aplicação concreta de ADR-0064 Caso A em Model**
(P156G/H/I aplicaram-no em Layout). Patamar empírico cross-
domínio do ADR meta P156K cresce.

Target pós-passo: cobertura Model ~50% → ~55% (+5pp). Subset
restante (`TableHeader`, `TableFooter`) materializado em P157C
subsequente.

---

## Estado actual antes de começar

- 63 ADRs após P157A (28 EM VIGOR; ADR-0060 IMPLEMENTADO).
- Layout: 78% (inalterado em P157/P157A — escopo Model).
- Cobertura Model: ~50% pós-P157A.
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  P156L, P157, P157A — refactors aditivos preservam contrato L0).
- 1335 tests (lib+integ+diagnostic); zero violations linter.
- 53 variants Content; 43 stdlib funcs.
- Padrões consolidados pós-P157A: granularidade N=10;
  inventariar N=8; Smart→Option N=7; §análise risco N=8;
  reuso `Sides<T>` N=2; reuso `extract_length` N=7; reuso
  `extract_tracks` N=2 (`pub(super)` desde P157A).

**Diagnóstico P157** declarou (§3 + §5):
- P157B: variant `Content::TableCell` + 4 fields (`x`, `y`,
  `colspan`, `rowspan`).
- Granularidade N=11.
- Tests ~12-18.
- ADR-0064 Caso A para `x`/`y`.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-model-fase-2-passo-157.md`
  — §3 e §5 (esboço P157B).
- `00_nucleo/materialization/typst-passo-157a-relatorio.md` —
  decisão de módulo (`stdlib/structural.rs` continuação) e
  precedente Table.
- `00_nucleo/adr/typst-adr-0064-smart-para-option-default.md`
  — Caso A (semântica "auto = computa do contexto").
- `00_nucleo/adr/typst-adr-0054-perfil-graded.md` —
  fundamento para colspan/rowspan armazenados sem placement.
- `00_nucleo/DEBT.md` — DEBT-34e (placement em Grid;
  permanece aberto após P157B; explicitação em §análise de
  risco).
- `01_core/src/entities/content.rs` — variant `Content::Table`
  (P157A) para padrão estrutural.
- `01_core/src/rules/stdlib/structural.rs` — `native_table`
  (P157A) para padrão de stdlib func.
- `lab/typst-original/crates/typst-library/src/model/table.rs`
  — `TableCell` vanilla (referência).

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 1 feature (variant `Content::TableCell` +
stdlib `native_table_cell`). 4 fields novos com 2 deles
aplicando ADR-0064 Caso A (`x`/`y`). Sem algoritmo de placement
(diferido em DEBT-34e). Reusa infraestrutura P157A.

Granularidade preservada: 1 feature → mantém N=11 do padrão.

**Risco baixo-médio**:
- **Baixo** porque é variant aditivo análogo a P156G/H/I/J e
  P157A — padrão consolidado.
- **Médio** porque é **primeira aplicação concreta de ADR-0064
  Caso A em Model** — patamar empírico cresce, mas a regra de
  tradução foi formalizada e validada em Layout. Riscos de
  ambiguidade do Caso A devem ser nulos se o inventário .1
  cobrir vanilla com cuidado.

---

## Decisões já tomadas

- **Variant Content**: `Content::TableCell` com 5 fields:
  ```rust
  TableCell {
      body:    Box<Content>,
      x:       Option<usize>,
      y:       Option<usize>,
      colspan: Option<usize>,
      rowspan: Option<usize>,
  }
  ```

- **`x`/`y`**: ADR-0064 Caso A. Vanilla usa `Smart<usize>`
  com semântica "auto = computa do contexto (auto-placement)";
  cristalino traduz para `Option<usize>`. `None` ↔ Auto.

- **`colspan`/`rowspan`**: ADR-0064 Caso C. Vanilla usa
  `NonZeroUsize` com default 1; cristalino traduz para
  `Option<usize>` para preservar distinção semântica entre
  "default" e "1 explícito" sem ambiguidade. Validação em
  stdlib func: zero rejeitado com diagnóstico claro.

- **Algoritmo de placement**: **diferido per ADR-0054 graded**.
  Fields armazenados; `layout_grid` ignora `x`/`y`/`colspan`/
  `rowspan` por agora — DEBT-34e permanece aberto. Documentado
  no relatório §análise de risco como limitação consciente.

- **`body: Box<Content>`**: paridade P156H Boxed e P156J Repeat
  (single child via Box, não Arc).

- **Stdlib func `native_table_cell`** em `stdlib/structural.rs`
  (módulo Model; continuação per decisão P157A — não criar
  novo módulo).

- **Naming `TableCell`**: directo (sem conflito com variants
  existentes; difere de Box→Boxed P156H que tinha conflito
  std::Box). Confirmar em sub-passo .1 que não há colisão
  com tipo `TableCell` em vanilla externo importado.

- **Helper stdlib novo**: `extract_usize_or_none` privado em
  `stdlib/structural.rs` para parse de `x`/`y` (Auto → None;
  Int → Some). Promoção a `pub(super)` ou helper público
  diferida até reuso (política N=3-4 consistente).

- **Validação `colspan/rowspan >= 1`**: sim, com erro hard
  (paridade vanilla `NonZeroUsize`). Zero ou negativo
  rejeitados.

## Decisões diferidas

- **Algoritmo de placement** (auto-placement quando `x`/`y =
  None`; resolução de colisões; expansão visual de spans):
  **DEBT-34e**. Fora de scope P157B. Decisão sobre quando
  fechar DEBT-34e fica para passo posterior — possivelmente
  agregado com decisão P157C ou refactor dedicado.

- **`align`/`stroke`/`fill`/`inset` per cell** (vanilla suporta
  override per cell): **diferidos** per ADR-0054 graded.
  Documentar em diagnóstico .1 como limitação consciente.
  Refino futuro fica para passo dedicado.

- **`breakable: bool` per cell** (vanilla suporta): **diferido**.
  Documentar.

- **Layout E2E test "cell em posição explícita"**: limitado
  porque algoritmo de placement não usa `x`/`y`. Tests E2E
  cobrem **armazenamento e propagação** apenas (cell criada
  com `x=Some(2)` armazena `Some(2)` no Content); não cobrem
  placement visual. Documentar.

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-table-cell-passo-157b.md`
com 7 itens canónicos (ADR-0034) + 2 itens específicos para
field semântico não-trivial:

1. Assinatura vanilla `TableCell` minimal — confirmar 4 fields
   posicionais críticos (`x`, `y`, `colspan`, `rowspan`) +
   fields diferidos (`align`, `stroke`, `fill`, `inset`,
   `breakable` — diferidos per ADR-0054 graded; documentar).
2. Comportamento observável (cell na posição explícita
   sobrepõe placement automático; spans expandem visualmente;
   cell sem position usa auto-placement linear).
3. ADR-0064 caso aplicável **confirmado**: Caso A para `x`/`y`;
   Caso C para `colspan`/`rowspan`.
4. Variants Content existentes a estender (nenhuma; novo
   variant).
5. Helpers stdlib reusáveis: nenhum directo (parse de `usize`
   é trivial); helper novo `extract_usize_or_none` para `x`/`y`.
6. Limitações aceites (placement algorítmico diferido
   DEBT-34e; align/stroke/fill/inset/breakable diferidos).
7. Tests planeados (variant present + stdlib happy path +
   defaults + edge cases ADR-0064 Caso A — range 12-18 per
   esboço P157 §5).
8. **(Específico naming)** Confirmar zero conflito de nome
   `TableCell` em namespace cristalino e vanilla externo
   importado.
9. **(Específico ADR-0064 Caso A)** Documentar a tradução
   explícita: `Smart<usize> → Option<usize>` é **Caso A**
   (não Caso C) porque vanilla `Auto` significa "computa do
   contexto" (auto-placement), não "valor literal fixo".

### .2 Adicionar variant `Content::TableCell`

`01_core/src/entities/content.rs`:
- Adicionar variant `TableCell { body, x, y, colspan, rowspan }`
  com tipos per §"Decisões já tomadas".
- Cobrir todos os 9 sítios pattern-match Content existentes
  (paridade P156I/J/L e P157A).
- Construtor `Content::table_cell(body, x, y, colspan, rowspan)`
  com convenção dos passos anteriores.

### .3 Adicionar stdlib func `native_table_cell`

`01_core/src/rules/stdlib/structural.rs` (módulo Model
continuação per P157A):
- Func `table_cell(body, x: none, y: none, colspan: none, rowspan: none) -> content`.
- Helper privado `extract_usize_or_none` para `x`/`y`/`colspan`/
  `rowspan` (Auto/none → `None`; Int não-negativo → `Some`).
  - **Para `x`/`y`**: aceita Auto, none, Int >= 0.
  - **Para `colspan`/`rowspan`**: aceita Auto, none, Int >= 1
    (zero rejeitado com diagnóstico claro).
- Validações: int negativo rejeitado; Float rejeitado; named
  arg desconhecido rejeitado; body inválido rejeitado.

Registado em `eval/mod.rs::make_stdlib` como `table.cell` →
`Func::native("table.cell", native_table_cell)` per convenção
namespacing vanilla, ou como `table_cell` flat — **decisão
deferida a sub-passo .1** (verificar precedente em vanilla e
em outras stdlib funcs cristalinas e.g. `terms.item`).

Re-exportado em `stdlib/mod.rs`.

### .4 Layout para `Content::TableCell`

`01_core/src/rules/layout/mod.rs` ou `layout/grid.rs`:

- Pattern arm novo em `layout_content` para `Content::TableCell { body, .. }`.
- **Comportamento minimal**: render `body` no contexto actual
  como single cell; ignora `x`/`y`/`colspan`/`rowspan`. Equivale
  a `body` literal na posição actual do grid layouter.
- **Preservação de fields**: `materialize_time` e `walk` em
  `introspect.rs` preservam todos os 5 fields (incluindo `x`/
  `y`/`colspan`/`rowspan` ignorados pelo layout).

**NÃO modifica** `layout_grid` em si. Algoritmo de placement
fica diferido em DEBT-34e.

### .5 Tests

- **Unit tests `Content::TableCell`** em `entities/content.rs`
  (~6):
  - Constructor default (todos None).
  - Constructor com `x` e `y` explícitos.
  - Constructor com colspan e rowspan explícitos.
  - `is_empty` proxy via body.
  - `plain_text` recurse no body sem multiplicar (paridade
    P156I/J).
  - `PartialEq` cobertura (5 vias: body, x, y, colspan,
    rowspan).
  - `map_text` recurse + preserva fields.
- **Stdlib tests** em `stdlib/mod.rs` (~7-8):
  - Defaults (todos none).
  - `x = 2`, `y = 3` (Caso A explícito; armazenado).
  - `x = auto` (= Auto; → None).
  - `colspan = 2`, `rowspan = 3` (Caso C; armazenado).
  - `colspan = 0` rejeitado (zero não permitido).
  - `colspan = -1` rejeitado (negativo não permitido).
  - Named arg desconhecido rejeitado.
  - Naming convention `table.cell` vs `table_cell` (decidido
    em .1) verificada por test.
- **Layout E2E tests** em `layout/tests.rs` (~2):
  - `layout_table_cell_renderiza_body_no_contexto_actual` —
    cell renderiza body uma vez (sem multiplicar por colspan).
  - `layout_table_cell_fields_armazenados_e_preservados` —
    confirmação que `x`/`y`/`colspan`/`rowspan` são acessíveis
    via PartialEq após materialização.

**Δ esperado**: +15 a +18 tests (range alinhado com esboço
P157 §5 e com complexidade similar a P157A com mais variantes
de campo).

### .6 Propagação de hashes

`crystalline-lint --fix-hashes .` para propagar hash novo de
`entities/content.rs` aos prompts L0 que o referenciam.

Se hash for preservado (refactor aditivo, paridade P157A),
reportar "Nothing to fix" e confirmar no relatório.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1335 + Δ** tests, zero falhas
   (Δ esperado +15 a +18).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **54** (53 → 54).
4. Contagem stdlib funcs: **44** (43 → 44).
5. Cobertura Model: **~55%** (~50% → ~55%) — entrada
   `table.cell` ou `TableCell` marcada `implementado parcial`
   (algoritmo placement diferido per ADR-0054 graded) em
   `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
6. Hash actualizado em prompts L0 (`crystalline-lint --check-hashes`
   passa) ou hash preservado (passo aditivo per P157A).
7. **DEBT-34e permanece aberto** (placement diferido); confirmar
   no relatório §"Estado pós-passo".
8. Naming convention final (`table.cell` vs `table_cell`)
   documentada no relatório §"Decisões tomadas em .1" com
   justificação.
9. ADR-0064 Caso A aplicado pela primeira vez em Model;
   ADR-0064 Caso C aplicado pela terceira vez globalmente
   (P156I/J + P157B); confirmação no relatório §5.
10. `layout_grid` original NÃO modificado (paridade verificação
    P157A #8).

---

## Critério de conclusão

- Verificações 1-10 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-157b-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=8 → 9; nona aplicação
    consecutiva — primeira aplicação concreta de ADR-0064
    Caso A em Model; documentar peso real do risco vs
    cerimonial).
  - Slope cumulativo Model (mesa P155-P157B).
  - ADR-0061 §"Aplicações cumulativas" anotada com P157B
    (slope Layout "—"; nota cross-domínio).
  - **Confirmação**: ADR-0064 Caso A primeira aplicação Model
    (auto-validação cumulativa); Caso C terceira aplicação
    global (P156I/J + P157B). ADR-0065 critério aplicável
    (#2 escolha de tipo se inventário .1 detectar decisão
    significativa).
  - **DEBT-34e**: confirmar permanência aberto e justificar
    decisão graded.

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla `TableCell` minimal tem
  fields obrigatórios não previstos (e.g. `colspan` é `Smart<NonZeroUsize>`
  em vez de `NonZeroUsize` default 1) → ajustar tradução para
  Caso A em vez de Caso C; documentar na decisão arquitectural.
- Naming convention `table.cell` (com ponto) entrar em conflito
  com sintaxe de evaluation cristalina → fallback para
  `table_cell` flat com mapping explícito; documentar em .1.

**Cenários específicos**:
- Inventário .1 revela que vanilla suporta `colspan: usize`
  literal sem `NonZeroUsize` (apenas validation runtime) →
  manter Caso C com validação cristalina; sem mudança.
- Helper `extract_usize_or_none` ter complexidade superior
  ao esperado quando recebe `Auto` (e.g. variants alargados)
  → simplificar para apenas `Option<i64>` interno e converter;
  documentar.
- Pattern-match exhaustive falhar em variant existente fora
  de `content.rs` (paridade P157A) → grep por `Content::` em
  todo o crate.
- Tests E2E E2E para "cell renderiza no contexto actual"
  divergir do comportamento vanilla por causa do placement
  diferido → tests E2E focados em **armazenamento e propagação**,
  não em placement visual. Limitar tests para o que está
  materializado per ADR-0054 graded.

---

## Notas operacionais

- **Décima primeira aplicação de materialização**. Patamar
  empírico forte. Sem reformulação esperada (mantém precedente
  N=10 sem reformulação).
- **§análise de risco no relatório** com peso real (nona
  aplicação consecutiva). Primeira aplicação Model com ADR-0064
  Caso A — documentar como precedente para futuro P157C
  (Caso D).
- **DEBT-34e permanece aberto**. P157B contribui para
  fechamento futuro armazenando os fields necessários ao
  algoritmo, mas não fecha por si. Mencionar explicitamente
  no relatório §"Estado pós-passo" que fechamento de DEBT-34e
  fica para passo posterior — possivelmente agregado com
  P157C ou refactor dedicado.
- **Reuso de `extract_tracks` mantém N=2**. P157B introduz
  helper novo `extract_usize_or_none`, não reusa `extract_tracks`.
- **`extract_usize_or_none` N=1**: novo helper. Candidato a
  `pub(super)` se P157C ou outro passo Model precisar (e.g.
  `TableHeader.repeat-rows: Smart<usize>`).
- **ADR-0064 Caso A patamar Model**: primeira aplicação
  concreta. Reforça auto-validação ADR meta P156K com
  diversidade cross-domínio (Layout em P156G/H/I + Model em
  P157B = aplicação Caso A em N=4 passos cross-domínio).

---

## Pós-passo

Após conclusão de P157B:

**Layout fica em 78% inalterado**. **Model passa a ~55%**
(~50% → ~55%; +5pp). **DEBT-34e permanece aberto**.

**Próxima decisão (per spec do diagnóstico P157 §3)**:
- **P157C** — `Content::TableHeader` + `Content::TableFooter`
  par simétrico. Fecha "table foundations" declarado em
  ADR-0060. Granularidade N=12. Tests ~10-15. Aplicação
  concreta de ADR-0064 Caso D (`repeat: bool` default true).

Outras direcções pendentes:
- Continuar Fase 3 Layout (columns/colbreak — DEBT-56;
  quebra granularidade).
- Footnote area.
- Promover ADR-0061 a IMPLEMENTADO.
- Promover `extract_length` a helper público (N=7 patamar
  forte).
- Atacar Introspection (17% cobertura).
- Fechar DEBT-34e (placement Grid completo) — refactor
  dedicado se prioritário; ou agregado com P157C ou passo
  posterior.

ADR-0060 mantém `IMPLEMENTADO`. ADR-0061 mantém `PROPOSTO`
(Layout não tocado).

Padrão granularidade 1-2 features/passo (N=11 com P157B se
fechar sem reformulação) **NÃO** é formalizado em ADR. Continua
candidato.

**Pausa natural após P157B — segundo sub-passo Model Fase 2;
ADR-0064 Caso A primeira aplicação Model; padrões cross-domínio
consolidam-se. Decisão humana sobre próxima direcção tem
máxima informação.**

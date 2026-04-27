# Relatório P157C — `Content::TableHeader` + `Content::TableFooter` (Model Fase 2 sub-passo 3 — fecha "table foundations")

Terceiro e último sub-passo substantivo de Model Fase 2.
**Décima segunda aplicação consecutiva de materialização**.
**Primeira aplicação concreta de ADR-0064 Caso D em domínio
Model** — após P157C, todos os 4 casos canónicos A/B/C/D
validados em Layout E em Model (3/4 em Model; Caso B só Layout).
**Saturação cross-domínio cross-caso ADR-0064 atingida**;
ADR meta P156K atinge maturidade empírica.

---

## 1. Resumo do executado

### 1.1 Diagnóstico (.1)

Ficheiro novo:
`00_nucleo/diagnosticos/diagnostico-table-header-footer-passo-157c.md`
(7 itens canónicos ADR-0034 + 2 itens específicos para par
simétrico).

**Decisões arquiteturais documentadas**:
- **Field `body: Box<Content>`** (não vanilla `#[variadic]
  children: Vec<TableItem>`): divergência intencional per
  ADR-0033 para uniformidade com containers cristalinos.
- **Paridade absoluta TableHeader↔TableFooter**: ambos diferem
  `level` (vanilla tem só em Header).
- **Helper parametrizado `extract_bool_with_default`**: combina
  parse de bool com default arbitrário; distinto de
  `extract_weak` (key="weak"/default=false). Separação de
  domínios per ADR-0037.
- **Naming `table_header`/`table_footer` flat** (paridade P157B).

### 1.2 Variants `Content::TableHeader` + `Content::TableFooter` (.2)

Adicionados a `01_core/src/entities/content.rs` (54 → **56**
variants; +2 par).

```rust
TableHeader { body: Box<Content>, repeat: bool }
TableFooter { body: Box<Content>, repeat: bool }
```

Construtores `Content::table_header(body, repeat)` e
`Content::table_footer(body, repeat)`.

Cobertura exaustiva de **9 sítios pattern-match estruturais
com par simétrico em entradas adjacentes** (paridade visualmente
óbvia em todos os arms).

### 1.3 Stdlib funcs simétricas (.3)

Adicionadas a `01_core/src/rules/stdlib/structural.rs`
(continuação P157A/B).

Helper privado novo:
```rust
fn extract_bool_with_default(args, fn_name, field, default: bool) -> SourceResult<bool>
```

`Value::Bool(b)` → `b`. `Value::None` ou ausência → `default`.
Outros tipos → erro hard.

`native_table_header` e `native_table_footer` com implementação
simétrica linha-a-linha (excepto naming). Validações:
- body required em ambos.
- `repeat=Int` rejeitado.
- Named arg desconhecido rejeitado (mensagem inclui menção de
  `level`/`repeat-rows` scope-out per ADR-0054 graded).

Registadas em `eval/mod.rs::make_stdlib`. Re-exportadas em
`stdlib/mod.rs`.

### 1.4 Layout para Header+Footer (.4)

2 pattern arms minimais em `layout_content`
(`01_core/src/rules/layout/mod.rs`):

```rust
Content::TableHeader { body, repeat: _ } => self.layout_content(body),
Content::TableFooter { body, repeat: _ } => self.layout_content(body),
```

**Render minimal**: body uma vez no contexto actual. `repeat`
armazenado mas ignorado per ADR-0054 graded — **DEBT-56**
permanece aberto.

**`layout_grid` NÃO modificado** (paridade verificações
P157A #8 e P157B #10).

### 1.5 Tests simétricos (.5)

**+26 tests novos** (range esperado +18-23 ultrapassado por par
simétrico que duplica naturalmente; documentado).

**12 unit tests** (6 pares Header↔Footer):
- Constructor default `repeat=true` (par).
- Constructor `repeat=false` explícito (par).
- `is_empty` proxy via body (par).
- `plain_text` recurse no body (par).
- `PartialEq` cobertura (par).
- `map_text` recurse + preserva `repeat` (par).
- 1 extra: `table_header_e_footer_sao_variants_distintos`.

**8 stdlib tests** (4 pares):
- Defaults `repeat=true` (par).
- `repeat=false` explícito (par).
- Sem body rejeitado (par).
- `repeat=Int` rejeitado (par).
- Named arg desconhecido rejeitado (par).

**3 layout E2E tests** (1 par + integrativo):
- `layout_table_header_renderiza_body_no_contexto_actual`.
- `layout_table_footer_renderiza_body_no_contexto_actual`.
- **Integrativo**: `layout_table_com_header_cell_footer_renderiza_tudo`
  — Table com Header + Cell + Footer renderiza os 4 conteúdos.

### 1.6 Hashes + cobertura (.6)

`crystalline-lint --fix-hashes .` reportou **"Nothing to fix"**
(refactor aditivo; preserva hash do prompt L0; paridade P157A/B).

Tabela cobertura actualizada:
- A.6 Model: 2 novas entradas `table.header` + `table.footer`
  como sub-entradas de `table` (não inflam agregação per padrão
  P154A; footnote ²⁶).
- B Content variants: 54 → **56** (+`TableHeader` + `TableFooter`;
  footnote ²⁷).
- B total: 70/13/5/17/1=106 → **72/13/5/15/1=106**.
- Cobertura user-facing total: **~61.0% (inalterada)** —
  ganho qualitativo via expansão estrutural completa de "table
  foundations".
- Cobertura arquitectural total: 78% → **80%** (+2pp via
  fechamento de "table foundations" — variants Content vanilla
  extra ausentes desce de ~1 a 0).

ADR-0061 §"Aplicações cumulativas" actualizada para pós-P157C.
ADR-0060 ganha anotação P157C. README ADRs ganha entrada P157C
antes de P157B.

---

## 2. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo test` workspace: 1353 + Δ; zero falhas (Δ esperado +18-23) | **Δ=+26** (1353 → 1379 lib+integ+diag); zero falhas; range esperado ultrapassado por par simétrico |
| 2 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 3 | Variants Content: 56 (54 → 56; +2 par) | **✓ 56** (`TableHeader` + `TableFooter` adicionados) |
| 4 | Stdlib funcs: 46 (44 → 46; +2 par) | **✓ 46** (`table_header` + `table_footer` registados) |
| 5 | Cobertura Model agregada inalterada (~50%); sub-entradas adicionadas | **✓** entradas `table.header` + `table.footer` como sub-entradas de `table` (footnote ²⁶); cobertura agregada Model **inalterada** com ganho qualitativo |
| 6 | Hash actualizado em prompts L0 | **✓** `crystalline-lint --fix-hashes` reportou "Nothing to fix" (refactor aditivo); lint clean |
| 7 | DEBT-56 permanece aberto | **✓** documentado em §6 + diagnóstico §6 + nota P157C |
| 8 | Paridade interna TableHeader↔TableFooter | **✓** tests adjacentes em pares; pattern-match com entradas simétricas em todos os 9 sítios |
| 9 | ADR-0064 Caso D primeira aplicação Model; patamar Caso D N=4 | **✓** §3.1 do diagnóstico confirma; §5 deste relatório actualiza patamares |
| 10 | `layout_grid` original NÃO modificado | **✓** zero diff em `01_core/src/rules/layout/grid.rs`; arms novos em `layout/mod.rs` são triviais single-render |
| 11 | "Table foundations" declarado em ADR-0060 fica fechado | **✓** P157A + P157B + P157C completam subset declarado (4 variants: Table, TableCell, TableHeader, TableFooter; mesa em §4) |

**Build limpo**: `cargo build` 1.12s sem warnings novos.

---

## 3. Análise de risco — peso real (décima aplicação consecutiva; primeiro par simétrico em Model)

P157C é **primeiro par simétrico em Model**. §análise de risco
preserva precedente N=9 (P156F-P157B) → **N=10**.

### 3.1 Riscos materializados durante o passo

| Risco | Materializado? | Mitigação aplicada |
|-------|:--------------:|---------------------|
| Vanilla TableHeader ter field obrigatório não previsto (`level`) | Sim | Inventário .1 §1.4 detectou `level: NonZeroU32` em Header (não em Footer); decisão documentada — **diferir em ambos** para preservar paridade simétrica cristalina |
| Vanilla `children: Vec<TableItem>` divergir de `body: Box<Content>` | Sim | Inventário .1 §1.4 documentou divergência aceite per ADR-0033 (uniformidade com containers cristalinos) |
| Range de tests +18-23 ser excedido por par simétrico | Sim | Δ=+26 (excedeu por 3); documentado como característica natural de pares simétricos em §1.5 |
| Helper `extract_bool_with_default` colidir com `extract_weak` | Não | Helpers separados (key e default arbitrários vs específicos); separação de domínios per ADR-0037 |

### 3.2 Riscos avaliados mas não materializados

| Risco | Avaliação inicial | Razão de não-materialização |
|-------|-------------------|----------------------------|
| ADR-0064 Caso D ter ambiguidade em Model | Baixo | Caso D já validado em Layout (P156D/G/J); aplicação Model é mecânica |
| Pattern-match exhaustive falhar fora de `content.rs` | Baixo | Cobertura sistemática 9 sítios cobre todos os arms (paridade P157A/B) |
| Quebra de tests pré-existentes Table/TableCell | Nulo | Variants pré-existentes não tocados |
| Tests integrativos Header+Cell+Footer falharem por ordem semântica | Baixo | Test focado em **renderização** (não ordem semântica que requer DEBT-56) |

### 3.3 Riscos não-aplicáveis

- **Algoritmo de repetição em page breaks**: explicitamente
  diferido em DEBT-56 per ADR-0054 graded.
- **Quebra de paridade observável vs vanilla**: divergência
  estrutural aceite per ADR-0033 (`body` vs `children`; naming
  flat); paridade observável estrutural preservada.

### 3.4 Conclusão de risco

**Risco residual: muito baixo após inventário**. Todos os riscos
materializados (level apenas em Header; children vs body;
range tests excedido) foram **detectados pelo sub-passo .1**
e **resolvidos com decisões documentadas** per ADR-0033/0054/0065.

**§análise de risco preserva precedente cross-domínio**: P157C
é primeiro par simétrico em Model — patamar #4 cresce para
N=10 sem reformulação. **Tratamento simétrico em
pattern-match** (subpadrão emergente N=2: P156D + P157C)
torna paridade visualmente óbvia.

---

## 4. Mesa fechamento "table foundations" — P157A + P157B + P157C

**Subset declarado em ADR-0060 §"Decisão 1" sub-passo 3**:
> P157 (renumerado de P156): table foundations — Content::Table
> + sub-elementos TableCell, TableHeader, TableFooter (M+;
> reaproveita Content::Grid parcial para layout).

**Materialização cristalina cumulativa**:

| Sub-passo | Variant Content | Stdlib func | Decisão arquitectural-chave | Tests Δ |
|-----------|-----------------|-------------|----------------------------|--------:|
| **P157A** | `Content::Table { columns, rows, children }` | `native_table` | Módulo `stdlib/structural.rs` (continuação Model existente) | +16 |
| **P157B** | `Content::TableCell { body, x, y, colspan, rowspan }` | `native_table_cell` | Naming flat `table_cell`; ADR-0064 Caso A primeira Model + Caso C primeira variação `usize` | +18 |
| **P157C** | `Content::TableHeader { body, repeat }` + `Content::TableFooter { body, repeat }` | `native_table_header` + `native_table_footer` | Par simétrico; ADR-0064 Caso D primeira Model; saturação cross-domínio cross-caso A/B/C/D | +26 |

**Conjunto fechado**:
- 4 variants Content novos (Table + TableCell + TableHeader + TableFooter).
- 4 stdlib funcs novas.
- 2 helpers privados parametrizados (`extract_usize_or_none_min` em P157B; `extract_bool_with_default` em P157C).
- 3 sub-passos M cada — granularidade preservada N=10/11/12.
- **+60 tests** acumulados (P157A: +16; P157B: +18; P157C: +26).
- **Zero novos DEBTs** abertos durante a série (DEBT-34e e DEBT-56 pré-existentes mantêm-se abertos; recebem contribuição via storage).
- **Zero reformulações mid-passo**.

**Cobertura cumulativa**:
- Model agregada: ~45% → ~50% (+5pp em P157A; inalterada em P157B/C — sub-entradas qualitativas).
- Arquitectural: 76-77% → **80%** (+3pp; +2pp em P157C — variants Content vanilla extra ausentes desce de ~1 a 0).

**Atributos vanilla scope-out totais** (per ADR-0054 graded):
- Table: gutter, column_gutter, row_gutter, inset, align, fill, stroke, summary (8).
- TableCell: align, stroke, fill, inset, breakable + internals kind/is_repeated (6).
- TableHeader: level, repeat-rows + algoritmo repetição (3).
- TableFooter: algoritmo repetição (1).
- Total: ~18 atributos vanilla scope-out per ADR-0054 graded; refinos futuros potenciais.

**Decisões arquitecturais reusadas**:
- Naming flat (P157B/C — limitação FieldAccess).
- Body simplificado vs vanilla children (P157C divergência).
- Layout delegação a `layout_grid` sem modificação (P157A/B/C).
- Helpers privados parametrizados (P157B/C subpadrão emergente).

**ADR-0060 §"Decisão 1" sub-passo 3 fica integralmente
materializado**. Status `IMPLEMENTADO` mantido (Fase 1 fechada
P155 não muda; Fase 2 sub-passo 3 fechada P157C; Fase 2
restantes — P158 figure-kinds + P159 bibliography — prosseguem
per roadmap).

---

## 5. ADR-0061 §"Aplicações cumulativas" — confirmações

§"Aplicações cumulativas" actualizada para pós-P157C:

### 5.1 Padrões metodológicos pós-P157C

| # | Padrão | Pré-P157C | Pós-P157C |
|---|--------|----------:|----------:|
| 1 | Granularidade 1-2 features/passo | 11 | **12** (cross-domínio fortalecido — 3 sub-passos Model consecutivos fecham conjunto coerente) |
| 2 | "Inventariar primeiro" pré-decisão | 9 | **10** (P157C reforça critério #6 divergência da spec) |
| 3 | "Smart→Option/default" | 8 | **9** (Caso D primeiro Model — saturação cross-domínio cross-caso) |
| 4 | "§análise de risco no relatório" | 9 | **10** (par simétrico Model) |
| 5 | "Reuso de template containers" | 4 | 4 (inalterado) |
| 6 | "Antecipar especificidades técnicas" | 2-3 | 2-3 |
| 7 | Helper `extract_length` reuso | 7 | 7 (inalterado) |
| 8 | Reuso `Sides<T>` | 2 | 2 (inalterado) |
| 9 | Reuso `extract_tracks` | 2 | 2 (inalterado) |
| 10 | Helper `extract_usize_or_none_min` (P157B) | N=4 usos | 4 usos (inalterado) |
| 11 | **Helper `extract_bool_with_default`** (novo subpadrão P157C) | — | **N=2 usos** no mesmo passo |
| 12 | **"Par simétrico em pattern-match"** (novo subpadrão P157C) | — | **N=2 aplicações concretas** (P156D HSpace+VSpace + P157C Header+Footer) |

### 5.2 Auto-validação cumulativa de ADRs meta P156K — saturação atingida

P157C confirma utilidade dos ADRs meta com aplicação cross-domínio
cross-caso completa:

- **ADR-0064**:
  - Caso A: N=4 (75% Layout + 25% Model) — inalterado.
  - Caso B: N=1 (100% Layout) — Caso B só Layout (candidato
    futuro Model).
  - Caso C: N=3 (66% Length + 33% `usize`) — inalterado.
  - Caso D: N=3 (Layout 100%) → **N=4** (75% Layout + 25%
    Model — primeira Model).
  - **3/4 casos validados em Model**; **4/4 validados em
    Layout** — saturação cross-domínio cross-caso atingida.
  - Patamar empírico ADR meta atinge maturidade.
- **ADR-0065**:
  - Critério #1 (naming) — explícito P157B + reforçado P157C
    (paridade decisão).
  - Critério #5 (scope) — reforçado.
  - Critério #6 (divergência da spec) — explícito P157B + 
    **reforçado P157C** (divergência `body` vs `children` per
    ADR-0033).

**Padrão emergente confirmado**: cada passo da série P156-P157
valida cumulativamente critérios distintos de ADR-0065 e
patamares de ADR-0064. **Após P157C, ADR-0064 atinge saturação
empírica cross-domínio cross-caso** — possível ADR meta XS de
"caso completion" candidato.

---

## 6. DEBT-56: status pós-P157C

**DEBT-56 permanece aberto**. P157C contribui ao armazenar
field necessário ao algoritmo (`repeat: bool` em ambos
TableHeader e TableFooter) mas não fecha — implementação do
algoritmo de repetição em page breaks fica para refactor
multi-region dedicado (escopo L+).

**Caminho de fechamento sugerido** (per spec §"Pós-passo"):
- Refactor multi-region completo (column flow + header/footer
  repeat); pode incluir fechamento de DEBT-34e (placement
  Grid completo) num único refactor agregado L+.
- Pode ser passo independente ou agregado.

Decisão sobre fechamento de DEBT-56 fica para passo posterior;
P157C não força a decisão.

---

## 7. Estado pós-P157C

- **Cobertura Layout**: **78%** (inalterada).
- **Cobertura Model agregada**: ~50% (inalterada — sub-entradas).
  Ganho qualitativo via expansão estrutural completa de "table
  foundations".
- **Cobertura arquitectural total**: 78% → **80%** (+2pp;
  variants Content vanilla extra ausentes desce de ~1 a 0).
- **Variants Content**: **56** (era 54; +`TableHeader` +
  `TableFooter`; +2 par).
- **Stdlib funcs**: **46** (era 44; +`table_header` +
  `table_footer`; +2 par).
- **Helper novo**: `extract_bool_with_default` privado em
  `stdlib/structural.rs`.
- **Tests**: **1141** typst-core lib (era 1115; +26). Workspace:
  1141 + 215 + 24 + 21 = **1401** (era 1375).
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados (DEBT-34e e DEBT-56
  permanecem abertos; P157C contribui via storage).
- **ADR-0060**: `IMPLEMENTADO` mantido; ganha anotação P157C.
  **Promoção a R1 candidata** se decisão humana for prioritária.
- **ADR-0061**: `PROPOSTO` mantido; §"Aplicações cumulativas"
  actualizada.
- **README ADRs**: entrada P157C adicionada antes de P157B.
- **Reservas P158/P159/ADR-0062**: inalteradas.
- **Hash `content.rs`**: `ec58d849` (preservado — refactor
  aditivo).
- **Total ADRs**: **63** (inalterado).

### 7.1 "Table foundations" fechado

ADR-0060 §"Decisão 1" sub-passo 3 fica integralmente
materializado. Mesa em §4 documenta o conjunto. Marca conceptual
importante — primeira fase Model (Fase 2 sub-passo 3) fechada
com 3 sub-passos M cada, granularidade preservada N=10/11/12.

---

## 8. Decisão pós-P157C

Per spec do passo §"Pós-passo", próximas opções (sem candidata
pré-acordada):

1. **P158** — figure-kinds (Model Fase 2 continuação per
   ADR-0060).
2. **P159** — bibliography + cite (Model XL; DEBT-55 + ADR-0062
   reservada hayagriva).
3. Continuar Fase 3 Layout (columns/colbreak — DEBT-56;
   inclui repeat de header/footer).
4. Footnote area (sub-fase prioritária ADR-0061 Decisão 5).
5. Promover ADR-0061 a IMPLEMENTADO.
6. Promover `extract_length` a helper público (N=7 patamar
   forte).
7. Atacar Introspection (17% cobertura).
8. Fechar DEBT-34e e DEBT-56 (refactor placement Grid +
   multi-region; pode ser agregado L+).
9. **Promover ADR-0060 a R1** (passo administrativo XS) com
   confirmação de Fase 2 sub-passo 3 fechado.
10. **ADR meta XS de "ADR-0064 caso completion"** (saturação
    cross-domínio cross-caso atingida).

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`.

**Padrão granularidade N=12 NÃO é formalizado** — continua
candidato. P157C consolida o padrão sem quebra. **Patamar
empírico cross-domínio cross-caso ADR-0064 atinge saturação** —
candidato a ADR meta administrativo XS futuro.

---

## 9. Fechamento

P157C fecha como **terceiro e último sub-passo Model Fase 2 —
"table foundations" integralmente fechado**. **Primeira aplicação
concreta de ADR-0064 Caso D em domínio Model** — patamar
empírico cross-domínio cresce. **Saturação cross-domínio
cross-caso ADR-0064**: 4/4 casos validados em Layout; 3/4 (A,
C, D) validados em Model — Caso B só Layout.

**Padrão cross-domínio fortalecido**: 3 sub-passos Model
consecutivos (P157A/B/C) sem reformulação fecham conjunto
coerente per ADR-0060 §"Decisão 1" sub-passo 3.

**Auto-validação cumulativa ADRs meta P156K atinge maturidade
empírica**: ADR-0064 ganha diversidade em 3 dimensões
(cross-domínio + cross-tipo + cross-caso); ADR-0065 ganha
aplicação explícita de critérios #1, #5, #6 em múltiplos passos.

**Novos subpadrões emergentes P157C**:
- Helper privado parametrizado `extract_bool_with_default`
  (N=2 usos).
- "Par simétrico em pattern-match" (N=2 aplicações concretas:
  P156D HSpace+VSpace + P157C Header+Footer).

**DEBT-34e e DEBT-56 permanecem abertos**; P157A/B/C contribuem
via storage de fields necessários aos algoritmos. Caminho de
fechamento documentado em §6 deste relatório (refactor
multi-region L+).

ADR-0060 mantém `IMPLEMENTADO` (promoção a R1 candidata);
ADR-0061 mantém `PROPOSTO`.

**Pausa natural após P157C — table foundations fechado;
ADR-0064 atinge saturação cross-domínio cross-caso; padrões
P156-P157 consolidados a maturidade empírica. Decisão humana
sobre próxima direcção (10 candidatas documentadas) tem
máxima informação acumulada.**

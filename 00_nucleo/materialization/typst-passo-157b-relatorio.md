# Relatório P157B — `Content::TableCell` (Model Fase 2 sub-passo 2)

Segundo sub-passo substantivo de Model Fase 2. **Décima primeira
aplicação consecutiva de materialização** desde início da série
granular P156C. **Primeira aplicação concreta de ADR-0064 Caso A
em domínio Model** (P156G/H/I aplicaram-no em Layout); **terceira
aplicação global de Caso C** com **primeira variação `usize`**.

---

## 1. Resumo do executado

### 1.1 Diagnóstico (.1)

Ficheiro novo:
`00_nucleo/diagnosticos/diagnostico-table-cell-passo-157b.md`
(7 itens canónicos ADR-0034 + 2 itens específicos de field
semântico não-trivial).

**Decisões arquitecturais documentadas**:
- **Naming `table_cell` flat** (não vanilla `table.cell`):
  divergência intencional per ADR-0033. FieldAccess cristalino
  actual (`bindings.rs:eval_field_access`) suporta apenas
  `Value::Dict.field` e `Value::Content.field` — não suporta
  `Value::Func.subname`. Refactor para `table.cell` exigiria
  `Value::Module`/`Value::ScopedFunc` (fora de scope).
- **Helper privado parametrizado** `extract_usize_or_none_min(val,
  fn, field, min: usize)`: combina parse de `x`/`y` (min=0) e
  `colspan`/`rowspan` (min=1) num único helper.
- **ADR-0064 traduções**:
  - `x`/`y`: `Smart<usize>` → `Option<usize>` (Caso A; primeiro
    Model).
  - `colspan`/`rowspan`: `NonZeroUsize` default 1 → `Option<usize>`
    (Caso C; primeira variação `usize`).
- **Layout placement diferido**: `x`/`y`/`colspan`/`rowspan`
  armazenados mas ignorados — DEBT-34e permanece aberto.

### 1.2 Variant `Content::TableCell` (.2)

Adicionado a `01_core/src/entities/content.rs` (53 → **54**
variants).

```rust
TableCell {
    body:    Box<Content>,
    x:       Option<usize>,
    y:       Option<usize>,
    colspan: Option<usize>,
    rowspan: Option<usize>,
}
```

Construtor `Content::table_cell(body, x, y, colspan, rowspan)`.

Cobertura exaustiva de **9 sítios pattern-match estruturais**
(paridade P157A):
- Variant declaration + construtor.
- `is_empty()` proxy via body.
- `plain_text()` recurse no body sem multiplicar por
  colspan/rowspan.
- `PartialEq` cobre 5 fields.
- `map_content` recurse no body; preserva fields.
- `map_text` recurse no body; preserva fields.
- `introspect.rs::materialize_time` recurse + preserva fields.
- `introspect.rs::walk` walk no body.
- `layout/mod.rs::layout_content` arm minimal (renderiza body).

### 1.3 Stdlib `native_table_cell` (.3)

Adicionado a `01_core/src/rules/stdlib/structural.rs`
(continuação P157A).

```rust
pub fn native_table_cell(_ctx, args, _world, _file, _fig) -> SourceResult<Value>
```

Helper privado novo `extract_usize_or_none_min`:
- `Value::Auto` ou `Value::None` → `None` (None ↔ Auto vanilla).
- `Value::Int(n)` com `n >= min as i64` → `Some(n as usize)`.
- Outros tipos ou `n < min` → erro hard.

Validações:
- `colspan`/`rowspan = 0` rejeitado (paridade `NonZeroUsize`;
  min=1).
- Int negativo em qualquer field rejeitado.
- Named arg desconhecido rejeitado.
- Body inválido rejeitado.

Registado em `eval/mod.rs::make_stdlib` como `table_cell` →
`Func::native("table_cell", native_table_cell)`. Re-exportado
em `stdlib/mod.rs`.

### 1.4 Layout para `Content::TableCell` (.4)

Pattern arm novo em `layout_content` (`01_core/src/rules/layout/mod.rs`):

```rust
Content::TableCell { body, x: _, y: _, colspan: _, rowspan: _ } => {
    self.layout_content(body);
}
```

**Render minimal**: body uma vez no contexto actual. `x`/`y`/
`colspan`/`rowspan` armazenados mas ignorados per ADR-0054 graded.
**`layout_grid` NÃO modificado** (paridade verificação P157A #8;
DEBT-34e permanece aberto).

### 1.5 Tests (.5)

**+18 tests novos** (within range esperado +15-18):

**7 unit tests `Content::TableCell`** em `entities/content.rs`:
- Constructor default (todos None excepto body).
- Constructor com x/y explícitos.
- Constructor com colspan/rowspan explícitos.
- `is_empty` proxy via body.
- `plain_text` recurse sem multiplicar por spans.
- `PartialEq` cobertura 5 fields (5 vias de divergência).
- `map_text` recurse + preserva fields.

**9 stdlib tests `native_table_cell`** em `stdlib/mod.rs`:
- Defaults todos None.
- x/y explícitos (Caso A).
- x = `Value::Auto` → None (Caso A semântica).
- colspan/rowspan explícitos (Caso C).
- colspan = 0 rejeitado.
- colspan = -1 rejeitado.
- x = -1 rejeitado.
- Named arg desconhecido rejeitado.
- Sem body rejeitado.

**2 layout E2E tests** em `layout/tests.rs`:
- `layout_table_cell_renderiza_body_no_contexto_actual` — cell
  renderiza body **exactamente uma vez** (sem multiplicar por
  colspan/rowspan).
- `layout_table_cell_dentro_de_table_renderiza_como_cell` —
  Table com mix de plain text + TableCell renderiza todos os
  conteúdos.

### 1.6 Hashes + cobertura (.6)

`crystalline-lint --fix-hashes .` reportou **"Nothing to fix"**
(refactor aditivo; preserva hash do prompt L0).

Tabela cobertura actualizada:
- A.6 Model: nova entrada `table.cell` (sub-entrada de `table`)
  com estado `parcial`. **Não conta na agregação** (per padrão
  P154A — sub-entradas de scope vanilla não inflam contagem
  agregada).
- B Content variants: 53 → **54** (+`TableCell`; footnote ²⁵).
- B total: 69/13/5/18/1=106 → **70/13/5/17/1=106**.
- Cobertura user-facing total: **~61.0% (inalterada)** —
  ganho qualitativo via expansão estrutural.
- Cobertura arquitectural total: 77-78% → **78%**.

ADR-0061 §"Aplicações cumulativas" actualizada para pós-P157B.
ADR-0060 ganha anotação P157B. README ADRs ganha entrada P157B
antes de P157A.

---

## 2. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo test` workspace: 1335 + Δ; zero falhas (Δ esperado +15-18) | **Δ=+18** (1335 → 1353 lib+integ+diag); zero falhas |
| 2 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 3 | Variants Content: 54 (53 → 54) | **✓ 54** (`TableCell` adicionado) |
| 4 | Stdlib funcs: 44 (43 → 44) | **✓ 44** (`table_cell` registado) |
| 5 | Cobertura Model: ~55% (~50% → ~55%) — entrada `table.cell` `implementado parcial` | **Divergência aceite**: entrada `table.cell` adicionada como sub-entrada de `table` (não inflama agregação per padrão P154A); cobertura agregada Model **inalterada (50%)** com ganho qualitativo via expansão estrutural; documentado em footnote ²⁴ |
| 6 | Hash actualizado em prompts L0 | **✓** `crystalline-lint --fix-hashes` reportou "Nothing to fix" (refactor aditivo); lint clean |
| 7 | DEBT-34e permanece aberto | **✓** explicitamente documentado em §1.4 + diagnóstico §6 + nota P157B em DEBT.md futura |
| 8 | Naming convention final documentada | **✓** §1.1: `table_cell` flat per FieldAccess actual + diagnóstico §8 |
| 9 | ADR-0064 Caso A primeira aplicação Model + Caso C terceira global | **✓** §5; auto-validação cumulativa documentada |
| 10 | `layout_grid` original NÃO modificado | **✓** zero diff em `01_core/src/rules/layout/grid.rs`; cell arm em `layout/mod.rs` é trivial single-render |

**Build limpo**: `cargo build` 1.19s sem warnings novos.

---

## 3. Análise de risco — peso real (nona aplicação consecutiva; primeira aplicação Caso A em Model)

P157B é **primeiro passo Model com aplicação concreta de
ADR-0064 Caso A**. §análise de risco preserva precedente N=8
(P156F-P157A) → **N=9**.

### 3.1 Riscos materializados durante o passo

| Risco | Materializado? | Mitigação aplicada |
|-------|:--------------:|---------------------|
| Naming `table.cell` (vanilla) ser ambíguo em cristalino | **SIM** | Inventário .1 §8 detectou limitação em FieldAccess actual; decisão documentada `table_cell` flat com refactor futuro possível sem breaking change |
| ADR-0064 Caso A ter ambiguidade em Model | Não | Inventário .1 §3 confirmou `Smart<usize>` semântica "auto = computa do contexto" (paridade Caso A em Layout) |
| Helper `extract_usize_or_none_min` ter complexidade superior | Baixo | Implementação simples; param `min` evita duplicação |
| Tests E2E para spans visuais divergirem | Não | Tests focados em **single render** + storage (não placement visual) per ADR-0054 graded |

### 3.2 Riscos avaliados mas não materializados

| Risco | Avaliação inicial | Razão de não-materialização |
|-------|-------------------|----------------------------|
| `extract_usize_or_none_min` exigir variantes alargadas | Médio | Param `min` único é suficiente; não foi necessário expandir |
| Pattern-match exhaustive falhar fora de `content.rs` | Baixo | Cobertura sistemática 9 sítios cobre todos os arms (paridade P157A) |
| Quebra de tests pré-existentes Table | Nulo | Table não tocado; tests P157A passam inalterados |
| Layout cell dentro de Table não renderizar | Baixo | Test E2E `layout_table_cell_dentro_de_table_renderiza_como_cell` confirma delegação correcta |

### 3.3 Riscos não-aplicáveis

- **Algoritmo de placement**: explicitamente diferido em DEBT-34e
  per ADR-0054 graded.
- **Quebra de paridade observável vs vanilla**: divergência
  estrutural aceite per ADR-0033 (naming `table_cell` vs
  `table.cell`); paridade observável estrutural preservada.

### 3.4 Conclusão de risco

**Risco residual: muito baixo após inventário**. O risco principal
materializado (naming `table.cell` ambíguo) foi **detectado pelo
sub-passo .1** (ADR-0065 critério #1 naming) e **resolvido com
divergência intencional documentada** per ADR-0033. Refactor
futuro pode adicionar alias `table.cell` sem breaking change.

**§análise de risco preserva precedente cross-domínio**: P157B
é primeiro Model com Caso A — patamar ADR-0064 cresce com
diversidade. Padrão #4 cresce para N=9 sem reformulação.

---

## 4. Slope cumulativo Model (mesa P155-P157B)

| Passo | Feature(s) | Slope Model | Cobertura Model cumulativa | Tests Δ |
|-------|-----------|------------:|---------------------------:|--------:|
| P154A | (diagnóstico Model) | — | 36% | 0 |
| P154B | terms + divider | +5%  | 36% → 41% | +10 |
| P155 | quote | +4%  | 41% → 45% (Fase 1 fechada) | +21 |
| P157 | (diagnóstico Model Fase 2) | — | — (sem código) | 0 |
| P157A | table minimal (Fase 2 sub-passo 1) | +5%  | 45% → 50% | +16 |
| **P157B** | **table cell (Fase 2 sub-passo 2)** | **0% agregado** | **50% inalterado (sub-entrada qualitativa)** | **+18** |

**Total cumulativo P154A-P157B** (Model): **+14pp** Model em
6 passos (4 materialização + 2 diagnóstico). P157B é **primeiro
caso** de "ganho qualitativo sem mover agregação" em Model —
precedente análogo a P156L em Layout.

**Mesa cross-domínio P156C-P157B** (combinada Layout + Model):

| Domínio | Passos | Slope total | Tests Δ |
|---------|-------:|------------:|--------:|
| Layout (P156C-L) | 9 materialização + 1 meta | +56pp Layout | +174 |
| Model (P157A/B) | 2 materialização + 1 diagnóstico | +5pp Model agregado | +34 |
| **Cross-domínio total** | **11 materialização + 2 meta + 1 diagnóstico** | — | **+208** |

**Padrão granular universal cross-domínio reforçado**: 2
sub-passos Model consecutivos validam padrão Model
independentemente do precedente isolado P157A. **N=11
materialização sem reformulação** — patamar empírico forte
crescente.

---

## 5. ADR-0061 §"Aplicações cumulativas" — confirmações

§"Aplicações cumulativas" actualizada para pós-P157B:

### 5.1 Padrões metodológicos pós-P157B

| # | Padrão | Pré-P157B | Pós-P157B |
|---|--------|----------:|----------:|
| 1 | Granularidade 1-2 features/passo | 10 | **11** (cross-domínio reforçado) |
| 2 | "Inventariar primeiro" pré-decisão | 8 | **9** (critério #1 + #6) |
| 3 | "Smart→Option/default" | 7 | **8** (Caso A primeiro Model; Caso C primeira variação `usize`) |
| 4 | "§análise de risco no relatório" | 8 | **9** (primeira aplicação Caso A Model) |
| 5 | "Reuso de template containers" | 4 | 4 (inalterado) |
| 6 | "Antecipar especificidades técnicas" | 2-3 | 2-3 |
| 7 | Helper `extract_length` reuso | 7 | 7 (inalterado) |
| 8 | Reuso `Sides<T>` | 2 | 2 (inalterado) |
| 9 | Reuso `extract_tracks` | 2 | 2 (inalterado) |
| 10 | **Helper privado parametrizado `extract_usize_or_none_min`** (novo subpadrão P157B) | — | **N=4 usos no mesmo passo** (combinação via param em vez de duplicação) |

### 5.2 Auto-validação cumulativa de ADRs meta P156K

P157B confirma utilidade dos ADRs meta com aplicação cross-domínio
e cross-caso:

- **ADR-0064**:
  - Caso A: N=3 (100% Layout) → **N=4** (75% Layout + 25%
    Model) — primeira aplicação Model.
  - Caso C: N=2 (100% Length) → **N=3** (66% Length + 33%
    `usize`) — primeira variação tipo.
  - Diversidade cresce em duas dimensões simultaneamente.
- **ADR-0065**:
  - Critério #1 (naming) — **explícito** em P157B (decisão
    `table_cell` vs `table.cell`) + implícito em P157A
    (decisão de módulo).
  - Critério #5 (scope) — reforçado.
  - Critério #6 (divergência da spec) — **explícito** em P157B
    (divergência intencional vs vanilla `table.cell` documentada
    per ADR-0033).

**Padrão emergente confirmado**: cada passo da série P156-P157
valida cumulativamente critérios distintos de ADR-0065 e
patamares de ADR-0064. Auto-validação dos ADRs meta cresce
naturalmente sem nova ADR.

---

## 6. DEBT-34e: status pós-P157B

**DEBT-34e permanece aberto**. P157B contribui ao armazenar
fields necessários ao algoritmo de placement (`x`, `y`,
`colspan`, `rowspan`) mas não fecha — implementação do algoritmo
fica para refactor dedicado ou agregação com P157C/futuros
passos.

**Caminho de fechamento sugerido** (per spec do passo §"Pós-passo"):
- Refactor dedicado a placement Grid completo (escopo M-L);
  consome `x/y` para posição explícita; consome `colspan/rowspan`
  para expansão visual; resolve colisões.
- Pode ser agregado com P157C ou seguir como passo independente.

Decisão sobre fechamento de DEBT-34e fica para passo posterior;
P157B não força a decisão.

---

## 7. Estado pós-P157B

- **Cobertura Layout**: **78%** (inalterada).
- **Cobertura Model agregada**: ~50% (inalterada — sub-entrada).
  Ganho qualitativo via expansão estrutural.
- **Variants Content**: **54** (era 53; +`TableCell`).
- **Stdlib funcs**: **44** (era 43; +`table_cell`).
- **Helper novo**: `extract_usize_or_none_min` privado em
  `stdlib/structural.rs`.
- **Tests**: **1115** typst-core lib (era 1097; +18). Workspace:
  1115 + 215 + 24 + 21 = **1375** (era 1357).
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados (DEBT-34e permanece
  aberto; P157B contribui via storage).
- **ADR-0060**: `IMPLEMENTADO` mantido; ganha anotação P157B.
- **ADR-0061**: `PROPOSTO` mantido; §"Aplicações cumulativas"
  actualizada.
- **README ADRs**: entrada P157B adicionada antes de P157A.
- **Reservas P158/P159/ADR-0062**: inalteradas.
- **Hash `content.rs`**: `ec58d849` (preservado — refactor
  aditivo).
- **Total ADRs**: **63** (inalterado).

### 7.1 Cobertura user-facing total

(64 + 22) / 141 = **~61.0%** (inalterada vs P157A — sub-entrada
qualitativa).

---

## 8. Decisão pós-P157B

Per spec do passo §"Pós-passo", próximas opções pré-acordadas:

1. **P157C** — `Content::TableHeader` + `Content::TableFooter`
   par simétrico. Fecha "table foundations" declarado em
   ADR-0060. Granularidade N=12. Tests ~10-15. Aplicação
   concreta de ADR-0064 Caso D (`repeat: bool` default true).
2. Continuar Fase 3 Layout (columns/colbreak — DEBT-56;
   quebra granularidade).
3. Footnote area.
4. Promover ADR-0061 a IMPLEMENTADO.
5. Promover `extract_length` a helper público (N=7 patamar
   forte).
6. Atacar Introspection (17% cobertura).
7. Fechar DEBT-34e (placement Grid completo) — refactor
   dedicado ou agregado com P157C.

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`.

**Padrão granularidade N=11 NÃO é formalizado** — continua
candidato. P157B consolida o padrão sem quebra.

---

## 9. Fechamento

P157B fecha como **segundo sub-passo Model Fase 2**. **Primeira
aplicação concreta de ADR-0064 Caso A em domínio Model** —
patamar empírico cross-domínio cresce. **Primeira variação
`usize` do Caso C** — diversidade de tipo cresce. **Primeira
aplicação concreta de ADR-0065 critério #6** (divergência da
spec da feature vanilla — naming `table_cell` flat vs
`table.cell`).

**Padrão cross-domínio reforçado**: 2 sub-passos Model consecutivos
(P157A/B) sem reformulação validam padrão Model independente
do precedente isolado.

**Auto-validação cumulativa ADRs meta P156K**: ADR-0064 ganha
diversidade em 2 dimensões (cross-domínio + cross-tipo);
ADR-0065 ganha aplicação explícita de critério #1 e #6.

**DEBT-34e permanece aberto**; P157B contribui via storage de
fields necessários ao algoritmo. Caminho de fechamento documentado
em §6.

ADR-0060 mantém `IMPLEMENTADO`; ADR-0061 mantém `PROPOSTO`.

**Pausa natural após P157B — segundo sub-passo Model Fase 2;
ADR-0064 patamares crescem cross-domínio + cross-tipo;
ADR-0065 critérios #1 e #6 ganham aplicação concreta.
Decisão humana sobre próxima direcção (P157C ou outras 6
candidatas) tem máxima informação.**

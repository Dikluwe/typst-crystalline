# Diagnóstico `Content::TableCell` — Passo P157B

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
segundo sub-passo Model Fase 2. **Décima segunda aplicação
consecutiva** do padrão diagnóstico-primeiro; **oitava** sob
critério estendido de ADR-0065 (P156C/D/G/H/J/L + P157/A/B).

**Primeira aplicação concreta de ADR-0064 Caso A em domínio
Model** (P156G/H/I aplicaram-no em Layout; agora cross-domínio).

---

## 1. Assinatura vanilla `TableCell` minimal

Fonte: `lab/typst-original/.../model/table.rs:734-782` (~50 linhas).

```rust
#[elem(name = "cell", title = "Table Cell")]
pub struct TableCell {
    pub body: Content,                                       // #[required]
    pub x: Smart<usize>,                                     // Caso A
    pub y: Smart<usize>,                                     // Caso A
    pub colspan: NonZeroUsize,                               // #[default(NonZeroUsize::ONE)] — Caso C
    pub rowspan: NonZeroUsize,                               // idem
    pub inset: Smart<Sides<Option<Rel<Length>>>>,            // diferido
    pub align: Smart<Alignment>,                             // diferido
    pub fill: Smart<Option<Paint>>,                          // diferido
    pub stroke: Sides<Option<Option<Arc<Stroke>>>>,          // diferido
    pub breakable: Smart<bool>,                              // diferido
    pub kind: Smart<TableCellKind>,                          // #[internal] — diferido
    pub is_repeated: bool,                                   // #[internal] — diferido
}
```

**Subset minimal P157B** (5 fields críticos; consistente com
spec):

| Field vanilla | Tipo vanilla | Tradução cristalina P157B | Caso ADR-0064 |
|---------------|--------------|---------------------------|:--------------:|
| `body` | `Content` (req) | `Box<Content>` | — |
| `x` | `Smart<usize>` | `Option<usize>` (None ↔ Auto) | **A** |
| `y` | `Smart<usize>` | `Option<usize>` (None ↔ Auto) | **A** |
| `colspan` | `NonZeroUsize` (default 1) | `Option<usize>` (None ↔ default 1) | **C** |
| `rowspan` | `NonZeroUsize` (default 1) | `Option<usize>` (None ↔ default 1) | **C** |

**Subset diferido para refinos futuros** (per ADR-0054 graded;
6 fields):
- `inset`, `align`, `fill`, `stroke` per-cell (refinos M após
  pattern Block/Box).
- `breakable` (depende multi-region — DEBT-56).
- `kind`, `is_repeated` (internal vanilla — não user-facing).

---

## 2. Comportamento observável (subset minimal)

**Vanilla**:
- `table.cell(x: 2, y: 3)[body]` posiciona cell explicitamente
  na coluna 2, linha 3 (zero-indexed) — sobrepõe auto-placement.
- `table.cell(colspan: 2)[body]` faz cell ocupar 2 colunas
  visualmente (expansão).
- Cell sem `x`/`y` usa **auto-placement** (algoritmo de
  resolução de colisões).

**Cristalino P157B** (per ADR-0054 graded):
- ✓ Variant existe; fields armazenados.
- ✓ Cell renderiza body no contexto actual (single render).
- ✗ Sem auto-placement explícito (`x`/`y` armazenados mas
  `layout_grid` não os consulta — DEBT-34e).
- ✗ Sem expansão visual de spans (colspan/rowspan armazenados
  mas ignorados — DEBT-34e).
- ✓ Show rules futuras podem aceder fields via `get_field`
  (extensão futura).

**Divergência aceite per ADR-0033 + ADR-0054**:
- Comportamento observável de P157B difere de vanilla quando
  `x`/`y`/colspan/rowspan são especificados — fields são
  no-ops em layout. Aceitar como limitação consciente até
  fechamento de DEBT-34e.
- Tests E2E focam **armazenamento e propagação**, NÃO
  comportamento de placement visual.

---

## 3. ADR-0064 caso aplicável

### 3.1 `x`/`y` — Caso A (primeira aplicação concreta em Model)

Vanilla `Smart<usize>` com semântica:
- `Auto` ⇔ "computa do contexto" (auto-placement algoritmo).
- `Custom(n)` ⇔ posição explícita.

Cristalino: `Option<usize>`. `None` ⇔ Auto (sem posição
explícita, auto-placement aplicável quando DEBT-34e fechar);
`Some(n)` ⇔ posição explícita.

**Auto-validação cumulativa de ADR-0064 P156K**:
- N=4 aplicação concreta de Caso A globalmente (P156G Block.width;
  P156H Box.width; P156I Stack.spacing; **P157B TableCell.x/y**).
- **Primeira aplicação Model**. ADR-0064 ganha diversidade
  cross-domínio empírica (era 100% Layout antes; agora 75%
  Layout + 25% Model).

### 3.2 `colspan`/`rowspan` — Caso C (terceira aplicação concreta)

Vanilla `NonZeroUsize` com default 1 (não-`Default::default() ==
0`); cristalino `Option<usize>` com `None` ⇔ default 1.

**Validação cristalina obrigatória**: zero rejeitado em stdlib
func (paridade `NonZeroUsize`).

**Auto-validação cumulativa de ADR-0064 P156K**:
- N=3 aplicação concreta de Caso C globalmente (P156I Stack.spacing
  Length default zero; P156J Repeat.gap Length default zero;
  **P157B TableCell.colspan/rowspan usize default 1**).
- **Primeira variação `usize` do Caso C** (anteriores eram
  `Length`). Patamar empírico cresce com diversidade de tipo.

### 3.3 Tabela de patamares ADR-0064 pós-P157B

| Caso | Pré-P157B | Pós-P157B |
|------|----------:|----------:|
| A | N=3 (P156G/H/I) | **N=4** (+P157B x/y; primeiro Model) |
| B | N=1 (P156I Dir) | N=1 (inalterado) |
| C | N=2 (P156I/J) | **N=3** (+P157B colspan/rowspan; primeiro usize) |
| D | N=4-5 (P156D/G/J + impl P157A Header.repeat) | N=4-5 (inalterado) |

---

## 4. Variants Content existentes a estender

**Nenhuma**. `Content::TableCell` é variant novo — sem encaixe
em variants existentes. Análogo estrutural a P156G Block /
P156H Boxed (variant rico com 5 fields).

---

## 5. Helpers stdlib reusáveis

### 5.1 `extract_length` (N=7) — irrelevante

`extract_length` não aplicável (P157B usa `usize`, não `Length`).

### 5.2 `extract_tracks` (N=2) — irrelevante

P157B não consome tracks.

### 5.3 Helper novo `extract_usize_or_none`

Helper privado em `stdlib/structural.rs`:
```rust
/// Coage `Value` para `Option<usize>` per ADR-0064 Caso A.
/// `Auto` / `none` (`Value::None`) → `None`.
/// `Int(n)` com n >= 0 → `Some(n as usize)`.
/// Outros → erro hard via Result.
fn extract_usize_or_none(val: &Value, fn_name: &str, field: &str) -> SourceResult<Option<usize>>
```

**Variante `extract_usize_or_none_min1`** (para `colspan`/
`rowspan`): rejeita `Some(0)` adicionalmente. Pode ser combinada
no helper base com bool param `allow_zero`, ou helpers separados.
**Decisão**: combinada via param `min: usize` para evitar
duplicação de código.

```rust
fn extract_usize_or_none_min(
    val: &Value,
    fn_name: &str,
    field: &str,
    min: usize,
) -> SourceResult<Option<usize>>
```

`min=0` para `x`/`y`; `min=1` para `colspan`/`rowspan`.

**Promoção a `pub(super)` ou helper público**: diferida até
N=2-3 reuso em outros passos Model (e.g. P157C Header.level
ou refactor futuro). Política consistente N=3-4 mínima.

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P157B | Refino futuro |
|---------|--------------|---------------|
| Algoritmo auto-placement quando `x`/`y` = None | ✗ scope-out | **DEBT-34e** (refactor placement Grid completo) |
| Expansão visual de colspan/rowspan | ✗ scope-out | DEBT-34e (mesmo refactor) |
| Resolução de colisões `x`/`y` explícitos | ✗ scope-out | DEBT-34e |
| `align`/`stroke`/`fill`/`inset` per cell | ✗ scope-out | refino M após Block/Box pattern |
| `breakable` per cell | ✗ scope-out | depende DEBT-56 |
| Variant existe + fields armazenados | ✓ implementado | — |
| Stdlib func + validações | ✓ implementado | — |
| Show rules `get_field` para acesso aos fields | ✗ scope-out | extensão futura quando show rules expandirem |

**DEBT-34e permanece aberto após P157B**. P157B contribui para
fechamento futuro armazenando os fields necessários ao
algoritmo de placement, mas não fecha a DEBT por si.

---

## 7. Tests planeados

### 7.1 Unit tests `Content::TableCell` (~6)

Em `entities/content.rs`:
1. Constructor default — todos None excepto body (vazio).
2. Constructor com `x`/`y` explícitos.
3. Constructor com colspan/rowspan explícitos.
4. `is_empty()` proxy via `body.is_empty()`.
5. `plain_text()` recurse no body sem multiplicar por
   colspan/rowspan.
6. `PartialEq` cobertura (5 vias: body, x, y, colspan, rowspan).
7. `map_text` recurse + preserva fields.

### 7.2 Stdlib tests `native_table_cell` (~7-8)

Em `stdlib/mod.rs`:
1. Defaults (todos None excepto body).
2. `x = 2`, `y = 3` (Caso A explícito).
3. `x = auto` (`Value::Auto` → None).
4. `colspan = 2`, `rowspan = 3` (Caso C explícito).
5. `colspan = 0` rejeitado (zero não permitido para spans).
6. `colspan = -1` (Int negativo) rejeitado.
7. Named arg desconhecido rejeitado.
8. body inválido rejeitado.

### 7.3 Layout E2E tests (~2)

Em `layout/tests.rs`:
1. `layout_table_cell_renderiza_body_no_contexto_actual` —
   cell renderiza body uma vez (sem multiplicar por colspan).
2. `layout_table_cell_dentro_de_table_renderiza_como_cell` —
   `Content::Table` com `Content::TableCell` em children
   delega correctamente; cell body aparece como FrameItem::Text.

**Δ esperado**: +15 a +18 tests.

---

## 8. Decisão de naming: `table_cell` flat (não `table.cell`)

Inspecção de `01_core/src/rules/eval/bindings.rs:124-156`
(`eval_field_access`):

```rust
pub(super) fn eval_field_access(...) {
    let target = eval_expr(access.target(), ...)?;
    match target {
        Value::Dict(d) => d.get(field.as_str())...,    // dict.field
        Value::Content(c) => c.get_field(field.as_str())..., // content.body
        other => Err(...) // field access não suportado
    }
}
```

**Field access cristalino actual** suporta apenas:
- `Value::Dict.field` → lookup em dict.
- `Value::Content.field` → `get_field` em variant Content.

**`table.cell` exigiria** suporte para `Value::Func.subname`
(namespacing de funcs) ou `Value::Module.subname` (módulo de
funcs). **Nenhum existe actualmente** em cristalino — vanilla
usa `#[scope]` em proc-macro que cristalino não replica.

**Decisão**: `table_cell` flat (snake_case). Justificação:
1. FieldAccess actual NÃO suporta namespacing de funcs.
2. Sem precedente cristalino de `terms.item`, `figure.caption`,
   etc. (todos são funcs flat ou inexistentes).
3. Implementar `table.cell` exigiria refactor substantivo do
   FieldAccess + introdução de `Value::Module` ou
   `Value::ScopedFunc` — fora de scope P157B.
4. Refactor para `table.cell` no futuro pode adicionar alias
   `table.cell` sem breaking change (se decidido).

**Divergência intencional vs vanilla** per ADR-0033:
- Paridade observável estrutural preservada (mesma feature;
  nome ligeiramente diferente).
- Documentar como divergência consciente em §análise de risco
  do relatório.

**Naming alternativo rejeitado** (`tablecell` sem underscore):
- Viola convenção snake_case da stdlib cristalina (`make_calc_module`,
  `extract_length`, etc.).
- `table_cell` segue convenção `make_xxx_yyy`/`extract_xxx_yyy`.

---

## 9. ADR-0064 Caso A primeira aplicação Model — patamar empírico

| Caso | Aplicações pré-P157B | Domínios | Pós-P157B | Domínios |
|------|---------------------:|----------|----------:|----------|
| A | 3 (P156G/H/I) | 100% Layout | **4** (+P157B x/y) | **75% Layout + 25% Model** |
| C | 2 (P156I/J) | 100% Layout | **3** (+P157B colspan/rowspan) | **66% Layout + 33% Model** |

**P157B introduz cross-domínio dos Casos A e C**. ADR-0064
ganha utilidade empírica em Model — auto-validação cumulativa
de ADR meta P156K cresce com diversidade de domínio.

---

## Resumo executivo

P157B materializa `Content::TableCell` minimal:
- Variant `TableCell { body, x, y, colspan, rowspan }` com 5
  fields.
- 4 fields ADR-0064: 2 Caso A (`x`/`y`); 2 Caso C (`colspan`/
  `rowspan`).
- Stdlib `native_table_cell` em `stdlib/structural.rs`
  (continuação P157A); naming **`table_cell` flat** (não
  `table.cell` — FieldAccess actual não suporta namespacing
  de funcs).
- Helper privado novo `extract_usize_or_none_min(val, fn, field,
  min: usize)` para parse de `x`/`y` (min=0) e `colspan`/`rowspan`
  (min=1).
- Layouter delega a render de body simples; **`x`/`y`/colspan/
  rowspan ignorados em layout per ADR-0054 graded**; DEBT-34e
  permanece aberto.

**Decisões arquitecturais P157B**:
- **Naming `table_cell` flat** vs vanilla `table.cell` —
  divergência intencional documentada per ADR-0033.
- **Helper `extract_usize_or_none_min`** combina `x`/`y`
  (min=0) e `colspan`/`rowspan` (min=1) num só helper para
  evitar duplicação.
- **Layout placement diferido em DEBT-34e**: fields armazenados
  e propagados (materialize_time, walk, map_*) mas não
  consumidos por `layout_grid`.

**Decisões diferidas para P157C/futuros**:
- TableHeader/Footer (P157C).
- align/stroke/fill/inset/breakable per cell (refinos futuros).
- Algoritmo de placement completo (DEBT-34e).

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido (este doc).
- ADR-0054: graded scope-out de placement + 6 atributos vanilla.
- ADR-0060: variant novo per Decisão 4 (Model Fase 2 sub-passo 2).
- ADR-0064: **Caso A primeira aplicação Model** (cross-domínio);
  Caso C terceira aplicação global (primeiro usize).
- ADR-0065: critério #1 (naming `table_cell` flat) +
  critério #5 (scope) implícitos.

**Tests planeados**: Δ +15-18.

**Risco**: baixo-médio. Mitigação: variant aditivo análogo a
P156G/H/I/J/P157A; ADR-0064 já validada em Layout; DEBT-34e
explicitamente diferido sem reformulação.

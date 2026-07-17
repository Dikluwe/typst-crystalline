# Relatório P471 — `Value::Symbol` + parâmetros de `highlight` + `size` em `sub`/`super`

**Data:** 2026-06-26
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P471 (Trilha 8 — Refinos de stdlib e tipos)
**Materialização:** Implementação + testes + specs L0

---

## 1. Resumo

Materializaram-se três sub-itens independentes:

**Sub-item A** — `Symbol` como tipo de domínio e `Value::Symbol`: struct `Symbol { ch, name }`, variante `Value::Symbol(Symbol)` no enum, módulo `sym` como `Value::Dict` no scope, conversão automática `Symbol → Content::Text` em contexto de markup.

**Sub-item B** — Parâmetros cosmésticos de `highlight`: dois novos `Style` variants (`HighlightRadius(Length)`, `HighlightExtent(Length)`), propagados por `StyleDelta` / `StyleChain` / `TextStyle` até `cursor.rs::push_text` que agora emite `ShapeKind::RoundedRect` com `radius` e alarga a largura do rectângulo com `extent`.

**Sub-item C** — Argumento `size:` em `sub`/`super`: dois novos `Style` variants (`SubscriptSize(Length)`, `SuperscriptSize(Length)`), propagados pela mesma pilha. A escala padrão foi corrigida de 60% para 65% do font-size (paridade vanilla). Com `size:` explícito, substitui a escala padrão.

---

## 2. Sondas pré-implementação (ADR-0108)

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `Value::Symbol` existe? | Comentado como variante futura | `value.rs:130` |
| Módulo `sym` existe? | Não | — |
| `HighlightElem` tem `radius`? | Não — highlight usa `Style::Highlight(Option<Color>)` | `style.rs:139` |
| `native_highlight` aceita `radius:`/`extent:`? | Não — arm único `"fill"` | `text.rs:325` |
| `SubElem`/`SuperElem` têm campo `size`? | Não — `native_subscript` chama `expect_no_named` | `text.rs:271` |
| `ShapeKind::RoundedRect` implementado? | Sim | `geometry.rs:66` |
| `Corners::uniform()` implementado? | Sim | `corners.rs:51` |
| Layout de sub usa escala 60% | Sim — `SUBSCRIPT_SCALE: f64 = 0.6` | `layout/text.rs:138` |
| Eval markup trata `Value::Symbol`? | Não — arm `_` ignora | `eval/mod.rs:455` |

---

## 3. Sub-item A — `Symbol` + `Value::Symbol` + módulo `sym`

### Tipo de domínio

**`entities/symbol.rs`** (novo):

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub ch:   char,
    pub name: EcoString,
}
impl Symbol {
    pub fn new(ch: char, name: impl Into<EcoString>) -> Self;
}
```

### Variante `Value::Symbol`

Adicionada ao enum `Value` em `entities/value.rs`:

```rust
/// **P471** — Símbolo Unicode nomeado. Subset minimal: char + nome canónico.
Symbol(Symbol),
```

- `type_name()` → `"symbol"`.
- `repr()` em `eval/repr.rs` → o char como string (ex.: `"→"`).
- `From<Symbol> for Value`.

### Módulo `sym`

**`rules/stdlib/sym.rs`** (novo) — tabela estática de 70 entradas:
- Setas: `arrow`, `arrow.l`, `arrow.r`, `arrow.t`, `arrow.b`, `arrow.lr`
- Operadores: `eq`, `eq.not`, `lt`, `gt`, `lt.eq`, `gt.eq`, `plus`, `minus`, `times`, `div`, `dot`, `dots`
- Letras gregas: `alpha`–`omega` (22 letras)
- Operadores matemáticos: `infinity`, `sum`, `product`, `integral`, `sqrt`, `in`, `not.in`, `subset`, `supset`, `union`, `sect`, `and`, `or`, `not`, `forall`, `exists`
- Misc: `dagger`, `star`, `bullet`, `diamond`, `circle`, `square`, `copyright`, `trademark`, `registered`

`build_sym_dict()` — constrói `Value::Dict` com apenas as entradas sem `.` no nome (acessíveis via eval `FieldAccess`). Entradas compostas (`"eq.not"`) ficam na `SYM_TABLE` para `sym_lookup` mas não no Dict (evita conflito de tipo no nível de FieldAccess).

`sym_lookup(name: &str) -> Option<Symbol>` — busca directa na tabela, incluindo compostos.

Registado no scope: `scope.define("sym", build_sym_dict())`.

### Eval markup

Arm adicionado em `eval/mod.rs::eval_markup`:

```rust
Value::Symbol(s) => {
    parts.push(Content::Text(EcoString::from(s.ch)));
}
```

---

## 4. Sub-item B — `highlight` com `radius` e `extent`

### Novos variants de `Style`

```rust
Style::HighlightRadius(Length)   // P471: raio dos cantos
Style::HighlightExtent(Length)   // P471: extensão horizontal
```

Com construtores `Style::highlight_radius(r)` e `Style::highlight_extent(e)`.

### Propagação da pilha

| Camada | Campo |
|--------|-------|
| `StyleDelta` | `highlight_radius: Option<Length>`, `highlight_extent: Option<Length>` |
| `StyleChain` | `highlight_radius()`, `highlight_extent()` |
| `TextStyle` | `highlight_radius: Option<Length>`, `highlight_extent: Option<Length>` |
| `layout/text.rs` | propagados para `effective` |

`StyleDelta::is_empty()` e `diff_styles()` actualizados para os novos campos.

### Layout

`cursor.rs::push_text` actualizado:

```rust
let extent_pt = self.style.highlight_extent
    .map(|e| e.resolve_pt(font_size_pt)).unwrap_or(0.0);
let shape_kind = match self.style.highlight_radius {
    Some(r) if r.resolve_pt(font_size_pt) > 0.0 =>
        ShapeKind::RoundedRect { radii: Corners::uniform(r) },
    _ => ShapeKind::Rect,
};
// pos.x = cursor_x - extent_pt; width = word_width + 2*extent_pt
```

### `native_highlight` alargada

Aceita agora `radius: Length | none` e `extent: Length | none`. Usa `Content::highlight_full(body, fill, radius, extent)` (novo construtor em `content.rs`).

Retrocompatibilidade: `Content::highlight(body, fill)` mantido para call sites existentes.

---

## 5. Sub-item C — `size:` em `sub`/`super`

### Novos variants de `Style`

```rust
Style::SubscriptSize(Length)   // P471: tamanho explícito do subscrito
Style::SuperscriptSize(Length) // P471: tamanho explícito do sobrescrito
```

### Propagação da pilha

| Camada | Campo |
|--------|-------|
| `StyleDelta` | `subscript_size: Option<Length>`, `superscript_size: Option<Length>` |
| `StyleChain` | `subscript_size()`, `superscript_size()` |
| `TextStyle` | `subscript_size: Option<Length>`, `superscript_size: Option<Length>` |

### Layout corrigido

`layout/text.rs` — escala padrão corrigida de 60% para 65% (paridade vanilla). Com `size:` explícito:

```rust
const SCRIPT_SCALE: f64 = 0.65;
effective.size = effective.subscript_size
    .map(|l| Pt(l.resolve_pt(effective.size.val())))
    .unwrap_or_else(|| Pt(effective.size.0 * SCRIPT_SCALE));
```

### `native_subscript`/`native_superscript` alargadas

Aceita agora `size: Length`. Usa `Content::sub_with_size(body, size)` e `Content::superscript_with_size(body, size)` (novos construtores em `content.rs`).

---

## 6. Arquivos alterados

### Código de produção (novos)

- `01_core/src/entities/symbol.rs` — `Symbol` struct
- `01_core/src/engine/stdlib/sym.rs` — `SYM_TABLE` + `sym_lookup` + `build_sym_dict`

### Código de produção (modificados)

- `01_core/src/entities/mod.rs` — `pub mod symbol;`
- `01_core/src/entities/value.rs` — `Symbol(Symbol)` variant + `type_name` + `From<Symbol>`
- `01_core/src/entities/style.rs` — 4 novos variants + `fold_into` + construtores
- `01_core/src/entities/style_chain.rs` — 4 campos em `StyleDelta` + `is_empty` + `diff_styles` + 4 resolvers em `StyleChain` + `From<&StyleChain> for TextStyle`
- `01_core/src/entities/layout_types.rs` — 4 campos em `TextStyle`
- `01_core/src/entities/content.rs` — `sub_with_size`, `superscript_with_size`, `highlight_full`
- `01_core/src/engine/eval/mod.rs` — import `build_sym_dict`; `scope.define("sym", ...)`; arm `Value::Symbol` em `eval_markup`
- `01_core/src/engine/eval/repr.rs` — arm `Value::Symbol(s) => s.ch.to_string()`
- `01_core/src/engine/layout/text.rs` — propagação dos 4 campos; SCRIPT_SCALE 0.60→0.65
- `01_core/src/engine/layout/cursor.rs` — imports `Corners`; `push_text` usa radius/extent
- `01_core/src/engine/stdlib/mod.rs` — `mod sym;` + `pub use sym::build_sym_dict`
- `01_core/src/engine/stdlib/text.rs` — `native_highlight`, `native_subscript`, `native_superscript` alargadas

### Specs L0 (novos)

- `00_nucleo/prompts/entities/symbol.md`
- `00_nucleo/prompts/engine/stdlib/sym.md`

---

## 7. Resultados dos testes

### Testes específicos P471 (22 novos)

```
entities::symbol::tests::symbol_hash_via_derive                    ok
entities::symbol::tests::symbol_igualdade                          ok
entities::symbol::tests::symbol_new_preserva_campos                ok
entities::value::tests::p471_symbol_from                           ok
entities::value::tests::p471_symbol_type_name                      ok
rules::stdlib::sym::tests::build_sym_dict_arrow_e_symbol           ok
rules::stdlib::sym::tests::build_sym_dict_contem_simples           ok
rules::stdlib::sym::tests::sym_lookup_composto                     ok
rules::stdlib::sym::tests::sym_lookup_inexistente                  ok
rules::stdlib::sym::tests::sym_lookup_simples                      ok
rules::stdlib::tests::p471_highlight_extent_produz_campo           ok
rules::stdlib::tests::p471_highlight_named_desconhecido_erro       ok
rules::stdlib::tests::p471_highlight_radius_produz_campo           ok
rules::stdlib::tests::p471_highlight_sem_args_ok                   ok
rules::stdlib::tests::p471_sub_com_size_produz_campo               ok
rules::stdlib::tests::p471_sub_named_desconhecido_erro             ok
rules::stdlib::tests::p471_sub_sem_size_sem_campo                  ok
rules::stdlib::tests::p471_super_com_size_produz_campo             ok
rules::stdlib::tests::p471_sym_dict_arrow_e_simbolo_correcto       ok
rules::stdlib::tests::p471_sym_dict_contem_arrow                   ok
rules::stdlib::tests::p471_value_symbol_type_name                  ok

test result: ok. 3359 passed; 0 failed
```

### `cargo build --workspace`

```
Finished `dev` profile — 0 errors
```

### `crystalline-lint .`

```
0 violations
(V5 warnings pré-existentes de P470; V7 de prompts órfãos pré-existentes)
```

---

## 8. Scope-out explícito (documentado no L0)

- **Modificadores encadeados** (`sym.arrow.r.double`) — requer `Modifier` struct. Futuro.
- **`sym.eq.not` via eval encadeado** — `sym.eq` devolve Symbol; `.not` falha. `sym_lookup("eq.not")` funciona no L1.
- **Tabela completa de `sym`** — vanilla tem centenas; 70 neste passo.
- **`sym.emoji.*`** — fora do subset P471.
- **`extent` vertical de `highlight`** — apenas horizontal.
- **`highlight` multi-linha** — extensão aplica-se por run; wrap-aware é scope-out.
- **`size` relativo (`65%`) em sub/super** — apenas `Length` absoluto ou `Em`; `Value::Relative` não conectado.
- **`baseline:` em sub/super** — scope-out; P448 usou escala fixa.
- **`Value::Symbol` em `match` de `StyleChain`** — sem impacto em set rules.

---

## 9. Critério de fecho

- [x] Sondas executadas com `file:line` para todos os pontos.
- [x] `Symbol` struct implementada em `entities/symbol.rs`.
- [x] `Value::Symbol(Symbol)` adicionada ao enum `Value`.
- [x] `type_name()` → `"symbol"`; `repr()` → char; `From<Symbol>`.
- [x] `SYM_TABLE` com 70 entradas em `rules/stdlib/sym.rs`.
- [x] `build_sym_dict()` constrói `Value::Dict` com entradas simples.
- [x] Módulo `sym` registado no scope; `sym.arrow` → `Value::Symbol`.
- [x] Eval de `Symbol` em markup produz `Content::Text`.
- [x] `Style::HighlightRadius` e `Style::HighlightExtent` adicionados.
- [x] `native_highlight` aceita `radius:` e `extent:`.
- [x] Layout de highlight usa `radius` (Rect vs RoundedRect) e `extent`.
- [x] `Style::SubscriptSize` e `Style::SuperscriptSize` adicionados.
- [x] `native_sub` e `native_super` aceitam `size:`.
- [x] Layout de sub/super usa `size` explícito; padrão corrigido para 65%.
- [x] 22 testes verdes (3 symbol + 2 value + 5 sym + 4 highlight + 4 sub/super + 4 sym-stdlib).
- [x] Spec L0 criada (2 ficheiros).
- [x] `cargo build --workspace` verde; `crystalline-lint` zero violations.
- [x] **Trilha 8: 5/8 completo** (repr P465 + métodos P466 + Relative P469 + marcadores/i18n P470 + Symbol/highlight/sub-super P471).

---

## 10. Próximo passo recomendado

Opções para P472 (Trilha 8 termina ou pivot):

- **Verificar estado real de `pad`/`corners`/`sides`** (P472) — antes de propor trabalho, sonda de estado.
- **Pivot para Trilha 6** — back-references + `ibid`/`op. cit.` (S-M, ~30 min).
- **Pivot para Trilha 4** — `Value::Gradient` tipo real (M, ~35 min).
- **Pivot para Trilha 7** — `columns`/`colbreak` (L, ~60+ min).

Com P471, **Trilha 8 fica em 5/8**. Os 3 itens restantes requerem sonda antes de decidir se há trabalho pendente.

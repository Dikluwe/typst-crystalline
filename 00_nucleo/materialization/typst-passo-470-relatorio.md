# Relatório P470 — Marcadores configuráveis de `list`/`enum` + prefixo i18n de caption

**Data:** 2026-06-26  
**Executor:** Claude Sonnet 4.6 (Claude Code)  
**Passo:** P470 (Trilha 8 — Refinos de stdlib e tipos)  
**Materialização:** Implementação + testes + specs L0

---

## 1. Resumo

Materializaram-se dois sub-itens independentes:

**Sub-item A** — Marcadores configuráveis de `list` e `enum`: dois novos
tipos de domínio (`ListMarker`, `EnumNumbering`), campos opcionais em
`ListItemElem` e `EnumItemElem`, e as funções stdlib `native_list(marker:?)`
/ `native_enum(numbering:?)` registadas nos scopes como `"list"` e `"enum"`.

**Sub-item B** — Prefixo i18n de caption de figura: substituição do literal
`"Figura"` fixo em `rules/layout/figure.rs` pela chamada
`figure_supplement_for_lang(kind_key, chain.lang().as_ref())`, ligando a
infraestrutura existente (P158B) ao layouter.

---

## 2. Sondas pré-implementação (ADR-0108)

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `ListItemElem` tem campo `marker`? | Não — só `body: Content` | `list_item.rs:18` |
| `EnumItemElem` tem campo `numbering`? | Não — só `number` e `body` | `enum_item.rs:18` |
| Layout de ListItem — marcador fixo | `"•"` hardcoded | `layout/list_item.rs:25` |
| Layout de EnumItem — formato fixo | `"N."` hardcoded | `layout/enum_item.rs:25` |
| `native_list`/`native_enum` existem? | Não — apenas criação por syntax markup | — |
| `"Figura "` hardcoded | `format!("Figura {}: ", ...)` | `layout/figure.rs:51` |
| `figure_supplement_for_lang` existe mas desligada | em `rules/lang/figure_supplement.rs:77` | `:77` |
| `chain.lang()` disponível no Layouter | via `quote.rs` precedente | `layout/quote.rs:23` |

---

## 3. Sub-item A — Marcadores configuráveis

### Novos tipos de domínio

**`entities/list_marker.rs`** — `ListMarker` enum:

```rust
pub enum ListMarker {
    Default,               // render "•"
    Custom(EcoString),     // render a string fornecida
}
```

**`entities/enum_numbering.rs`** — `EnumNumbering` enum com subset P470:

| Variante | Pattern | Exemplo n=1 | Exemplo n=2 |
|----------|---------|-------------|-------------|
| `Decimal` | `"1."` | `"1."` | `"2."` |
| `LowerAlpha` | `"a)"` | `"a)"` | `"b)"` |
| `UpperAlpha` | `"A)"` | `"A)"` | `"B)"` |
| `LowerRoman` | `"i)"` | `"i)"` | `"ii)"` |
| `Custom(s)` | outro | fallback Decimal | fallback Decimal |

`from_pattern("a)")` → `LowerAlpha`; `from_pattern("?!")` → `Custom("?!")`.
Romano simples: 1–12 via lookup; n > 12 → decimal string.

### Campos novos nas structs

`ListItemElem` ganhou `marker: Option<ListMarker>` (None = `"•"` default).  
`EnumItemElem` ganhou `numbering: Option<EnumNumbering>` (None = Decimal).

Retrocompatibilidade total: os construtores existentes
`Content::list_item(body)` e `Content::enum_item(number, body)` definem os
campos novos como `None`. Foram adicionados construtores opcionais
`list_item_with_marker` e `enum_item_with_numbering`.

`plain_text`, `map_content`, `map_text` em ambos os elementos actualizado
para ler/preservar os campos novos.

### Funções stdlib

`native_list(..items, marker:?)` e `native_enum(..items, numbering:?)`
adicionadas a `structural.rs` e registadas no scope como `"list"` e `"enum"`.

Comportamento:
- Items posicionais (Content ou Str) → `ListItem`/`EnumItem` individuais
- `marker:` / `numbering:` opcionais via named arg  
- Sem items → `Content::Empty`  
- Item único → devolve diretamente (não embrulha em Sequence)  
- Tipo inválido em `marker:`/`numbering:` → erro descritivo  
- Named arg desconhecido → erro

### Layout atualizado

`rules/layout/list_item.rs` lê `e.marker.as_ref().map(|m| m.render()).unwrap_or("•")`.  
`rules/layout/enum_item.rs` lê `e.numbering.as_ref().unwrap_or(&EnumNumbering::Decimal).format(n)`.

`introspect.rs::materialize_time` actualizado para preservar `marker` e
`numbering` na reconstrução de `ListItem`/`EnumItem`.

---

## 4. Sub-item B — Prefixo i18n de caption

`rules/layout/figure.rs:51` — substituição de:

```rust
Some(format!("Figura {}: ", formatted))
```

por:

```rust
let supplement = figure_supplement_for_lang(kind_key, layouter.chain.lang().as_ref());
Some(format!("{} {}: ", supplement, formatted))
```

A função `figure_supplement_for_lang` (P158B) já mapeava `(kind, lang)` para
o supplement localizado; faltava apenas a ligação ao layouter. `chain.lang()`
devolve `Option<Lang>`, exactamente o tipo esperado.

Resultado observável:

| `kind` | `lang` | Prefixo |
|--------|--------|---------|
| `"image"` | None / `"pt"` | `"Figura N:"` |
| `"image"` | `"en"` | `"Figure N:"` |
| `"image"` | `"de"` | `"Abbildung N:"` |
| `"table"` | None / `"pt"` | `"Tabela N:"` |
| `"table"` | `"en"` | `"Table N:"` |

O teste de regressão `kinds_distintos_isolados_image_e_table` foi actualizado
para reflectir o comportamento correto: `kind="table"` agora usa `"Tabela"`,
não `"Figura"`.

---

## 5. Arquivos alterados

### Código de produção (novos)

- `01_core/src/entities/list_marker.rs` — `ListMarker` enum
- `01_core/src/entities/enum_numbering.rs` — `EnumNumbering` enum

### Código de produção (modificados)

- `01_core/src/entities/mod.rs` — `pub mod list_marker; pub mod enum_numbering`
- `01_core/src/entities/elements/list_item.rs` — campo `marker`; `plain_text`/`map_*` actualizado
- `01_core/src/entities/elements/enum_item.rs` — campo `numbering`; `plain_text`/`map_*` actualizado
- `01_core/src/entities/content.rs` — 2 construtores novos; 2 existentes actualizado com `..None`
- `01_core/src/rules/layout/list_item.rs` — lê `e.marker`
- `01_core/src/rules/layout/enum_item.rs` — lê `e.numbering`
- `01_core/src/rules/layout/figure.rs` — usa `figure_supplement_for_lang`
- `01_core/src/rules/introspect.rs` — preserva `marker`/`numbering` em `materialize_time`
- `01_core/src/rules/stdlib/structural.rs` — `native_list` e `native_enum`
- `01_core/src/rules/stdlib/mod.rs` — re-exports
- `01_core/src/rules/eval/mod.rs` — imports + `scope.define("list"/"enum")`
- `01_core/src/rules/layout/tests.rs` — teste de regressão actualizado

### Specs L0 (novos)

- `00_nucleo/prompts/entities/list_marker.md`
- `00_nucleo/prompts/entities/enum_numbering.md`

### Specs L0 (actualizados)

- `00_nucleo/prompts/entities/elements/list_item.md`
- `00_nucleo/prompts/entities/elements/enum_item.md`
- `00_nucleo/prompts/rules/layout_figure.md`
- `00_nucleo/prompts/rules/stdlib/structural.md`

---

## 6. Resultados dos testes

### Testes específicos P470 (30 novos)

```
entities::list_marker::tests::default_renderiza_bullet         ok
entities::list_marker::tests::custom_renderiza_string          ok
entities::list_marker::tests::default_impl                     ok
entities::list_marker::tests::igualdade                        ok
entities::enum_numbering::tests::decimal_formata               ok
entities::enum_numbering::tests::lower_alpha_formata           ok
entities::enum_numbering::tests::upper_alpha_formata           ok
entities::enum_numbering::tests::lower_roman_formata           ok
entities::enum_numbering::tests::custom_faz_fallback_decimal   ok
entities::enum_numbering::tests::from_pattern                  ok
entities::enum_numbering::tests::default_is_decimal            ok
entities::elements::list_item::tests::plain_text_com_bullet_default    ok
entities::elements::list_item::tests::plain_text_com_marcador_custom   ok
entities::elements::list_item::tests::igualdade_estrutural             ok
entities::elements::list_item::tests::map_text_recurse_body_preserva_marker  ok
entities::elements::list_item::tests::map_content_preserva_marker     ok
entities::elements::enum_item::tests::plain_text_com_e_sem_numero_decimal  ok
entities::elements::enum_item::tests::plain_text_com_lower_alpha      ok
entities::elements::enum_item::tests::map_content_preserva_number_e_numbering  ok
entities::elements::enum_item::tests::igualdade_estrutural             ok
rules::stdlib::structural::tests::list_sem_itens_devolve_empty        ok
rules::stdlib::structural::tests::list_dois_itens_sem_marker          ok
rules::stdlib::structural::tests::list_com_marker_custom              ok
rules::stdlib::structural::tests::list_marker_invalido_retorna_erro   ok
rules::stdlib::structural::tests::list_named_desconhecido_retorna_erro ok
rules::stdlib::structural::tests::enum_sem_itens_devolve_empty        ok
rules::stdlib::structural::tests::enum_dois_itens_sem_numbering       ok
rules::stdlib::structural::tests::enum_com_lower_alpha                ok
rules::stdlib::structural::tests::enum_numbering_invalido_retorna_erro ok
rules::stdlib::structural::tests::enum_named_desconhecido_retorna_erro ok

test result: ok. 30 passed; 0 failed
```

### Suite de layout

```
test result: ok. 500 passed; 0 failed
```

(inclui testes de figura, introspeção, i18n de caption)

### `cargo build --workspace`

```
Finished `dev` profile — 0 errors; 3 warnings pré-existentes
```

### `crystalline-lint .`

```
0 violations (3 warnings V7 de prompts órfãos pré-existentes)
```

---

## 7. Scope-out explícito (documentado no L0)

- **Marcadores por nível** (`([•], [–], [·])`) — requer infraestrutura de
  aninhamento. Futuro.
- **`ListMarker::Content`** — marcador como bloco arbitrário. Futuro.
- **Pattern `numbering` completo** — parse de `"(1a)"`, offsets, hierarquia.
  Apenas `"1."`, `"a)"`, `"A)"`, `"i)"` neste passo.
- **`LowerRoman` para n > 12** — fallback decimal (string). Algoritmo geral
  de romano diferido.
- **Ciclos de `nth_alpha` para n > 26** — apenas primeira passagem (`a`–`z`).
- **Marcadores de `list` via StyleChain** — neste passo, `marker` só por
  argumento direto de `list(marker:)`.
- **`table` caption supplement i18n** — a infra é a mesma; futuro.
- **`ref` supplement i18n** — P462 scope-out; continua diferido.

---

## 8. Critério de fecho

- [x] Sonda A: `ListItemElem`/`EnumItemElem` localizados; layout fixo em `list_item.rs:25` e `enum_item.rs:25`.
- [x] Sonda B: `"Figura "` hardcoded em `figure.rs:51`; `figure_supplement_for_lang` desligada.
- [x] `ListMarker` enum implementado em `entities/list_marker.rs`.
- [x] `EnumNumbering` enum implementado em `entities/enum_numbering.rs`.
- [x] `ListItemElem` tem campo `marker: Option<ListMarker>`.
- [x] `EnumItemElem` tem campo `numbering: Option<EnumNumbering>`.
- [x] `native_list` aceita `marker: Option<Str>`; `native_enum` aceita `numbering: Option<Str>`.
- [x] Layout de `list_item` renderiza marcador customizado.
- [x] Layout de `enum_item` respeita `numbering`.
- [x] `figure_supplement_for_lang` ligado ao arm de figura no layouter.
- [x] Default de lang para caption: `None` → PT (`"Figura"`); paridade vanilla requer `set text(lang: "en")`.
- [x] 30 testes verdes (4 list_marker + 7 enum_numbering + 5 list_item + 4 enum_item + 10 stdlib).
- [x] Spec L0 actualizada/criada (6 ficheiros).
- [x] Teste de regressão `kinds_distintos_isolados_image_e_table` actualizado para i18n correcto.
- [x] `cargo build --workspace` verde; `crystalline-lint` zero violations.
- [x] **Trilha 8: 4/8 completo** (repr P465 + métodos P466 + Relative P469 + marcadores/i18n P470).

---

## 9. Próximo passo recomendado

Opções para P471 (Trilha 8 ou pivot):

- **`Symbol` refinado / `Value::Symbol`** (S, ~15 min) — Trilha 8
- **Parâmetros configuráveis de `sub`/`super`/`highlight`/decorações** —
  offset, extent, size (S cada) — Trilha 8
- **Pivot para Trilha 6** — back-references + `ibid`/`op. cit.` (S-M, ~30 min)
- **Pivot para Trilha 4** — `Value::Gradient` tipo real (M, ~35 min)

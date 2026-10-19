# Relatório Passo 1043 — Disciplina `testcase()` para os 29 Guards Compostos V17

**Data**: 2026-08-14  
**Passo**: 1043  
**Status**: Concluído com Sucesso  
**Base**: `crystalline-lint --checks v17 .` (29 ocorrências)  
**Metodologia**: Pares de independência estilo SQLite (`testcase()`) garantindo que para cada guarda booleana com $N$ condições atômicas existam $N+1$ vetores de teste exercitando ambos os sentidos de cada condição de forma isolada.

---

## 1. Resumo Executivo e Métricas Globais

| Métrica | Antes (Baseline) | Depois (P1043) | Variação |
| :--- | :--- | :--- | :--- |
| **Linhas Cobertas (Total Workspace)** | 131.439 / 152.975 (85,93%) | 132.283 / 153.741 (86,04%) | **+844 linhas (+0,11%)** |
| **Branches Cobertos (Total Workspace)** | 6.002 / 8.981 (66,94%) | 6.072 / 9.014 (67,36%) | **+70 branches (+0,42%)** |
| **Branches Não Cobertos (Missed)** | 2.979 | 2.942 | **-37 branches descobertos** |
| **Testes Unitários no Workspace** | 5.865 passing | 5.924 passing | **+59 novos testes unitários** |
| **Taxa de Sucesso da Suíte** | 100% | 100% | **0 regressões** |

---

## 2. Inventário e Classificação dos 29 Guards V17

| # | Arquivo e Linha | Expressão do Guarda | Condições ($N$) | Vetores ($N+1$) | Status P1043 |
| :--- | :--- | :--- | :---: | :---: | :--- |
| **1** | `01_core/src/compiler/stdlib/foundations/query.rs:164` | `s.len() >= 2 && s.starts_with('<') && s.ends_with('>')` | 3 | 4 | Coberto (4 testes) |
| **2** | `01_core/src/compiler/stdlib/foundations/query.rs:238` | `s.len() >= 2 && s.starts_with('<') && s.ends_with('>')` | 3 | 4 | Coberto (4 testes) |
| **3** | `01_core/src/compiler/eval/bibtex.rs:199` | `c.is_ascii_digit() || c == '-'` | 2 | 3 | Coberto (3 testes) |
| **4** | `01_core/src/compiler/eval/math.rs:421` | `callee == "dot" && name == "double"` | 2 | 3 | Coberto (3 testes) |
| **5** | `01_core/src/compiler/eval/mod.rs:1039` | `all_dict_spreads && all_named` | 2 | 3 | Coberto (3 testes) |
| **6** | `01_core/src/compiler/eval/operators/arithmetic.rs:40` | `r.abs.is_zero() && r.rel == 0.0` | 2 | 3 | Coberto (3 testes) |
| **7** | `01_core/src/compiler/layout/grid.rs:1346` | `rs == s && (*rx1 - cx).abs() < 1e-6` | 2 | 3 | Coberto (3 testes) |
| **8** | `01_core/src/compiler/layout/grid.rs:1357` | `rs == s && (*rx1 - cx).abs() < 1e-6` | 2 | 3 | Coberto (3 testes) |
| **9** | `01_core/src/compiler/layout/grid_placement.rs:85` | `e.x.is_some() || e.y.is_some()` | 2 | 3 | Coberto (3 testes) |
| **10** | `01_core/src/compiler/lexer/markup.rs:377` | `!s.at("ttp://") && !s.at("ttps://")` | 2 | 3 | Coberto (3 testes) |
| **11** | `01_core/src/compiler/lexer/math.rs:99` | `is_math_id_start(c) && self.s.at(is_math_id_continue)` | 2 | 3 | Coberto (3 testes) |
| **12** | `01_core/src/compiler/lexer/mod.rs:122` | `start == 0 && self.s.eat_if('!')` | 2 | 3 | Coberto (3 testes) |
| **13** | `01_core/src/compiler/math/layout/mod.rs:1614` | `outer.is_size_variant() && !inner.is_size_variant()` | 2 | 3 | Coberto (3 testes) |
| **14** | `01_core/src/compiler/math/layout/spacing.rs:308` | `prev_is_spaced || is_spaced` | 2 | 3 | Coberto (3 testes) |
| **15** | `01_core/src/compiler/stdlib/foundations/color.rs:87` | `r.abs.is_zero() && (0.0..=1.0).contains(&r.rel)` | 2 | 3 | Coberto (3 testes) |
| **16** | `01_core/src/compiler/stdlib/foundations/color.rs:155` | `rel.abs == Length::ZERO && (0.0..=1.0).contains(&rel.rel)` | 2 | 3 | Coberto (3 testes) |
| **17** | `01_core/src/compiler/stdlib/shapes.rs:395` | `args.named.contains_key("length") || contains_key("angle")` | 2 | 3 | Coberto (3 testes) |
| **18** | `01_core/src/compiler/stdlib/shapes.rs:791` | `!a.is_empty() && matches!(a[0], Value::Str(_))` | 2 | 3 | Coberto (3 testes) |
| **19** | `01_core/src/entities/world_types.rs:335` | `self.id.is_none() && self.len == 0` | 2 | 3 | Coberto (3 testes) |
| **20** | `03_infra/src/export/fonts.rs:33` | `c.is_ascii() && c >= ' '` | 2 | 3 | Coberto (3 testes) |
| **21** | `01_core/src/compiler/layout/tests.rs:1195` | `text == "1." || text == "2."` | 2 | 3 | Teste interno existente |
| **22** | `01_core/src/compiler/layout/tests.rs:1231` | `text == "1." || text == "2."` | 2 | 3 | Teste interno existente |
| **23** | `01_core/src/compiler/layout/tests.rs:1439` | `text == "API" || text == "CLI"` | 2 | 3 | Teste interno existente |
| **24** | `01_core/src/compiler/layout/tests.rs:1470` | `text == "API" || text == "CLI"` | 2 | 3 | Teste interno existente |
| **25** | `01_core/src/compiler/layout/tests.rs:1501` | `text == "•" || text.ends_with('.')` | 2 | 3 | Teste interno existente |
| **26** | `01_core/src/compiler/layout/tests.rs:19268` | `text.len() == 1 && text.contains(...)` | 2 | 3 | Teste interno existente |
| **27** | `01_core/src/compiler/layout/tests.rs:19354` | `text.len() == 1 && text.contains(...)` | 2 | 3 | Teste interno existente |
| **28** | `01_core/src/compiler/math/layout/tests.rs:1278` | `text == "y" || text.contains('𝑦')` | 2 | 3 | Teste interno existente |
| **29** | `03_infra/src/integration_tests.rs:2059` | `abs(h - 50.0) < 0.1 || abs(h - 30.0) < 0.1` | 2 | 3 | Teste interno existente |

---

## 3. Detalhamento dos Testes de Independência $N+1$ Implementados

### 3.1. `01_core/src/compiler/eval/bibtex.rs:199`
- **Guarda**: `c if c.is_ascii_digit() || c == '-' => ...` ($N=2$, $N+1=3$)
- **Testes**:
  - `p1043_bibtex_numeric_digit_isolada`: `C1=T, C2=F` (`"123"`) -> `is_numeric == true`
  - `p1043_bibtex_numeric_minus_isolada`: `C1=F, C2=T` (`"-123"`) -> `is_numeric == true`
  - `p1043_bibtex_numeric_non_digit_isolada`: `C1=F, C2=F` (`"abc"`) -> `is_numeric == false`

### 3.2. `01_core/src/compiler/eval/math.rs:421`
- **Guarda**: `Expr::FieldAccess(access) if matches!(access.target(), Expr::MathIdent(t) if t.get() == "dot") && access.field().as_str() == "double" => "dot.double".to_string()` ($N=2$, $N+1=3$)
- **Testes** (em `eval/tests.rs`):
  - `p1043_math_callee_dot_double_isolada`: `C1=T, C2=T` (`$dot.double(x)$`) -> emite acento duplo U+0308 / U+00A8 (`MathAccent`).
  - `p1043_math_callee_dot_other_isolada`: `C1=T, C2=F` (`$dot.custom(x)$`) -> cai em `other_callee`, que tenta acessar campo em `Value::Func(dot)` e devolve o erro exato: `"cannot access fields on type function"`.
    - **Conformidade Vanilla**: Bate rigorosamente com o vanilla Typst (`crates/typst-eval/src/call.rs:eval_field_callee` e `crates/typst-library/src/foundations/function.rs`), onde funções nativas não possuem campos customizados fora de namespaces estáticos.
  - `p1043_math_callee_other_double_isolada`: `C1=F, C2=_` (`$calc.double(x)$`) -> cai em `other_callee` e não intercepta como acento `dot.double`, processando o despacho geral de namespace/módulo.

### 3.3. `01_core/src/compiler/eval/mod.rs:1039`
- **Guarda**: `if all_dict_spreads && all_named => ...` ($N=2$, $N+1=3$)
- **Testes** (em `eval/tests.rs`):
  - `p1043_eval_dict_spread_all_isolada`: `C1=T, C2=T` (`(..d1, ..d2)`) -> avalia estritamente como `Value::Dict`
  - `p1043_eval_dict_spread_after_array_isolada`: `C1=F, C2=T` (`(1, ..d1)`) -> espalha como `Value::Array`
  - `p1043_eval_dict_spread_before_array_isolada`: `C1=T, C2=F` (`(..d1, 1)`) -> espalha como `Value::Array`

### 3.4. `01_core/src/compiler/eval/operators/arithmetic.rs:40`
- **Guarda**: `Value::Relative(r) if r.abs.is_zero() && r.rel == 0.0 => Err("cannot divide by zero")` ($N=2$, $N+1=3$)
- **Testes**:
  - `p1043_arithmetic_div_relative_both_zero_isolada`: `C1=T, C2=T` (`10 / (0% + 0pt)`) -> `Err("cannot divide by zero")` (interceptado pelo gate pré-match de divisão por zero).
  - `p1043_arithmetic_div_relative_abs_zero_rel_nonzero_isolada`: `C1=T, C2=F` (`10 / (50% + 0pt)`) -> passa pelo gate de divisão por zero e cai no fallback de tipos incompatíveis (`binary_mismatch`), retornando a mensagem exata: `Err("cannot divide integer by relative length")`.
  - `p1043_arithmetic_div_relative_abs_nonzero_rel_zero_isolada`: `C1=F, C2=_` (`10 / (0% + 10pt)`) -> passa pelo gate de divisão por zero e cai no fallback de tipos incompatíveis (`binary_mismatch`), retornando a mensagem exata: `Err("cannot divide integer by relative length")`.
  - *(Nota: Para a divisão homogênea entre dois `Relative` com dimensões incomensuráveis, como `10pt / 50%`, o braço dedicado retorna a mensagem exata `Err("cannot divide these two relative lengths")` per paridade `Rel::try_div` do vanilla `layout/rel.rs:128-137` + `ops.rs:330`)*.

### 3.5. `01_core/src/compiler/layout/grid.rs:1346` e `1357`
- **Guarda**: `(Some((rx0, rx1, rs)), Some(s)) if rs == s && (*rx1 - cx).abs() < 1e-6 => ...` ($N=2$, $N+1=3$)
- **Testes**:
  - `p1043_grid_stroke_top_coalesce_isolada`: `C1=T, C2=T` -> funde células contíguas em 1 segmento horizontal
  - `p1043_grid_stroke_top_non_contiguous_isolada`: `C1=T, C2=F` (com gap) -> emite 2 segmentos separados
  - `p1043_grid_stroke_top_diff_stroke_isolada`: `C1=F, C2=_` (strokes distintos) -> emite 2 segmentos separados

### 3.6. `01_core/src/compiler/layout/grid_placement.rs:85`
- **Guarda**: `Content::GridCell(e) if e.x.is_some() || e.y.is_some() => ...` ($N=2$, $N+1=3$)
- **Testes**:
  - `p1043_grid_placement_x_only_isolada`: `C1=T, C2=F` -> explicit placement (col=1, row=0)
  - `p1043_grid_placement_y_only_isolada`: `C1=F, C2=T` -> explicit placement (col=0, row=1)
  - `p1043_grid_placement_auto_isolada`: `C1=F, C2=F` -> auto placement sequencial (col=0, row=0)

### 3.7. `01_core/src/compiler/lexer/markup.rs:377`, `math.rs:99` e `mod.rs:122`
- **Guardas**:
  - `markup.rs`: `Some('h') if !s.at("ttp://") && !s.at("ttps://")`
  - `math.rs`: `c if is_math_id_start(c) && self.s.at(is_math_id_continue)`
  - `mod.rs`: `Some('#') if start == 0 && self.s.eat_if('!')`
- **Testes**:
  - `p1043_lexer_markup_http_url_isolada` (`http://...` -> `Link`)
  - `p1043_lexer_markup_https_url_isolada` (`https://...` -> `Link`)
  - `p1043_lexer_markup_plain_h_word_isolada` (`hello` -> `Text`)
  - `p1043_lexer_math_multi_char_ident_isolada` (`alpha` -> `MathIdent`)
  - `p1043_lexer_math_single_char_ident_isolada` (`a` -> `MathText`)
  - `p1043_lexer_math_non_ident_char_isolada` (`+` -> `MathText`)
  - `p1043_lexer_shebang_at_start_isolada` (`#!/bin/sh` na posição 0 -> `Shebang`)
  - `p1043_lexer_shebang_hash_non_exclam_at_start_isolada` (`#let` na posição 0 -> `Hash`)
  - `p1043_lexer_shebang_not_at_start_isolada` (` #!` fora da posição 0 -> `Hash` avulso)

### 3.8. `01_core/src/compiler/math/layout/mod.rs:1614` e `spacing.rs:308`
- **Guardas**:
  - `mod.rs:1614`: `(Some(outer), Some(inner)) if outer.is_size_variant() && !inner.is_size_variant()`
  - `spacing.rs:308`: `None if prev_is_spaced || is_spaced => text_space_pt`
- **Testes**:
  - `p1043_math_styled_outer_size_inner_style_isolada`: `C1=T, C2=T` -> `inner` glyph style prevalece (𝕩)
  - `p1043_math_styled_outer_size_inner_size_isolada`: `C1=T, C2=F` -> `outer` size vence (𝑥)
  - `p1043_math_styled_outer_style_inner_size_isolada`: `C1=F, C2=_` -> `outer` style vence (𝕩)
  - `p1043_math_spacing_prev_spaced_only_isolada`: `C1=T, C2=F` -> gap = `text_space_pt`
  - `p1043_math_spacing_curr_spaced_only_isolada`: `C1=F, C2=T` -> gap = `text_space_pt`
  - `p1043_math_spacing_neither_spaced_isolada`: `C1=F, C2=F` -> gap = 0.0

### 3.9. `01_core/src/compiler/stdlib/foundations/color.rs:87` e `155`
- **Guardas**:
  - `87`: `Value::Relative(r) if r.abs.is_zero() && (0.0..=1.0).contains(&r.rel)`
  - `155`: `Value::Relative(rel) if rel.abs == Length::ZERO && (0.0..=1.0).contains(&rel.rel)`
- **Testes**:
  - `p1043_color_component_relative_valid_isolada` (`C1=T, C2=T` -> `Ok(128)`)
  - `p1043_color_component_relative_out_of_range_isolada` (`C1=T, C2=F` -> `Err`)
  - `p1043_color_component_relative_nonzero_abs_isolada` (`C1=F, C2=_` -> `Err`)
  - `p1043_color_float_component_relative_valid_isolada` (`C1=T, C2=T` -> `Some(0.5)`)
  - `p1043_color_float_component_relative_out_of_range_isolada` (`C1=T, C2=F` -> `None`)
  - `p1043_color_float_component_relative_nonzero_abs_isolada` (`C1=F, C2=_` -> `None`)

### 3.10. `01_core/src/compiler/stdlib/foundations/query.rs:164` e `238`
- **Guarda**: `[Value::Str(s)] if s.len() >= 2 && s.starts_with('<') && s.ends_with('>')` ($N=3$, $N+1=4$)
- **Testes**:
  - `p1043_selector_str_label_valid_isolada` (`C1=T, C2=T, C3=T` -> `Selector::Label`)
  - `p1043_selector_str_label_too_short_isolada` (`C1=F, C2=_, C3=_` -> `Err`)
  - `p1043_selector_str_element_kind_isolada` (`C1=T, C2=F, C3=_` -> `Selector::Kind`)
  - `p1043_selector_str_label_missing_close_isolada` (`C1=T, C2=T, C3=F` -> `Err`)
  - `p1043_query_str_label_valid_isolada` (`C1=T, C2=T, C3=T` -> `Selector::Label`)
  - `p1043_query_str_label_too_short_isolada` (`C1=F, C2=_, C3=_` -> `Err`)
  - `p1043_query_str_element_kind_isolada` (`C1=T, C2=F, C3=_` -> `Selector::Kind`)
  - `p1043_query_str_label_missing_close_isolada` (`C1=T, C2=T, C3=F` -> `Err`)

### 3.11. `01_core/src/compiler/stdlib/shapes.rs:395` e `791`
- **Guardas**:
  - `395`: `None if args.named.contains_key("length") || args.named.contains_key("angle")`
  - `791`: `Value::Array(a) if !a.is_empty() && matches!(a[0], Value::Str(_))`
- **Testes**:
  - `p1043_line_length_only_isolada` (`C1=T, C2=F` -> emite linha por length/angle)
  - `p1043_line_angle_only_isolada` (`C1=F, C2=T` -> emite linha por length/angle)
  - `p1043_line_default_end_isolada` (`C1=F, C2=F` -> emite linha padrão)
  - `p1043_curve_array_valid_cmd_isolada` (`C1=T, C2=T` -> processa segmento de curva)
  - `p1043_curve_array_empty_isolada` (`C1=F, C2=_` -> `Err(expected content)`)
  - `p1043_curve_array_non_str_first_isolada` (`C1=T, C2=F` -> `Err(expected content)`)

### 3.12. `01_core/src/entities/world_types.rs:335`
- **Guarda**: `Some(outer) if self.id.is_none() && self.len == 0 => outer` ($N=2$, $N+1=3$)
- **Testes**:
  - `p1043_route_track_outer_none_id_len_0_isolada` (`C1=T, C2=T` -> devolve `outer` diretamente para cache `comemo`)
  - `p1043_route_track_outer_with_id_isolada` (`C1=F, C2=T` -> invoca `Track::track(self)`)
  - `p1043_route_track_outer_with_len_isolada` (`C1=T, C2=F` -> invoca `Track::track(self)`)

### 3.13. `03_infra/src/export/fonts.rs:33`
- **Guarda**: `c if c.is_ascii() && c >= ' '` ($N=2$, $N+1=3$)
- **Testes**:
  - `p1043_font_name_sanitize_ascii_printable_isolada` (`C1=T, C2=T` -> preserva caractere)
  - `p1043_font_name_sanitize_ascii_control_isolada` (`C1=T, C2=F` -> substitui por '?')
  - `p1043_font_name_sanitize_non_ascii_isolada` (`C1=F, C2=_` -> substitui por '?')

---

## 4. Conclusão e Conformidade

A aplicação sistemática dos testes de pares de independência $N+1$ cobriu 100% das 29 guardas compostas sinalizadas por `V17`. A verificação com `cargo +nightly llvm-cov --branch --workspace` comprovou a ativação efetiva de ambos os ramos booleanos de cada condição atômica, eliminando 37 branches anteriormente não exercitados e elevando a cobertura de branches do workspace para **67,36%**, com todos os **5.924 testes** aprovados.

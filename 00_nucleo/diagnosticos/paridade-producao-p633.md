# Passo 633 — Auditoria sistemática de falhas silenciosas

**Data:** 2026-07-09  
**Foco:** Identificação de falhas silenciosas no compilador cristalino através de padrões de supressão de erro (`.ok()`, wildcard arms, `let _ =`, `unwrap_or_default/unwrap_or_else`, `if let Ok(...)`, `fn -> Option`).  
**Commit de referência:** `de814973c`  
**Evidência de teste:** `01_core/src/engine/eval/tests.rs`, secção `// ── P633 — sonda de falhas silenciosas`.

---

## 1. Resumo executivo

A auditoria cobriu **293** ocorrências distribuídas por seis padrões de supressão de erro. Cada ocorrência foi classificada como **Inofensivo** (comportamento esperado/defensivo), **Suspeito** (requer teste ou análise adicional) ou **Confirmado** (demonstrado por teste ou inspeção que a informação de erro é perdida sem diagnóstico).

### Totais por padrão

| Padrão | Total | Inofensivo | Suspeito | Confirmado |
|---|---:|---:|---:|---:|
| `.ok()` | 48 | 42 | 4 | 2 |
| Wildcard arms (`_ => {}` / `_ => Ok(Value::None)` / `_ => None`) | 113 | 101 | 3 | 9 |
| `let _ = ...` | 22 | 18 | 4 | 0 |
| `unwrap_or_default()` / `unwrap_or_else()` | 69 | 49 | 18 | 2 |
| `if let Ok(...)` | 9 | 3 | 1 | 5 |
| `fn -> Option<T>` | 32 | 26 | 1 | 5 |
| **Total** | **293** | **239** | **31** | **23** |

### Contagem final

- **Inofensivo:** 239 (81,6 %) — fallback defensivo, comportamento documentado ou dead code.
- **Suspeito:** 31 (10,6 %) — casos que carecem de confirmação adicional; vários em L3 (fontes/subsetting/embed) não foram cobertos por testes diretos nesta passagem.
- **Confirmado:** 23 (7,8 %) — falhas silenciosas reproduzidas por teste ou inspeção.

---

## 2. Falhas silenciosas confirmadas — lista priorizada

Ordenadas por gravidade: **perda/corrupção de conteúdo** > **comportamento silenciosamente degradado**. Nenhum caso confirmado nesta passagem foi classificado como puramente de desempenho ou cosmético.

### 2.1 Perda ou corrupção de conteúdo / estado

Estes casos fazem desaparecer informação do utilizador, alteram o significado do documento ou descartam atualizações de estado sem diagnóstico.

| # | Local | Descrição | Prova |
|---|---|---|---|
| 1 | `01_core/src/entities/ast/expr.rs:382` | Escape unicode inválido em *code strings* (`"\u{FFFFFFFF}"`) é descartado e o literal original é preservado, em vez de produzir erro. | `p633_invalid_unicode_escape_preserved` |
| 2 | `01_core/src/entities/ast/markup.rs:107` | Escape unicode inválido em *markup* (`[\u{FFFFFFFF}]`) é convertido em `\0` / texto literal, mascarando o erro de sintaxe. | `p633_invalid_unicode_escape_markup_preserved` |
| 3 | `01_core/src/engine/eval/mod.rs:819` | O catch-all de `eval_expr` devolve `Ok(Value::None)` para variantes de `Expr` não migradas. Faz com que `#break`, `#continue` e `#return` no topo desapareçam sem erro. | `p633_break_top_level_silently_none`, `p633_continue_top_level_silently_none`, `p633_return_top_level_silently_none` |
| 4 | `01_core/src/engine/eval/from_tags.rs:64` | Callback de `state.update(func)` cujo `apply_func` retorna `Err` é descartado com comentário "defensive ignore". A atualização de estado não ocorre e o utilizador não recebe diagnóstico. | Inspeção do código + testes de `state.update` |
| 5 | `01_core/src/engine/eval/bibliography.rs:146` | `hay_entry_to_bib_entry` devolve `None` quando `key.is_empty() \|\| (author.is_empty() && title.is_empty())`, omitindo entradas bibliográficas sem aviso. | Inspeção do código |
| 6 | `01_core/src/engine/layout/bib_csl.rs:214` | `bib_entry_to_hayagriva` devolve `None` por falha YAML ou chave ausente; o caller usa `filter_map`, omitindo a entrada da bibliografia renderizada. | Inspeção do código |
| 7 | `01_core/src/engine/layout/grid.rs:322` | `place_cells(cells, num_cols).unwrap_or_default()` descarta o erro de grid inválida e renderiza vetor vazio, escondendo conflitos de `colspan` ou células inválidas. | Comentário explícito no código + `p633_grid_columns_string_silent_auto` |

### 2.2 Comportamento silenciosamente degradado

Nestes casos a avaliação continua, mas a intenção do utilizador é ignorada: regras `#set`, argumentos de `counter.display`, dimensões de página, etc. O documento produzido difere do esperado sem qualquer aviso.

| # | Local | Descrição | Prova |
|---|---|---|---|
| 8 | `01_core/src/engine/eval/rules.rs:678` | `eval_expr(named.expr(), scopes, ctx, engine).ok()` silencia erros no argumento `numbering` de `#set math.equation`; variável indefinida é ignorada. | `p633_set_equation_numbering_undefined_silent` |
| 9 | `01_core/src/engine/eval/rules.rs:695` | `#set math.equation(numbering: <não-Str>)` ignora o tipo inválido sem diagnóstico. | `p633_set_equation_numbering_int_silent` |
| 10 | `01_core/src/engine/eval/rules.rs:819` | `#set figure(numbering: <não-Str>)` ignora o tipo inválido sem diagnóstico. | `p633_set_figure_numbering_int_silent` |
| 11 | `01_core/src/engine/eval/rules.rs:848` | `#set table(numbering: <não-Str>)` ignora o tipo inválido sem diagnóstico. *Ver P639: `table.numbering` é uma extensão do cristalino (P459); o vanilla não tem esta propriedade.* | `p633_set_table_numbering_int_silent` |
| 12 | `01_core/src/engine/eval/rules.rs:772` | `#set page(numbering: <não-Str/None>)` converte silenciosamente para sem numeração. | `p633_set_page_numbering_int_silent` |
| 13 | `01_core/src/engine/eval/rules.rs:785` | `#set page(columns: <não-Int>)` ignora o tipo inválido sem diagnóstico. | `p633_set_page_columns_string_silent` |
| 14 | `01_core/src/engine/eval/rules.rs:967` | `#set text(weight: <não-Int>)` ignora o tipo inválido sem diagnóstico. | `p633_set_text_weight_string_silent` |
| 15 | `01_core/src/engine/eval/rules.rs:706` | `value_to_eco_string` devolve `None` para tipos inválidos; `#set document(title: 123)` não define o título nem reporta erro. | `p633_set_document_title_int_silent` |
| 16 | `01_core/src/engine/eval/rules.rs:746` | `extract_pt` devolve `None` para tipos inválidos; `#set page(width: "foo")` mantém a dimensão anterior sem erro. | `p633_set_page_width_string_silent` |
| 17 | `01_core/src/engine/stdlib/layout.rs:161` | `parse_track_sizing` devolve `None` para tipos inválidos; `grid(columns: "foo")` silenciosamente vira `grid(columns: auto)`. | `p633_grid_columns_string_silent_auto` |
| 18 | `01_core/src/engine/eval/bindings.rs:141` | Argumento posicional de `counter.display(pattern?)` que não avalia para `Str` é ignorado sem aviso. | `p633_counter_display_invalid_arg_silent` |
| 19 | `01_core/src/engine/eval/bindings.rs:156` | Valor do argumento nomeado `at:` inválido em `counter.display` é ignorado silenciosamente. | `p633_counter_display_at_invalid_silent` |
| 20 | `01_core/src/engine/eval/bindings.rs:164` | `counter.display(...)` ignora argumentos posicionais e nomeados não reconhecidos (ex.: typo em `at:`). | `p633_counter_display_at_invalid_silent` |
| 21 | `01_core/src/engine/eval/bindings.rs:273` | Equivalente a `bindings.rs:156` no despacho de método sobre `Value::Counter`. | `p633_counter_display_at_invalid_silent` |
| 22 | `01_core/src/engine/eval/bindings.rs:280` | Equivalente a `bindings.rs:141` no despacho de método sobre `Value::Counter`. | `p633_counter_display_invalid_arg_silent` |
| 23 | `01_core/src/engine/eval/bindings.rs:284` | `counter.display(...)` ignora argumento posicional quando o pattern já está definido. | `p633_counter_display_invalid_arg_silent` |

---

## 3. Testes diretos adicionados

Os testes foram introduzidos em `01_core/src/engine/eval/tests.rs` sob a secção `// ── P633 — sonda de falhas silenciosas: testes de confirmação/refutação`. Dividem-se em confirmadores (esperam `Ok`) e refutadores (esperam `Err`).

### Confirmadores de falha silenciosa

| Teste | O que confirma |
|---|---|
| `p633_break_top_level_silently_none` | `#break` no topo do documento avalia para `Ok(Value::None)` sem erro (`eval_expr` catch-all). |
| `p633_continue_top_level_silently_none` | Idem para `#continue`. |
| `p633_return_top_level_silently_none` | Idem para `#return 1`. |
| `p633_set_page_width_string_silent` | `#set page(width: "foo")` não gera erro. |
| `p633_set_page_numbering_int_silent` | `#set page(numbering: 123)` não gera erro. |
| `p633_set_page_columns_string_silent` | `#set page(columns: "foo")` não gera erro. |
| `p633_set_document_title_int_silent` | `#set document(title: 123)` não gera erro. |
| `p633_set_text_weight_string_silent` | `#set text(weight: "foo")` não gera erro. |
| `p633_set_equation_numbering_int_silent` | `#set math.equation(numbering: 123)` não gera erro. |
| `p633_set_equation_numbering_undefined_silent` | `#set math.equation(numbering: nao_existe)` não gera erro (`.ok()` descarta o `Err` de `eval_expr`). |
| `p633_set_figure_numbering_int_silent` | `#set figure(numbering: 123)` não gera erro. |
| `p633_set_table_numbering_int_silent` | `#set table(numbering: 123)` não gera erro. |
| `p633_counter_display_invalid_arg_silent` | `#context(counter("x").display(123))` não gera erro. |
| `p633_counter_display_at_invalid_silent` | `#context(counter("x").display("1.", at: 123))` não gera erro. |
| `p633_grid_columns_string_silent_auto` | `grid(columns: "foo")[A]` não gera erro. |
| `p633_invalid_unicode_escape_preserved` | `"\u{FFFFFFFF}"` não gera erro; o literal é preservado. |
| `p633_parse_error_expr_silent_none` | `#let x = 0xZZ` resulta em `x = none` (catch-all de `eval_expr`). |
| `p633_invalid_unicode_escape_markup_preserved` | `[\u{FFFFFFFF}]` não gera erro; o conteúdo é preservado. |

### Refutadores (caminhos que não são falhas silenciosas)

| Teste | O que refuta |
|---|---|
| `p633_counter_update_string_rejeitado` | `#counter("x").update("abc")` gera erro; o caminho `unwrap_or(0)` em `bindings.rs` não é atingível por syntax pública. |
| `p633_state_method_dead_code_path` | `#state("x", 0).get(1, 2)` gera erro de *field access*; o branch `eval_state_method` não é atingível porque `state()` devolve `Content`, não `Value::State`. |

---

## 4. Notas sobre o âmbito

1. **Ficheiros de teste excluídos:** a auditoria não percorreu `tests/`, `benches/` nem código de *fixtures*; concentrou-se no código de produção L1–L4.
2. **L3 não coberta por testes diretos:** alguns casos suspeitos em `03_infra/src/export/` (subsetting de fontes, `Face::parse` de dados de fonte variável/instanciada) não foram testados diretamente nesta passagem. Mantêm-se como **Suspeito**.
3. **Critério de confirmação:** um caso só foi promovido a **Confirmado** quando foi demonstrado por teste ou por inspeção directa do código que a informação de erro é perdida sem diagnóstico ao utilizador. Casos que resultam em fallback documentado, em `None` devolvido por helper cuja semântica é "não aplicável", ou em dead code inacessível por syntax pública, foram reclassificados como **Inofensivo**.

---

## 5. Detalhe por padrão

### 5.1 Padrão `.ok()`

**Totais:** 48 ocorrências — 42 Inofensivo, 4 Suspeito, 2 Confirmado.

#### Inofensivo (42)

- `01_core/src/entities/ast/markup.rs:105` — `u32::from_str_radix(hex, 16).ok().and_then(std::char::from_u32).unwrap_or_default()`; nó `Escape` só é produzido após validação do lexer.
- `01_core/src/entities/ast/markup.rs:293` — `node.len().try_into().ok()`; `HeadingMarker` tem comprimento > 0.
- `01_core/src/entities/ast/markup.rs:319` — `node.text_str().trim_end_matches('.').parse().ok()`; função devolve `Option<u64>`.
- `01_core/src/entities/decimal.rs:31` — `InnerDecimal::from_str(s).ok().map(Self)`; construtor devolve `Option<Self>`.
- `01_core/src/entities/version.rs:95` — `parts[0].parse::<u64>().ok()?`; função devolve `Option<Self>`.
- `01_core/src/entities/version.rs:96` — `parts[1].parse::<u64>().ok()?`; idem.
- `01_core/src/entities/version.rs:97` — `parts[2].parse::<u64>().ok()?`; idem.
- `01_core/src/entities/world_types.rs:65` — `time::Month::try_from(month).ok()?`; `Datetime::new_date` devolve `Option`.
- `01_core/src/entities/world_types.rs:66` — `time::Date::from_calendar_date(year, month, day).ok()?`; idem.
- `01_core/src/entities/world_types.rs:76` — `time::Month::try_from(month).ok()?`; idem para `new_datetime`.
- `01_core/src/entities/world_types.rs:77` — `time::Date::from_calendar_date(year, month, day).ok()?`; idem.
- `01_core/src/entities/world_types.rs:78` — `time::Time::from_hms(hour, minute, second).ok()?`; idem.
- `01_core/src/entities/world_types.rs:348` — `self.upper.compare_exchange(...).ok()`; atualização best-effort.
- `01_core/src/entities/style_chain.rs:451` — `u16::try_from(*i).ok()`; peso fora de `u16` é ignorado por padrão histórico.
- `01_core/src/entities/style_chain.rs:491` — `Lang::from_str(s).ok()`; código de língua inválido resulta em `None` com fallback.
- `01_core/src/entities/value.rs:338` — `Regex::new(s).ok()`; `cast_regex` documentadamente devolve `None` para padrões inválidos.
- `01_core/src/engine/eval/rules.rs:963` — `u16::try_from(*n).ok()`; comentário confirma "silent skip" histórico.
- `01_core/src/engine/layout/text.rs:43` — `u16::try_from(*n).ok()`; idem.
- `01_core/src/engine/layout/text.rs:57` — `Lang::from_str(s).ok()`; idem a `style_chain.rs:491`.
- `01_core/src/engine/lexer/markup.rs:67` — `u32::from_str_radix(hex, 16).ok().and_then(std::char::from_u32).is_none()`; `.ok()` é usado como predicado e erro é emitido a seguir.
- `01_core/src/engine/stdlib/primitives_constructors.rs:245` — `num_str.parse::<f64>().ok()?`; parser de duração devolve `Option`.
- `01_core/src/engine/stdlib/primitives_constructors.rs:256` — `num_str.parse::<u64>().ok()?`; idem.
- `03_infra/src/export/builder.rs:56` — `std::env::var("CRYSTALLINE_PDF_FIXED_EPOCH").ok()`; variável de ambiente opcional para testes.
- `03_infra/src/export/builder.rs:57` — `s.parse::<i64>().ok()`; idem.
- `03_infra/src/export/builder.rs:58` — `time::OffsetDateTime::from_unix_timestamp(ts).ok()`; idem.
- `03_infra/src/export/builder.rs:175` — `Face::parse(font_data, 0).ok()?`; `cff_table_data` devolve `Option`; caller trata como TrueType.
- `03_infra/src/export/builder.rs:437` — `Face::parse(&embed_font_data, 0).ok()`; dados já validados mais acima.
- `03_infra/src/export/mod.rs:142` — `filter_map(|(_, data)| Face::parse(data, 0).ok())`; contagem ativa fallback Helvetica.
- `03_infra/src/export/mod.rs:175` — `filter_map(|(_, data)| Face::parse(data, 0).ok())`; idem.
- `03_infra/src/font_variant.rs:182` — `Command::new(&python).spawn().ok()?`; ausência de Python devolve `None`.
- `03_infra/src/font_variant.rs:187` — `stdin.write_all(data).ok()?`; erro raro; `wait_with_output` verifica o resultado.
- `03_infra/src/font_variant.rs:190` — `child.wait_with_output().ok()?`; idem.
- `03_infra/src/font_metrics.rs:102` — `Face::parse(data, 0).ok()?`; `FontBookMetrics::from_bytes` devolve `None` para bytes inválidos.
- `03_infra/src/font_metrics.rs:266` — `Face::parse(slice, 0).ok()?`; `CachedFace::new` devolve `None` para fonte inválida.
- `03_infra/src/fonts.rs:44` — `std::fs::read(&self.path).ok()?`; `FontSlot::get` devolve `None` se ilegível.
- `03_infra/src/fonts.rs:48` — `ttf_parser::Face::parse(&data, 0).ok()?`; validação de fonte.
- `03_infra/src/fonts.rs:130` — `std::fs::read(path).ok().and_then(...).unwrap_or(1)`; fallback seguro.
- `03_infra/src/fonts.rs:162` — `ttf_parser::Face::parse(data, index).ok()?`; `font_info_from_bytes` devolve `None`.
- `03_infra/src/image_sizer.rs:17` — `imagesize::blob_size(data).ok().map(...)`; cabeçalho não reconhecido devolve `None`.
- `03_infra/src/shaper.rs:448` — `ttf_parser::Face::parse(font.as_slice(), 0).ok()?`; `load_fallback` devolve `None`.
- `03_infra/src/shaper.rs:466` — `ttf_parser::Face::parse(font.as_slice(), 0).ok()`; `face_covers_char` devolve `false`.
- `02_shell/src/cli.rs:178` — `u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok()?`; `parse_uuid_bytes` devolve `None`.

#### Suspeito (4)

- `01_core/src/engine/layout/bib_csl.rs:286` — `hayagriva::io::from_yaml_str(&yaml).ok()?`; erro de parse YAML descarta entrada bibliográfica sem diagnóstico.
- `03_infra/src/export/builder.rs:694` — `Face::parse(&embed_data, 0).ok()`; dados de fonte variável instanciada podem não ter sido validados.
- `03_infra/src/export/subset.rs:72` — `oxifont_subset::subset_with_gid_set(...).ok()?`; erro do subsetter descartado.
- `03_infra/src/export/subset.rs:75` — `ttf_parser::Face::parse(&subset_data, 0).ok()?`; se o subsetter produzir bytes inválidos, o erro é descartado.

#### Confirmado (2)

- `01_core/src/entities/ast/expr.rs:382` — `u32::from_str_radix(sequence, 16).ok().and_then(std::char::from_u32)`; escape `\u{...}` inválido é descartado e convertido de volta no texto literal. **Promovido a Confirmado** por `p633_invalid_unicode_escape_preserved`.
- `01_core/src/engine/eval/rules.rs:678` — `eval_expr(named.expr(), scopes, ctx, engine).ok()`; erro de avaliação do argumento `numbering` de `#set math.equation` é silenciado. **Promovido a Confirmado** por `p633_set_equation_numbering_undefined_silent`.

---

### 5.2 Padrões wildcard

**Totais:** 113 ocorrências — 101 Inofensivo, 3 Suspeito, 9 Confirmado.

#### 5.2.1 `_ => {}`

**Totais:** 33 — 24 Inofensivo, 2 Suspeito, 7 Confirmado.

##### Inofensivo (24)

- `01_core/src/entities/layout_types.rs:500` — `plain_text_items` ignora itens de frame que não são texto.
- `01_core/src/engine/eval/operators.rs:32` — verificação de divisão por zero; catch-all cobre tipos não numéricos.
- `01_core/src/engine/eval/bindings.rs:348` — `counter.at()` valida argumentos posicionais; tipos inválidos ignorados, erro reportado depois.
- `01_core/src/engine/eval/bindings.rs:351` — `counter.at()` espera argumento posicional; argumentos nomeados ignorados, erro reportado depois.
- `01_core/src/engine/eval/closures.rs:336` — métodos especiais (`counter.*`) caem fora; fallback para chamada normal.
- `01_core/src/engine/layout/helpers.rs:206` — `collect_items_at` ignora content que não produz itens de frame.
- `01_core/src/engine/layout/sub_frame.rs:194` — cálculo de altura de linha ignora itens não-texto.
- `01_core/src/engine/math/layout/frac.rs:64` — posicionamento de texto em fracção ignora itens não-texto.
- `01_core/src/engine/math/layout/frac.rs:85` — idem.
- `01_core/src/engine/introspect.rs:120` — coleta de chaves de referência ignora content não navegável.
- `03_infra/src/export/gradients/mod.rs:222` — coleta de gradientes ignora itens sem gradiente.
- `03_infra/src/export/gradients/mod.rs:286` — idem.
- `03_infra/src/export/images.rs:207` — coleta de imagens ignora itens sem imagem.
- `03_infra/src/export/images.rs:323` — idem.
- `03_infra/src/export/builder.rs:90` — ajuste de padding base64 para múltiplos de 3.
- `03_infra/src/export/builder.rs:1598` — coleta de links ignora itens não-link.
- `03_infra/src/export/fonts.rs:63` — coleta de codepoints; comentário explica que Image/Line/Glyph não contribuem.
- `03_infra/src/export/fonts.rs:88` — coleta de codepoints de texto plano ignora itens não-texto.
- `03_infra/src/export/fonts.rs:119` — coleta de glyph IDs ignora itens não-glyph.
- `03_infra/src/export/fonts.rs:142` — coleta de shaped cluster texts ignora itens não-shaped.
- `03_infra/src/export/fonts.rs:184` — coleta de char representativo ignora itens não-shaped.
- `03_infra/src/export/stream.rs:604` — emissão de operadores PDF ignora itens sem path.
- `03_infra/src/pipeline.rs:178` — substituição de ContextBlock ignora containers não listados.
- `03_infra/src/shaper.rs:150` — shapeamento ignora itens não-texto.

##### Suspeito (2)

- `01_core/src/engine/eval/rules.rs:736` — `#set document(...)` ignora chaves nomeadas desconhecidas sem aviso (possível typo silencioso). **Mantém Suspeito** — não testado.
- `01_core/src/engine/eval/rules.rs:788` — `#set page(...)` ignora chaves nomeadas desconhecidas sem aviso. **Mantém Suspeito** — não testado.

##### Confirmado (7)

- `01_core/src/engine/eval/bindings.rs:164` — `counter.display(...)` ignora argumentos posicionais e nomeados não reconhecidos. **Promovido a Confirmado** por `p633_counter_display_invalid_arg_silent` / `p633_counter_display_at_invalid_silent`.
- `01_core/src/engine/eval/bindings.rs:284` — `counter.display(...)` ignora argumento posicional quando pattern já está definido. **Promovido a Confirmado** pelos mesmos testes.
- `01_core/src/engine/eval/rules.rs:695` — `#set equation(numbering: <não-Str>)` ignora tipo inválido. **Promovido a Confirmado** por `p633_set_equation_numbering_int_silent`.
- `01_core/src/engine/eval/rules.rs:819` — `#set figure(numbering: <não-Str>)` ignora tipo inválido. **Promovido a Confirmado** por `p633_set_figure_numbering_int_silent`.
- `01_core/src/engine/eval/rules.rs:848` — `#set table(numbering: <não-Str>)` ignora tipo inválido. **Promovido a Confirmado** por `p633_set_table_numbering_int_silent`. *P639 confirmou que `table.numbering` é uma extensão cristalina (P459): no vanilla `table` não tem `numbering`; a numeração de tabelas faz-se via `#figure(table(...), caption: ...)`. A validação de tipo introduzida em P636 mantém-se correcta dentro do cristalino, mas não é paridade com o vanilla.*
- `01_core/src/engine/eval/rules.rs:772` — `#set page(numbering: <não-Str/None>)` converte silenciosamente para sem numeração. **Promovido a Confirmado** por `p633_set_page_numbering_int_silent`.
- `01_core/src/engine/eval/rules.rs:785` — `#set page(columns: <não-Int>)` ignora tipo inválido. **Promovido a Confirmado** por `p633_set_page_columns_string_silent`.

#### 5.2.2 `_ => Ok(Value::None)`

**Totais:** 1 — 0 Inofensivo, 0 Suspeito, 1 Confirmado.

##### Confirmado (1)

- `01_core/src/engine/eval/mod.rs:819` — `eval_expr` devolve `Ok(Value::None)` para variantes de `Expr` não migradas. Faz expressões não suportadas desaparecerem sem erro. **Confirmado** por `p633_break_top_level_silently_none`, `p633_continue_top_level_silently_none`, `p633_return_top_level_silently_none` e `p633_parse_error_expr_silent_none`.

#### 5.2.3 `_ => None`

**Totais:** 79 — 77 Inofensivo, 1 Suspeito, 1 Confirmado.

##### Inofensivo (77)

Os 76 casos originalmente inofensivos mantêm-se; acrescenta-se `bindings.rs:124` reclassificado de Suspeito.

- `01_core/src/entities/elements/emph.rs:58` — `get_field` desconhecido.
- `01_core/src/entities/elements/strong.rs:61` — `get_field` desconhecido.
- `01_core/src/entities/elements/heading.rs:104` — `get_field` desconhecido.
- `01_core/src/entities/func.rs:199` — `element_name` para funções que não são `FuncRepr::Element`.
- `01_core/src/entities/func.rs:265` — `namespace` para funções que não são nativas.
- `01_core/src/entities/math_style.rs:110` — `digit_base` para estilo sem base numérica.
- `01_core/src/entities/math_style.rs:144` — `bmp_exception` para caractere/estilo sem exceção BMP.
- `01_core/src/entities/counter_format.rs:79` — `token_kind` para caractere não numérico.
- `01_core/src/entities/value.rs:246` — `cast_bool` para tipo não-Bool.
- `01_core/src/entities/value.rs:251` — `cast_int` para tipo não-Int.
- `01_core/src/entities/value.rs:259` — `cast_float` para tipo não numérico.
- `01_core/src/entities/value.rs:265` — `cast_str` para tipo não-Str.
- `01_core/src/entities/value.rs:270` — `cast_array` para tipo não-Array.
- `01_core/src/entities/value.rs:275` — `cast_dict` para tipo não-Dict.
- `01_core/src/entities/value.rs:280` — `cast_align` para tipo não-Align.
- `01_core/src/entities/value.rs:285` — `cast_bytes` para tipo não-Bytes.
- `01_core/src/entities/value.rs:317` — `cast_duration` para tipo não compatível.
- `01_core/src/entities/value.rs:328` — `cast_version` para tipo não compatível.
- `01_core/src/entities/value.rs:339` — `cast_regex` para tipo não compatível.
- `01_core/src/entities/content.rs:2779` — `get_field` para campo/conteúdo desconhecido.
- `01_core/src/entities/content.rs:3030` — `morph_canon` retorna `None` para manter o nó inalterado.
- `01_core/src/engine/eval/bindings.rs:91` — `extract_counter_key` para argumento inválido.
- `01_core/src/engine/eval/bindings.rs:124` — `counter.update()` converte argumento inválido em `0`. **Reclassificado para Inofensivo**: teste `p633_counter_update_string_rejeitado` prova que o caminho `unwrap_or(0)` não é atingível por syntax pública; é dead code.
- `01_core/src/engine/eval/bindings.rs:478` — `value_to_query_selector` para seletor não suportado.
- `01_core/src/engine/eval/math.rs:184` — `filter_map` de argumentos posicionais em `frac`.
- `01_core/src/engine/eval/math.rs:198` — `filter_map` de argumentos posicionais em `sqrt`.
- `01_core/src/engine/eval/math.rs:213` — `filter_map` de argumentos posicionais em `root`.
- `01_core/src/engine/eval/math.rs:229` — `filter_map` de argumentos posicionais em `vec`.
- `01_core/src/engine/eval/math.rs:243` — `filter_map` de argumentos posicionais em `cases`.
- `01_core/src/engine/eval/math.rs:274` — `filter_map` de argumentos posicionais em matrizes.
- `01_core/src/engine/eval/math.rs:366` — `filter_map` de argumentos posicionais em função math genérica.
- `01_core/src/engine/eval/mod.rs:382` — conversão de resultado de eval para `Option<Content>`.
- `01_core/src/engine/eval/mod.rs:391` — idem.
- `01_core/src/engine/eval/rules.rs:57` — `kind_to_node` para `ElementKind` não mapeado.
- `01_core/src/engine/eval/rules.rs:714` — filtro de strings dentro de array para metadados do documento.
- `01_core/src/engine/eval/rules.rs:723` — `value_to_eco_string` para tipo não suportado.
- `01_core/src/engine/eval/rules.rs:751` — `extract_pt` para tipo não suportado.
- `01_core/src/engine/introspect/extract_payload.rs:89` — content não locatable.
- `01_core/src/engine/layout/figure.rs:33` — `figure.numbering` pattern apenas aceita `Value::Str`.
- `01_core/src/engine/layout/image.rs:80` — `extract_pt` apenas aceita Float/Length.
- `01_core/src/engine/layout/table.rs:33` — `table.numbering` pattern apenas aceita `Value::Str`.
- `01_core/src/engine/layout/references.rs:153` — `default_supplement_for_key` para chave sem suplemento predefinido.
- `01_core/src/engine/layout/bib_csl.rs:164` — `resolve_style_name` apenas aceita estilos Independent.
- `01_core/src/engine/layout/cursor.rs:159` — procura de direção RTL apenas em itens de texto.
- `01_core/src/engine/layout/cursor.rs:233` — procura de leading apenas em itens de texto.
- `01_core/src/engine/layout/equation.rs:173` — `equation.numbering` pattern apenas aceita `Value::Str`.
- `01_core/src/engine/layout/heading.rs:50` — `heading.numbering.pattern` apenas aceita `Value::Str`.
- `01_core/src/engine/layout/mod.rs:1881` — walk de introspecção para content não suportado.
- `01_core/src/engine/layout/text.rs:36` — cast de `text.size` apenas aceita `Value::Length`.
- `01_core/src/engine/layout/text.rs:40` — cast de `text.fill` apenas aceita `Value::Color`.
- `01_core/src/engine/layout/text.rs:44` — cast de `text.weight` apenas aceita `Value::Int`.
- `01_core/src/engine/layout/text.rs:48` — cast de `text.tracking` apenas aceita `Value::Length`.
- `01_core/src/engine/layout/text.rs:52` — cast de `par.leading` apenas aceita `Value::Length`.
- `01_core/src/engine/layout/text.rs:59` — cast de `text.lang` apenas aceita `Value::Str`.
- `01_core/src/engine/layout/text.rs:63` — cast de `text.dir` apenas aceita `Value::Dir`.
- `01_core/src/engine/layout/text.rs:87` — cast de variantes de fonte dentro de `text.font`.
- `01_core/src/engine/layout/text.rs:107` — cast de entrada de fonte inválida.
- `01_core/src/engine/layout/text.rs:112` — cast de `text.font` inválido.
- `01_core/src/engine/layout/columns.rs:95` — `body_dir` para content sem direção definida.
- `01_core/src/engine/layout/columns.rs:108` — `styles_dir` apenas aceita `Value::Dir`.
- `01_core/src/engine/layout/sub_frame.rs:108` — procura de leading apenas em itens de texto.
- `01_core/src/engine/math/layout/attach.rs:47` — `base_char` apenas para `MathIdent`/`MathText`.
- `01_core/src/engine/parse/patterns.rs:212` — `node_key` apenas aceita `Ident`/`Str`.
- `01_core/src/engine/stdlib/collections.rs:91` — dispatch de métodos de string; método inexistente cai fora.
- `01_core/src/engine/stdlib/collections.rs:777` — `value_cmp` para tipos não comparáveis.
- `01_core/src/engine/stdlib/shapes.rs:56` — `parse_color` para cor/nome desconhecido.
- `01_core/src/engine/stdlib/shapes.rs:69` — `parse_paint` para paint inválido.
- `01_core/src/engine/stdlib/shapes.rs:272` — `extract_coordinate` para array inválido.
- `01_core/src/engine/stdlib/transforms.rs:109` — `extract_angle_rad` para tipo inválido.
- `01_core/src/engine/stdlib/figure_image.rs:39` — `infer_kind_from_body` para body não reconhecido.
- `01_core/src/engine/stdlib/figure_image.rs:142` — `image(fit:)` apenas aceita `Value::Str`.
- `01_core/src/engine/stdlib/layout.rs:154` — `extract_alignment` para argumento não alinhamento.
- `01_core/src/engine/stdlib/structural.rs:1515` — `bibliography(style:)` apenas aceita Str/None.
- `01_core/src/engine/stdlib/structural.rs:1521` — `bibliography(locale:)` apenas aceita Str/None.
- `01_core/src/engine/stdlib/text.rs:155` — `replace(count:)` apenas aceita `Value::Int`.
- `03_infra/src/layout_bidi.rs:672` — `item_height` apenas para itens de texto.

##### Suspeito (1)

- `01_core/src/engine/eval/closures.rs:222` — parâmetros `Placeholder` e `Destructuring` são descartados sem aviso (comentário indica "adiado"). **Mantém Suspeito** — não testado.

##### Confirmado (1)

- `01_core/src/engine/eval/rules.rs:967` — `#set text(weight: <não-Int>)` ignora tipos inválidos silenciosamente. **Promovido a Confirmado** por `p633_set_text_weight_string_silent`.

---

### 5.3 Padrão `let _ = ...`

**Totais:** 22 ocorrências — 18 Inofensivo, 4 Suspeito, 0 Confirmado.

#### Inofensivo (18)

Os 15 casos originalmente inofensivos mantêm-se; acrescentam-se 3 casos de `bindings.rs` reclassificados de Suspeito para Inofensivo por serem dead code.

- `01_core/src/contracts/world.rs:42` — parâmetro não usado no corpo padrão do trait.
- `01_core/src/contracts/world.rs:50` — idem.
- `01_core/src/engine/layout/boxed.rs:66` — `baseline` recebido mas ainda não aplicado.
- `01_core/src/engine/layout/place.rs:78` — `dx` aplicado mais tarde em `flush`.
- `01_core/src/engine/layout/place.rs:79` — `dy` idem.
- `01_core/src/engine/layout/place.rs:80` — `scope` parâmetro sentinela (DEBT-37).
- `01_core/src/engine/layout/metrics.rs:47` — corpo padrão de método de trait.
- `01_core/src/engine/layout/metrics.rs:56` — idem.
- `01_core/src/engine/layout/metrics.rs:65` — idem.
- `01_core/src/engine/layout/metrics.rs:74` — idem.
- `01_core/src/engine/layout/metrics.rs:83` — idem.
- `01_core/src/engine/scopes.rs:142` — `base` (Library) ainda é stub.
- `01_core/src/engine/stdlib/figure_image.rs:56` — parâmetro não usado nesta implementação nativa.
- `01_core/src/engine/introspect.rs:1097` — `level` extraído mas não necessário nesta versão.
- `01_core/src/engine/introspect.rs:1298` — `figure_number` usado na linha seguinte; no-op.
- `01_core/src/engine/eval/bindings.rs:222` — `let _ = eval_args(args, ...)?` em `state.get()`. **Reclassificado para Inofensivo**: `state()` devolve `Content`, não `Value::State`; branch inacessível (teste `p633_state_method_dead_code_path`).
- `01_core/src/engine/eval/bindings.rs:254` — `let _ = eval_args(args, ...)?` em `counter.step()`. **Reclassificado para Inofensivo**: sem teste directo, mas segue a mesma lógica de método inacessível via `state()`/`counter()` quando o valor já é `Content`.
- `01_core/src/engine/eval/bindings.rs:258` — `let _ = eval_args(args, ...)?` em `counter.get()`. **Reclassificado para Inofensivo**: mesmo raciocínio.

#### Suspeito (4)

- `03_infra/src/export/gradients/function_dict.rs:24` — `let _ = function_id;` parâmetro não incorporado no dicionário devolvido.
- `03_infra/src/export/gradients/function_dict.rs:55` — idem; ramo Type 3 stitching.
- `03_infra/src/export/gradients/function_dict.rs:81` — idem; ramo de 2 stops.
- `03_infra/src/export/gradients/function_dict.rs:115` — idem; ramo Type 3 stitching.

#### Confirmado

Nenhum.

---

### 5.4 Padrões `unwrap_or_default()` / `unwrap_or_else()`

**Totais:** 69 ocorrências — 49 Inofensivo, 18 Suspeito, 2 Confirmado.

#### Inofensivo (49)

Os 46 casos originalmente inofensivos mantêm-se; acrescentam-se 3 casos de `expr.rs` reclassificados de Suspeito para Inofensivo (dead code defensivo).

- `entities/ast/math.rs:71` — string vazia não é numérica.
- `entities/bib_store.rs:144` — `Vec` vazio se key nunca citada.
- `entities/elements/figure.rs:37` — caption opcional ausente.
- `entities/elements/math_underover.rs:27` — campo `over` opcional.
- `entities/elements/math_underover.rs:29` — campo `under` opcional.
- `entities/elements/table.rs:48` — caption opcional.
- `entities/introspector.rs:456` — query por kind sem resultados.
- `entities/introspector.rs:535` — query por label não encontrada.
- `entities/style_chain.rs:620` — fonte padrão documentada.
- `rules/eval/bibliography.rs:161` — autor vazio tolerado.
- `rules/eval/bibliography.rs:164` — título ausente combinado com autor.
- `engine/layout/cite.rs:29` — forma de citação com default explícito.
- `engine/layout/cite.rs:31` — estilo de citação com default explícito.
- `engine/layout/cite.rs:88` — autor vazio cai na key.
- `engine/layout/cite.rs:103` — número de citação não encontrado.
- `engine/layout/figure.rs:54` — pattern falha → número cru.
- `engine/layout/outline.rs:56` — título default do outline.
- `engine/layout/outline.rs:81` — página ainda desconhecida.
- `engine/layout/outline.rs:87` — heading sem numeração.
- `engine/layout/outline.rs:130` — título default do LoF.
- `engine/layout/outline.rs:157` — título default do LoT.
- `engine/layout/table.rs:48` — pattern falha → número cru.
- `engine/layout/references.rs:109` — pattern `(1)` falha → número cru.
- `engine/layout/references.rs:135` — supplement default para figura.
- `engine/layout/decorations.rs:51` — nenhum collector ativo.
- `engine/layout/equation.rs:143` — pattern falha → número cru.
- `engine/layout/text.rs:143` — tamanho de subscrito default.
- `engine/layout/text.rs:148` — tamanho de sobrescrito default.
- `engine/layout/metrics.rs:95` — `advance_shaped` indisponível.
- `rules/lexer/markup.rs:357` — lookup em tabela estática.
- `rules/math/layout/attach.rs:51` — sem `MathKernInfo`.
- `rules/parse/parser.rs:263` — coluna do token calculada do lexer.
- `rules/stdlib/assert.rs:54` — mensagem default de assert.
- `rules/stdlib/shapes.rs:138` — `square()` sem `height`.
- `rules/stdlib/figure_image.rs:87` — inferência de `kind`.
- `rules/stdlib/figure_image.rs:144` — `fit` default de `image()`.
- `export/gradients/relative.rs:33` — `RelativeTo` default.
- `export/builder.rs:1290` — nó de outline sem parent.
- `export/builder.rs:1399` — título do documento opcional.
- `export/builder.rs:1408` — keywords opcionais.
- `export/builder.rs:1417` — autor opcional.
- `export/builder.rs:1508` — dicionário `/Info` opcional.
- `font_variant.rs:31` — peso default derivado de `bold`.
- `font_variant.rs:159` — binário Python default.
- `shaper.rs:64` — resolução vazia dispara fallback.
- `shaper.rs:181` — idem.
- `entities/ast/expr.rs:308` — parse de literal inteiro. **Reclassificado para Inofensivo**: parser rejeita/split literais malformados; caminho inacessível.
- `entities/ast/expr.rs:316` — parse de literal float. **Reclassificado para Inofensivo**: mesmo raciocínio.
- `entities/ast/expr.rs:331` — parse de literal com unidade. **Reclassificado para Inofensivo**: mesmo raciocínio.

#### Suspeito (18)

- `entities/ast/markup.rs:109` — escape `\` sem caractere seguinte vira `\0`.
- `entities/ast/markup.rs:268` — nó `Ref` sem `RefMarker` interno produz target vazio.
- `rules/eval/bindings.rs:174` — label/counter ausente em `numbering` retorna string vazia.
- `rules/eval/bindings.rs:293` — idem para `counter`.
- `rules/introspect/heading.rs:53` — heading numerado fora de ciclo de contador.
- `engine/layout/cite.rs:85` — citação numérica perde o número e mostra a key.
- `engine/layout/cite.rs:117` — idem no ramo Prose.
- `engine/layout/references.rs:104` — referência a label numerada com counter ausente.
- `engine/layout/references.rs:110` — `flat_counter_at("equation", loc)` ausente.
- `engine/layout/references.rs:117` — figure/table counter ausente.
- `engine/layout/mod.rs:790` — `CounterDisplay` sem localização known.
- `engine/layout/mod.rs:793` — counter global ausente → `"0"`.
- `rules/scopes.rs:89` — `exit()` sem `enter()` correspondente.
- `rules/stdlib/loading.rs:292` — linha CSV com mais colunas do que cabeçalho.
- `rules/stdlib/foundations.rs:752` — `counter_at(key, label)` com label/counter ausente.
- `rules/stdlib/foundations.rs:792` — `counter_final(key)` na iteração 0.
- `rules/introspect.rs:383` — `CounterDisplay` sem counter local.
- `world.rs:218` — `directory_of` com `FileId` inexistente.

#### Confirmado (2)

- `01_core/src/engine/layout/grid.rs:322` — `place_cells(cells, num_cols).unwrap_or_default()` descarta erro de grid inválida. **Confirmado**.
- `01_core/src/entities/ast/markup.rs:107` — escape unicode inválido em markup vira `\0` / literal. **Promovido a Confirmado** por `p633_invalid_unicode_escape_markup_preserved`.

---

### 5.5 Padrão `if let Ok(...)`

**Totais:** 9 ocorrências — 3 Inofensivo, 1 Suspeito, 5 Confirmado.

#### Inofensivo (3)

Os 2 casos originalmente inofensivos mantêm-se; acrescenta-se `bindings.rs:118` reclassificado de Suspeito.

- `03_infra/src/fonts.rs:206` — ficheiros ilegíveis ou inválidos ignorados intencionalmente durante descoberta.
- `03_infra/src/shaper.rs:324` — fonte não parseável omitida da lista de candidatas.
- `01_core/src/engine/eval/bindings.rs:118` — `counter.update(...)` com tipo não-inteiro. **Reclassificado para Inofensivo**: teste `p633_counter_update_string_rejeitado` prova que o caminho é inacessível por syntax pública.

#### Suspeito (1)

- `03_infra/src/export/builder.rs:250` — `Face::parse(data, 0)` em embed de fonte; se falhar, cai silenciosamente para Helvetica.

#### Confirmado (5)

- `01_core/src/engine/eval/from_tags.rs:64` — callback de `state.update(func)` cujo `apply_func` retorna `Err` é descartado. **Confirmado**.
- `01_core/src/engine/eval/bindings.rs:141` — argumento posicional de `counter.display(pattern?)` não-Str ignorado. **Promovido a Confirmado** por `p633_counter_display_invalid_arg_silent`.
- `01_core/src/engine/eval/bindings.rs:156` — argumento nomeado `at:` inválido em `counter.display` ignorado. **Promovido a Confirmado** por `p633_counter_display_at_invalid_silent`.
- `01_core/src/engine/eval/bindings.rs:273` — equivalente a `bindings.rs:156` no despacho sobre `Value::Counter`. **Promovido a Confirmado**.
- `01_core/src/engine/eval/bindings.rs:280` — equivalente a `bindings.rs:141` no despacho sobre `Value::Counter`. **Promovido a Confirmado**.

---

### 5.6 Padrão `fn -> Option<T>`

**Totais:** 32 ocorrências — 26 Inofensivo, 1 Suspeito, 5 Confirmado.

#### Inofensivo (26)

- `eval/bindings.rs:72` — `extract_counter_key`; filtro natural.
- `eval/bindings.rs:460` — `value_to_query_selector`; caller levanta `SourceDiagnostic`.
- `eval/math.rs:43` — `lookup_math_op`; fallback `MathIdent`.
- `eval/rules.rs:47` — `kind_to_node`; caller converte `None` em erro.
- `introspect/extract_payload.rs:18` — predicado/option natural.
- `layout/image.rs:76` — `extract_pt` opcional.
- `layout/references.rs:149` — `default_supplement_for_key`; fallback.
- `layout/bib_csl.rs:160` — `resolve_style_name`; fallback local.
- `layout/bib_csl.rs:203` — `form_to_purpose`; `Normal` ↔ `None`.
- `layout/columns.rs:72` — `body_dir`; fallback.
- `layout/columns.rs:100` — `styles_dir`; fallback LTR.
- `layout/metrics.rs:55` — `glyph_to_char` default de trait.
- `layout/metrics.rs:82` — `advance_shaped` default de trait.
- `parse/code.rs:226` — API de reparse incremental.
- `parse/parser.rs:512` — controle de memoização.
- `parse/parser.rs:638` — controle de profundidade com erro associado.
- `parse/parser.rs:650` — idem.
- `stdlib/shapes.rs:31` — `parse_color`; caller reporta erro.
- `stdlib/shapes.rs:63` — `parse_paint`; caller reporta erro.
- `stdlib/shapes.rs:265` — `extract_coordinate`; caller reporta erro.
- `stdlib/transforms.rs:105` — `extract_angle_rad`; caller reporta erro.
- `stdlib/primitives_constructors.rs:212` — `parse_duration`; caller reporta erro.
- `stdlib/figure_image.rs:30` — `infer_kind_from_body`; fallback `"image"`.
- `stdlib/layout.rs:347` — `extract_length`; maioria dos callers reporta erro.
- `stdlib/structural.rs:2223` — `infer_asset_kind`; fallback.
- `export/builder.rs:174` — `cff_table_data`; fallback TrueType.

#### Suspeito (1)

- `03_infra/src/export/subset.rs:96` — `subset_font` devolve `None` em múltiplas causas de falha; o caller faz fallback para fonte completa, aumentando o PDF sem aviso. **Mantém Suspeito** — não testado nesta passagem.

#### Confirmado (5)

- `01_core/src/engine/eval/bibliography.rs:146` — `hay_entry_to_bib_entry` ignora silenciosamente entradas sem key/título.
- `01_core/src/engine/layout/bib_csl.rs:214` — `bib_entry_to_hayagriva` omite entradas por YAML/chave ausente.
- `01_core/src/engine/eval/rules.rs:706` — `value_to_eco_string` devolve `None` para tipos inválidos; `#set document(title: 123)` silencioso.
- `01_core/src/engine/eval/rules.rs:746` — `extract_pt` devolve `None` para tipos inválidos; `#set page(width: "foo")` silencioso.
- `01_core/src/engine/stdlib/layout.rs:161` — `parse_track_sizing` devolve `None` para tipos inválidos; `grid(columns: "foo")` vira `auto`.

---

## 6. Conclusão e recomendações

A auditoria P633 confirmou **23 falhas silenciosas** que merecem correção ou, no mínimo, emissão de diagnóstico. A maior concentração está em:

1. **Catch-all de `eval_expr`** (`mod.rs:819`), que faz desaparecer construções não migradas.
2. **Regras `#set`**, onde tipos inválidos são ignorados em vez de produzirem erro.
3. **`counter.display`**, onde argumentos inválidos são descartados.
4. **Bibliografia e estado**, onde entradas/erros são omitidos sem aviso.

Recomenda-se que os casos **Confirmados** sejam tratados em passos subsequentes, priorizando os de perda/corrupção de conteúdo. Os **Suspeitos** em L3 (subsetting/embed de fontes) devem ser auditados com testes de exportação PDF numa passagem dedicada.

# Relatório de Paridade — P638

**Passo:** 638  
**Data:** 2026-07-09  
**Foco:** Sondar o código vanilla (`lab/typst-original/`) nas áreas já tocadas por P633–P637 para verificar se alguma das 23 falhas silenciosas confirmadas no cristalino também existia no vanilla.  
**Commit de referência do cristalino:** `bb8cea050` (P637).  
**Binário vanilla usado:** `lab/typst-original/target/release/typst` (compilado a partir do mesmo snapshot do vanilla em quarentena).

---

## 1. Resumo executivo

A sonda cobriu as três áreas definidas no passo — avaliação/regras `#set`, layout de texto/grid, exportação PDF/bibliografia — e comparou cada um dos 23 casos confirmados em P633 com o equivalente no vanilla.

**Conclusão principal:** em **22 dos 23 casos**, o vanilla **já produzia um erro claro** (ou tratava a situação sem perder informação de diagnóstico). A única excepção é o **caso 7** — erros de placement de grid (`place_cells(...).unwrap_or_default()` no cristalino) — onde o vanilla também não reporta o problema ao utilizador, embora por um mecanismo diferente (não detecta o conflito em vez de detectar e descartar).

Isto significa que:

- As correcções de **P634**, **P635**, **P636** e **P637** são, na sua grande maioria, **correcções de paridade** — o cristalino estava a fazer pior do que o vanilla.
- O **caso 7 (grid)** não é uma simples imitação do vanilla; quando for corrigido, será uma **melhoria além do vanilla**.
- Não foi encontrado nenhum padrão novo no vanilla que justifique copiar um comportamento silencioso para o cristalino.

---

## 2. Metodologia

### 2.1 Varredura dos seis padrões de P633 no vanilla

Os comandos indicados no passo foram corridos nas pastas equivalentes do vanilla:

```bash
grep -rn "\.ok()" lab/typst-original/crates/typst-eval/src/ lab/typst-original/crates/typst-library/src/model/ lab/typst-original/crates/typst-layout/src/ lab/typst-original/crates/typst-pdf/src/ --include="*.rs" | wc -l
# 11

grep -rn "_ => {}" lab/typst-original/crates/typst-eval/src/ lab/typst-original/crates/typst-library/src/model/ --include="*.rs"
# 2 ocorrências (ambas inofensivas: contagem de caracteres em decode_library e ignorar display="left-margin" não-suportado no CSL)

grep -rn "let _ = " lab/typst-original/crates/typst-eval/src/ lab/typst-original/crates/typst-layout/src/ --include="*.rs"
# 3 ocorrências (todas propagam o erro com `?`; inofensivas)

grep -rn "unwrap_or_default()\|unwrap_or_else(|| " lab/typst-original/crates/typst-eval/src/ lab/typst-original/crates/typst-library/src/model/ --include="*.rs"
# 26 ocorrências (maioria defaults documentados; nenhuma nova suspeita confirmada)
```

Complementarmente, foram também corridos os padrões `if let Ok(...)` e `fn -> Option<`:

```bash
grep -rn "if let Ok(" lab/typst-original/crates/typst-eval/src/ lab/typst-original/crates/typst-library/src/model/ lab/typst-original/crates/typst-layout/src/ lab/typst-original/crates/typst-pdf/src/ --include="*.rs" | wc -l
# 2 (ambos inofensivos)

grep -rn "fn .*-> Option<" lab/typst-original/crates/typst-eval/src/ lab/typst-original/crates/typst-library/src/model/ lab/typst-original/crates/typst-layout/src/ lab/typst-original/crates/typst-pdf/src/ --include="*.rs" | wc -l
# 100 (maioria getters/lookups naturais; nenhum novo confirmado como falha silenciosa)
```

### 2.2 Testes directos no binário vanilla

Para cada um dos 23 casos confirmados em P633, foi criado um ficheiro `.typ` mínimo e compilado com o vanilla. Os resultados estão registados na tabela abaixo. Os ficheiros de teste residem em `temp_p638/` (não commitados).

---

## 3. Tabela comparativa — 23 casos do cristalino vs. vanilla

| # | Caso do cristalino (P633) | Equivalente no vanilla | Vanilla também falha em silêncio? | Notas |
|---|---------------------------|------------------------|-----------------------------------|-------|
| 1 | `01_core/src/entities/ast/expr.rs:382` — escape unicode inválido em code string | `typst-syntax` (lexer/parser) | **Não** | Erro claro: `invalid Unicode codepoint: FFFFFFFF` |
| 2 | `01_core/src/entities/ast/markup.rs:107` — escape unicode inválido em markup | `typst-syntax` (lexer/parser) | **Não** | Erro claro: `invalid Unicode codepoint: FFFFFFFF` |
| 3 | `01_core/src/rules/eval/mod.rs:819` — catch-all `eval_expr` engole `#break`/`#continue`/`#return` | `typst-eval/src/code.rs:74-154` (match exaustivo) + `flow.rs:26-36` + `lib.rs:88-91` | **Não** | Erros claros: `cannot break outside of loop`, etc. |
| 4 | `01_core/src/rules/eval/from_tags.rs:64` — `state.update(func)` descarta `Err` do callback | `typst-library/src/introspection/state.rs:498-504` (`func.call(...)?`) | **Não** | Erro do callback propagado (testado: divisão por zero) |
| 5 | `01_core/src/rules/eval/bibliography.rs:146` — entrada sem key/título omitida | `typst-library/src/model/bibliography.rs:377-381` | **Não** | Chave vazia → erro `bibliography contains entry with empty key`. Sem título → aceite (renderiza degradado, mas não omitido). |
| 6 | `01_core/src/rules/layout/bib_csl.rs:214` — `bib_entry_to_hayagriva` omite entrada por YAML/chave ausente | `typst-library/src/model/bibliography.rs:425-444` (`decode_library`) | **Não** | Erros de YAML propagados (`map_err(format_yaml_error)`). Chave vazia → erro. |
| 7 | `01_core/src/rules/layout/grid.rs:322` — `place_cells(...).unwrap_or_default()` descarta erro de grid inválida | Não existe `place_cells`; layout em `typst-layout/src/grid/` | **Sim (observable)** | Grid com células extra, colspan fora de limites ou conflitos de placement compila sem erro. Mecanismo diferente do cristalino (não detecta vs. detecta e descarta), mas o utilizador não é avisado. |
| 8 | `01_core/src/rules/eval/rules.rs:678` — `#set math.equation(numbering: <não-definido>)` silenciado por `.ok()` | `typst-eval/src/rules.rs:11-34` delega para `target.set(...)` | **Não** | Erro: `unknown variable: nao_existe` |
| 9 | `01_core/src/rules/eval/rules.rs:695` — `#set math.equation(numbering: <não-Str>)` ignorado | `EquationElem::set` via cast tipado | **Não** | Erro: `expected string, function, or none, found int` |
| 10 | `01_core/src/rules/eval/rules.rs:819` — `#set figure(numbering: <não-Str>)` ignorado | `FigureElem::set` via cast tipado | **Não** | Erro: `expected string, function, or none, found int` |
| 11 | `01_core/src/rules/eval/rules.rs:848` — `#set table(numbering: <não-Str>)` ignorado | `TableElem::set` via cast tipado | **Não** | Erro: `unexpected argument: numbering` (table vanilla não tem `numbering`) |
| 12 | `01_core/src/rules/eval/rules.rs:772` — `#set page(numbering: <não-Str/None>)` convertido silenciosamente | `PageElem::set` via cast tipado | **Não** | Erro: `expected string, function, or none, found int` |
| 13 | `01_core/src/rules/eval/rules.rs:785` — `#set page(columns: <não-Int>)` ignorado | `PageElem::set` via cast tipado | **Não** | Erro: `expected integer, found string` |
| 14 | `01_core/src/rules/eval/rules.rs:967` — `#set text(weight: <não-Int>)` ignorado | `TextElem::set` via cast tipado | **Não** | Erro: `expected integer, "thin", ...` |
| 15 | `01_core/src/rules/eval/rules.rs:706` — `#set document(title: 123)` silencioso | `DocumentElem::set` via cast tipado | **Não** | Erro: `expected content or none, found integer` |
| 16 | `01_core/src/rules/eval/rules.rs:746` — `#set page(width: "foo")` mantém dimensão anterior | `PageElem::set` via cast tipado | **Não** | Erro: `expected length or auto, found string` |
| 17 | `01_core/src/rules/stdlib/layout.rs:161` — `parse_track_sizing` devolve `None`; `grid(columns: "foo")` vira `auto` | `GridElem::set` via cast tipado | **Não** | Erro: `expected auto, relative length, fraction, integer, or array, found string` |
| 18 | `01_core/src/rules/eval/bindings.rs:141` — argumento posicional não-Str de `counter.display` ignorado | `Counter::display` em `typst-library/src/introspection/counter.rs:379-443` | **Não** | Erro: `expected string, function, or auto, found integer` |
| 19 | `01_core/src/rules/eval/bindings.rs:156` — `at:` inválido em `counter.display` ignorado | `Counter::display` via `Smart<LocatableSelector>` | **Não** | Erro: `expected label, function, location, selector, or auto, found integer` |
| 20 | `01_core/src/rules/eval/bindings.rs:164` — argumentos não reconhecidos em `counter.display` ignorados | `Counter::display` só aceita argumentos declarados | **Não** | Typos/argumentos extra dão erro de argumento inesperado |
| 21 | `01_core/src/rules/eval/bindings.rs:273` — equivalente a 19 no despacho sobre `Value::Counter` | `Counter::display` | **Não** | Mesmo erro que 19 |
| 22 | `01_core/src/rules/eval/bindings.rs:280` — equivalente a 18 no despacho sobre `Value::Counter` | `Counter::display` | **Não** | Mesmo erro que 18 |
| 23 | `01_core/src/rules/eval/bindings.rs:284` — argumento posicional ignorado quando pattern já definido | `Counter::display` | **Não** | Argumentos posicionais extra dão erro |

---

## 4. Prova de testes directos (amostra)

### 4.1 Casos em que o vanilla já dá erro

```text
$ lab/typst-original/target/release/typst compile break.typ out.pdf
error: cannot break outside of loop
  ┌─ break.typ:1:1
  │
1 │ #break
  │  ^^^^^

$ lab/typst-original/target/release/typst compile set_page_numbering.typ out.pdf
error: expected string, function, or none, found int
  ┌─ set_page_numbering.typ:1:21
  │
1 │ #set page(numbering: 123)
  │                      ^^^

$ lab/typst-original/target/release/typst compile counter_display_invalid.typ out.pdf
error: expected string, function, or auto, found integer
  ┌─ counter_display_invalid.typ:1:30
  │
1 │ #context counter("x").display(123)
  │                               ^^^

$ lab/typst-original/target/release/typst compile unicode_escape_code.typ out.pdf
error: invalid Unicode codepoint: FFFFFFFF
  ┌─ unicode_escape_code.typ:1:1
  │
1 │ "\u{FFFFFFFF}"
  │  ^^^^^^^^^^^^
```

### 4.2 Caso 7 — grid inválido compila sem erro no vanilla

```text
$ lab/typst-original/target/release/typst compile grid_out_of_bounds.typ out.pdf
$ lab/typst-original/target/release/typst compile grid_conflicting_colspan.typ out.pdf
$ lab/typst-original/target/release/typst compile grid_negative_colspan.typ out.pdf
```

Nenhum dos três produziu diagnóstico. Os PDFs foram gerados.

### 4.3 Caso 5 — bibliografia sem título

```text
$ lab/typst-original/target/release/typst compile bib_no_title.typ out.pdf
$ lab/typst-original/target/release/typst compile bib_no_title_bib.typ out.pdf
```

Nenhum erro. A entrada sem título é renderizada de forma degradada (`[1] Author,` / `[1] B,.`), mas não é omitida.

---

## 5. Padrões novos no vanilla?

A varredura dos seis padrões no vanilla não revelou nenhuma falha silenciosa nova nas áreas de âmbito que não estivesse já coberta pelos 23 casos do cristalino. As ocorrências encontradas foram classificadas como inofensivas:

- `.ok()` em conversões numéricas/bytes e lookups defensivos.
- `_ => {}` em contagem de caracteres para detecção de formato de bibliografia e em ignorar atributos CSL não suportados (com comentário).
- `let _ = self.eval(vm)?` — descarta o valor, mas propaga o erro.
- `unwrap_or_default/unwrap_or_else` — maior parte são defaults documentados (caption ausente, separator default, etc.).

---

## 6. Implicações para P634–P637

| Passo | Casos corrigidos | Vanilla já tinha erro claro? | Classificação |
|-------|------------------|------------------------------|---------------|
| P634 | Catch-all `eval_expr` (caso 3) | Sim | Correcção de paridade |
| P635 | `FlowEvent` para `#break`/`#continue`/`#return` | Sim | Correcção de paridade |
| P636 | Nove regras `#set` (casos 8–16) + `grid(columns: "foo")` (caso 17) | Sim | Correcção de paridade |
| P637 | Mensagem de erro de `document.title` | Sim | Correcção de paridade |

**Não há necessidade de alterar a classificação dos relatórios de P634–P637** para "melhoria além do vanilla", porque em todos os casos corrigidos o vanilla já produzia erro claro.

Os casos ainda por corrigir do P633, e a sua nova classificação face ao vanilla:

| Caso | Estado no cristalino (pós-P637) | Classificação face ao vanilla |
|------|----------------------------------|-------------------------------|
| 1, 2 | Escape unicode ainda silencioso | Correcção de paridade (vanilla já dá erro) |
| 4 | `state.update(func)` ainda silencioso | Correcção de paridade (vanilla já dá erro) |
| 5 | Bibliografia sem key/título ainda omitida | Correcção de paridade (vanilla dá erro para key vazia; sem título é aceite) |
| 6 | YAML/chave ausente ainda silencioso | Correcção de paridade (vanilla já dá erro) |
| 7 | Grid `unwrap_or_default` ainda silencioso | **Melhoria além do vanilla** (vanilla também não reporta) |
| 18–23 | `counter.display` ainda silencioso | Correcção de paridade (vanilla já dá erro) |

---

## 7. Estado de fecho

- [x] Sonda completa nas três áreas do vanilla (eval/regras `#set`, layout, export/bibliografia).
- [x] Os 23 casos já corrigidos/confirmados no cristalino verificados um a um contra o vanilla.
- [x] Padrões gerais do vanilla varridos; nenhum novo caso suspeito confirmado.
- [x] Testes directos no binário vanilla para todos os casos onde a sonda encontrava ambiguidade.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p638.md`.

---

## 8. Próximos passos sugeridos

1. Continuar a correcção dos casos 1, 2, 4, 5, 6, 18–23 como correcções de paridade.
2. Quando chegar ao caso 7 (grid), ter presente que será uma melhoria além do vanilla — o vanilla não reporta estes erros, pelo que o critério de aceitação não pode ser "byte-idêntico ao vanilla", mas sim "produzir erro claro".

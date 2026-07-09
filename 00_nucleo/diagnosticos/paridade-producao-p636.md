# Relatório de Paridade — P636

**Passo:** 636  
**Data:** 2026-07-09  
**Foco:** Corrigir nove casos de `#set` rules que ignoravam tipo inválido silenciosamente.  
**Hash do commit:** 79dfd4449

---

## Resumo

O Passo P633 confirmou nove casos em que uma regra `#set` recebia um valor do tipo errado e, em vez de produzir erro, ignorava o valor silenciosamente. Todos partilhavam a mesma causa estrutural: funções auxiliares (`value_to_eco_string`, `extract_pt`, etc.) devolviam `None` ou `.ok()` descartava o erro, e o caller tratava isso como "não fazer nada". Este passo converte esses nove casos em erros claros, no formato do vanilla (`expected {expected}, found {found}`).

---

## Sonda

### Padrão comum

Em `01_core/src/rules/eval/rules.rs`:

- `value_to_eco_string` (document title/author/keywords) devolvia `Option<EcoString>`, convertendo tipos inválidos em `None`.
- `extract_pt` (page width/height/margin) devolvia `Option<f64>`, convertendo tipos inválidos em `None`.
- `page.numbering`, `page.columns`, `figure.numbering`, `table.numbering`, `math.equation.numbering` usavam match com braço `_ => {}` ou `_ => None` que ignorava tipos inválidos.
- `text.weight` usava `Option<u16>` com fallback silencioso.
- `math.equation.numbering` ainda usava `.ok()` para descartar erros de avaliação (variável indefinida).

### Formato da mensagem de erro do vanilla

`lab/typst-original/crates/typst-library/src/foundations/cast.rs:325-335` gera mensagens no formato:

```
expected {expected}, found {found}
```

O cristalino usa nomes de tipo em inglês (`int`, `str`, `array`, etc.) via `Value::type_name()`, pelo que as mensagens seguem o mesmo formato com esses nomes.

---

## Implementação

### Ficheiros alterados

1. **`01_core/src/rules/eval/rules.rs`**:
   - Adiciona helper `type_mismatch(expected, found, span)`.
   - `math.equation.numbering`: propaga erros de `eval_expr` e rejeita tipos inválidos.
   - `document.title`/`author`/`keywords`: `value_to_eco_string` passa a devolver `SourceResult<Option<EcoString>>`; rejeita tipos inválidos e arrays com elementos não-string.
   - `page.width`/`height`/`margin`/`numbering`/`columns`: `extract_pt` devolve `SourceResult<Option<f64>>`; outros campos validam tipos explicitamente.
   - `figure.numbering`, `table.numbering`: rejeitam tipos inválidos.
   - `text.weight`: rejeita tipos inválidos e nomes simbólicos desconhecidos; `Value::Int` fora do range `u16` dá erro.

2. **`01_core/src/rules/eval/tests.rs`**:
   - Nove testes P633 renomeados de `_silent` para `_error` e invertidos para `p633_eval_fails`.
   - Teste Passo 129 (`eval_set_text_weight_simbolico_desconhecido_silent_passo_129`) atualizado para esperar erro.

3. **`00_nucleo/prompts/rules/eval.md`**: secção §P636 adicionada; hash atualizado para `aa004f05`.

### Mensagens de erro introduzidas

| Propriedade | Mensagem para tipo inválido |
|---|---|
| `page.width`/`height`/`margin` | `expected length, float, or int, found {tipo}` |
| `page.numbering` | `expected string or none, found {tipo}` |
| `page.columns` | `expected int, found {tipo}` (e `columns must be at least 1` para int < 1) |
| `document.title`/`author`/`keywords` | `expected string or array of strings, found {tipo}` |
| `text.weight` | `expected int or string, found {tipo}` (ou `unknown font weight name: {nome}` / `font weight must be between 100 and 900`) |
| `math.equation.numbering` | `expected string or none, found {tipo}` |
| `figure.numbering` | `expected string or none, found {tipo}` |
| `table.numbering` | `expected string or none, found {tipo}` |

---

## Validação

### Testes P633 invertidos

```
running 9 tests
test rules::eval::tests::tests::p633_set_page_width_string_error ... ok
test rules::eval::tests::tests::p633_set_page_numbering_int_error ... ok
test rules::eval::tests::tests::p633_set_figure_numbering_int_error ... ok
test rules::eval::tests::tests::p633_set_equation_numbering_undefined_error ... ok
test rules::eval::tests::tests::p633_set_table_numbering_int_error ... ok
test rules::eval::tests::tests::p633_set_page_columns_string_error ... ok
test rules::eval::tests::tests::p633_set_equation_numbering_int_error ... ok
test rules::eval::tests::tests::p633_set_document_title_int_error ... ok
test rules::eval::tests::tests::p633_set_text_weight_string_error ... ok
```

### Suite completa

```
cargo test --workspace
```

Resultado: todos os testes passaram (workspace completo; typst-core: 3626 passed; 0 failed).

### Linter

```
crystalline-lint .
```

Resultado: `✓ No violations found`.

### Testes directos (binário release)

Uso correcto:

```typst
#set page(numbering: "1")
#set text(weight: 700)
#set document(title: "Título correcto")
Texto.
```

Resultado: PDF gerado sem erro.

Caso de erro:

```typst
#set page(numbering: 123)
```

Resultado: `error: expected string or none, found int`.

---

## Estado de fecho

- [x] Sonda completa.
- [x] Nove casos corrigidos, com erro claro.
- [x] Testes de P633 invertidos e a passar.
- [x] Uso correcto sem regressão.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p636.md`.

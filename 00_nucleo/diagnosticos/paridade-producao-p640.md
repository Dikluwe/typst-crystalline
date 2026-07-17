# Relatório de Paridade — P640

**Passo:** 640  
**Data:** 2026-07-09  
**Foco:** Corrigir os seis casos de `counter.display(...)` que ignoravam argumentos inválidos, unificando a lógica duplicada.  
**Dependências:** P633 (casos confirmados), P638 (mensagens de erro do vanilla).

---

## 1. Sonda

### 1.1 Duplicação aparente

A sonda indicada no passo (`sed -n '130,300p' 01_core/src/engine/eval/bindings.rs`) revelou duas implementações de `counter.display`:

- `eval_counter_method` (`bindings.rs:96-194`) — destinada a chamadas do tipo `counter("x").display(...)`.
- `eval_counter_method_value` (`bindings.rs:238-323`) — destinada a despacho de método sobre `Value::Counter`.

As duas implementações repetiam a mesma lógica de parsing de `pattern` e `at:`.

### 1.2 Dead code

`grep` mostrou que `eval_counter_method` e `extract_counter_key` só eram referenciadas pelas suas próprias definições:

```bash
grep -rn "extract_counter_key\|eval_counter_method\b" 01_core/src/ --include="*.rs" | grep -v "//" | grep -v "pub(super) fn" | grep -v "pub fn"
# (sem resultados)
```

Conclusão: **a "duplicação" era, na verdade, código morto + uma implementação viva**. A forma pública de chamar `counter.display` no cristalino passa por `eval_counter_method_value` (despacho sobre `Value::Counter` em `closures.rs`). A implementação antiga em `eval_counter_method` nunca era atingida.

---

## 2. Implementação

### 2.1 Ficheiros alterados

1. **`01_core/src/engine/eval/bindings.rs`**:
   - Removido `extract_counter_key` (dead code).
   - Removido `eval_counter_method` (dead code).
   - Adicionadas três funções auxiliares:
     - `parse_counter_display_args` — parse/validação unificada dos argumentos de `counter.display`.
     - `extract_display_at_label` — extrai `Label` do argumento nomeado `at:`.
     - `render_counter_at_label` — renderiza `counter.display(..., at: <label>)`.
   - Refactorizado o braço `"display"` de `eval_counter_method_value` para usar as funções acima.

2. **`01_core/src/engine/eval/tests.rs`**:
   - `p633_counter_display_invalid_arg_silent` → `p633_counter_display_invalid_arg_error` (source ajustado para forçar avaliação imediata via `at:`).
   - `p633_counter_display_at_invalid_silent` → `p633_counter_display_at_invalid_error` (source ajustado para forçar avaliação imediata).
   - Adicionados 6 testes P640:
     - `p640_counter_display_pattern_works`
     - `p640_counter_display_at_label_works`
     - `p640_counter_display_unknown_named_arg_error`
     - `p640_counter_display_too_many_positional_args_error`
     - `p640_counter_display_invalid_arg_message`
     - `p640_counter_display_at_invalid_message`

### 2.2 Mensagens de erro

| Caso | Mensagem de erro (formato vanilla) |
|---|---|
| Argumento posicional inválido | `expected string, function, or auto, found {tipo}` |
| `at:` inválido | `expected label, function, location, selector, or auto, found {tipo}` |
| Argumento nomeado desconhecido | `unexpected argument: {nome}` |
| Muitos argumentos posicionais | `counter.display() takes at most one positional argument` |

### 2.3 Detalhe técnico: avaliação dentro de `#context`

Os testes originais de P633 usavam `#context(counter("x").display(...))`. O eval cristalino cria uma closure para `#context` sem avaliar imediatamente o body; a avaliação só acontece durante layout. Por isso, os testes P633 foram ajustados para usar `at:` sem contexto, forçando a avaliação imediata no eval e permitindo verificar os erros introduzidos.

---

## 3. Validação

### 3.1 Testes específicos

```bash
cargo test -p typst-core counter_display -- --nocapture
```

Resultado:

```text
test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 3606 filtered out
```

### 3.2 Suite completa

```bash
cargo test --workspace
```

Resultado: todos os testes passaram.

### 3.3 Linter

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## 4. Estado de fecho

- [x] Sonda completa: duplicação confirmada e dead code identificado.
- [x] Lógica unificada numa função auxiliar (`parse_counter_display_args`).
- [x] Seis casos corrigidos, com erro claro.
- [x] Testes de P633 invertidos e a passar.
- [x] Uso correcto sem regressão.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p640.md`.

---

## 5. Próximo passo

Continuar com os casos restantes de P633: escapes unicode (1, 2), `state.update` (4), bibliografia (5, 6).

# Relatório de Paridade — P648

**Passo:** 648  
**Data:** 2026-07-09  
**Foco:** Propagar erros de sintaxe do parser pelo eval sem repetir as regressões de P634.  
**Dependências:** P634 (débito identificado), P643 (escape unicode inválido em markup).  
**Hash do commit com as alterações:** `9c7b7d367`

---

## 1. Sumário

Foram corrigidos os dois últimos casos de falha silenciosa causada por erros de sintaxe descartados:

- **`#let x = 0xZZ`** passou a produzir erro (`invalid hexadecimal number: 0xZZ`).
- **Escape unicode inválido em markup** (`#let x = [\u{FFFFFFFF}]`) passou a produzir erro (`invalid Unicode codepoint: FFFFFFFF`).

A propagação é **selectiva**: apenas mensagens de lexer confirmadas como genuínas são propagadas. As 18 regressões observadas na sonda de P634 (smart quotes, `#set` dentro de blocos, `break`/`continue`/`return` dentro de ciclos/funções, etc.) não se repetiram.

A alteração exigiu uma pequena mudança no parser (`enter_modes`) para impedir que o re-lexing de fronteira entre modos descartasse o erro de lexer original.

---

## 2. Sonda

### 2.1 Reconstituição das regressões de P634

Sonda temporária em `01_core/src/engine/eval/mod.rs`: propagação indiscriminada de todos os `root.errors()` como `SourceDiagnostic::error`.

Resultado:

- `cargo test -p typst-core` falhou em **18 testes** (a suite cresceu desde P634):
  - Smart quotes: `eval_markup_apostrophe_possessivo_emite_u2019`, `eval_markup_smart_quotes_duplas_curly_com_lang_en`, `eval_markup_smart_quotes_simples_curly_com_lang_en`.
  - Fluxo de controlo: `p635_break_in_while_stops_loop`, `p635_break_in_for_stops_loop`, `p635_continue_in_for_skips_iteration`, `p635_return_stops_function_body`, `p635_nested_loops_break_only_inner`, `p635_return_from_function_with_value`, `p635_return_without_value`.
  - `#set` aninhado: `set_aninhado_multiple_niveis`, `set_dentro_bloco_nao_vaza_para_fora`, `set_dentro_closure_nao_afecta_caller`, `set_em_content_block_nao_vaza`, `set_false_reverte_set_true_em_bloco`, `show_rule_respeita_escopo_lexico`.
  - Casos já conhecidos: `p633_parse_error_expr_silent_none`, `p643_invalid_unicode_escape_markup_still_silent`.

### 2.2 Classificação das causas

|Mensagem de erro observada|Causa|Decisão|
|---|---|---|
|`expected semicolon or line break`|Parser recovery em código válido (fim de instrução implícito).|Não propagar.|
|`the character '#' is not valid in code`|Lexer assinala `#` dentro de blocos de código; `#set` aninhado em markup é válido.|Não propagar.|
|Smart quotes (mensagens do lexer/parser em nós de erro)|Aspas curly são válidas em markup; o parser cria nós de erro que devem ser ignorados.|Não propagar.|
|`invalid Unicode codepoint: ...`|Erro de lexer genuíno em markup.|Propagar.|
|`invalid hexadecimal number: ...`|Erro de lexer genuíno em code, mas perdido no re-lexing de fronteira.|Propagar + preservar no parser.|

---

## 3. Implementação

### 3.1 `01_core/src/engine/eval/mod.rs`

Adicionada, no entrypoint `eval_with_full_error`, propagação selectiva de erros de parser:

```rust
let syntax_errors: Vec<SourceDiagnostic> = root
    .errors()
    .into_iter()
    .filter(|e| {
        let msg = e.message.as_str();
        msg.starts_with("invalid hexadecimal number:")
            || msg.starts_with("invalid Unicode codepoint:")
    })
    .map(|e| SourceDiagnostic::error(e.span, e.message.to_string()))
    .collect();
if !syntax_errors.is_empty() {
    return Err(syntax_errors);
}
```

Só estas duas classes de mensagem são propagadas; todo o resto continua a ser ignorado pelo eval, evitando as regressões de P634.

### 3.2 `01_core/src/engine/parse/parser.rs`

Alterada `enter_modes` para preservar selectivamente erros de lexer quando o token de lookahead é re-lexado ao mudar de modo (ex: Code → Markup no fim de `#let x = 0xZZ`). Sem isto, o erro `invalid hexadecimal number: 0xZZ` era descartado e substituído por um nó `Text` em markup.

A preservação aplica-se apenas às mesmas duas classes de mensagem do ponto 3.1. Outras mensagens de lexer (como `"the character '#' is not valid in code"`) continuam a ser descartadas no re-lexing, mantendo as regressões de P634 afastadas.

### 3.3 `01_core/src/engine/eval/tests.rs`

- `p633_parse_error_expr_silent_none` → renomeado para `p648_parse_error_hex_literal_errors` e invertido para confirmar erro.
- `p643_invalid_unicode_escape_markup_still_silent` → renomeado para `p648_invalid_unicode_escape_markup_errors` e invertido para confirmar erro.

---

## 4. Validação

```bash
cargo test --workspace
```

Resultado: todos os crates passaram, sem falhas.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### 4.1 Testes específicos de P648

```bash
cargo test -p typst-core p648_ -- --nocapture
```

- `p648_parse_error_hex_literal_errors` — ok
- `p648_invalid_unicode_escape_markup_errors` — ok

### 4.2 Regressões de P634 confirmadas como não repetidas

Foram corridos individualmente os 18 testes que falharam na sonda indiscriminada; todos passaram.

### 4.3 Validação em CLI

```bash
cat > /tmp/p648-0xzz.typ <<'EOF'
#let x = 0xZZ
EOF
./target/release/typst /tmp/p648-0xzz.typ /tmp/p648-0xzz.pdf
```

Saída:

```text
/tmp/p648-0xzz.typ:1:9: error: invalid hexadecimal number: 0xZZ
exit code: 1
```

```bash
cat > /tmp/p648-escape-markup.typ <<'EOF'
#let x = [\u{FFFFFFFF}]
EOF
./target/release/typst /tmp/p648-escape-markup.typ /tmp/p648-escape-markup.pdf
```

Saída:

```text
/tmp/p648-escape-markup.typ:1:11: error: invalid Unicode codepoint: FFFFFFFF
exit code: 1
```

---

## 5. Decisão

- A propagação de erros de parser pelo eval não pode ser catch-all sem causar regressões massivas.
- A propagação selectiva por prefixo de mensagem é suficiente para os casos identificados em P634/P643.
- Para `0xZZ` em contexto de markup, a causa raiz estava no re-lexing de fronteira do parser, não no eval. A correção foi feita no parser de forma cirúrgica e limitada às mesmas classes de erro.
- Nenhum bug de parser adicional que marque código válido como erro foi introduzido; as regressões conhecidas foram testadas uma a uma.

---

## 6. Estado da sequência de falhas silenciosas

Com este passo, a linha de trabalho iniciada em P633 está fechada: todos os casos confirmados de falha silenciosa ou perda de erro estão corrigidos ou reclassificados com razão documentada.

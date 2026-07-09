# Relatório de Paridade — P643

**Passo:** 643  
**Data:** 2026-07-09  
**Foco:** Escape unicode inválido (`\u{FFFFFFFF}`) — corrigir o silêncio no cristalino.  
**Dependências:** P633 (casos 1, 2 confirmados), P638 (mensagem do vanilla confirmada).

---

## 1. Sonda — onde vive o problema

### 1.1 Code string (`"\u{FFFFFFFF}"`)

O lexer de strings (`01_core/src/rules/lexer/code.rs:224-237`) captura o texto literal e devolve `SyntaxKind::Str` sem validar o conteúdo do escape. A validação acontece em `01_core/src/entities/ast/expr.rs:357-393`, no método `Str::get()`, que usava:

```rust
u32::from_str_radix(sequence, 16)
    .ok()
    .and_then(std::char::from_u32)
```

Quando `from_u32` devolvia `None`, o escape era ignorado e o texto literal original era preservado.

### 1.2 Markup (`[\u{FFFFFFFF}]`)

O lexer de markup (`01_core/src/rules/lexer/markup.rs:59-75`) **já detecta** o escape inválido:

```rust
if u32::from_str_radix(hex, 16)
    .ok()
    .and_then(std::char::from_u32)
    .is_none()
{
    return self.error(format!("invalid Unicode codepoint: {}", hex));
}
```

No entanto, o resultado é um **nó de erro** no AST, e o eval de markup (`01_core/src/rules/eval/mod.rs:419-588`) não propaga nós de erro do parser. O CLI do cristalino compila `[\u{FFFFFFFF}]` com exit code 0 — o erro do lexer é silenciado pelo eval.

### 1.3 Implicação da sonda

A correção de **code string** pode ser feita isoladamente em `Str::get()`. A correção de **markup** exige propagar erros de parser no eval, o mesmo risco de regressão identificado em P634 (`0xZZ`, smart quotes, `#set` dentro de blocos). Por isso, markup não é corrigido neste passo.

---

## 2. Implementação

### 2.1 Code string

`01_core/src/entities/ast/expr.rs`:

- Adicionados imports de `SourceDiagnostic` e `SourceResult`.
- `Str::get()` passou a devolver `SourceResult<String>`.
- Quando `from_u32` devolve `None`, a função devolve:

```rust
Err(vec![SourceDiagnostic::error(
    span,
    format!("invalid Unicode codepoint: {}", sequence.to_uppercase()),
)])
```

A mensagem replica a do vanilla, confirmada por P638.

### 2.2 Callers atualizados

`Str::get()` é chamado em vários locais de eval; todos foram atualizados para propagar o erro:

- `01_core/src/rules/eval/mod.rs:599` (`Expr::Str`)
- `01_core/src/rules/eval/mod.rs:770` (dict key)
- `01_core/src/rules/eval/rules.rs:1076` (set text font)
- `01_core/src/rules/eval/rules.rs:1083` (font array)
- `01_core/src/rules/eval/rules.rs:1500` (font dict legacy)
- `01_core/src/entities/ast/code.rs:162` (`ModuleImport::bare_name` — mapeado para `BareImportError::PathInvalid`)

### 2.3 Markup — não alterado

`Escape::get()` em `01_core/src/entities/ast/markup.rs` foi deixado inalterado. O lexer já produz o erro correcto; o problema é a propagação pelo eval, que deve ser tratada no passo dedicado ao parser/lexer (já recomendado em P634).

---

## 3. Testes

### 3.1 Code string — agora dá erro

`p643_invalid_unicode_escape_code_string_errors` substituiu `p633_invalid_unicode_escape_preserved`:

```rust
let world = MockWorld::new("#let x = \"\\u{FFFFFFFF}\"");
let err = eval_for_test(&world, &world.source).unwrap_err();
let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
assert!(msg.contains("invalid Unicode codepoint: FFFFFFFF"));
```

### 3.2 Markup — comportamento actual documentado

`p643_invalid_unicode_escape_markup_still_silent` substituiu `p633_invalid_unicode_escape_markup_preserved`, documentando que o markup ainda é silencioso até ao passo dedicado ao parser:

```rust
let world = MockWorld::new("#let x = [\\u{FFFFFFFF}]");
let module = eval_for_test(&world, &world.source).expect("eval deve suceder");
assert!(module.scope().get("x").is_some());
```

---

## 4. Validação

```bash
cargo test --workspace
```

Resultado: **todos os testes passaram**.

```bash
crystalline-lint .
```

Resultado: **✓ No violations found**.

---

## 5. Decisão

- **Code string** corrigido: erro propagado, mensagem igual à do vanilla.
- **Markup** deixado para passo dedicado ao parser/lexer, porque o lexer já detecta o erro e a falha está na propagação de nós de erro pelo eval — o mesmo domínio de P634 (`0xZZ`).

---

## 6. Estado de fecho

- [x] Sonda mínima completa — code string corrigível isoladamente; markup requer passo parser/lexer.
- [x] Code string corrigido com mensagem igual à do vanilla.
- [x] Callers de `Str::get()` atualizados.
- [x] Teste de code string invertido para confirmar erro.
- [x] Teste de markup actualizado para documentar comportamento silencioso.
- [x] `cargo test --workspace` sem regressões.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p643.md`.

---

## 7. Hash do commit

`PENDING`

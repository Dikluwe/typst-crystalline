# Relatório de Paridade — P637

**Passo:** 637  
**Data:** 2026-07-09  
**Foco:** Confirmar se `document.title` no vanilla aceita array (como `author`/`keywords`) ou só string. Corrigir a mensagem de erro se necessário.  
**Hash do commit:** bb8cea050

---

## Verificação

### Código fonte do vanilla

`lab/typst-original/crates/typst-library/src/model/document.rs:158-167`:

```rust
pub title: Option<Content>,
pub author: OneOrMultiple<EcoString>,
pub description: Option<Content>,
pub keywords: OneOrMultiple<EcoString>,
```

- `title` é `Option<Content>` — um único valor de conteúdo, **não array**.
- `author` e `keywords` são `OneOrMultiple<EcoString>` — string ou array de strings.

### Conclusão

A mensagem partilhada introduzida em P636 (`"expected string or array of strings"`) estava incorrecta para `title`. `title` só deve aceitar string (no cristalino, `DocumentInfo.title` é `EcoString`; content fica fora de scope).

---

## Implementação

### Ficheiros alterados

1. **`01_core/src/rules/eval/rules.rs`**:
   - `value_to_eco_string` ganha um parâmetro `allow_array: bool`.
   - `document.title` chama com `allow_array = false`.
   - `document.author`/`keywords` chamam com `allow_array = true`.

2. **`01_core/src/rules/eval/tests.rs`**:
   - `p637_document_title_array_is_error`: confirma que array em `title` dá erro.
   - `p637_document_author_array_works`: confirma que array em `author` funciona.
   - `p637_document_keywords_array_works`: confirma que array em `keywords` funciona.

3. **`00_nucleo/prompts/rules/eval.md`**: secção §P636 actualizada para separar `title` de `author`/`keywords`; hash actualizado para `a8523b4b`.

### Mensagens de erro resultantes

| Propriedade | Mensagem para tipo inválido |
|---|---|
| `document.title` | `expected string, found {tipo}` |
| `document.author`/`keywords` | `expected string or array of strings, found {tipo}` |

---

## Validação

### Testes P637

```
running 3 tests
test rules::eval::tests::tests::p637_document_title_array_is_error ... ok
test rules::eval::tests::tests::p637_document_author_array_works ... ok
test rules::eval::tests::tests::p637_document_keywords_array_works ... ok
```

### Suite completa

```
cargo test --workspace
```

Resultado: todos os testes passaram.

### Linter

```
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## Estado de fecho

- [x] Tipo de `title` confirmado no código fonte do vanilla.
- [x] Mensagem corrigida para `title` (só string; `author`/`keywords` mantêm array).
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p637.md`.

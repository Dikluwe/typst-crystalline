# Prompt L0 — `stdlib/assert` — módulo `assert`
Hash do Código: 479b38f4

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/assert.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com marco
P66 (prova de fogo dos named args, DEBT-16).
**ADRs**: ADR-0037 (coesão por domínio), ADR-0054 (perfil graded).
**Convenções partilhadas**: ver `00_nucleo/prompts/rules/stdlib/_comum.md`.

---

## Módulo `assert` — função nativa `assert`

Este módulo implementa a função global `assert` de Typst: verificação de
condição booleana em tempo de avaliação, com mensagem de erro customizável.

---

### `native_assert(condition, message?)`

**Assinatura**: `assert(condition: Bool, message: Str | Content?) -> None`

**Argumentos**:
- 1º posicional `condition`: `Bool` obrigatório.
- `message`: argumento nomeado opcional. Aceita `Str`, `Content` (converte via
  `plain_text()`), ou outro tipo (usa `type_name()`). Default:
  `"Asserção falhou"`.
- Não aceita outros argumentos nomeados.

**Semântica**:
- Se `condition == true`, devolve `Value::None` (sem output).
- Se `condition == false`, devolve `Err` com a mensagem resolvida.

**Paridade vanilla**: Equivalente a `assert(cond, message: "...")` / `assert(cond)`.

**Limitações / scope-outs**:
- Não produz backtrace detalhado além da mensagem.
- Mensagem default em português ("Asserção falhou"); vanilla usa inglês.

**Testes canónicos**:
```
assert(true) -> Ok(None)
assert(false) -> Err "Asserção falhou"
assert(false, message: "condição falhou") -> Err "condição falhou"
assert(1 == 1) -> Ok(None)
assert(1 == 2, message: [erro rico]) -> Err "erro rico"  (Content convertido para plain_text)
assert() -> Err "assert() requer 1 argumento posicional (condição)"
assert(123) -> Err "assert() requer condição booleana"
assert(true, foo: 1) -> Err "argumento nomeado inesperado: 'foo'"
```

# Prompt L0 — `stdlib/assert` — módulo `assert`
Hash do Código: c4394a42

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/assert.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com marco
P66 (prova de fogo dos named args, DEBT-16). P723: namespace `assert.eq` /
`assert.ne` (paridade vanilla `foundations/mod.rs:198-247`), isolado via
`cetz` (`shapes.typ:151,249,484,897,1387`, `anchor.typ:123,186`,
`boolean.typ:189`).
**ADRs**: ADR-0037 (coesão por domínio), ADR-0054 (perfil graded).
**Convenções partilhadas**: ver `00_nucleo/prompts/compiler/stdlib/_comum.md`.

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
  `plain_text()`), ou outro tipo (usa `type_name()`).
- Não aceita outros argumentos nomeados.

**Semântica**:
- Se `condition == true`, devolve `Value::None` (sem output).
- Se `condition == false`, devolve `Err` com a mensagem resolvida.

**Paridade vanilla (P843 F7 — mensagens medidas em `temp/p843/f7_*.typ`)**:
- Default: `"assertion failed"`.
- Com `message:` → `"assertion failed: {msg}"` — concatenado directo, sem
  prefixo `message:` (antes: mensagem nua).
- As mensagens são o observável (ADR-0107) — inglês, verbatim do vanilla
  (`foundations/mod.rs:179-181`). Antes de P843: `"Asserção falhou"`.

**Limitações / scope-outs**:
- Não produz backtrace detalhado além da mensagem.

**Testes canónicos**:
```
assert(true) -> Ok(None)
assert(false) -> Err "assertion failed"
assert(false, message: "custom msg") -> Err "assertion failed: custom msg"
assert(1 == 1) -> Ok(None)
assert(1 == 2, message: [erro rico]) -> Err "assertion failed: erro rico"  (Content convertido para plain_text)
assert() -> Err "assert() requer 1 argumento posicional (condição)"
assert(123) -> Err "assert() requer condição booleana"
assert(true, foo: 1) -> Err "argumento nomeado inesperado: 'foo'"
```

---

### `native_assert_eq` / `native_assert_ne` — namespace de `assert` (P723)

**Assinatura**: `assert.eq(left: any, right: any, message: Str | Content?) -> None`;
`assert.ne` idem.

**Registo**: `assert` passa a `Func::native_with_namespace` (mesmo mecanismo
de P513 `curve` / P512 `grid`, `eval/mod.rs`), com entradas `eq` e `ne`.

**Semântica** (paridade vanilla `foundations/mod.rs:198-247`):
- `assert.eq`: erro se `left != right`; `assert.ne`: erro se `left == right`.
  Igualdade com a coerção Int↔Float do `==` da linguagem (mesma regra de
  `value_eq` em `operators.rs`; vanilla usa o `PartialEq` de `Value`, que
  coage Int/Float).
- Sem `message`: mensagem default do vanilla, em inglês (a mecânica **é** o
  observável — mensagem de erro):
  - eq: `equality assertion failed: value {repr(left)} was not equal to {repr(right)}`
  - ne: `inequality assertion failed: value {repr(left)} was equal to {repr(right)}`
- Com `message`: `equality assertion failed: {message}` /
  `inequality assertion failed: {message}`.
- Sucesso → `Value::None` (sem output).

**Consumidor confirmado**: `cetz` 0.5.2 — `shapes.typ:151` (`assert.eq` no
caminho de qualquer forma, incluindo `line`/`circle` do documento de
reprodução), `shapes.typ:249,484,897,1387`, `anchor.typ:123,186`,
`boolean.typ:189` (`assert.ne`).

**Testes canónicos**:
```
assert.eq(1, 1)              -> Ok(None)
assert.eq(1, 2)              -> Err "equality assertion failed: value 1 was not equal to 2"
assert.eq(1, 2, message: "x") -> Err "equality assertion failed: x"
assert.eq(1, 1.0)            -> Ok(None)   (coerção Int↔Float)
assert.ne(1, 2)              -> Ok(None)
assert.ne(1, 1)              -> Err "inequality assertion failed: value 1 was equal to 1"
assert.ne(1, 1, message: "x") -> Err "inequality assertion failed: x"
```

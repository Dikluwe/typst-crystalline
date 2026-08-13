# Prompt L0 — `stdlib/foundations/str` — `str`, `str.from-unicode`, `regex`
Hash do Código: edbc87cb

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/str.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1032 — extraído de `foundations.rs`; `regex` absorvido de
`text/regex.rs` (débito do Passo 1022).
**ADRs**: ADR-0107 (paridade linguagem), ADR-0109 (atomização forma B).
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Funções

### `native_str` — `str(v)` / `str(int, base:)`

**Assinatura**: `str(v: any) -> str` / `str(int: Int, base: Int) -> str`

**Semântica**: Converte o valor para string. Suporta `none`, `bool`, `int`,
`float`, `str`, `auto`, `length`, `ratio`, `angle`, `bytes` (UTF-8). Rejeita
`color`. `base` aplica-se só a `Int`, entre 2 e 36.

**Testes canônicos**:
```
str(42)              -> "42"
str(3.0)             -> "3.0"
str(255, base: 16)   -> "ff"
str(12pt + 1em)      -> "12pt + 1em"
str(<bytes UTF-8>)   -> "hello"
str(<bytes inválidos>) -> Err "bytes are not valid UTF-8"
str(red)             -> Err "str() não suporta color"
```

### `native_str_from_unicode` — `str.from-unicode(codepoint)`

**Assinatura**: `str.from-unicode(codepoint: int) -> str`

**Semântica**: Converte um scalar Unicode num carácter. Rejeita valores
inválidos e `\0`.

### `native_regex` — `regex(pattern)`

**Assinatura**: `regex(pattern: str) -> regex`

**Semântica**: Compila uma pattern regex e devolve `Value::Regex`. Pattern
inválida → erro com a mensagem do motor. Argumentos nomeados rejeitados.
Movido de `text/regex.rs` porque `regex` é constructor de tipo do domínio
`foundations`, não função de texto/estilo.

**Testes canônicos**:
```
regex("a+")           -> Value::Regex compilada
regex("[")            -> Err "regex inválida: ..."
regex(1)              -> Err "regex() espera string, recebeu integer"
regex("a", foo: 1)    -> Err (nomeado inesperado)
```

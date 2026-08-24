# Prompt L0 — `stdlib/foundations/str` — `str`, `str.from-unicode`
Hash do Código: 17cf37a3

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/str.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1032 — extraído de `foundations.rs`. P1140.1-A move `regex`
para a sua unidade dona `foundations/regex.rs` sem mudança de comportamento.
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

`native_regex` e seus testes pertencem ao L0 irmão
`compiler/stdlib/foundations/regex.md` desde P1140.1-A.

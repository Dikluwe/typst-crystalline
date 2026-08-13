# Prompt L0 — `stdlib/foundations/len` — `len(v)`
Hash do Código: ae552e1e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/len.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1032 — extraído de `foundations.rs`.
**ADRs**: ADR-0107 (paridade linguagem).
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Função

### `native_len` — `len(v)`

**Assinatura**: `len(v: str | array | dict) -> int`

**Semântica**: Extensão do cristalino (o vanilla não tem função global `len`).
- `Str` → número de caracteres Unicode (`chars().count()`).
- `Array` → número de elementos.
- `Dict` → número de entradas.

> **Nota P1031**: a contagem por codepoints desta função global diverge do
> método `.len()` de `str`, que conta bytes UTF-8 (paridade vanilla).
> Mudar/remover é comportamento por defeito → gate ADR-0127; não implementado
> aqui.

**Testes canônicos**:
```
len("abc")           -> 3
len("é")             -> 1
len((1, 2, 3))       -> 3
len((:))             -> 0
len(1)               -> Err "len() não suporta int"
len()                -> Err "len() requer 1 argumento"
```

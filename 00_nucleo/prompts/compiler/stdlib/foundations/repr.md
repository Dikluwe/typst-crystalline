# Prompt L0 — `stdlib/foundations/repr` — `repr(v)`
Hash do Código: ad59b55a

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/repr.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1032 — extraído de `foundations.rs`.
**ADRs**: ADR-0107 (paridade linguagem).
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Função

### `native_repr` — `repr(v)`

**Assinatura**: `repr(v: any) -> str`

**Semântica**: Devolve uma representação textual reconhecível do valor.
A implementação real vive em `compiler/eval/repr.rs` (`repr_value`); este nó
é apenas a nativa que a expõe no scope global.

**Testes canônicos**:
```
repr(1)             -> "1"
repr(1.5)           -> "1.5"
repr("abc")         -> "\"abc\""
repr(none)          -> "none"
repr(6pt)           -> "6pt"
repr(rgb("#ff0000")) -> "rgb(\"#ff0000\")"
repr()              -> Err "repr() requer 1 argumento"
```

# Prompt L0 — `stdlib/foundations/color` — construtores de cor
Hash do Código: c7d0f7ec

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/color.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1032 — extraído de `foundations.rs`.
**ADRs**: ADR-0083 (color spaces), ADR-0107 (paridade linguagem).
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Funções

### `native_rgb` — `rgb(...)`

Forma numérica: Int [0,255] ou Ratio [0%,100%] por componente.
Forma hex: string 3/4/6/8 dígitos, `#` opcional. Mensagens verbatim do vanilla.

### `native_luma` — `luma(l)`

Aceita Int [0,255], Ratio ou `Relative` sem parte absoluta. Fallback silencioso
para branco em qualquer erro ou ausência de argumento (paridade vanilla
`Component`, medido P705).

### `native_oklab` / `native_oklch` / `native_linear_rgb` / `native_cmyk` /
### `native_hsl` / `native_hsv`

Construtores dos respetivos espaços de cor. `linear_rgb` aceita Int/Ratio e
rejeita Float (P736). Os restantes aceitam Float/Int.

## 2. Critérios de verificação

```
rgb(255, 0, 128)              -> Color::rgb(...)
rgb(50%, 0%, 0%)              -> rgb("#800000")
rgb("#FF0000")                -> vermelho opaco
rgb(300, 0, 0)                -> Err "number must be between 0 and 255"
rgb("FFFFF")                  -> Err "color string has wrong length"
luma(128)                     -> cinza 50%
luma(300)                     -> branco (fallback silencioso)
luma()                        -> branco
linear_rgb(255, 0, 0)         -> vermelho linear
linear_rgb(0.5, 0.5, 0.5)     -> Err "expected integer or ratio, found float"
cmyk(0.0, 1.0, 0.0, 0.0)      -> magenta
```

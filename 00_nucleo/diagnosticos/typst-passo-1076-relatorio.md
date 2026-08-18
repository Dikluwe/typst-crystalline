# Relatório de Execução — Passo 1076: `color.mix` em sRGB Difere 1 Unidade no Canal Verde — Achado #11 do P1031

**Data**: 2026-08-18
**Passo**: 1076 — `color.mix` em sRGB Difere 1 Unidade no Canal Verde (Achado #11 do P1031)
**Gate**: `ADR-0127` (Classificação: Mudança de Comportamento por Defeito / Paridade com a Linguagem Typst)
**Status**: CONCLUÍDO COM ÊXITO (Causa comprovada, arredondamento *ties-to-even* implementado, 100% de paridade com Vanilla Typst)

---

## 1. Identificação Factual da Causa Raiz (§1 e §2 do L0)

### 1.1 Auditoria do Vanilla Typst e da Crate `palette`
No Vanilla Typst (`crates/typst-library/src/visualize/color.rs:1525`), a conversão de float para representação de 8 bits por canal (`u8`) é delegada para a crate `palette` (`into_format::<u8, u8>()`).

Na implementação do trait `FromStimulus<f32>` para `u8` em `palette-0.7.6/src/stimulus.rs:155-167`:
```rust
// Float to uint conversion with rounding to nearest even number. Formula
// follows the form (x_f32 + C23_f32) - C23_u32, where x is the component. From
// Hacker's Delight, p. 378-380.
const C23: u32 = 0x4b00_0000;
let f = scaled + f32::from_bits(C23);
(f.to_bits().saturating_sub(C23)) as u8
```
A crate `palette` adota **Round to Nearest Even** (*Banker's Rounding* / arredondamento para o inteiro par mais próximo).

### 1.2 Auditoria do Crystalline
No Crystalline ([`01_core/src/entities/color.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/color.rs)), a função `to_srgb()` utilizava:
```rust
(g.clamp(0.0, 1.0) * 255.0).round() as u8
```
O método `f32::round()` do Rust adota *round ties away from zero* (empate afasta do zero).

* Para o canal verde no mix 50% de `rgb(255, 65, 54)` com `rgb(0, 116, 217)`:
  * $g_0 = 65 / 255$, $g_1 = 116 / 255 \implies g_{\text{mix}} = 0.35490196 \implies g \times 255 = 90.5$.
  * **Vanilla (`palette` ties-to-even)**: $90.5 \to \mathbf{90}$ (`0x5a`), pois 90 é par.
  * **Crystalline anterior (`.round()`)**: $90.5 \to \mathbf{91}$ (`0x5b`).

A causa raiz foi **exatamente a Hipótese #1** (modo de arredondamento no limiar de `.5`).

---

## 2. Alterações Implementadas (§4 do L0)

Em [`01_core/src/entities/color.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/color.rs):

```rust
/// Converte float em [0.0, 1.0] para u8 em [0, 255] com arredondamento
/// "round ties to even" (paridade palette crate / Vanilla Typst, Hacker's Delight).
#[inline]
pub fn f32_to_u8_ties_even(val: f32) -> u8 {
    let scaled = (val.clamp(0.0, 1.0) * 255.0).min(255.0);
    const C23: u32 = 0x4b00_0000;
    let f = scaled + f32::from_bits(C23);
    (f.to_bits().saturating_sub(C23)) as u8
}

pub fn to_srgb(&self) -> (u8, u8, u8, u8) {
    let (r, g, b, a) = self.to_rgba_f32();
    (
        Self::f32_to_u8_ties_even(r),
        Self::f32_to_u8_ties_even(g),
        Self::f32_to_u8_ties_even(b),
        Self::f32_to_u8_ties_even(a),
    )
}
```

---

## 3. Medição Diferencial e Validação Ampla (§3 e §5 do L0)

Foram executados múltiplos casos com proporções e cores variadas no binário oficial (`/usr/local/bin/typst`) e no binário compilado (`target/release/typst`):

### 3.1 Tabela de Medição

| # | Operação / Documento | Vanilla Typst | Crystalline (Antes) | Crystalline (P1076) | Paridade |
| :---: | :--- | :---: | :---: | :---: | :---: |
| 1 | `#rgb(255, 65, 54).mix(rgb(0, 116, 217), space: rgb)` | `#805a88` | `#805b88` ❌ | `#805a88` | **100% IDÊNTICO** |
| 2 | `#rgb(255, 65, 54).mix(rgb(0, 116, 217), space: oklab)` | `#a37095` | `#a37095` | `#a37095` | **100% IDÊNTICO** |
| 3 | `#color.mix(c, d, weight: 25%, space: rgb)` | `#bf4e5f` | `#bf4e5f` | `#bf4e5f` | **100% IDÊNTICO** |
| 4 | `#color.mix(c, d, weight: 75%, space: rgb)` | `#4067b0` | `#4067b0` | `#4067b0` | **100% IDÊNTICO** |
| 5 | `#color.mix(white, black, weight: 50%, space: rgb)` | `#808080` | `#808080` | `#808080` | **100% IDÊNTICO** |

---

## 4. Prompts e Testes Automatizados

1. **Prompt L0**:
   * [`00_nucleo/prompts/entities/color.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/color.md) atualizado com a prova matemática e selado (Hash: `18570291`).
2. **Testes Unitários**:
   * Adicionados testes dedicados `p1076_mix_srgb_ties_to_even_paridade_vanilla` e `p1076_ties_to_even_halfway_behavior` em [`01_core/src/entities/color.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/color.rs).
   * Atualizado teste existente `p744_space_nomeado_mix_negate_rotate` em [`01_core/src/compiler/eval/tests.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/eval/tests.rs).
3. **Linter & Testes**:
   * `crystalline-lint .`: **0 erros, 0 avisos de drift**.
   * `cargo test --workspace`: **5.950 testes aprovados (100% PASS)**.

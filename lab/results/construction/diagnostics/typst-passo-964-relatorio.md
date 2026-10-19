# Passo 964 — Relatório (variantes gregas de símbolo + ∂ no plano itálico math)

**Data**: 2026-08-04
**Estado da árvore**: commit base `acacf036d` (P963); alterações deste passo por cima.

---

## 1. Fase A — divergência confirmada, com codepoints reais (não suposição)

A suspeita da auditoria confirmou-se por medição directa (ToUnicode via
`pdftotext`, doc isolado `temp/p964/greek.typ` com as 4 linhas da secção 2):
o vanilla mapeia as formas de símbolo gregas para o plano
mathematical-italic; o cristalino deixava-as no bloco grego. Tabela medida:

| char | vanilla | cristalino (antes) |
|---|---|---|
| ϵ U+03F5 | U+1D716 𝜖 | U+03F5 |
| ϑ U+03D1 | U+1D717 𝜗 | U+03D1 |
| ϖ U+03D6 | U+1D718 𝜘 | U+03D6 |
| ϕ U+03D5 | U+1D719 𝜙 | U+03D5 |
| ϱ U+03F1 | U+1D71A 𝜚 | U+03F1 |
| ϰ U+03F0 | U+1D71B 𝜅 | U+03F0 |
| ∂ U+2202 | U+1D715 𝜕 | U+2202 |

Confirmação contra a fonte canónica (codex `styling.rs`, grupo "greek"): as
7 formas pertencem ao grupo e o vanilla mapeia-as. E o vanilla **não**
mapeia ϐ (U+03D0), Ϝ/ϝ (U+03DC/DD), ϴ (U+03F4), ∇ (U+2207) — o cristalino
já coincidia nesses; guardas adicionados para não os tocar.

## 2. Fase B — implementação (TDD directo, mapeamento de tabela)

- L0: `entities/math_style.md` §P964 (tabela medida + escopo revogado de
  P809 registado).
- Testes RED: `p964_greek_variantes_e_partial_no_plano_italico` (7 mapeamentos
  + 4 guardas não-mapeados) e `p964_is_math_italic_default_cobre_variantes`.
  Testes antigos actualizados (codificavam o scope-out revogado):
  `p809_greek_outros_kinds_passthrough` (removidas as asserções ϑ/∂
  pass-through) e `p809_is_math_italic_default` (ϑ passa a true).
- Correcção: `is_math_italic_default` cobre os 7 codepoints; `greek_plain`
  (Plain+italic) mapeia cada um explicitamente (fora dos blocos contíguos).
- Suite: `cargo test --workspace` — **5708 testes, 0 falhas**.
- **Confirmação de codepoint**: o doc isolado da secção 2 produz agora
  codepoints **byte-idênticos aos do vanilla** (conjunto completo comparado,
  `iguais: True`).

## 3. Revalidação

- Doc de 30 secções: as 4 variantes da secção 2 mapeiam como o vanilla.
  `compare.py`: sec 2 med|dx| 0.561 (posições já eram próximas — a
  correcção é de identidade de codepoint); sec 26: 3.65 → **2.89**.
- **Resíduo registado (fora do escopo da tabela)**: φ/ψ dentro de
  `bra(phi)`/`ket(psi)`/`expval(...)` — funções de UTILIZADOR definidas via
  templates de markup (`#let bra(x) = [⟨#x\|]`) — ficam no bloco grego
  porque o conteúdo produzido por templates de markup dentro de math não é
  atravessado por `apply_math_default` (que só recursa em containers math).
  É a fronteira "conteúdo de função de utilizador em math" — mecanismo
  distinto de tabela de símbolos, candidato a passo próprio (afecta as
  contagens residuais de U+1D711/1D713 vs vanilla).
- Benchmark: ver tabela (hyperfine, "antes" = release pós-P963; JSONs em
  `tools/perf/results/p964-canonical/`).

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 85.39 | 86.64 | 1.015 |
| 02-lorem | 104.85 | 104.68 | 0.998 |
| 03-images | 91.90 | 92.21 | 1.003 |
| 04-math | 117.46 | 118.73 | 1.011 |
| 05-tables | 88.72 | 89.86 | 1.013 |
| 06-long | 285.67 | 287.56 | 1.007 |
| 07-context | 123.85 | 125.13 | 1.010 |

Ratio médio **1.008** — sem regressão (mapeamento de tabela; spread dentro
do ruído de medição).

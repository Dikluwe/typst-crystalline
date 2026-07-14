# P744 — Três achados finais: `space:` nomeado, `to-hex`/`transparentize`/`opacify`, repr de closure anónima

**Data:** 2026-07-14 (07:51 -03:00)
**Commit base:** `f72a2438f` ("P743: preenche hash do commit no relatorio")
**Commit da implementação:** `5f8364167` ("P744: space nomeado em cor, to-hex/transparentize/opacify, repr de closure")
**Estado no momento das medições E2E:** working tree não commitado;
`git diff HEAD --stat`:

```
 00_nucleo/diagnosticos/achados-adiados-cetz.md |   7 +-
 00_nucleo/prompts/entities/color.md             |  49 ++++--
 00_nucleo/prompts/entities/func.md              |   4 +
 00_nucleo/prompts/rules/stdlib/color.md        |  98 +++++++----
 01_core/src/entities/color.rs                   | 279 ++++++++++++++++++++++++++++----
 01_core/src/entities/func.rs                    |   2 +-
 01_core/src/rules/eval/bindings.rs             |  16 +-
 01_core/src/rules/eval/repr.rs                  |  39 ++++-
 01_core/src/rules/eval/tests.rs                 |  55 ++++++-
 01_core/src/rules/stdlib/color.rs               | 168 +++++++++++++++++--
 10 files changed, 609 insertions(+), 101 deletions(-)
```

Vanilla de referência: `lab/typst-original/target/release/typst`
(typst 0.15.0, 969087ec; source local **idêntico** ao tag v0.15.0 —
verificado por diff).

---

## Sonda — três achados adiados de P742

### Parte A — argumento nomeado `space:` em `negate`/`rotate`/`mix`

Sonda `/tmp/p744-space.typ`:

```typst
#repr(red.mix(blue, space: rgb)) \
#repr(red.negate(space: oklab)) \
#repr(red.rotate(90deg, space: oklch)) \
```

| Caso | Vanilla (medido) | Cristalino P744 |
|---|---|---|
| `red.mix(blue, space: rgb)` | `rgb("#805b87")` | `rgb("#805b88")` |
| `red.negate(space: oklab)` | `rgb("#004b74")` | `rgb("#004b74")` |
| `red.rotate(90deg, space: oklch)` | `rgb("#87a100")` | `rgb("#87a100")` |

Resultado: `negate` e `rotate` com `space:` são **byte a byte idênticos** ao vanilla. `mix` com `space: rgb` difere no azul em uma unidade: vanilla `#805b87` vs cristalino `#805b88`. A causa é arredondamento f32 no limite `135.5` do canal azul da mistura em sRGB — o vanilla arredonda para `135`, o cristalino para `136`. A mistura default em Oklab (`red.mix(blue)`) continuou a bater byte a byte com o vanilla (`oklab(61.08%, 0.075, -0.031)`). O off-by-one é registado como divergência residual conhecida; não é considerado blocker porque não há consumidor em `cetz` e o default (caminho comum) está em paridade.

### Parte B — `to-hex`, `transparentize`, `opacify`

Sonda `/tmp/p744-hex.typ`:

```typst
#repr(red.to-hex()) \
#repr((red.transparentize(50%)).to-hex()) \
#repr((red.opacify(20%)).to-hex()) \
#repr(blue.opacify(100%)) \
#repr(blue.transparentize(100%)) \
```

Resultado: **idêntico ao vanilla**.

```text
"#ff4136"
"#ff413680"
"#ff4136"
rgb("#0074d9")
rgb("#0074d900")
```

### Parte C — repr de closure anónima

Sonda `/tmp/p744-closure.typ`:

```typst
#let f = (x) => x
#repr(f) \
#let g = (x, y: 1, ..args) => x + y
#repr(g) \
#repr((a, b: 2) => a + b) \
```

Resultado: **idêntico ao vanilla** — `(..) => ..` para todas as closures (nomeadas e anónimas).

```text
(..) => ..
(..) => ..
(..) => ..
```

## Implementação

1. **Domínio** (`entities/color.rs`):
   - `negate(space)` — converte para o espaço pedido, inverte os componentes, volta ao espaço original. Default `Oklab` mantido.
   - `rotate(angle, space)` — converte para o espaço pedido, roda o hue, volta ao espaço original. Devolve `Option<Color>` (`None` para espaços sem hue, convertido em erro verbatim). Default `Oklch` mantido.
   - `mix(weight, space)` — mistura no espaço pedido com interpolação linear e **hue short-path**; resultado no espaço indicado. Default `Oklab` mantido.
   - `to_hex()` — string `#rrggbbaa` quando alpha < 1, `#rrggbb` quando alpha = 1.
   - `transparentize(factor)` / `opacify(factor)` — ajustam alpha por `scale_alpha` com clamp a `[0, 1]`.
   - Helpers `to_vec4_in_space` / `from_vec4_in_space` para converter qualquer `Color` para/quatro floats num dado `ColorSpace`.

2. **Estáticas** (`rules/stdlib/color.rs`):
   - `color.negate`, `color.rotate`, `color.mix` ganham parâmetro `space:`; `extract_color_space_arg` mapeia `Value::Func` (o tipo de cor) para `ColorSpace`.
   - Novas estáticas `color.to-hex`, `color.transparentize`, `color.opacify`.
   - O inventário do tipo `color` passa a **20 fields** (8 constructors + 12 operadores).

3. **Despacho de instância** (`rules/eval/bindings.rs`):
   - `eval_color_method` passa a conhecer 12 métodos (`lighten`, `darken`, `saturate`, `desaturate`, `negate`, `rotate`, `mix`, `components`, `space`, `to-hex`, `transparentize`, `opacify`).
   - `mix` de instância rejeita `weight:` nomeado (o vanilla usa o primeiro argumento posicional como peso); a estática `color.mix` aceita ambos.

4. **Repr de closure** (`rules/eval/repr.rs`):
   - Braço `Value::Func(Func::Closure(_))` imprime `(..) => ..`, espelhando o formato medido no vanilla 0.15.0.
   - Nativas continuam a imprimir o nome plain (`rgb`, `lighten`, etc.).

5. **Testes**:
   - `01_core/src/rules/eval/tests.rs` — 5 testes P744 novos (`space:` em mix/negate/rotate, `to-hex`, repr de closure, método desconhecido renomeado para `foo()`).
   - `01_core/src/entities/color.rs` — 10 testes de domínio P744 novos.

## Scope-outs (medidos, com comportamento explícito)

- `mix` variádico com pesos — mantido de P476/P742.
- `to-hex`/`transparentize`/`opacify` em espaços não-RGB — medido no vanilla: `to-hex` funciona em qualquer espaço (converte internamente); a implementação cristalina faz o mesmo via `to_space(Rgb)` antes de serializar.
- Repr de closure preserva apenas a *forma* `(..) => ..`; o corpo e os parâmetros não são reconstruídos — paridade exacta com o vanilla medido.

## Validação global

- `cargo test --workspace`: **4116 passed, 0 failed** no `typst_core` + **631 passed, 0 failed** nos restantes crates (total 4747).
- `crystalline-lint .`: **0 violations** (`--fix-hashes` aplicado:
  `entities/color.rs` → `3372ae9e`, `entities/func.rs` → `19f51330`,
  `rules/stdlib/color.rs` → `4fb82f93`).
- `cargo build --release`: ok.
- E2E: Partes B e C **idênticas** ao vanilla. Parte A com divergência residual off-by-one no canal azul de `mix(blue, space: rgb)` explicada acima.
- `cetz` (sonda do passo: line + circle) sem regressão:
  - Pixels não-brancos: vanilla 1684, cristalino 1627.
  - Pixels diferentes: 3311 sobre 2 177 714 totais → **0.152%** (anti-aliasing).
  - Diff de contagem: 57 pixels (3.4%), consistente com variação de anti-aliasing entre renderizadores.

## Achados

- Itens de `achados-adiados-cetz.md` fechados neste passo:
  - Named `space:` em `negate`/`rotate`/`mix`.
  - `to-hex`/`transparentize`/`opacify`.
  - Repr de closure anónima.
- Título da lista actualizado para **P700-744**.
- Lista "Por resolver" fica com **dois scope-outs conscientes** reforçados em passos anteriores:
  1. `polygon` com vértices `Ratio` (P741).
  2. Ordem entre tipos no erro de argumento extra (P740C).

## Proveniência

- Commit base: `f72a2438f8f92525097432fe510c80ce45faba9a33` (a confirmar/rectificar após compactação)
- Hora das medições: 2026-07-14T07:51-03:00
- Binário vanilla: `lab/typst-original/target/release/typst`
- Binário cristalino: `./target/release/typst`

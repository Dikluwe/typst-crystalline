# P742 — Métodos de instância de cor + fields `rotate`/`components`/`space` + correcção de semântica dos operadores

**Data:** 2026-07-14 (00:19 -03:00)
**Commit base:** `bc87cceef` ("P741: preenche hash do commit no relatório")
**Commit da implementação:** `6b321acc11ff25097432fe510c80ce45faba9a33` ("P742: metodos de instancia de cor + rotate/components/space + semantica vanilla dos operadores")
**Estado no momento das medições E2E:** working tree não commitado;
`git diff HEAD --stat`:

```
 00_nucleo/prompts/entities/color.md     | 133 +++++---
 00_nucleo/prompts/entities/func.md      |  11 +-
 00_nucleo/prompts/engine/stdlib/color.md |  92 +++++-
 01_core/src/entities/color.rs           | 516 ++++++++++++++++++++++++++++----
 01_core/src/entities/content.rs         |  16 +-
 01_core/src/entities/func.rs            |  25 +-
 01_core/src/engine/eval/bindings.rs      |  40 +++
 01_core/src/engine/eval/closures.rs      |  15 +
 01_core/src/engine/eval/repr.rs          |   8 +-
 01_core/src/engine/eval/tests.rs         | 117 ++++++++
 01_core/src/engine/stdlib/color.rs       | 230 ++++++++++++--
 01_core/src/engine/stdlib/mod.rs         |  12 +-
 12 files changed, 1072 insertions(+), 143 deletions(-)
```

Vanilla de referência: `lab/typst-original/target/release/typst`
(typst 0.15.0, 969087ec; source local **idêntico** ao tag v0.15.0 —
verificado por diff).

---

## Sonda — duas descobertas que mudam o passo

### 1. O `red` vanilla não é `#ff0000`

Medido via `#red.components()` → `(100%, 25.49%, 21.18%, 100%)` e
confirmado no source: `RED = Rgb(1.0, 0.254902, 0.211765)`
(`visualize/color.rs:311` = `#ff4136`). O cristalino já usa estes bytes
(P687). Sem esta descoberta, nenhuma fórmula batia com os valores
medidos.

### 2. A semântica dos operadores P476/P477 estava errada

Sonda rotulada (`/tmp/p742-labeled.typ`, 11 casos) medida no vanilla:

| Caso | Vanilla (medido) | Cristalino pré-P742 |
|---|---|---|
| `red.lighten(20%)` | `rgb("#ff675e")` | oklch (diverge) |
| `red.darken(20%)` | `rgb("#cc342b")` | oklch (diverge) |
| `red.negate()` | `rgb("#004b74")` | `rgb("#00bec9")` (complemento sRGB) |
| `red.rotate(90deg)` | `rgb("#87a100")` | ausente (erro de campo) |
| `red.mix(blue)` | `oklab(61.08%, 0.075, -0.031)` | **paridade exacta** |
| `red.saturate(20%)` | `rgb("#ff372b")` | oklch (diverge) |
| `red.desaturate(20%)` | `rgb("#ff675e")` | oklch (diverge) |
| `red.components()` | `(100%, 25.49%, 21.18%, 100%)` | ausente |
| `red.components(alpha: false)` | `(100%, 25.49%, 21.18%)` | ausente |
| `red.space()` / `== rgb` | `rgb` / `true` | ausente |
| `luma(128).saturate(20%)` | erro "cannot saturate grayscale color" + hint "try converting your color to RGB first" | ausente |

As fórmulas foram identificadas no source (`color.rs:1549-1653` +
`palette 0.7.6`, a crate que o vanilla usa — `Cargo.lock`) e
**reproduzidas byte a byte num scratch Rust dedicado**
(`/tmp/p742-palette-test`): os 7 outputs calculados pelo scratch
coincidem exactamente com os medidos (incluindo `negate` → `#004b74`
via Oklab `(1-l, -a, -b)` e `rotate` → `#87a100` via Oklch hue+90).

Semântica correcta (fonte + scratch):

- **`lighten(f)`** — fórmula palette `increase(c,f)` (`c + (1-c)·f` para
  f≥0; `c·(1+f)` para f<0; clamp) **no espaço da própria cor** (canais
  de estímulo; Hsl→l, Hsv→v). Cmyk: `u·(1−f)` (tipo próprio do vanilla).
- **`darken(f)`** — `lighten(−f)`; Cmyk: `u + (1−u)·f`.
- **`saturate`/`desaturate`** — Luma → erro; Hsl/Hsv → `increase` sobre
  saturação; restantes → via Hsv e de volta ao espaço original.
- **`negate()`** — default Oklab: `(1−l, −a, −b)`, de volta ao original.
- **`rotate(deg)`** — default Oklch: hue+deg, de volta ao original.
- **`mix`** — inalterado (paridade já medida).

### Medições adjacentes (repr e igualdade de funções)

- `repr(color.rgb)` → `rgb`; `repr(color.lighten)` → `lighten` (nomes
  **plain**, não `color.rgb`); `repr(rgb)` → `rgb` (sem `#` — o
  cristalino imprimia `#rgb`).
- `color.rgb == rgb` → `true`; `red.space() == rgb` → `true`;
  `red.space() == color.rgb` → `true`; `luma(128).space()` → `luma`.

## Implementação

1. **Domínio** (`entities/color.rs`): operadores reescritos com a
   semântica medida; novos `space()`, `to_space()` (hub sRGB),
   `rotate()`, `components()` + enum `ColorComponent`
   (Ratio/Float/Angle); helpers `increase_p742`, `rgb_to_hsv/hsl_p742`
   (hexcone standard = palette), `srgb_to_luma_p742`. `saturate`/
   `desaturate` devolvem `Option` (`None` = Luma → o chamador emite o
   erro verbatim com span).
2. **Estáticas** (`rules/stdlib/color.rs`): `color.rotate`,
   `color.components`, `color.space` (tipo passa a **17 fields** —
   inventário medido P736); nomes plain nas funcs (paridade de repr);
   erros verbatim + hint em saturate/desaturate de Luma.
3. **Despacho de instância** (padrão P506): `eval_color_method` em
   `eval/bindings.rs` + braço `Value::Color` em `closures.rs` —
   intercepta só os 9 métodos (`is_color_instance_method`); sintetiza
   `Args` com a cor como primeiro posicional e **delega nas estáticas**
   (validação e mensagens idênticas nos dois caminhos). Método
   desconhecido cai no caminho genérico (comportamento pré-P742).
4. **`Func::eq` por nome para nativas** (`entities/func.rs`): o vanilla
   materializa nativas como singletons (identidade); o cristalino cria
   Arcs frescos por lookup — a identidade equivalente é o nome
   (fundamenta `red.space() == rgb`). Closures/Element/With/Plugin
   mantêm identidade de ponteiro.
5. **Repr de Func sem `#`** (`eval/repr.rs`): `repr(rgb)` → `rgb`;
   resolve também a interpolação em markup (`#red.space()` → "rgb").

### Testes antigos corrigidos (precedente P736/linear-rgb)

- `p476_negate_vermelho_da_ciano` (entities + stdlib): o critério
  "negate(vermelho) = ciano" estava errado (assunção pré-medição);
  corrigido para o valor medido `#005688` (vermelho puro).
- `p477_saturate_aumenta_chroma`: vermelho puro tem `s=1` em HSV —
  `saturate` não muda (palette não excede o máximo); reescrito sobre
  `srgb(1, 0.5, 0.5)` com valor medido no scratch.
- `p477_saturate/desaturate_preserva_l_e_h`: a preservação é de
  **hue e value em HSV**, não de l/h em Oklch; reescritos.
- `p240/p241_*_ptr_eq` (entities/content.rs): premissa "nativas com
  mesmo nome são distintas" refutada pela medição P742; o caso
  "distinto" passa a usar nome diferente (intenção preservada).

## Scope-outs (medidos, com erro explícito)

- Named `space:` em `negate`/`rotate`/`mix` — medido no vanilla
  (`mix(blue, space: rgb)` → `#805b87`); cristalino rejeita com
  "argumento nomeado inesperado 'space'". Defaults em paridade.
- `mix` variádico com pesos — mantido de P476.
- `to-hex`/`transparentize`/`opacify` — existem no vanilla
  (`color.rs:912-1131`), fora da sonda → achados adiados.
- Repr de closure anónima (`#function(...)` vs `(x) => x`) → achado
  adiado.
- Caso de canto conhecido: o binding global cristalino é `linear_rgb`
  (underscore, divergência pré-existente); `space()` de LinearRgb
  devolve o nome vanilla `linear-rgb` — a igualdade com o binding
  `linear_rgb` é `false` (nomes diferentes). Repr correcto.

## Validação global

- `cargo test --workspace`: **4794 passed, 0 failed** (4772 em P741 +
  22 testes novos).
- `crystalline-lint .`: 0 violations (`--fix-hashes` aplicado:
  entities/color.rs → 5611f442, entities/func.rs → 727ec810,
  rules/stdlib/color.rs → 83be6e71).
- `cargo build --release`: ok.
- E2E: os 11 casos da sonda rotulada e os 9 da sonda do passo são
  **byte a byte idênticos** ao vanilla no `pdftotext`
  (`/tmp/p742-cr.pdf` vs `/tmp/p742-labeled-van.pdf`).
- E2E erro: `red.mix(blue, space: rgb)` → erro explícito de scope-out
  (exit 1) — comportamento declarado.
- `cetz` inalterado: 1535/1478 px não-brancos, diff 0.1477%, B−A=+57 —
  idêntico a P736–P741.

## Achados

- Item "Métodos de instância de cor ausentes..." de
  `achados-adiados-cetz.md` → **fechado** (P742).
- Novos achados adiados: named `space:` em negate/rotate/mix;
  `to-hex`/`transparentize`/`opacify`; repr de closure anónima.
- **Declaração pedida pelo passo:** após P742, a lista "Por resolver"
  de `achados-adiados-cetz.md` **NÃO fica vazia** — restam 5 itens, dos
  quais 2 pré-existentes (polygon Ratio, ordem no erro de argumento
  extra — ambos scope-outs reforçados com custo medido) e 3 novos deste
  passo (todos prioridade baixa, sem consumidor em cetz).

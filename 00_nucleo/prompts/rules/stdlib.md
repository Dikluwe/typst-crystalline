# Prompt L0 — `rules/stdlib` — ÍNDICE (fatiado em P314)
Hash do Código: (a calcular no checkpoint A.4 — P314)

**Este prompt foi fatiado em P314** (ADR-0104, atomicidade para agentes) para
matar o imposto de hash: editar uma função re-hasheava ~12 ficheiros. Agora
cada `.rs` aponta para o seu prompt fino. Este ficheiro é só o índice — qualquer
linhagem antiga encontrada depois tem trilha aqui.

## Prompts finos que o substituem

| Prompt fino | Conteúdo | `.rs` que aponta |
|---|---|---|
| `stdlib/_comum.md` | contexto, convenção de assinatura, helpers, regras de promoção | `mod.rs`, `assert.rs`, `layout.rs`, `shapes.rs`, `transforms.rs`, `gradients.rs`, `structural.rs` |
| `stdlib/calc.md` | módulo `calc` (41 fn + 4 const), IEEE 754/`guard_float`, critérios calc | `calc.rs` |
| `stdlib/foundations.md` | `type`/`len`/`range`/`str`/`int`/`float`, `rgb`/`luma`, `state_display`, `counter_display` | `foundations.rs` |
| `stdlib/figure_image.md` | `image` (I/O via world) | `figure_image.rs` |
| `stdlib/text.md` | `smartquote`, `underline`/`strike`/`overline` | `text.rs` |
| `stdlib/math_style.md` | 12 funções math style | `math_style.rs` |

## Nota de deriva (F4) registada, não corrigida

Vários `.rs` implementam funções que **não** estavam specadas neste prompt
(ex.: `layout.rs` 17 funções, `shapes.rs` 6, `transforms.rs` 4, `assert.rs`,
`gradients.rs` 3, `structural.rs` 21, e em `foundations.rs` os `oklab`/`oklch`/
`cmyk`/`hsl`/`hsv`/`state*`/`counter*`/`query`/`here`/`locate`). P314 **não
inventou spec** para elas; apontam para `_comum.md` (a convenção partilhada que
era o único conteúdo que o prompt velho lhes dava). **Candidatos a spec
dedicada** num passo futuro.

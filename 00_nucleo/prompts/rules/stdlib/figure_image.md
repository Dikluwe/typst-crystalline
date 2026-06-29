# Prompt L0 — `stdlib/figure_image` — imagens
Hash do Código: 2891f245

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/figure_image.rs`
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104). Convenção e
helpers partilhados: ver `stdlib/_comum.md`.
**Nota de deriva (F4)**: `figure_image.rs` também define `native_figure`, não
specado em `stdlib.md`; preservado como candidato a spec dedicada (não inventado).

---

## Imagens (Passo 71 + P502)

| Função | Assinatura Typst | Implementação |
|--------|-----------------|---------------|
| `native_image` | `image(path, width?, height?, fit?)` | lê bytes via `ctx.world.read_bytes(path)`, cria `Content::Image` |

`fit` é `Str` named opcional (default `"cover"`); valores válidos `"contain"`,
`"cover"`, `"stretch"`.

`native_image` é a única função stdlib com I/O — usa `ctx.world.read_bytes(path)`
para aceder ao ficheiro (Passo 71 — DEBT-24).

# Prompt L0 — `math/layout/op` — `MathOp`
Hash do Código: 50140cd3

**Camada**: L1 · **Alvo**: `01_core/src/compiler/math/layout/op.rs`
**Origem**: fatiado de `math/layout/mod.rs` em **P909**, completando o padrão de fatiamento
iniciado em P314 (ADR-0104) para `frac`/`root`/`stretchy`/`assembly`/`matrix`/`cases`/
`delimited`. `layout_op` foi adicionado em **P298**, depois de P314, e nunca tinha sido movido.
Núcleo partilhado: ver `math/layout/_comum.md`.

---

`MathOp` — emite `text` como math child standard. Handler **trivial** (delegate puro a
`layout_node`) — a verdadeira inovação de P298 não está aqui.

## Interação cross-variant com `MathAttach` (a real lógica de P298)

`Content::MathOp { text, limits }` carrega uma flag `limits` que **não é consumida por
`layout_op`** — é lida directamente por `layout_attach` (`attach.rs`, ver `attach.md`
§"Empilhamento de limites") quando `MathOp` aparece como `base` de um `MathAttach`:
`Content::MathOp(e) => e.limits` (`attach.rs:78`). Quando `limits: true` (override explícito via
`op("...", limits: true)`), os scripts empilham verticalmente acima/abaixo da base
incondicionalmente quando `self.block` (modo bloco/display), **independentemente do carácter** —
ao contrário da regra geral de `is_limits` para operadores grandes/funções-limite, que depende de
`symbols::is_large_operator`/`symbols::is_limit_function`.

Mover `layout_op` para este arquivo **não move** essa lógica — ela fica em `attach.rs`, que já a
documenta (`attach.md`). Este ficheiro só regista o ponteiro para não perder a interação de vista
por estar fisicamente noutro arquivo.

**Critério**: `Content::MathOp { text, .. }` → `layout_op` devolve exactamente
`self.layout_node(text, style)` (delegate puro, sem lógica própria). `op("f", limits: true)` como
base de `MathAttach` em modo bloco → scripts empilhados (verificado em `attach.rs`/`attach.md`,
não aqui).

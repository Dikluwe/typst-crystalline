# P1193 — separar stdlib `context` de `ContextBlock`

**Estado:** EXECUTADO — GREEN EM 2026-08-25
**Baseline condicionado:** P1192 GREEN; V15=14.

Individualizar `compiler/stdlib/context.md` para `stdlib/context.rs` e criar
`entities/elements/context_block.md` para a entidade. Extrair a obrigação
semântica compartilhada de avaliação contextual para
`_nuclei/context/context-block.toml`; constructor/dispatch e estrutura/mapas da
entidade continuam específicos.

Sem alteração pública. Gate: V15 14→13, V26=0, ausência focal V5, testes de
context e Content, build, comparação fora da linhagem, dry-run bloqueado por
13 V15 e diagnóstico `typst-p1193-saneamento-context-owners.md`.

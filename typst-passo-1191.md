# P1191 — separar running matter de domínio e layout

**Estado:** EXECUTADO — GREEN EM 2026-08-25
**Baseline condicionado:** P1190 GREEN; V15=16, V26=0.

Manter `entities/page_running.md` para `entities/page_running.rs`; criar
`compiler/layout/page_running.md` para o compositor. Extrair somente invariantes
genuinamente comuns de header/footer para `_nuclei/layout/page-running.toml`;
algoritmo de composição e estrutura da entidade permanecem nos owners.

Protocolo: RED duplo, L0/Núcleo antes de headers, pins efetivos completos,
V26 antes de V5, testes focais e build. Gate: V15 16→15, V26=0, ausência focal,
zero mudança Rust fora da linhagem e dry-run bloqueado por 15 V15. Diagnóstico:
`typst-p1191-saneamento-page-running.md`.

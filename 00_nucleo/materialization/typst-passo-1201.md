# P1201 — individualizar geradores de linguagem

**Estado:** EXECUTADO — GREEN
**Baseline condicionado:** P1200 GREEN; V15=6.

Manter `compiler/lang.md` para `lang/mod.rs`; criar owners para
`equation_supplement`, `figure_supplement`, `outline_title` e `quotes`.
Extrair defaults linguísticos compartilhados comprovados para
`_nuclei/lang/defaults.toml`; locale/fallback específico permanece por owner.

Sem mudança de outputs. Gate V15 6→5, V26=0, cinco owners, V5 focal limpo,
testes lang e build; dry-run bloqueado por 5 V15. Diagnóstico:
`typst-p1201-saneamento-lang-owners.md`.

## Fechamento

Executado em 2026-08-26. V15=5, V26=0, V5 focal limpo, 21 testes lang GREEN
e `cargo build` GREEN. O dry-run foi bloqueado pelos cinco V15 restantes e
não escreveu. Evidências em
`00_nucleo/diagnosticos/typst-p1201-saneamento-lang-owners.md`.

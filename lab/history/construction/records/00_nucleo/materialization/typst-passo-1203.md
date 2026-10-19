# P1203 — sanear a fronteira E1 e seus cinco owners

**Estado:** EXECUTADO — GREEN
**Baseline condicionado:** P1202 GREEN; V15=4.

Reclassificar `entities/f_fronteira_e1.md`: criar owners para registry,
dynamic, emph, strong e test_callout; `dynamic` conserva apenas seu contrato
específico, não ownership coletivo. Extrair a fronteira fechada de Element para
`_nuclei/entities/element-boundary.toml`. Se o documento atual for
inteiramente transversal, removê-lo como prompt após preservar história em
diagnóstico; não escolher owner representativo.

Gate V15 4→3, V26=0, cinco owners, V5 focal limpo, testes Element/Content e
build. Dry-run bloqueado por 3 V15. Fechar
`typst-p1203-saneamento-fronteira-e1.md`.

## Fechamento

Executado em 2026-08-26. O documento transversal foi preservado como
diagnóstico; cinco owners 1:1 e o núcleo da fronteira foram criados. V15=3,
V26=0, V5 focal limpo, suítes Element/Content e `cargo build` GREEN. O dry-run
foi bloqueado pelos três V15 restantes e não escreveu. Evidências em
`00_nucleo/diagnosticos/typst-p1203-saneamento-fronteira-e1.md`.

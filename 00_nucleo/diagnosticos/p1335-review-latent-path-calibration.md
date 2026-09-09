# Correção preventiva do path do gate

O operador encontrou antes da chamada final de preservação um path P1322 ainda
copiado dentro de `preservation_review()` do leitor R0. Essa função não tinha
sido executada nesta auditoria; preflight, gates e runtime usam outros ramos.
O sucessor `p1335-review-check-r1.py` troca exclusivamente esse path pelo único
passo autorizado, `00_nucleo/materialization/typst-passo-1335.md`, antes de medir
o gate final. Não houve leitura do passo P1322 por essa função. O R0 permanece
como evidência de método latente defeituoso; não é o leitor do gate final.

O teste de preservação final revalida hashes do produto/L0/lab, evidências
históricas, inputs, passo1335, HEAD, branch e diffs completos da baseline.

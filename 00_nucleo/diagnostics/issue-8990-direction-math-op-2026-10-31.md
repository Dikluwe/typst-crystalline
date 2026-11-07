# #8990 — direção de `math.op` em texto RTL

Em 2026-10-31, no HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`
(branch `Tekt`, árvore limpa durante as medições), o caso oficial da
[Typst #8990](https://github.com/typst/typst/issues/8990) reproduziu o defeito
no oracle ratificado `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, mas não no
Tekt. O operador latino mantém `p.v.` sob `lang: "he"` e `dir: rtl` explícito;
o oracle defeituoso move o último ponto para a esquerda. Geometria e pintura
foram verificadas separadamente, contra controles LTR saudáveis.

Build em target exclusivo RAM e 55 verificações observáveis passaram; lint e
lineage reversa também passaram. L0 de `math/layout/op` auditado e preservado:
nenhuma correção produtiva, reseal, commit ou push foi necessária.

Ressalvas abertas: o Tekt não derivou direção RTL de `lang: "he"` sozinho na
prosa; operadores com caracteres hebraicos também apresentam diferenças de
fallback/geometria. O controle de prosa com `dir: rtl` explícito funciona,
distinguindo o resultado específico da #8990 dessas limitações. Não se
certificou RTL matemático geral ou a #7280.

[Relatório completo, limites e proveniência](../../lab/results/construction/diagnostics/issue-8990-2026-10-31/report.md).
As medições principais ocorreram entre 16:09:11 e 16:10:28 UTC; seus recibos
preservam os horários precisos, HEAD, diffstat vazio, fontes e hashes dos binários.

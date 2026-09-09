# P1333 — parecer final independente

**PASS_SCOPED** — correção da precedência da falha do primeiro posicional
de calc.abs, no contrato congelado. Revisor `/root/p1333_review`; não editou
L0, implementação, testes, oráculos ou relatório julgados. Regime A/B
executado sem atestação técnica de isolamento, selo de refinamento ou
mutation score.

Manifesto:
`75eea7c2a28cd8c75098d4d8fc5b2c733c30c09b7eca52a2ce7a772a24e0e99b`.
Freeze completo:
`bdbadb8d0c91dc9823c085489ae2c058f0a20e0d1db844ae1346176a316c1199`.
Owner final:
`c6630647094dbaa1008e8516a1d8f4480ff1834d04a810a9fa8355dc788ddf06`.
Binário candidato:
`79470612fc121fa42a6898846f85d0b0325edf517c90a830c01cde947f748ffe`.

O RED compilado anterior a C falhou pela precedência especificada, inclusive
em chamadas públicas coerentes. O GREEN final executou os mesmos testes:
39 aprovados. A revisão produtiva confirmou fórmulas e origens preservadas,
guards legados para ausência/primeiro válido e nenhuma alteração externa
a calc_abs após descontar snippets congelados e linhagem. A auditoria foi
repetida no source final formatado; todos os artefatos congelados conferiram.

Conferi hashes, exit zero e igualdade de inventário produtivo antes/depois
dos 12 gates registrados em `p1333-gates-audit.json`, SHA-256
`38f0db6a92fd1d633d38bd68e8214a2bfc24f094eff4da2ccfe318bdab10a925`.
Build, workspace, fmt, diff check, linhagem, V5/V15/V26, auditoria produtiva
e CLI estão sobre o mesmo estado final. Somei os resultados do workspace:
6.725 aprovados, zero falhas e três ignorados, sem somar o focal novamente.
Lint geral não tem erros; warnings/notas históricos e as mudanças textuais
de padrão estão corretamente distinguidos de novos achados no relatório.

Recalculei diretamente dos JSON CLI: normal, repetição e inversão têm
712/712 saídas iguais às expectativas congeladas e o mesmo binário.
Coincidência literal com vanilla passa de 532 a 608; ganho 76, regressão
zero nesse corpus, 104 dívidas preservadas. As métricas identificam o
baseline e os recibos com HEAD, diff/stat, inventário, argv e UTC; não são
percentual de paridade da linguagem.

Recibo independente B lido e hash conferido:
`0f0b797edeb4f080d0a0723627d570cfe798d91317dc0b2bedf0bd4ce2bf0744`.
B emite PASS nas três execuções, preserva a separação de entradas e limita
o veredito à CLI. Relatório final auditado:
`0117ea2bb04383b641eb9b62bc50191762ae800fda7f29271d582a8b4cd26566`.
Nenhuma correção de substância solicitada. `p1333-close.py` referencia
`unit-green-final` e gates finais `-r1`, e exige os pareceres antes de
produzir o fechamento; sua execução ainda é responsabilidade do integrador.

Limites mantidos: validação completa de ausência/sobras/named value e
âncora agregada permanece incompleta, nomeada P1334. Fixtures incoerentes
de Args são ensaios de robustez fora do domínio causal, sem prova de
paridade ou autorização a writers stale; evidência pública coerente
sustenta o ganho. Nenhum PASS geral de abs, calc ou linguagem é emitido.

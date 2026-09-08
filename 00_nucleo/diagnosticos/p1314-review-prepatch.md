# P1314 — revisão do freeze antes do patch

**Veredito: `GO_PREPATCH` no recorte P1314 congelado.**

Revisor `/root/p1314_review`, conferência 2026-09-08T12:37:33.643Z,
HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado.
O estado completo/diff-stat acompanha a proveniência do freeze e o baseline
listado abaixo. O parecer `p1314-review-precontract.md` permanece inalterado.

## Entradas conferidas e medição

Freeze `p1314-ab-freeze.json`, SHA-256
`41ac0730e01cd31cd9660b0e229e37d5e486a2c114927f3ac49c3475733756b9`;
medição independente final `p1314-ab-baseline-final.json`, SHA-256
`7edfae1052dd08cf6a9ccbcbaaf6cf74d0ddad0852c59986a7642822dad90539`.
L0 proprietário, SHA-256 integral
`022d7f0d5efd00d74702ceec84feb58c3aa70298afa5ee2d530a6c647f69d268`;
pin normativo `21231170b7092331ae09fafc656210bb94829658d822a15291de6ea0eb28dbfe`.
A exclusão permite somente a linha canônica `Hash do Código`; o passo tático
não é entrada normativa e seu status pode ser encerrado sem alterar o freeze.

Conferência própria somente leitura, no namespace host, verificou fisicamente
os 24 pins de entradas/fixtures, L0 e os dois executáveis. Baseline P1313
`cefb4b485cc25ae98871cfbd7a76925d0333d6d26906dc7f7426868e899285ce`;
vanilla ratificado a51e02804
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

As 234 expressões têm identificadores únicos e 936 expectativas nos quatro
perfis. Recalculei cada expectativa diretamente da medição apropriada,
incluindo a substituição explícita da mensagem Symbol sobre origem/trace
medidos no vanilla. As 144 expressões históricas usam o cwd P1313 original:
recomparei todos os seus resultados baseline literalmente com o candidato
P1313. São 13 expressões históricas com mudança explicitamente congelada;
as restantes mantêm exit/stdout/stderr. No conjunto total, 344 expectativas
por perfil distinguem o baseline do resultado requerido. Esses números são
propriedades dos artefatos identificados, não resultado de candidato P1314.

Inspeção dos casos verifica tipos inválidos, origem direta/multilinha,
With/nested With, Args/dict spread, sink, filter e map detached; primeira,
intermediária e última ocorrência inválida; últimas válidas vencedoras;
delimiter inteiro antes de row-type; opções antes de parsing/leitura;
Symbol separado e preservações de source/missing/excesso/unknown/sintaxe.
O runner compara observações completas e verifica unicidade/completude nas
ordens normal/repeat/reverse. Ausência, duplicação, crash ou drift não
produzem sucesso. Args sintético e zero chamadas World exigem testes locais
e revisão do owner, limitação declarada no freeze.

## Decisão

O L0 P1314 implementa a fronteira `CONTINUOUS_SCOPED` do parecer anterior:
substitui precisamente as preservações P1313 relativas a origem e validação
de todas as ocorrências de delimiter/row-type, mantém casts admitidos e
ordem externa, proíbe novos owners/entidades/dispatch/fase/I/O. ADR-0127
permite fluxo contínuo após L0 e RED→GREEN. Não surgiu ambiguidade que exija
paragem humana adicional neste recorte.

Pode iniciar o patch produtivo delimitado. O presente veredito não aprova
resultado futuro: GREEN local, build/workspace, lint/linhagem, A/B e replays
continuam pendentes de revisão final. Nenhum oráculo histórico pode ser
reescrito para acomodar o candidato.

Regime A/B executado sem atestação de isolamento técnico. O testador declara
ter visto incidentalmente diff histórico P1310–P1313 embutido no baseline,
antes de existir candidato P1314; não leu candidato nem testes locais. Isso
é registrado como limite de entradas, sem alegação de isolamento técnico.
O revisor não escreveu L0, oráculos, testes locais ou implementação.

# P1309 — plano adversarial D, revisão 0

Regime: **executado sem atestação de isolamento técnico**. Papel D recebe o
passo autorizado `00_nucleo/materialization/typst-passo-1309.md`, a skill
`/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md` e suas duas
referências, fontes/artefatos históricos e entradas P1309 congeladas. O contexto
herdado contém instruções do coordenador e normas do repositório; o filesystem
é compartilhado. Escrita exclusiva: `p1309-adversarial-*` e
`p1309-audit-attacks.py`. D não modifica runner, catálogo, classificação,
seleção, produto, L0 ou veredito E. Não executa mutantes Rust P1307.

Estado observado na preparação: HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, `git diff HEAD --stat` vazio.
As medições de aceitação terão proveniência, UTC, hashes e controle por caso.

## Critério de testemunha

Cada ataque altera uma cópia em memória de artefatos reais, mantendo uma cópia
controle sem a adulteração. O mesmo auditor externo deve aceitar o controle e
rejeitar a adulteração por razão pública relacionada à violação pretendida.
Falha genérica de parser, falta de arquivo, exceção, timeout ou rejeição do
controle não conta como ataque morto. Artefato já bloqueado não serve de
controle positivo integral: é necessário um gate focal aceito ou reportar
ausência de testemunha. Um auditor que apenas testa flags inseridas pelo próprio
adversário não fornece poder discriminatório.

As três ordens integrais, catálogo novo e ledger são inputs de B/A/C; D não os
fabrica. Na falta de API real do auditor, os ataques ficam preparados e o gate
§14 fica bloqueado, sem `mutation_score = 1.0`.

## Ataques e controles

| ID | Adulteração verificável | Controle e observável exigido |
|---|---|---|
| A01 | Omitir um probe do catálogo P1304 preservado na reconciliação P1309. | Controle retém seu ID, path e expressão; auditor compara conjunto e conteúdo histórico. |
| A02 | Omitir uma rota descoberta no inventário fresco e ausente do catálogo histórico. | Controle contém a mesma rota e origem inventariada; ausência de descoberta real deixa este ataque sem testemunha. |
| A03 | Fundir paths públicos semelhantes (`angle.deg`/`angle.rad` ou par real equivalente). | Controle mantém identidade própria e observação por path, sem alias inferido do nome. |
| A04 | Trocar envelopes vanilla/cristalino de uma célula real, mantendo a alegação de sides. | Controle amarra side, argv e SHA-256 ao manifesto/binários. |
| A05 | Substituir a identidade candidata fresca pela identidade histórica do binário P1308. | Controle tem recibo do build dedicado posterior ao congelamento; hash/`--version` isolado não prova frescor. |
| A06 | Trocar features HTML/a11y ou remover um perfil inteiro. | Controle preserva as quatro chaves e argv próprios; ambos devem ser checados. |
| A07 | Apagar `stderr` divergente e manter stdout igual para declarar MATCH. | Controle retém bytes/hash do canal lateral; gate classifica o transcript completo, inclusive em exit zero. |
| A08 | Mudar membro `VANILLA_ONLY` para `EXPECTED_FEATURE_DISABLED` retirando fundamento normativo. | Controle mantém classe de dívida ou apresenta gate L0/ADR específico e observado nos perfis. |
| A09 | Mudar aceitação apenas cristalina para extensão intencional sem cláusula L0/ADR. | Controle exige fundamento integral vigente; comentário produtivo e silêncio não bastam. |
| A10 | Manter presença/kind/repr de encoder e remover seus witnesses de comportamento para declará-lo fechado. | Controle preserva sentinelas de valores, erros, pretty, transporte e parent decoder exigidas. |
| A11 | Manter repr elidida igual e remover/corromper dados integrais do array de 256 posições. | Controle preserva comprimento e todos os elementos/canais; repr textual sozinho não fecha. |
| A12 | Renomear ID de MATCH histórico que passou a não-MATCH para ocultar transição. | Controle reconcilia ID/path/expressão e detecta regressão; ID novo não recicla identidade antiga. |
| A13 | Rotular timeout, JSON inválido ou execução incompleta como MATCH. | Controle conserva Unknown opaco como bloqueio e valores JSON válidos como caso positivo. |
| A14 | Acrescentar sentinela suplementar ao denominador/catálogo principal sem descoberta inventariada. | Controle publica cardinalidades e universos separadamente. |
| A15 | Tratar família de certificação pendente como falha funcional sem witness bilateral. | Controle mantém dívida em ledger separado e preserva a matriz funcional. |
| A16 | Escolher coorte inferior à prioridade ou violar desempate de owners/paths/risco/ID. | Controle recalcula a ordem a partir de todas as coortes elegíveis e da mesma evidência. |
| A17 | Declarar prontidão com Prompt L0 1:N ou pin/DAG de Núcleo inválido. | Controle exige ownership 1:1 e recibo de integridade; declaração booleana não substitui grafo. |
| A18 | Alterar transcript de uma célula somente em repeat ou reverse. | Controle tem mesmas chaves e observáveis integrais nas três ordens, ignorando apenas tempo/ordem logística. |

## Alertas de método comunicados antes da matriz

- `p1304-run-matrix.py:129-141`: em dois exits zero compara somente
  `stdout_base64`; warning divergente pode virar MATCH. JSON não é validado.
- `p1304-run-matrix.py:144` em diante: inversão é subconjunto selecionado;
  não satisfaz repetição integral mais inversão integral exigidas agora.
- `p1299-run-matrix.py:169-207`: catálogo histórico filtra entradas por classe
  não-MATCH e omite surfaces com `same=True`; não reenumera superfície integral.
- `p1304-owner-ledger.tsv`, paths `calc.deg`, `calc.log10`, `calc.rad`: as
  próprias linhas declaram ausência de cláusula explícita em `calc.md` e usam
  intenção registrada em Rust. P1309 §9 exige L0/ADR vigente, silêncio não basta.

Budget: duas revisões focais por causa sem ganho interrompem calibração e
exigem diagnóstico de insuficiência. Registrar hipótese, delta, controles que
passaram/regrediram, causa, duração e hashes. Não repetir corpus integral para
resolver falha que pode ser isolada no gate focal.

## Incidente anterior ao congelamento do corpus

O coordenador comunicou que A criou via importlib um cache Python ignorado por
Git, fora do allowlist, e o moveu para `/tmp`. A tentativa inicial é inválida
por §1, ainda que o produto tracked permaneça intacto. O recibo de preparação D
mantém os hashes iniciais por proveniência e não constitui selo válido da
retomada. O gate D exigirá baseline/manifesto sucessores, restauração comprovada
inclusive do filesystem ignorado, target fresco distinto e bytecode desativado.
Um segundo vazamento bloqueia a execução, sem nova retomada. O harness D usa
`sys.dont_write_bytecode = True` e filhos Python `-B` desde a primeira revisão.

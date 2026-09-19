# ADR-0131 — Checkpoints retomáveis na materialização segregada

**Estado:** EM VIGOR — APROVADA PELO DONO
**Data:** 2026-09-19
**Origem:** falha operacional da campanha P1462 R4
**Escopo:** governança de pré-selo, mutation testing e certificação Tekt

## Contexto medido

A campanha P1462 R4 foi executada sobre o commit
`61796fa6e183c08207e3828cec6d2e43da1af846` e ficou registrada no commit
`bb4ee22f9204764a75b0f191055e7800566942b8`.

Antes da interrupção externa, a ordem primária terminou integralmente:

- P00 foi `Preserved`;
- M01–M18 foram `Violated`;
- o vetor completo não teve `Unknown` semântico;
- o score observado nessa ordem foi `18/18`;
- a repetição confirmou P00–M11 com o mesmo vetor;
- M12 foi interrompido durante execução por limite externo do agente;
- a ordem reversa não chegou a começar.

O protocolo então converteu a interrupção de infraestrutura em `Unknown`,
descartou o vetor completo já concluído e proibiu a implementação. O custo
adicional não revelou falha do contrato nem mutante sobrevivente. A regra
protegeu o ritual de três ordens, mas não aumentou a confiança no fragmento.

## Decisão

### 1. `Interrupted` não é classificação semântica

Passam a existir estados distintos:

- `Preserved`, `Violated` e `Unknown` classificam uma unidade concluída;
- `Interrupted` registra que a unidade não terminou por causa externa;
- `Pending` registra unidade ainda não iniciada.

Timeout do próprio caso, crash do produto, identidade ambígua, parser opaco ou
saída insuficiente continuam `Unknown`. Encerramento do agente, perda da
sessão, limite de uso, queda do host ou cancelamento do runner são
`Interrupted`, desde que recibos permitam distinguir a causa externa. Nem
`Interrupted` nem `Pending` recebem crédito, mas também não reclassificam
unidades já concluídas.

### 2. Checkpoint é a unidade de validade

Cada caso ou mutante terminado deve persistir atomicamente um recibo com:

- hashes de prompt, contrato, oráculo, baseline, mutante e binário;
- comando, ordem lógica, ambiente relevante e timestamps;
- exit status, classificação, testemunha e hashes das saídas;
- versão do instrumento e predecessor causal.

Um checkpoint é reutilizável quando esses pins continuam idênticos. Uma
alteração invalida somente o checkpoint e os agregados que dependem
transitivamente da entrada alterada. Não se reinicia toda a campanha por uma
mudança sem relação causal.

### 3. Retomada começa na primeira unidade incompleta

Após interrupção externa:

1. auditar os pins e a integridade dos recibos concluídos;
2. descartar somente a unidade parcial;
3. reexecutar essa unidade desde o início em diretório temporário novo;
4. continuar a ordem sem repetir checkpoints válidos.

Isso é retomada, não retry seletivo de uma falha semântica. Um `Unknown`
produzido pelo próprio caso não pode ser convertido por repetição oportunista.

### 4. Um vetor discriminatório completo autoriza candidato

O pré-selo mínimo exige:

- controle positivo completo preservado;
- todos os mutantes válidos rejeitados;
- `mutation_score == 1.0`;
- zero `Unknown` semântico no vetor;
- todos os gates arquiteturais e inventários declarados para o recorte;
- autoridade verificadora diferente do implementador.

Cumprido esse conjunto, teste RED e implementação candidata ficam
autorizados. Repetição e reordenação deixam de ser duplicação universal
obrigatória antes do candidato. Elas são escolhidas por risco e executadas na
certificação final sobre:

- casos previamente instáveis ou interrompidos;
- fronteiras de ownership e targets neutros;
- ao menos um controle positivo;
- qualquer classe cuja ordem possa alterar a decisão.

Uma spec pode exigir repetição integral quando declarar antes da execução uma
ameaça concreta que a torne necessária. A frase genérica “provar
determinismo” não basta.

### 5. Budget limita trabalho futuro, não apaga prova

O budget conta computação ativa das unidades ainda não concluídas. Espera por
aprovação, indisponibilidade do agente e interrupção de infraestrutura não
consomem o budget semântico. Ao esgotar:

- não se iniciam novas unidades sem decisão do dono;
- checkpoints válidos permanecem válidos;
- o escopo pode ser retomado ou reduzido explicitamente;
- evidência concluída nunca é apagada nem convertida em `Unknown`.

Limites de revisões e campanhas são alertas de rendimento decrescente, não
sentenças irreversíveis como “última revisão local”. Nova hipótese causal ou
mudança de processo aprovada pode reabrir a execução sem renomear o trabalho
já válido.

### 6. Aplicação de recuperação ao P1462

Para P1462, esta ADR substitui somente as exigências operacionais de três
vetores completos, invalidação total por interrupção e proibição de retomada
presentes no contrato R2/R4. Não altera a semântica de `format.svg`, os 18
mutantes, o denominador, a exigência de score `1.0`, os Prompts L0 ou os gates
arquiteturais.

O vetor `direct` registrado em
`00_nucleo/diagnosticos/p1462-r4-mutation-results.json` pode compor o novo
pré-selo após auditoria dos pins. Antes de autorizar o candidato ainda é
obrigatório executar o inventário P1357, que não foi alcançado. O M12 parcial
da repetição não recebe crédito; ele pode ser retomado como unidade nova na
certificação final, junto de P00, M03, M13 e dos controles de fronteira
afetados. Não é necessário repetir M01–M18 três vezes.

## Prompts afetados

Nenhum Prompt L0 muda. Esta decisão governa a produção e conservação de
evidência; não altera contrato público, semântica da linguagem, ownership
produtivo nem comportamento por defeito.

## Consequências

**Positivas:** interrupções deixam de destruir resultados válidos; campanhas
podem ser retomadas; custo passa a seguir risco causal; implementação não fica
bloqueada por repetição pro forma depois de score completo.

**Negativas:** runners precisam persistir checkpoints atômicos; o verificador
deve auditar mais pins; um recibo incompleto não pode ser recuperado por
memória narrativa.

**Neutras:** `Unknown` continua sem crédito; mutante sobrevivente continua
bloqueante; separação de papéis e atestação permanecem inalteradas.

## Alternativas consideradas

| Alternativa | Resultado |
|---|---|
| Manter três ordens integrais antes do candidato | Rejeitada: P1462 demonstrou custo alto sem testemunha adicional |
| Tratar toda interrupção como `Unknown` | Rejeitada: mistura falha do produto com falha do executor |
| Aceitar qualquer prefixo parcial como score | Rejeitada: permite cobertura incompleta |
| Eliminar mutation testing e segregação | Rejeitada: remove poder discriminatório e independência úteis |
| Checkpoint retomável + repetição por risco | Adotada: preserva rigor sem descartar prova válida |

## Relação com decisões anteriores

- complementa ADR-0108 e ADR-0121 ao tornar a proveniência incremental;
- preserva ADR-0127, ADR-0129, V15 e V26;
- prevalece sobre regras operacionais anteriores que exijam reinício total,
  convertam interrupção externa em `Unknown` ou declarem uma revisão local
  irrecuperavelmente final;
- não autoriza adaptar oráculos ao candidato nem editar artefatos congelados.

## Critério de revisão

Revisitar se checkpoints válidos produzirem classificações divergentes sob os
mesmos pins, ou se repetição dirigida deixar de capturar uma classe de
instabilidade observada em produção. Nesse caso, ampliar o conjunto de risco
com evidência concreta, não restaurar duplicação universal por defeito.

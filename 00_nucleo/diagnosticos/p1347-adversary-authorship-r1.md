# P1347 — autoria adversarial R1 congelada

## Veredito

`AUTHORED_NOT_EXECUTED`

Esta revisão congela a suíte adversarial antes da entrega do checker/caller de
P1347. Nenhum checker, caller, receipt de delivery, candidato ou ficheiro de
oráculo P1347 foi lido ou executado. A execução full é proibida neste estágio e
permanece em **0**. O resultado é autoria, não evidência de aprovação nem
pré-selo.

Regime de independência declarado: `executado sem atestacao de isolamento`.

## Papel e fronteira

- Papel: adversário independente P1347 (`/root/p1346_adversary`).
- Saídas desta autoria: a suíte parametrizada, este relatório e o receipt de
  autoria R1.
- Entradas protegidas: passo P1347, contrato/binding/receipt P1347 e baseline
  adversarial P1346 R2, todos pinados abaixo.
- A suíte recebe futuramente `checker`, `caller` e `delivery_receipt` como
  `FrozenTargets`, cada qual com SHA-256 externo. A adaptação operacional também
  precisa chegar como arquivo independente com SHA-256; não é descoberta nem
  embutida durante a autoria.
- O único modo executado agora foi `--author-check`, que valida pins, esquema,
  cardinalidades e identidade dos casos. `run_with_adapter` não foi chamado.

## Entradas protegidas

| entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1347.md` | `c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c` |
| `p1347-contract-spec-r1.json` | `fac6e30c7bbee3b50eec6cad8c8a9a8b30f3c09027f63293e73b4bf2341db0ea` |
| `p1347-contract-binding-r1.json` | `de1dcc87e77ab5fd6c8395fa4faacfd74fd90769dcafb5d2ef5c0bd30fda5fd1` |
| `p1347-contract-receipt-r1.json` | `85c6ad312cf9ac83ff653e2a19ac96fdd9e29ad72d2eb4af90fe8fff14fafb1d` |
| `p1346-adversary-report-r2.json` | `60de39bf2588fab84953f3a2fefea4cfd842dac205f1b23b669830106ea6bdc6` |
| `p1346-adversary-report-r2.md` | `bd0ada205586718106e2e4851dfb8cfb7f50475967bd272567ef10d92512b466` |
| `p1346-adversary-receipt-r2.json` | `2fdf8efdd0f805c3dc321f34192a01715b273b9b071f51d231b80e71c88115a3` |

A sequência protegida contém exatamente **122 IDs ASCII distintos**, na ordem
exata, com SHA-256 canônico
`e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907`.
O relatório P1346 pinado liga **181 negativos mortos** para replay futuro e os
cinco sobreviventes herdados.

## Corpus adversarial congelado

Foram autorados **49 ataques focais válidos** e **11 controles**. As famílias
cobrem:

1. Os cinco sobreviventes P1346: stdout forjado sem filho; sequência duplicada;
   PID string; DAG autojulgado; e troca de FD/imagem depois da checagem.
2. Autoridade do worker: stdout sem filho mesmo com nonce e envelope corretos;
   ID omitido, extra, duplicado, reordenado e alias Unicode; PID booleano, zero,
   negativo, string, sibling ou correto sem `waitpid`; mistura independente de
   stdout, stderr e status; saída parcial/extra, timeout, sinal, exit inesperado e
   replay do desafio.
3. DAG answer-free: labels sem validador real, alteração de bytes aceita,
   anchor fornecida pelo mutante, validador não observado pelo pai, receipt com
   canal de resposta e substituição coordenada do anchor.
4. Imagem executada: swaps antes, dentro e depois do exec-stop, `dup` posterior,
   segundo exec e binário imitador que produz saída canônica.
5. Falhas fechadas de capacidade/proveniência: ausência ou falha de pidfd,
   ptrace, exec-stop, `/proc/<pid>/exe`, execução por FD, leitura dos seals,
   stream de sibling e detach prematuro do tracer.

Os controles preservam worker e DAG canônicos, exigem que as mutações T08–T11
sejam recusadas pelo validador real, permitem `Unknown` apenas para probe opaco
autenticado e projetam quatro controles P1346.

## Validade e modelo de ameaça

- **Worker/PID:** os vetores são válidos porque bytes corretos, nonce e PID
  declarados pelo próprio worker não provam descendência. A prova exigida é do
  pai: pipes próprios, identidade do filho e status obtido por espera desse
  mesmo processo.
- **IDs:** igualdade é byte a byte, incluindo ordem e cardinalidade; normalização
  Unicode ou comparação por conjunto destruiria a identidade da bateria.
- **DAG:** o processo mutante não pode emitir a própria classificação. Mudança de
  bytes, labels ou um anchor escolhido pelo atacante não substituem execução do
  validador real contra o anchor externo original.
- **Imagem:** validar caminho, FD ou processo antes do ponto de exec não basta.
  Swaps e imitadores testam a necessidade de exec-stop rastreado e medição
  parent-owned da imagem realmente carregada.
- **Falhas de capacidade:** a impossibilidade de obter pidfd, ptrace, exec-stop,
  `/proc/<pid>/exe`, FD-exec ou seals não autoriza downgrade para confiança no
  output. O resultado deve fechar como violação de autoridade.

Qualquer sobrevivente numa execução posterior bloqueia o pré-selo. O score só
poderá ser calculado depois do replay efetivo; nesta fase não há score de
execução.

## Medição de autoria

- Estado base: commit `496c45ac6279d4298079f848b24b5e97ba88edc1`.
- Hora: `2026-09-11T12:03:14-03:00`.
- Estado: working tree não commitado; o artefato autorado nesta fronteira era
  `?? 00_nucleo/diagnosticos/p1347-adversary-suite-r1.py` no instante da
  medição. O workspace é compartilhado, portanto o receipt registra também um
  fingerprint final do estado, sem atribuir mudanças concorrentes ao
  adversário.
- SHA-256 da suíte: `d51fa54da1c68dcc5c812847b90097dfe94f1b4eb83f1009294230eb5f37dd4c`.
- Resultado de `--author-check`: 49 ataques, 11 controles, 181 replays ligados,
  122 IDs, `full_runs=0`, candidato/oráculo não lidos ou executados e
  `AUTHORED_NOT_EXECUTED`.
- Sintaxe Python: válida por `compile(...)`, sem gerar bytecode.

Este documento não afirma que os ataques matam o checker futuro. Ele preserva a
separação temporal necessária: autoria adversarial agora; execução por
verificador independente somente depois de alvos congelados e pinados.

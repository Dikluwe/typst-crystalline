# P1346 — relatório adversarial focal R1

## Veredito

`ADVERSARIAL_R1_SURVIVORS_AND_NONEXECUTION_BLOCK_PRESEAL`

O gate canônico declara `140/140`, score `1.0` e treze controles sem
regressão, mas essa declaração não é sustentada pela execução observada. A
instrumentação mede somente `2/122` replays efetivos dos negativos P1345 e
`14/18` execuções efetivas dos transportes P1346. Os outros 120 resultados
históricos e T08–T11 são construídos como `Violated` sem submeter as mutações
ao verificador.

Em 27 ataques adicionais válidos, 24 foram rejeitados e três sobreviveram:
score adicional `24/27 = 0.8888888888888888`. Qualquer um deles já impediria
o pré-selo; não houve corpus full, candidato nem correção pelo adversário.

## Sobreviventes

### P1346-A01 — `HISTORICAL_NONEXECUTION`

O runner substituiu em memória os 122 resultados históricos entregues por
`_legacy_vectors` por `Preserved/PRESERVED`. Mesmo assim, `run_focal` publicou
`140` negativos corretamente violados e zero survivors. A implementação
descarta os resultados medidos e fabrica `Violated/PROTECTED_INPUT` para 120
linhas; apenas A21 e R2A17 atravessam subprocessos de preflight.

Essa é uma falha de poder discriminatório, não uma divergência cosmética do
relatório. O requisito era reexecutar 122 mutações, não herdar seus nomes e
reescrever a resposta esperada.

### P1346-A25 — `memfd-descriptor-identity`

Depois de o checker copiar, hashear e selar o memfd legítimo, o ataque troca o
descritor numérico `N` por outro memfd contendo executável de SHA-256
`64624aa34739cd848352179c919e78cd2aa4c8429e9c70b5840dfef72511cb65`
somente durante `exec`, e restaura o descritor selado antes do pós-hash. O
binário adversarial executa, emite uma resposta válida e o checker devolve
`Unknown/OPAQUE_PAYLOAD` com os hashes do binário legítimo.

Logo, os hashes pré/pós e o conjunto de seals não vinculam os bytes executados
ao processo filho. A exigência explícita de detectar FD swap/restore falha.

### P1346-A26 — ausência real de `memfd`

Quando `os.memfd_create` lança `OSError`, a exceção ocorre antes do bloco que a
converteria em `PROBE_AUTHORITY/Violated`. O resultado fica sem classificação
(`Unclassified/OSError`), contrariando a regra fail-closed e a proibição de
usar `Unknown` ou erro protocolar como substituto de `Violated`.

## Ataques rejeitados e controles

- Doze ataques de preflight manual foram rejeitados, cobrindo autoridade com
  flags ausentes, duplicação, abreviação, `--name=value`, positional/unknown,
  valor com prefixo de opção, alias `./`, digest em maiúsculas e trailing
  token.
- Oito mutações do delivery DAG foram rejeitadas contra o SHA recebido fora
  da banda: caller/checker/authorship trocados, verdict falho, ciclo, chave
  duplicada, ordem JSON não canônica e trailing whitespace.
- Três mutações reais da saída do probe — chave duplicada, ordem invertida e
  whitespace final — foram rejeitadas.
- A ausência efetiva de seals foi rejeitada como `PROBE_AUTHORITY`.
- Os quatro controles adversariais passaram: focal canônico, backward DAG,
  relocação byte-idêntica `.rs.txt` e probe selado canônico `Unknown`.

## Proveniência e orçamento

- instante: `2026-09-11T10:42:31-03:00`;
- `HEAD = 2f42d64253547734564513a1159ee6b584c1c4b4`;
- working tree não commitado;
- `sha256(git status --porcelain=v1) =
  f47b626bdf0656f671820278605aac008565b62995e47eaf4c62514e9c00af66`;
- `sha256(git diff HEAD --stat) =
  9783f07bc86c21e8874ebacec9d92ce5a12208e127748377c571b091a1c92e15`;
- stat rastreado: 76 ficheiros, 10.972 inserções, 833 remoções;
- trust anchor externo do delivery receipt:
  `f8d8f12776a17b08941d5969d8bdeebff183b62284d786c04481292f2c43fd6e`;
- runner SHA-256:
  `eef0a9992f0ce45bfbede66d904a4561f4252a3c6940715640e9350a1bb3c45a`;
- relatório JSON SHA-256:
  `79a850efeb6cbee0c48558ee8c3ddec0cb8e5dc49e9376d0769e2fbd9998ae03`;
- comando:
  `PYTHONDONTWRITEBYTECODE=1 python3 -B 00_nucleo/diagnosticos/p1346-adversary-runner-r1.py --output 00_nucleo/diagnosticos/p1346-adversary-report-r1.json`;
- temporários: `/dev/shm/p1346-*`;
- corpus full consumido: `0`;
- candidato lido/executado: não;
- inputs protegidos modificados: não;
- correção de contrato/oráculo executada pelo adversário: não.

## Limite do resultado

Regime: `executado sem atestacao de isolamento`. O filesystem compartilhado e
o contexto herdado impedem atestação forte. Este veredito cobre somente o gate
focal e o transporte P1346; não julga equivalência funcional geral, não sela
o contrato e não autoriza RED ou implementação.

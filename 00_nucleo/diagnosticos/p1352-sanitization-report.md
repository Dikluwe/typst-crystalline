# P1352 — relatório de sanitização dos artefatos de meta-análise

## Veredito

A sanitização foi concluída sem limiar mínimo de redução. Foram removidos 670
artefatos diagnósticos históricos dos passos P1328–P1350, totalizando
968.308.228 bytes (923,45 MiB) na árvore de trabalho. O volume é somente uma
medição do efeito da limpeza; não é critério de correção nem de encerramento.

As 15 dependências ou referências ainda ativas identificadas antes da remoção
foram preservadas. A verificação posterior encontrou zero referência ativa
quebrada. Nenhum Prompt L0, ADR, código produtivo, passo histórico ou artefato
final de P1351 foi removido.

## Proveniência da medição

- estado-base: commit `e5d0ca402a21ae19ad2dfab64ad0b5960623bd6b`;
- início da medição: `2026-09-11T20:00:37-03:00`;
- verificação final: `2026-09-11T20:20:26-03:00`;
- estado durante a verificação final: working tree não commitado, com as 670
  remoções já staged e o passo/ledger/relatório de P1352 ainda por consolidar;
- escopo medido: ficheiros rastreados em `00_nucleo/diagnosticos/` cujo nome
  começa por P1328–P1351.

Antes da sanitização, esse escopo continha 3.327 ficheiros e 1.083.427.035
bytes (1.033,24 MiB). Depois da sanitização, contém 2.657 ficheiros e
115.118.807 bytes (109,79 MiB).

## Regra aplicada

A seleção foi mecânica e conservadora:

- grupo A: artefatos de P1328–P1340 com pelo menos 1 MiB, salvo referências
  ativas — 327 ficheiros e 962.643.636 bytes (918,05 MiB);
- grupo B: artefatos de P1341–P1350 sem dependência ou referência ativa — 343
  ficheiros e 5.664.592 bytes (5,40 MiB);
- grupo C: duplicados pequenos byte a byte — nenhum ficheiro removido.

Foram preservados nominalmente:

- `p1329-nan-domain-probe-r2.json`;
- `p1331-baseline.json`;
- `p1332-baseline.json`;
- `p1336-measurement.json`;
- `p1337-measurement.json`;
- `p1338-measurement.json`;
- `p1339-full-final-crystalline-before-runs.json`;
- `p1339-full-final-vanilla-runs.json`;
- `p1342-implementation-tests.rs`;
- `p1342-topology-audit-r1.md`;
- `p1350-adapter-integration-blocker-r3.json`;
- `p1350-oracle-boundary-harness-r2.py`;
- `p1350-oracle-boundary-harness-r3.py`;
- `p1350-oracle-supervisor-r2.py`;
- `p1350-oracle-supervisor-r3.py`.

O inventário exato das remoções está em
`00_nucleo/diagnosticos/p1352-sanitization-ledger.tsv`. O ledger possui 672
linhas (cabeçalho do estado-base, cabeçalho tabular e 670 entradas), 126.999
bytes e SHA-256
`35f0969c424977666924ee5378680c5b0a6028b32e6032ced4122e3d0d8cd9c7`.
Cada entrada registra path, tamanho, blob Git, SHA-256, grupo e motivo.

## Verificações posteriores

- referências ativas quebradas: 0;
- autoteste de P1351: 22/22 casos aprovados;
- compatibilidade de fronteira de P1351: 31/31 casos aprovados, com
  `residual_stop=false`;
- `cargo build` com `TMPDIR=/dev/shm`: exit 0, 22,70 s;
- `crystalline-lint .`: exit 0; apenas avisos/informações preexistentes;
- `git diff --check`: aprovado;
- `git diff --cached --check`: aprovado.

## Limites e recuperação

Não foi criado arquivo paralelo, não houve reescrita de histórico e não foi
executado garbage collection. Por isso, esta operação reduz a árvore corrente,
mas não promete reduzir imediatamente o armazenamento interno de `.git`.

Qualquer item pode ser recuperado do estado-base com:

```bash
git show e5d0ca402a21ae19ad2dfab64ad0b5960623bd6b:<path> > <destino>
```

O passo de execução P1352, este relatório e o ledger são os únicos documentos
novos. A sanitização não altera contrato público, comportamento do produto,
fase do pipeline, Prompts L0 nem código das camadas L1–L4.

## Correção de escopo — P1322 em diante

Em `2026-09-12`, o recorte anterior foi corrigido: a acumulação danosa começa
em P1322, não em P1328. A partir do estado-base
`61a56c7c7de3e073f30a009acd2c59ffa9568d65`, havia 1.899 artefatos de
P1322–P1351, com 757.666.018 bytes (722,57 MiB).

Foram retirados 1.558 artefatos gerados — matrizes, recibos, executores,
corpora e resultados intermediários — com 727.578.137 bytes (693,87 MiB).
Permaneceram 341 ficheiros e 30.087.881 bytes (28,69 MiB): todos os 286
diagnósticos Markdown legíveis por humanos e 55 evidências não-Markdown ainda
referenciadas por Prompts L0, ADRs ou código. Todas as referências ativas
extraídas continuam resolvendo para ficheiros existentes.

Não foi criado novo passo, ledger, manifesto ou recibo. O diff do commit desta
correção é o inventário exato e o histórico Git conserva integralmente os
artefatos retirados. Prompts L0, ADRs, materialização e código não foram
alterados.

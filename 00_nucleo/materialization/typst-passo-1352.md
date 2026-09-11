# Passo 1352 — sanitizar artefatos diagnósticos acumulados

## Objetivo

Reduzir o lixo de meta-análise acumulado nos passos recentes sem apagar código
produtivo, Prompts L0, ADRs, passos históricos ou a pequena cadeia necessária
para executar o P1351.

Este é um saneamento mecânico/documental. Não usa materialização segregada,
não produz manifests, receipts, contratos, adversários ou versões de Oracle.
Produz somente um ledger de remoção e um relatório compacto.

## Medição de partida

Estado medido em `2026-09-11T20:00:37-03:00`, commit
`e5d0ca402a21ae19ad2dfab64ad0b5960623bd6b`, árvore limpa:

- diagnósticos `p1328-*` a `p1351-*`: **3.327 arquivos**;
- tamanho total desses arquivos: **1.083.427.035 bytes (1.033,24 MiB)**;
- arquivos com pelo menos 1 MiB: **336**;
- tamanho desses 336 arquivos: **979.905.824 bytes (934,51 MiB)**;
- por extensão: JSON = 1.157 arquivos / 990,45 MiB; TSV = 11 arquivos /
  32,49 MiB; Python = 352 arquivos / 3,17 MiB; Markdown = 272 arquivos /
  1,35 MiB.

Os maiores resíduos são matrizes, inventários e capturas integrais de execução:
três `p1335-matrix-*.json` somam aproximadamente 216 MiB; o
`p1339-verifier-full-c-public-r1.json` possui aproximadamente 40 MiB; capturas
de sentinelas, runs, workspace tests e lint repetem stdout/estado derivável.

## O que fica protegido

Nunca remover neste passo:

- qualquer arquivo fora de `00_nucleo/diagnosticos/`;
- qualquer passo em `00_nucleo/materialization/`;
- ADRs, Prompts L0, núcleos Tekt, código L1–L4, testes produtivos ou `lab/`;
- todos os três artefatos finais P1351;
- dependências transitivas atualmente carregadas pelo módulo P1351;
- qualquer arquivo citado por código, teste produtivo, ADR ou Prompt L0
  retido, salvo se a referência for substituída antes da remoção.

Allowlist mínima P1350/P1351:

- `p1350-adapter-integration-blocker-r3.json`;
- `p1350-oracle-supervisor-r2.py`;
- `p1350-oracle-supervisor-r3.py`;
- `p1350-oracle-boundary-harness-r2.py`;
- `p1350-oracle-boundary-harness-r3.py`;
- `p1351-boundary-compat-r1.py`;
- `p1351-ab-tests-r2.py`;
- `p1351-verifier-report.json`.

Antes de apagar, calcular recursivamente imports e paths **carregados em
runtime** por esses oito arquivos. Qualquer dependência executável adicional
entra na allowlist e é registrada no relatório; referências históricas de
proveniência em blockers/reports continuam recuperáveis pelo ledger e commit
base, sem reter a cadeia inteira. Não se corrige dependência apagando a
referência ou copiando o arquivo silenciosamente.

## Seleção de remoção

### Grupo A — blobs derivados grandes

Selecionar arquivos rastreados `p1328-*` a `p1340-*` em
`00_nucleo/diagnosticos/` com tamanho maior ou igual a 1.048.576 bytes.

Esses arquivos são saídas integrais de runs, matrizes, inventários, lint,
comparações ou ledgers derivados. A fonte e o histórico Git permanecem; o HEAD
não precisa carregar cópias de dezenas de megabytes para sustentar decisões já
registradas nos passos e commits.

Uma exceção só é permitida quando a checagem de referências provar dependência
ativa fora de materialization/context. A exceção deve constar nominalmente no
relatório.

### Grupo B — bundles protocolares encerrados

Selecionar todos os arquivos `p1341-*` a `p1350-*` em
`00_nucleo/diagnosticos/`, exceto a allowlist e sua dependência transitiva.

Isso inclui versões encerradas de authorship, receipts, manifests, contracts,
bindings, deliveries, adversarial suites, adapters, blockers intermediários e
checkers que só existem para reconstruir a cadeia protocolar. A cadeia continua
recuperável no commit base; no HEAD fica somente o fechamento compacto e o que
o P1351 realmente executa.

### Grupo C — duplicatas pequenas óbvias

Fora dos grupos A/B, remover apenas arquivos cujo conteúdo seja byte-a-byte
idêntico a outro arquivo retido. Manter o nome mais recente citado; registrar
ambos os paths e o SHA-256 comum. Não usar similaridade textual ou julgamento
semântico para apagar arquivos pequenos nesta etapa.

## Ledger e recuperação

Antes de remover, escrever
`00_nucleo/diagnosticos/p1352-sanitization-ledger.tsv` com uma linha por alvo:

```text
path<TAB>bytes<TAB>git_blob_sha1<TAB>sha256<TAB>grupo<TAB>motivo
```

O ledger é ordenado por path e não incorpora o conteúdo removido. Seu cabeçalho
registra o commit base completo. Todo arquivo permanece recuperável por:

```text
git show e5d0ca402a21ae19ad2dfab64ad0b5960623bd6b:<path>
```

Não criar tar, zip, diretório de archive ou cópia em outro ponto do repositório:
isso apenas deslocaria o lixo. Não executar `git filter-repo`, rebase, GC ou
reescrita de histórico. A remoção reduz o checkout/HEAD; os blobs antigos
continuam no histórico Git até uma decisão separada e explicitamente
destrutiva.

## Execução segura

1. exigir HEAD igual ao commit base e nenhuma alteração além deste passo;
2. construir a lista em `/dev/shm`, nunca por glob destrutivo não inspecionado;
3. normalizar e rejeitar qualquer path fora de `00_nucleo/diagnosticos/`;
4. calcular allowlist e dependências transitivas;
5. gerar ledger antes da remoção;
6. conferir contagem e soma de bytes do ledger contra a lista;
7. remover somente os paths exatos do ledger;
8. procurar referências quebradas fora de materialization/context;
9. executar os testes P1351 `22/22` e `31/31` com temporários em `/dev/shm`;
10. executar `cargo build`, `crystalline-lint .` e `git diff --check`;
11. produzir `p1352-sanitization-report.md` e um único commit.

O relatório registra HEAD/base, hora, comandos, número e bytes antes/depois,
quantidade por grupo, exceções, hashes do ledger, testes e limitações. Não cria
um protocolo de verificação adicional.

## Critérios de aceitação

- redução resultante registrada, sem meta mínima de bytes;
- nenhum path removido fora de `00_nucleo/diagnosticos/`;
- P1351 continua executável com `22/22` e `31/31`;
- nenhuma referência ativa quebrada em ADR/L0/código/testes produtivos;
- build, linter e diff-check passam;
- ledger permite localizar cada blob removido no commit base;
- árvore final contém apenas o ledger, o relatório e os arquivos retidos;
- nenhum archive paralelo nem reescrita de histórico.

Preservar toda dependência ativa e registrar a redução real. Volume removido
não é critério de correção: não apagar código ou evidência ativa para atingir
uma métrica arbitrária.

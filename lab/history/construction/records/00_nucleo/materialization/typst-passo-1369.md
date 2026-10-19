# Passo 1369 — medição bilateral do vetor efetivo S02

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Consumir o observador P1368 numa matriz bilateral real e produzir um novo
veredito focal para precedência de PDF tagueado, sem alterar o produto.

## Entradas congeladas

- vanila ratificado: `lab/typst-original/target/release/typst`, identificador administrativo
  [dado administrativo removido];
- cristalino: `target/release/typst`, identificador administrativo
  [dado administrativo removido];
- `pdfinfo` 26.05.0, identificador administrativo
  [dado administrativo removido];
- observador P1368, identificador administrativo
  [dado administrativo removido];
- contrato P1368, identificador administrativo
  [dado administrativo removido].

## Matriz e observável

Executar os três casos `top_level`, `document_over_cli` e `cli_only` para
vanila e cristalino nos perfis `default`, `html`, `a11y-extras` e
`html+a11y`: 24 compilações e, somente após cada sucesso, no máximo 24 chamadas
`pdfinfo`. Cada tripla alimenta `classify_s02`; `Unknown` nunca vira match.

O resultado bilateral é `MATCH` por perfil somente quando os dois produtos
obtêm `Preserved` e o mesmo vetor canônico `(true,false,true)`. Resultado
incompleto é `Unknown`; vetores completos diferentes são `DIFFERENCE`.

## Segurança de execução

O runner usa um processo por vez, diretório temporário isolado por caso,
timeout de 45 s, `RLIMIT_AS=2 GiB`, `RLIMIT_FSIZE=16 MiB`, `RLIMIT_CPU=45 s`,
canais em arquivos temporários e leitura máxima de 1 MiB por canal. Não usa
`PIPE`, paralelismo, Cargo, watcher, rede deliberada ou suíte ampla.

Falha de limite, sinal, timeout, output ausente ou observador inválido produz
`Unknown` com reason code. O runner não repete automaticamente uma falha.

## L0, regime e alegação

Somente `00_nucleo/diagnosticos/` é alterado; nenhum Prompt L0 nem L1–L4 muda.
Protocolo completo executado sem atestação de isolamento, porque os papéis
compartilham workspace e autoridade desta sessão. O veredito cobre apenas S02.


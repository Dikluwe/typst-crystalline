# Passo 1370 — retificação causal do oráculo S02

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Substituir, sem reescrever o histórico, a premissa P1368/P1369 refutada de que
o setting do documento venceria a opção explícita do CLI. Reclassificar o
resultado P1369 já medido, sem repetir processos externos.

## Medição anterior à decisão

P1369 observou `(true,true,true)` para vanila e cristalino nos quatro perfis.
A fonte vanila pinada declara em `typst-pdf/src/lib.rs:89-100` que
`PdfOptions.format` substitui os defaults definidos pelo documento. Em
`typst-cli/src/compile.rs:162-166,664-672`, `--pdf-tagged=true` torna-se
`Smart::Custom(true)` nessa camada de override.

Logo, o vetor normativo retificado é:

```text
top-level tagged:true                 -> true
document tagged:false + CLI true      -> true
CLI true sem setting no documento     -> true
```

## Contrato

O reclassificador aceita somente relatório consolidado da limpeza com identificador administrativo
[dado administrativo removido],
tamanho máximo 64 KiB, identidades congeladas, exatamente 24 jobs únicos e
observações booleanas completas.

Cada produto/perfil é `Preserved` somente para `(true,true,true)`. Outro vetor
completo é `Violated`; falta, duplicação, valor não booleano ou observação
incompleta é `Unknown`. O bilateral é `MATCH` somente quando os dois produtos
são `Preserved` com o mesmo vetor. Não ler os verdicts derivados P1369, pois
eles incorporam o oráculo refutado; reconstruir a classificação das
observações brutas.

## Teste e implementação

Um teste in-memory cobre a matriz canônica, os sete vetores booleanos
incorretos, uma observação opaca e uma matriz estruturalmente incompleta. A
execução produtiva lê uma vez o JSON pinado e escreve somente o resumo P1370.

## Regime e memória

Protocolo completo executado sem atestação de isolamento. Nenhum L0 ou L1–L4
muda. Proibidos Cargo, Typst, `pdfinfo`, watch, rede e suíte ampla. Teste com
timeout de 30 s; input e output limitados a 64 KiB; nenhuma execução externa
além do próprio Python de reclassificação.


# Passo 1370 — retificação causal do oráculo S02

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

O reclassificador aceita somente `p1369-results.json` com SHA-256
`1e6c38c2c7fe387e4877d18eb6890e11a25d8572e3a39483bbaf3261dded7b05`,
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


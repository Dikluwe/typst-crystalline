# Passo 1368 — observador efetivo S02 para PDF tagueado

## Objetivo

Fechar a limitação instrumental `S02_TOP_LEVEL_EFFECT` do P1357 sem alterar o
produto: transformar a linha pública `Tagged:` do `pdfinfo` num valor semântico
canônico e comparar as três testemunhas que isolam precedência.

## Contrato do harness

`p1368_s02_observer.py` fornece:

```python
parse_tagged_field(stdout: bytes) -> Observation
classify_s02(top_level, document_over_cli, cli_only) -> Verdict
```

O parser aceita exatamente uma linha `Tagged: yes|no`, ignorando caixa e
espaço externo. Ausência, duplicação, valor diferente, UTF-8 inválido ou
budget excedido produz `Unknown`, nunca `MATCH`.

O classificador retorna `Preserved` somente para o vetor canônico:

```text
top-level tagged:true                 -> true
document tagged:false + CLI true      -> false
CLI true sem setting no documento     -> true
```

Qualquer valor completo diferente retorna `Violated`; qualquer entrada opaca
retorna `Unknown`. Repetição e ordem das linhas não alteram o resultado, salvo
duplicação de `Tagged:`, que é ambígua e permanece `Unknown`.

## Implementação e teste

Um teste Python exato, sem subprocesso, filesystem ou PDF real, cobre o vetor
positivo, mutações dos três bits e entradas opacas. O módulo produtivo do
harness pode oferecer CLI posterior para observar PDFs, mas este passo não
executa vanila, cristalino, `pdfinfo`, Cargo, watch nem suíte ampla.

## L0 e ADR-0127

O delta pertence somente a `00_nucleo/diagnosticos/`: é instrumentação do
experimento e não legitima nem altera L1–L4. Nenhum Prompt L0 é criado ou
resselado.

## Regime, memória e alegação

Protocolo completo, executado sem atestação de isolamento por workspace
compartilhado. Máximo de duas execuções do teste exato, timeout de 30 s,
1 MiB por stream e 4 MiB agregados. A alegação cobre somente a capacidade do
observador de tornar S02 discriminatório; não declara paridade do produto.


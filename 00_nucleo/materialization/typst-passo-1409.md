# Passo 1409 — reclassificar rotas compiladas contaminadas por `--jobs`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** implementado e verificado no recorte.

## Objetivo

Reexecutar as seis rotas de elementos que P1400 deixou `Unknown` porque o
instrumento passou `compile --jobs 1` ao cristalino, cuja CLI rejeita essa
opção. A nova medição deve retirar o par de argumentos de **ambos** os
produtos, preservar os oráculos do vanila e produzir uma classificação útil
antes de escolher a próxima mudança pública.

## Proveniência inicial

- HEAD: `6bd6aeed8ac24526747bf21d37d4de39652e5466`.
- Working tree inicial: limpo.
- Instante: `2026-09-15T13:58:41-03:00`.
- Vanila ratificado: `lab/typst-original/target/release/typst`, revisão
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`, identificador administrativo
  [dado administrativo removido].
- Cristalino: `target/debug/typst`, identificador administrativo
  [dado administrativo removido].
- Fonte dos casos e oráculos: relatório consolidado da limpeza.
- Fonte da seleção: relatório consolidado da limpeza.

## Medição antes da decisão

P1400 registou nas seis rotas selecionadas `OUTPUT_MISSING` na execução
qualificada. Os recibos mostram uma causa anterior à semântica Typst:

```text
error: unexpected argument '--jobs' found
```

O vanila concluiu os mesmos comandos porque aceita a opção. A enumeração
mecânica dos oráculos contém exatamente 6 rotas e 14 sondas por ordem:

- `outline.entry.indented`, `outline.entry.inner`, `outline.entry.page` e
  `outline.entry.prefix`, cada uma com construção, invocação e forma
  qualificada;
- `par.line` e `raw.line`, cada uma com forma qualificada.

## Classificação e decisão

- A opção `--jobs` é mecânica de execução, não superfície da linguagem
  medida por estas rotas (ADR-0107).
- A inferência de ausência de `par.line` ou `raw.line` seria inválida enquanto
  a compilação não alcançar a expressão. É refutada se a execução simétrica
  sem `--jobs` terminar com uma classificação estável diferente de ausência.
- P1409 é um reparo do instrumento em `lab/` e não altera L1–L4; portanto não
  cria Prompt L0 nem aciona o gate de mudança pública da ADR-0127.

## Contrato do instrumento

1. Selecionar somente os seis IDs congelados que eram `Unknown` em P1400.
2. Exigir que cada template compilado contenha exatamente um par adjacente
   `--jobs 1`; retirar o mesmo par para vanila e cristalino.
3. Não alterar feature, formato, fonte, destino, oráculo esperado nem regra de
   classificação de P1400.
4. Executar `direct`, `direct-repeat` e `reverse`, sempre serialmente.
5. Cada processo herda `RAYON_NUM_THREADS=1`, recebe `RLIMIT_AS=512 MiB`,
   timeout de 15 s e streams limitados a 64 KiB pelo executor P1400.
6. Divergência entre ordens, drift do oráculo vanila, timeout, swap, OOM ou
   truncamento resulta em `Unknown`; nunca em ausência.
7. Brutos ficam sob `/tmp`; o repositório recebe somente relatório compacto.

## Testes e ataques

- autoteste mata remoção só no cristalino, remoção de flag sem o valor,
  aceitação de par ausente/duplicado e mutação de argumentos vizinhos;
- gate confirma cardinalidade 6/14 e que todos os casos eram `Unknown` por
  `OUTPUT_MISSING` em P1400;
- campanha bilateral confirma estabilidade entre as três ordens e oráculo
  vanila sem drift;
- a campanha totaliza no máximo 84 processos de produto, sem paralelismo e
  sem compilar qualquer harness Rust.

## Veredito

### Execução

- O autoteste de normalização passou e rejeitou par ausente, valor diferente
  de `1`, par duplicado, par não adjacente, remoção parcial e encerramentos por
  sinal tratados como simples artefato ausente.
- O build release do HEAD inicial terminou sob limite de 2 GiB, um job,
  incremental/debug desligados e 512 codegen units em 9 min 25 s. identificador administrativo do
  binário produzido:
  [dado administrativo removido].
- A campanha final executou 84 processos, com paralelismo 1, RSS máximo de
  68.676 KiB dentro do executor, swap zero, nenhum timeout e nenhuma
  truncagem. As três ordens produziram a mesma classificação.
- `par.line` chegou ao diagnóstico específico `function par does not contain
  field line` nas três ordens e foi classificado `ABSENT`.
- As formas qualificadas dos quatro campos de `outline.entry` retornaram o
  mesmo `repr` nos dois produtos. A construção via `eval --in`, porém, abortou
  no cristalino por falha de alocação nas três ordens; por isso as quatro
  rotas permanecem `Unknown`, sem promoção parcial.
- `raw.line` também abortou por falha de alocação nas três ordens e permanece
  `Unknown`; não há evidência válida de ausência nem de presença funcional.
- Uma reprodução focal adicional de `outline.entry.indented` sob 2 GiB e 15 s
  abortou em 0,46 s ao pedir mais 316.096 bytes, com RSS máximo de 163.316 KiB
  e swap zero. Isso refuta a hipótese de que o teto de 512 MiB, sozinho,
  explique o comportamento e aponta crescimento/reserva virtual sem limite
  no caminho `eval --in`.
- Resultado compacto: relatório consolidado da limpeza.
- Estado final medido em `2026-09-15T14:16:57-03:00`, sobre o mesmo HEAD
  inicial e working tree não commitado com três ficheiros novos: este passo,
  o instrumento P1409 e o resultado compacto. O resultado tem identificador administrativo
  [dado administrativo removido];
  o instrumento tem identificador administrativo
  [dado administrativo removido].
- `python3 -m py_compile`, `git diff --check` e `crystalline-lint .`
  terminaram com código zero; o linter manteve zero violations e os warnings
  preexistentes. Nenhum harness monolítico de Rust foi compilado ou executado.

**Veredito:** `PASS` para o reparo simétrico do instrumento e para a
classificação de `par.line` como ausente. `Unknown` obrigatório para os quatro
campos de `outline.entry` e para `raw.line` enquanto o estouro em `eval --in`
e no show de raw não for isolado. Protocolo Tekt executado sem atestação de
isolamento.

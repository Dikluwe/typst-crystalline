# Passo 1416 — uma numeração transversal em `#columns()` segmentado

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Corrigir a duplicação de `par.line` entre segmentos irmãos de
`#columns(N)[... #colbreak() ...]` quando as linhas usam métricas uniformes.
Cada ordinal transversal deve consumir e emitir um único número no bordo
externo configurado do contentor. Colunas de página continuam sob a regra
P1415 e não participam desta deduplicação.

## Estado e proveniência inicial

- baseline: commit `f4537577af3daba48cb880d7300e2e986f47702c`, árvore limpa
  antes da criação das oito fixtures P1416;
- medição inicial: `2026-09-16T07:54:00-03:00`;
- cristalino: `target/release/typst`, identificador administrativo
  [dado administrativo removido];
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário identificador administrativo
  [dado administrativo removido];
- regime: protocolo Tekt completo em rehearsal, executado sem atestação de
  isolamento.

Todas as sondas receberam 512 MiB de espaço virtual, uma thread e timeout de
15 s. O maior RSS da primeira matriz foi 42.796 KiB no cristalino e
31.764 KiB no vanila; swap permaneceu zero.

## Medição antes da decisão

No caso mínimo, o cristalino produziu
`1 before / 2 left + 3 right / 4 after`; o vanila produziu
`1 before / 2 left + right / 3 after`. Com três colunas, o cristalino consumiu
três números internos e o vanila somente um. Com duas linhas à esquerda e
três à direita, o vanila emitiu ordinais internos `2, 3, 4`, isto é, o máximo
de linhas entre as colunas, e não a soma `2 + 3` do cristalino.

Uma primeira coluna vazia não remove a ocorrência: o vanila põe o número no
bordo esquerdo do contentor e o texto na segunda coluna. `number-margin:right`
move a ocorrência única ao bordo direito externo. Callback funcional conserva
a mesma cardinalidade: `L1 before / L2 left + right / L3 after`.

O controle `#set page(columns: 2)` coincide bilateralmente e continua a
numerar as duas colunas conforme P1415. Já a fixture com 18pt na primeira
coluna e 8pt na segunda não segue a equivalência simples por ordinal; baselines
e quantidades observáveis diferem. Ela é um refutador de generalização e fica
`Unknown`, não um positivo deste passo.

## Oráculos congelados

| Fixture | identificador administrativo | Papel |
|---|---|---|
| `segmented-two-columns.typ` | [dado administrativo removido] | positivo mínimo |
| `segmented-three-columns.typ` | [dado administrativo removido] | cardinalidade N |
| `segmented-empty-first.typ` | [dado administrativo removido] | origem externa |
| `segmented-unequal-lines.typ` | [dado administrativo removido] | máximo, não soma |
| `segmented-callback.typ` | [dado administrativo removido] | transcript único |
| `segmented-right-margin.typ` | [dado administrativo removido] | bordo final externo |
| `page-columns-control.typ` | [dado administrativo removido] | controle P1415 |
| `segmented-style-heights.typ` | [dado administrativo removido] | opaco `Unknown` |

As fixtures são externas ao candidato e não podem ser adaptadas ao patch.
Timeout, OOM, swap, output ausente ou promoção do caso opaco contam como
`Unknown`, nunca como preservação.

## Contrato interno

1. Somente `#columns()` segmentado por `colbreak()` instala a coordenação
   transversal. `#set page(columns:)` preserva P1415; fluxo contínuo sem
   separador permanece fora deste recorte.
2. Ao entrar no contentor, guardar contador/página de `par.line`. Cada segmento
   irmão recomeça desse mesmo snapshot lógico.
3. Manter o maior número de ordinais já materializados. Um segmento suprime
   ordinais até esse limite, sem callback, item ou fixup; somente sua cauda
   nova emite. Ao sair, o contador avança pelo máximo de linhas entre os
   segmentos, nunca pela soma.
4. A numeração emitida é compensada horizontalmente para o bordo externo do
   contentor: origem da primeira coluna para `left/start`, fim da última para
   `right/end`. Conteúdo textual permanece na sua coluna.
5. Pattern e callback percorrem a mesma decisão de cardinalidade. Request
   suprimida não nasce; request de cauda usa o inteiro documental correto e
   continua realizada fora de L1.
6. O estado é privado, finito e escalar, integra checkpoint/rollback e é
   restaurado ao sair. Não há vetor por linha, cópia de página, recursão ou
   custo dependente da magnitude do contador.

## Limites e refutadores

Este passo não reivindica paridade para métricas verticais diferentes entre
segmentos, RTL, colunas contínuas sem `colbreak()`, nested columns nem mudança
local de `par.line` entre irmãos. Esses casos exigem medição de baselines e
política própria. Não ordenar por texto extraído, não deduplicar por string do
número e não executar `Func` em `columns.rs` ou Cursor.

A mudança corrige paridade interna de um contrato público já materializado;
não cria campo público, default, assinatura nem fase. Pelo ADR-0127, segue em
fluxo contínuo com L0 atualizado antes do código e RED→GREEN focal.

## Protocolo de memória

- probes: 512 MiB, uma thread, timeout 15 s, saída em `/tmp`;
- check/build: 2 GiB, um job, incremental desligado, debug zero,
  `codegen-units=512`, linker `gold` para release;
- lint: 1 GiB, uma thread, timeout 60 s;
- proibidos `cargo test --workspace`, `cargo test -p typst-core --lib` e
  matrizes amplas antes do GREEN focal.

## Aceitação

Os seis positivos segmentados coincidem bilateralmente em texto e ordem; o
callback não cria request duplicada; o controle de colunas de página permanece
verde; P1414/P1415 focais não regridem. A fixture de alturas distintas
permanece relatada como `Unknown`. Formatação, diff check, build focal e
`crystalline-lint .` passam nos limites sem OOM ou swap.

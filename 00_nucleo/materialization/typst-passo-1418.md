# Passo 1418 — consolidação física de patterns nas colunas de página

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Aplicar a regra vertical P1417 separadamente em cada coluna física criada por
`#set page(columns: N)`, conservando em todas elas o estilo tipográfico
compartilhado capturado na primeira ocorrência numerável da composição. O
contador continua documental ou por página e a última coluna continua forçada
à margem final conforme P1415.

## Regime Tekt e proveniência inicial

- regime: protocolo completo em rehearsal, executado sem atestação de
  isolamento; uma única autoridade escreve obrigação, oráculos, candidato e
  veredito, portanto não há alegação de segregação atestada;
- baseline: commit `573c42c14`, árvore limpa antes das quatro fixtures P1418;
- medição inicial: `2026-09-16T09:28:44-03:00`;
- cristalino: `target/release/typst`, identificador administrativo
  [dado administrativo removido];
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário identificador administrativo
  [dado administrativo removido];
- working tree na medição: somente `lab/parity/corpus/p1418/` não rastreado.

As sondas receberam 512 MiB de espaço virtual, uma thread e timeout de 15 s.
O maior RSS inicial foi 43.636 KiB no cristalino e 31.128 KiB no vanila; swap
permaneceu zero.

## Medição antes da decisão

Com primeira coluna a 18 pt e segunda a 8 pt, o cristalino P1417 emitiu cinco
números; o vanila emitiu quatro, suprimindo a ocorrência central da segunda
coluna. Todos os números vanila têm a morfologia de 18 pt. Com a ordem de
tamanhos invertida, o vanila preserva três números na primeira coluna e dois na
segunda; todos têm a morfologia de 8 pt, inclusive os que acompanham texto de
18 pt.

Em três colunas, a fonte pinada reinicia `prev_bottom` por coluna: as colunas
de 8 pt aceitam primeira e terceira baselines, mas nunca comparam sua primeira
baseline com o fundo da coluna irmã. Os números continuam no estilo de 18 pt
da primeira coluna. Isso coincide com
`layout_line_numbers(..., column, output)` em
`lab/typst-original/crates/typst-layout/src/flow/compose.rs:996-1100`: a lista
é recolhida do frame de uma coluna, enquanto `config.shared` fornece o estilo
comum e o índice físico decide a margem final.

Callback heterogêneo exibe a mesma divergência, mas a altura do conteúdo
retornado só existe depois da realização fora de L1. Ele permanece `Unknown`;
este passo não converte callback em pattern nem presume sua altura.

## Oráculos congelados

| Fixture | identificador administrativo | Classificação |
|---|---|---|
| `page-first-large-second-small.typ` | [dado administrativo removido] | positivo: shared 18 pt |
| `page-first-small-second-large.typ` | [dado administrativo removido] | positivo: shared 8 pt |
| `page-three-shared-first-style.typ` | [dado administrativo removido] | positivo: isolamento por coluna N=3 |
| `page-callback-heterogeneous-unknown.typ` | [dado administrativo removido] | opaco `Unknown` |

Os casos foram escritos antes do candidato. Eles rejeitam os mutantes “estilo
local por coluna”, “uma deduplicação global para a página”, “máximo ordinal
P1416” e “callback tratado como pattern”. Como esta execução não possui
autoridades isoladas nem campanha formal de mutation testing, isso é poder
discriminatório observado em rehearsal, não selo independente.

## Contrato interno

1. Pattern string em coluna de página segmentada registra a mesma candidata
   P1417 e não incrementa contador durante o flush.
2. A primeira coluna que contém candidata captura um único `TextStyle`
   compartilhado no estado transitório do fluxo de página; colunas seguintes
   nunca o substituem por estilo local.
3. `close_current_column` ordena e consolida somente a cauda da coluna que está
   fechando, antes da translação. `prev_bottom` nasce vazio em cada coluna
   física. O modo segmentado direto aplica a mesma drenagem no seu owner.
4. Candidata aceita avança scope/contador, formata no estilo compartilhado e
   usa margem/alinhamento/clearance causais. A última coluna força margem final.
5. `#columns()` continua no caminho transversal P1417; callbacks continuam no
   caminho imediato P1415/P1416 e não entram no buffer.
6. Estado é privado, finito e transacional. Não se adiciona Engine, frame,
   documento, recursão ou página inteira ao buffer.

Uma primeira tentativa local instalou a drenagem somente em
`layout_segmented`; os três positivos permaneceram RED porque os wrappers de
estilo escondem o `colbreak()` do splitter e essas construções percorrem
`layout_flow`/`close_current_column`. Esta hipótese foi abandonada após uma
revisão sem ganho; a correção passa ao owner físico medido, sem alterar os
oráculos.

## Limites e `Unknown`

Callback heterogêneo, primeira coluna sem ocorrência numerável, mudança local
de `par.line` entre colunas, RTL e colunas aninhadas ficam `Unknown`. Timeout,
OOM, swap, output ausente ou promoção de opaco nunca contam como sucesso.

A mudança corrige paridade interna de comportamento público existente, sem
campo público, default, assinatura ou mudança de fase. Pelo ADR-0127 segue em
fluxo contínuo: L0 primeiro, RED→GREEN focal, resselo e revalidação.

## Protocolo de memória

- probes: 512 MiB, uma thread, timeout 15 s, outputs em `/tmp`;
- check/build: 2 GiB, um job, incremental desligado, debug zero,
  `codegen-units=512`, linker `gold` no release;
- lint: 1 GiB, uma thread, timeout 60 s;
- proibidos testes de workspace e matrizes amplas antes do GREEN focal.

## Aceitação

Os três positivos coincidem bilateralmente em texto, ordem e morfologia dos
números marginais; o callback opaco permanece `Unknown`. Os três positivos
P1417 e os sete controles uniformes P1416 não regridem. Formatação, diff check,
build release e `crystalline-lint .` passam nos limites sem OOM ou swap.

## Resultado da materialização

Medição final em `2026-09-16T09:45:27-03:00`, sobre working tree não
commitada derivada de `573c42c14`. Arquivos alterados: os três prompts L0,
`columns.rs`, `cursor.rs`, `layout/mod.rs`, este passo e as quatro fixtures
P1418. O binário cristalino final tem identificador administrativo
[dado administrativo removido].

Os três positivos têm texto simples idêntico ao vanila e as bounding boxes dos
números confirmam o estilo compartilhado de 18 pt ou 8 pt esperado. O modo
`pdftotext -layout` ainda expõe diferenças de baseline do conteúdo textual das
colunas, anteriores e fora deste contrato; elas não alteram sequência,
cardinalidade ou morfologia dos números. O callback heterogêneo continuou
`Unknown`, sem promoção implícita.

O maior RSS da matriz final foi 43.064 KiB, incluindo o opaco, sob 512 MiB e
sem swap. Os três positivos P1417 e os sete controles P1416 permaneceram
verdes. O build release terminou sob 2 GiB após uma tentativa atingir o timeout
de 180 s sem OOM e a retomada terminar em 81 s. Regime final: protocolo Tekt
completo em rehearsal, executado sem atestação de isolamento.

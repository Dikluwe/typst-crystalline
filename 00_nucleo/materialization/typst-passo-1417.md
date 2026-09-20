# Passo 1417 — consolidação vertical de padrões `par.line` em colunas segmentadas

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Substituir a equivalência por ordinal do P1416 pela regra vertical observada no
vanila para `par.line(numbering: "...")` através das regiões irmãs de
`#columns()`: ordenar as ocorrências pela baseline
e suprimir uma ocorrência cuja baseline esteja acima do fundo do último número
aceite. O estilo tipográfico do número pertence ao contexto exterior comum, não
ao estilo local da linha numerada.

## Estado e proveniência inicial

- baseline: commit `af587e30bb9d188d8a8eb1064bb36175b50920da`, árvore limpa
  antes da criação das cinco fixtures P1417;
- medição inicial: `2026-09-16T08:12:10-03:00`;
- cristalino: `target/release/typst`, identificador administrativo
  [dado administrativo removido];
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário identificador administrativo
  [dado administrativo removido];
- alterações ainda não commitadas na medição: somente
  `lab/parity/corpus/p1417/` com as cinco fixtures congeladas abaixo;
- regime: protocolo Tekt completo em rehearsal, executado sem atestação de
  isolamento.

Todas as sondas receberam 512 MiB de espaço virtual, uma thread e timeout de
15 s. O maior RSS observado foi 42.856 KiB no cristalino e 31.744 KiB no
vanila; swap permaneceu zero.

## Medição antes da decisão

Na fixture com 18 pt à esquerda e 8 pt à direita, o cristalino P1416 produziu
`1 before / 2 left-a + right-a / right-b / 4 right-c / 3 left-b / 5 after`.
O vanila produziu
`1 before / 2 / 3 left-a + right-a / right-b / 4 right-c / 5 left-b / 6 after`.
A inversão da coluna alta confirma que a decisão não é o máximo de ordinais:
o vanila aceita novamente quando a baseline seguinte já não intersecta o
número anterior.

Na fonte pinada, `layout_line_numbers` recolhe marcadores, ordena-os por `y` e
descarta `y < prev_bottom`; depois define
`prev_bottom = y + max(frame.height(), 1pt)`
(`lab/typst-original/crates/typst-layout/src/flow/compose.rs:996-1100`). O
marcador é colocado na baseline da linha
(`lab/typst-original/crates/typst-layout/src/inline/line.rs:648`). Os números
do caso heterogêneo têm todos o tamanho exterior padrão, apesar de as linhas
terem 18 pt e 8 pt: o estilo local não é a morfologia do número marginal.

O controle heterogêneo de `#set page(columns: 2)` também diverge no P1416, mas
sua configuração tipográfica compartilhada nasce no owner exterior da página,
fora deste recorte. Ele permanece `Unknown` e refuta a aplicação oportunista
do estilo da primeira linha local. Já um callback heterogêneo
permanece opaco: sua altura só é conhecida após realização e pode alterar a
supressão posterior. Este passo não aproxima essa circularidade por texto,
ordinal ou altura presumida.

## Oráculos congelados

| Fixture | identificador administrativo | Papel |
|---|---|---|
| `heterogeneous-left-tall.typ` | [dado administrativo removido] | ordem vertical transversal |
| `heterogeneous-right-tall.typ` | [dado administrativo removido] | assimetria/refutador do máximo ordinal |
| `heterogeneous-right-margin.typ` | [dado administrativo removido] | bordo externo final |
| `page-columns-heterogeneous-control.typ` | [dado administrativo removido] | refutador `Unknown` de página |
| `heterogeneous-callback-unknown.typ` | [dado administrativo removido] | refutador `Unknown` |

As fixtures são externas ao candidato. Timeout, OOM, swap, ausência de output
ou promoção do callback opaco contam como `Unknown`, nunca como preservação.

## Contrato interno

1. Durante uma coluna segmentada, pattern string não é emitido imediatamente:
   registrar somente baseline, página, pattern e `StyleChain` da ocorrência.
   Callback conserva o caminho P1415/P1416 existente.
2. Em `#columns()`, consolidar todas as candidatas irmãs numa única sequência
   estável por baseline. Colunas de página preservam o caminho P1415.
3. Para cada candidata em ordem, suprimir se `baseline < prev_bottom`. Para a
   aceita, formatar o próximo contador e avançar
   `prev_bottom = baseline + max(altura_do_numero, 1pt)`.
4. A altura e a largura do número usam o `TextStyle` exterior capturado na
   entrada do owner de colunas. Margem, alinhamento e clearance continuam
   resolvidos da chain causal da ocorrência. A numeração permanece alinhada à
   baseline causal.
5. O contador documental/por página avança somente para candidatas aceitas.
   O bordo marginal é o bordo externo total de `#columns()`.
6. O buffer é privado, finito e linear no número de linhas candidatas; integra
   checkpoint/rollback e é drenado ao sair. Não retém frames, callback
   realizado, documento, Engine ou página inteira.

## Limites e refutadores

Callbacks com métricas heterogêneas, colunas de página heterogêneas, mudança
local da configuração `par.line` entre irmãs, RTL e colunas aninhadas
permanecem `Unknown`. Não executar `Func`
em L1, não deduplicar por número/string, não usar ordem textual extraída e não
aplicar a regra entre colunas físicas distintas de página.

A mudança é correção interna de paridade de comportamento já público; não cria
campo público, default, assinatura ou fase. Pelo ADR-0127 segue em fluxo
contínuo, com L0 atualizado antes do código e gate RED→GREEN focal.

## Protocolo de memória

- probes: 512 MiB, uma thread, timeout 15 s, saída em `/tmp`;
- check/build: 2 GiB, um job, incremental desligado, debug zero,
  `codegen-units=512`, linker `gold` no release;
- lint: 1 GiB, uma thread, timeout 60 s;
- proibidos `cargo test --workspace`, `cargo test -p typst-core --lib` e
  matrizes amplas antes do GREEN focal.

## Aceitação

Os três positivos de `#columns()` coincidem bilateralmente em texto, ordem e
posição marginal; os controles uniformes P1414-P1416 não regridem. Callback e
colunas de página heterogêneas permanecem documentados como `Unknown`.
Formatação, diff check,
build focal e `crystalline-lint .` passam nos limites sem OOM ou swap.

## Resultado da materialização

Medição final em `2026-09-16T08:52:20-03:00`, sobre working tree não
commitada derivada de `af587e30bb9d188d8a8eb1064bb36175b50920da`. Arquivos
alterados: os três prompts L0 deste passo, `columns.rs`, `cursor.rs`,
`layout/mod.rs`, este passo e as cinco fixtures de `lab/parity/corpus/p1417/`.
O binário cristalino final tem identificador administrativo
[dado administrativo removido].

As três provas positivas coincidiram byte a byte no texto extraído com o
vanila ratificado. O maior RSS dessa matriz final foi 41.088 KiB, sob o limite
de 512 MiB; swap permaneceu zero. Os sete controles verdes P1416 — inclusive
callback uniforme e colunas de página uniformes — também coincidiram. Os dois
refutadores heterogêneos continuam `Unknown`, como especificado. O build
release terminou sob 2 GiB; a primeira tentativa final chegou ao timeout de
180 s durante compilação/ligação, sem OOM, e a retomada com o mesmo limite
terminou em 88 s.

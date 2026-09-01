# P1290 — recibo do autor do contrato

**Regime:** protocolo Tekt completo, fases 1–2 somente
**Papel:** autor do contrato
**Executor:** `/root/p1290_contract`
**Manifesto:** SHA-256
`8b18feaf2b0f75b3918d04b84e9dcf368d79df96fe7b9e2e92d2b727f4b10cda`

## Ordem causal

1. a invalidação SHA-256
   `c2e81e5db2810d720d80e2340cc79a600b12c3eea3725d20f342edd3e2d03d0e`
   reiniciou as fases 1–2 no primeiro ponto afetado;
2. o substituto sem smartquotes foi medido apenas com vanilla e baseline
   cristalino original congelado;
3. o conteúdo normativo de `compiler/eval/repr.md` foi atualizado primeiro;
4. o L0 passou de SHA-256
   `e931f91a21f76334cd24ad155b6ec79ffdcb9ea2dcbc3555830ef8b9728e5432`
   para `d4a7a25da0016bbb59bb84e0aa841b9597f9dc1e77695cf7923b1ac3dc99d1a5`;
5. só depois foram atualizados probe, manifesto e contrato candidato;
6. `p1290-contract.json` validou como JSON e tem SHA-256
   `f0936c3de1ed8715bec043b01b9d283a9be2ac1fee422fc94b835192dd3f7a50`.

O contrato congela o fragmento observável de arrays/tuplas e `MathOp`, não
equivalência funcional geral. Mede antes de decidir, separa língua de mecânica,
marca inferência e evidencia o que a refutaria, conforme ADR-0108.

## Capacidades e isolamento

As allowlists e denylist completas estão no manifesto. O autor não leu o
conteúdo de `01_core/src/compiler/eval/repr.rs`, patch/candidato atual, testes ou
oráculos candidatos, outros passos de materialização, recibos de implementação
ou veredito. Não escreveu Rust, testes, oráculos, mutações ou veredito. O
contexto herdado foi o pedido explícito das fases 1–2.

O filesystem é compartilhado: as capacidades foram respeitadas e registradas,
mas o ambiente não oferece atestação técnica de isolamento de leitura. A
linguagem correta nesta fase é **execução segregada declarada, ainda não
selada**, não “segregado e atestado”.

## Gate de saída

Não surgiu mudança de contrato Rust público nem outro gatilho de paragem do
ADR-0127. A proveniência de `SmartQuote` já pré-resolvida ficou explicitamente
`Unknown`; este owner só representa o conteúdo recebido. O contrato proíbe
inferência por glifo, mudança de fase e edição de outro owner. Não há
ambiguidade humana dentro do caso substituto medido; os demais `Unknown`
impedem generalização.

O header do consumer não foi ressellado. Não foram executados gate
discriminatório, mutation score, RED→GREEN, implementação, lint final ou
certificado; pertencem às fases seguintes e a autoridades separadas.

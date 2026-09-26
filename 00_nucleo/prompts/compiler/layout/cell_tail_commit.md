# Prompt L0 — commit de caudas diferidas de célula
Hash do Código: 7ea0449b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/cell_tail_commit.rs`
**Vanilla ratificado:** `9f2b6e8715237cb086899a42873660fe744622e8`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## P1504 — medição antes da decisão

No estado P1503 descrito em `00_nucleo/materialization/typst-passo-1504.md`,
`compiler/layout/cursor.rs:1087-1189` implementa
`flush_pending_cell_tails`; `new_page:603-609` chama-o após preparar a
origem da nova região. O método lê a fila `pending_cell_tails`, mede os
items locais, rebaseia align órfão e emite fill, body e stroke nessa ordem.
`compiler/layout.md` possui o buffer e `compiler/layout/slicing.md` possui
o rebase geométrico; nenhum desses owners é a emissão da cauda.

## Decisão e contrato

Este consumer drena apenas as caudas de célula pendentes no topo da região
já aberta: ignora caudas acima do limite de forwardings vigente, mede a
extensão local, mantém z-order fill→items→stroke, atualiza paths de align
órfão no mesmo referencial dos items e avança `cursor_y` até o maior fundo
emitido. O Cursor decide quando iniciar a região e chama o hook; não copia
o buffer nem reinterpreta Grid/Table. O estado continua em `Layouter` sob
`compiler/layout.md`. Não há campo, assinatura pública, fase ou default novo.

A aceitação compara texto, páginas, z-order e geometria de grid/table com
caudas, com/sem fill/stroke, align aninhado e mais de um forwarding. O
controle sem caudas não cria item nem avanço. Bytes PDF e igualdade de
estrutura Rust não são critérios. Refutam a extração: alterar a ordem de
emissão, deslocar fixups para outra fase, duplicar estado ou fazer este
owner decidir a transição de página.

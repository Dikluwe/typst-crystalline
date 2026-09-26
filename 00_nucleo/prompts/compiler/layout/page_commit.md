# Prompt L0 — montagem da página física
Hash do Código: 7ba0eddd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/page_commit.rs`
**Vanilla ratificado:** `9f2b6e8715237cb086899a42873660fe744622e8`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## P1504 — medição antes da decisão

Na working tree P1503 não commitada em 2026-09-25T21:27:48Z, HEAD
`0de5edabed197d23b647cfc68ebefda2f5c2a8d5`,
`compiler/layout/cursor.rs:361-618` contém `new_page`.
`new_page:396-572` calcula dimensões finais, aplica fixups, compõe marginais e
canvas, e constrói `Page`; `:573-618` prepara a região seguinte. Os
consumers `compiler/layout/page_running.md` e
`compiler/layout/page_canvas.md` já possuem, respectivamente, a composição
de conteúdo marginal e dos layers decorativos. A proveniência da medição
estrutural está em `00_nucleo/materialization/typst-passo-1504.md`.

## Decisão e contrato

Este consumer é dono da **montagem da página física que acaba de fechar**:
fixups de items, snapshot de numeração e número lógico provisório, layers
marginais e decorativos, bleed, background/foreground e construção/push de
`Page`. Recebe `Layouter` e dimensões finais da região; não escolhe quando
avançar, não drena floats ou footnotes e não prepara o cursor da região
seguinte. `compiler/layout/cursor.md` conserva `new_page` como coordenador da
transição e o cálculo de dimensões auto. `compiler/layout.md` conserva o
estado, checkpoint e o dispatcher. A realização de conteúdo marginal e de
canvas continua nos seus consumers próprios; esta unidade apenas os compõe
na página. Nenhum estado, assinatura pública, default ou fase novos.

A montagem terminal em `Layouter::finish` permanece no consumer
`compiler/layout.md`; sua eventual unificação exige medição e passo próprios.

O número textual da margem usa o número lógico conhecido da
`known_page_store` quando disponível, com fallback físico, conforme P1486.1
de `compiler/layout.md`; `Page.logical_number` inicia com o número físico
provisório. A ordem causal de fixups, marginais, canvas e push permanece a
do baseline. A aceitação é linguagem: páginas, texto, posição, estilo,
numeração e diagnósticos iguais antes/depois em controles de página auto,
header/footer, bleed, footnotes e colunas. Bytes PDF e estrutura interna de
`Page` não são gates de paridade.

Refutam a fronteira: precisar decidir paginação aqui; duplicar
`page_running`/`page_canvas`; exigir cópia do estado de região; alargar
visibilidade além do módulo de layout; ou mudar o momento em que inserções
e notas entram nos items. A extração é content-preserving, não uma otimização.

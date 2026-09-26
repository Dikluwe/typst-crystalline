# Prompt L0 — transação e replay do sufixo de flow
Hash do Código: c6fc265f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/flow_suffix.rs`
**Vanilla ratificado:** `9f2b6e8715237cb086899a42873660fe744622e8`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## P1504 — medição antes da decisão

Na working tree P1503 descrita em
`00_nucleo/materialization/typst-passo-1504.md`,
`compiler/layout/cursor.rs:361-372` aceita ou rejeita uma transição de
região conforme a transação de sufixo, e `:753-808` executa a unidade
posterior ao marker, valida novas inserções, aceita ou restaura o
checkpoint e repete a ocorrência. `compiler/layout/mod.rs:797-1417`
possui a estrutura e captura/restauração do checkpoint; `place.rs`
possui fitting e reservas. O marker e o drain permanecem associados ao
avanço de região.

## Decisão e contrato

Este consumer é dono da **decisão transacional do sufixo**, não da política
de página: aceita uma unidade estável, rejeita uma unidade cuja inserção
posterior não cabe, restaura atomicamente a cauda pelo checkpoint do
`Layouter`, pede a transição ordinária e repete a mesma ocorrência uma vez
na nova região. Antes de uma transição normal, confirma a transação ativa
que não foi rejeitada; uma tentativa rejeitada não materializa página.
`compiler/layout.md` conserva campos, captura/restauração e dispatcher;
`compiler/layout/cursor.md` conserva `new_page`, marker, drain e
`flush_pending_floats`; `compiler/layout/place.md` conserva fitting.

O owner não conhece a variante do conteúdo futuro, não antecipa sua altura,
não promove floats posteriores ao prefixo e não migra apenas `current_line`.
PREFLOW e float-prefixo confirmados permanecem intocados; sufixo textual,
Place não-float, float posterior, tags e links não são duplicados. Controles
com/sem marker ou float, top/bottom, clearance e sub-layout preservam
páginas, posições e unicidade. Nenhum estado, API pública, default ou fase
novos. Refutam a extração: copiar o checkpoint aqui, mover fitting do
Place, materializar página na tentativa rejeitada ou especializar Block.

# Prompt L0 — motor geral de layout
Hash do Código: cf5d1526

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/mod.rs`
**Vanilla ratificado:** `a51e02804`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## Medição e contrato

O consumer define Layouter/PageConfig, entrypoints de layout, dispatcher
exaustivo e composição final de páginas. Domínios atomizados delegam por
funções estáticas descendentes. Métricas, introspecção e imagens são injetadas;
L1 não importa L3. Cursor, regiões, páginas, fixups e conteúdo diferido
preservam referencial e causalidade.

## Aceitação

Empty, texto, parágrafos, paginação, páginas auto, dispatch e composição final
são cobertos pela suíte de layout. Mudança pública/default/fase para no gate
ADR-0127.

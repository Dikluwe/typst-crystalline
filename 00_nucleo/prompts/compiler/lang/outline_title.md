# Prompt L0 — título localizado de outline
Hash do Código: 6a0bf427

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/lang/outline_title.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

P1034 mediu contra `a51e02804`: `en Contents`, `pt Sumário`, `de
Inhaltsverzeichnis`, `fr Table des matières`, `es Índice`, `it Indice` e `zh
目录`. Língua ausente ou desconhecida usa `Contents`.

## Contrato

`outline_title_for_lang(Option<&Lang>) -> &'static str` faz lookup exato na
tabela medida, retorna `Contents` fora dela e não consulta locale do processo.

## Aceitação

Preservar os sete títulos e os fallbacks `None`/desconhecido. Mudança do texto
gerado ou do default fica sob ADR-0127.

# Prompt L0 — aspas localizadas
Hash do Código: 32112a2f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/lang/quotes.rs`
**ADRs:** ADR-0052, ADR-0060, ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O consumer usa match exato do código `Lang`: aspas primárias específicas para
`pt`, `en`, `de`, `fr`, `es` e `it`; aspas simples específicas para `en`.
Códigos fora das tabelas usam pares curly ingleses determinísticos.

## Contrato

- `localize_quotes(&Lang)` devolve o par primário; francês preserva NBSP.
- `localize_single_quotes(&Lang)` devolve o par simples.
- `DEFAULT_QUOTES` é `“`/`”`; `DEFAULT_SINGLE_QUOTES` é `‘`/`’`.
- O lookup é linear, exato e puro; não interpreta região BCP47.

## Limites e aceitação

Sem smart-apostrophes, alternância aninhada ou inferência por ambiente. Os
pares existentes, NBSP e fallback devem permanecer iguais; mudanças públicas
ficam sob ADR-0127.

# Prompt L0 — suplemento localizado de figure
Hash do Código: 6c8ac553

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/lang/figure_supplement.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O consumer vigente contém 18 combinações de `image`, `table` e `raw` com
`pt`, `en`, `de`, `fr`, `es` e `it`. P1034 mediu no vanilla ratificado que
língua ausente/desconhecida usa os supplements ingleses. Kind desconhecido é
capitalizado pela primeira letra.

## Contrato

`figure_supplement_for_lang(kind, Option<&Lang>) -> String`:

- tenta primeiro match exato de kind e código de língua;
- usa `Figure`, `Table` ou `Listing` como fallback inglês dos kinds conhecidos;
- capitaliza a primeira letra de kind desconhecido de modo UTF-8 aware;
- permanece puro e independente do ambiente.

## Aceitação

Preservar as 18 entradas, fallbacks de língua e capitalização vigentes. Novas
formas ou mudança de default ficam sob o gate ADR-0127.

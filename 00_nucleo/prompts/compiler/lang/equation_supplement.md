# Prompt L0 — suplemento localizado de equação
Hash do Código: 00b0b7be

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/lang/equation_supplement.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

Probes P1140.4-C no vanilla ratificado `a51e02804` mediram: `en Equation`,
`pt Equação`, `de Gleichung`, `fr Équation`, `es Ecuación`, `it Equazione`.
Ausência ou língua fora da tabela produz `Equation`.

## Contrato

`equation_supplement_for_lang(Option<&Lang>) -> &'static str` faz lookup exato
na tabela medida e usa inglês para `None` ou código desconhecido. O helper é
puro e não conhece introspecção, eval ou layout.

## Aceitação

As seis entradas medidas e os dois caminhos de fallback preservam exatamente
os textos vigentes. Alterações dessa superfície pública exigem ADR-0127.

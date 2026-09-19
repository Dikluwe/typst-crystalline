# Prompt L0 — `labelled`
Hash do Código: 5fb0db7f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/introspect/labelled.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Computar a associação de Labelled ao alvo e seu texto resolvível sem alterar a ordem do walk ou fabricar identidade.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.

## P1459 — heading resolvido com língua causal

### Medição anterior à decisão

O braço Heading continha `lang == pt ? Seção : Section`. Em alvo alemão, o
texto resolvido ficava inglês, embora o vanilla ratificado use `Abschnitt`.
O parâmetro `lang` é capturado da `StyleChain` causal no braço `Label` durante
o walk do alvo. O parâmetro legacy do walk só atua quando essa chain não
declara idioma.

### Decisão

O braço Heading chama exclusivamente
`compiler::lang::heading_supplement_for_lang(lang)`. Conserva o counter
hierárquico e a representação interna legacy `{supplement} {número}` usada
por `resolved_labels`; essa string é transporte causal, não autorização para
um consumer reconstruir localização. Não conter tabela, branch por língua ou
literais de supplement. Equation e Figure ficam fora deste recorte.

O carrier de `#show heading: set text(lang:)` só existe na árvore renderizada,
não na árvore pré-show introspectada por P498. Alterar essa fronteira é mudança
de fase e fica explicitamente fora deste passo, sujeita a L0 próprio e
confirmação ADR-0127.

### Aceitação

Alvos `de`, `es`, `fr` e `it` produzem o prefixo do owner; `fr` produz
`Section`. A referência explícita continua a sobrepor esse transporte.

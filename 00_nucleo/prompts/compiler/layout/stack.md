# Prompt L0 — `stack`
Hash do Código: 5f7c616a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/stack.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Compor filhos de Stack por direção, spacing e alinhamento, medindo ascents/descents e preservando baseline e margens de bloco.

O fallback vertical `1.2em` é a altura de linha sintética usada quando a
medição de filhos não oferece extensão; é aproximação cristalina explícita,
não constante do vanilla. A fórmula fica documentada como rationale até que
o owner passe a derivá-la integralmente de `text_edges`/leading medidos.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.

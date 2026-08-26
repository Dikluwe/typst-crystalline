# Prompt L0 — `boxed`
Hash do Código: 6c94d33d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/boxed.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Compor Boxed inline, isolando extensão do body, largura/altura explícitas, overflow, clip, radius e coordenadas locais do Group.

### Proveniência das calibrações P1119/P1120

Os fatores `28.81670pt` e `7.633997pt` na base de 11pt não são defaults
normativos do vanilla nem métricas copiadas de sua implementação: são
calibrações empíricas do documento canônico registradas em
`diagnosticos/typst-passo-1119-relatorio.md` e reavaliadas em
`diagnosticos/typst-passo-1120-relatorio.md`. A divisão por `11.0` apenas
normaliza a medição para `em`, preservando escala com o tamanho da fonte.
Substituí-las exige nova medição da morfologia da linha; não inferir métrica
tipográfica inexistente apenas para remover um literal.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.

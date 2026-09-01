# Prompt L0 — `smartquote`
Hash do Código: 58673851

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/smartquote.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Escolher aspas inteligentes de abertura/fecho por idioma e estado local, avançando o texto sem perder alternância.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.

## P1286 — consumo da configuração local

### Medição anterior à decisão

`SmartQuoteElem` chega hoje com apenas `double`; a `StyleChain` vigente já
acompanha o leaf durante `Content::Styled`. O vanilla resolve
quotes/língua/alternative conjuntamente em `text/smartquote.rs:201-317`. A
auditoria P1286 mostrou que embrulhar uma chamada direta em `Content::Styled`
não é morfologicamente transparente para `content.func()`.

### Decisão

A free function continua dona da alternância. Para cada leaf, resolve
`alternative` e `quotes` pela precedência campo explícito do
`SmartQuoteElem` > StyleChain > default, pede ao owner
`compiler/lang/quotes` os pares single/double e escolhe open/close segundo
`e.double`. `quotes: auto` explícito apaga quotes herdadas; configuração
explícita sobrepõe `alternative` somente no membro configurado. Nenhum estado
global ou despacho dinâmico é criado; a forma B e o pin do Núcleo permanecem
inalterados. Os campos públicos novos ficam bloqueados pelo gate ADR-0127 do
owner `entities/elements/smartquote.md`.

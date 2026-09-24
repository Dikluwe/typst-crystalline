# Prompt L0 — `stack`
Hash do Código: dcde18e7

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
## P1493 — composição de stack neutra ao carrier HTML

### Medição anterior à decisão

`stack.rs:25-87,255-267` inclui posição e fundo de cada item na largura e no
descent do filho. O carrier zero-size podia assim alterar a composição por sua
posição, apesar de não possuir tinta.

### Contrato ratificado

Stack exclui somente o carrier privado P1493 de ascent, descent, largura e
avanço, mas continua a aplicar ao item a mesma translação do filho e a devolvê-lo
para extração posterior. Os demais itens mantêm a semântica existente.

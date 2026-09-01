# Prompt L0 — `block`
Hash do Código: e857945c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/block.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Compor Block com largura, altura, inset, outset, fill, stroke, radius, clipping, breakability, sticky e margens; início lógico respeita direção ativa.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.

## P1292 amendment-11 — transição Block → flow ordinário

### Medição anterior à decisão

Em página 100pt × 100pt, o controle sem `place.flush` e o controle sem qualquer
float são bilaterais entre si dentro de cada renderer. Depois de
`block(height: 30pt)`, o candidato posicionou `AFTER_FLOW` em `yMin=27.404pt`,
enquanto o vanilla ratificado o posicionou em `40.604pt`. A diferença exata é
`13.2pt`, isto é, o default relativo `1.2em` a 11pt.

No upstream, `layout/container.rs:347-360` define `block.spacing` como
`1.2em` e o projeta em `above`/`below`; `flow/collect.rs:232-278` resolve esses
espaçamentos e insere o `below` depois do frame. O float e Flush não participam
dessa fórmula. Logo a divergência é uma obrigação ordinária de Block já
existente, não uma compensação transacional de Cursor.

### Decisão owner-correct

Ao materializar um Block no flow vertical, este owner conserva a transição
após o frame: o fim físico do Block e o espaçamento `below` efetivo tornam-se a
origem da próxima unidade ordinária. O default continua relativo ao estilo
efetivo; valores explícitos de `above`, `below` ou `spacing` permanecem
soberanos e o colapso/precedência de espaçamento não é substituído por soma
cega.

No modelo corrente de estado do Layouter, a função de Block deve encerrar a
cadeia anterior sem apagar a origem que a próxima linha precisa: registrar o
fim/baseline lógico do frame e transportar o gap resolvido pela via ordinária
de flow. Ela não pode delegar esse reparo a um deslocamento global de
`current_items` em Cursor, nem condicionar o gap a Flush, float, replay ou
paginação.

Descendentes out-of-flow produzidos dentro do Block são rebased junto com a
origem do frame do Block; não escapam para o topo da página ou sub-frame. A
fórmula específica de Place não-float permanece no owner
`compiler/layout/place.md`; este owner fornece a fronteira e o referencial do
container, sem absorver alinhamento/offset de Place.

### Aceitação e refutadores

Com fonte efetiva de 11pt, Block 30pt seguido de Block textual mantém
`AFTER_FLOW yMin=40.604±0.002pt`; omitir float ou omitir Flush não altera esse
valor. Variar `block.spacing` deve variar a transição pela unidade declarada,
sem constante `13.2pt` assada.

Refutam: zerar a cadeia e iniciar a próxima linha em 30pt sem gap; somar
`1.2em` em Cursor; aplicar o gap somente durante replay; duplicar
above/below; mover Place/Flush para este arquivo; ou alterar API pública,
default, fase ou compatibilidade.

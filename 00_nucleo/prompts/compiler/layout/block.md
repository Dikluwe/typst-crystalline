# Prompt L0 — `block`
Hash do Código: f299b643

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

## P1373 — resolução de altura relativa

### Medição anterior à decisão

O P1372 isolou o comportamento com texto `ProbeAfter` idêntico. Nos perfis
`a11y-extras` e `html+a11y`, vanila posicionou `After` na página 1 em
`yMin=7,404`; cristalino posicionou na página 0 em `yMin=42,230`. Lone e
controle tiveram delta interno zero nos dois produtos. Na fonte cristalina,
`native_block` perde `Ratio` antes do layout; na fonte ratificada,
`flow/block.rs:323-331` resolve a altura relativa contra a base da região.

### Decisão owner-correct

Este owner recebe `Option<Rel<Length>>` já preservado. Quando presente,
captura a altura útil da região exterior antes de alterar
`regions.current.height` e resolve exatamente uma vez:

```text
resolved_height = height.resolve(Length::pt(outer_available_height))
                        .resolve_pt(effective_font_size)
```

O resultado finito e não negativo é usado coerentemente no fitting de
`breakable:false`, no limite temporário dos descendentes, em `parent_bbox`, no
avanço mínimo e na altura da shape. `100%` refere-se à altura útil completa da
região (altura menos margens), não à altura restante no cursor, ao font-size
ou ao tamanho natural do body. A região exterior é restaurada após o body.

Não resolver em eval, não usar `page_config.height` sem descontar margens, não
resolver novamente depois que a região local foi reduzida e não condicionar o
cálculo a metadata ou ao número de filhos.

### Aceitação e escopo

Em página 240pt com margens de 10pt, `height:100%` resolve para 220pt e empurra
o marcador posterior para a mesma página/posição observada no vanila P1372.
Altura absoluta mantém os testes existentes. P1374 completa width relativa e
altura fracionária; esses eixos não recebem crédito em P1373.

## P1374 — consumo proposto de width relativa e height fracionária

### Medição anterior à decisão

Na sonda width 60%, os 180pt úteis implicam 108pt; o vanila alinhou `Right`
com `xMin=93,668`, enquanto o cristalino truncou a região e cortou o texto. Na
sonda 1fr/2fr, o vanila preservou razão proporcional e o cristalino rejeitou a
entrada. A fonte ratificada delega a parcela fracionária ao distribuidor de
flow, não ao container isolado.

### Decisão proposta

Resolver width `Rel<Length>` uma única vez contra a largura útil exterior e
usar o mesmo valor para região dos descendentes, bbox, RTL e shape. Height
`Relative` mantém integralmente P1373. Height `Fractional` só é disposto quando
o owner de flow fornece uma parcela concreta; o Block não calcula espaço
restante nem soma pesos. Ausência dessa parcela em contexto fora da fatia
P1374 fecha em `Unknown`/sem crédito, não em altura zero silenciosa.

Não mudar inset/outset, spacing, breakability, clip ou parent bbox além de
substituir seus usos de width/height pelo valor resolvido único. Gate público
ADR-0127 antes do código.

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

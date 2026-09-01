# Prompt L0 — `place`
Hash do Código: 91f09481

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/place.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Materializar Place inline ou float com escopo, alinhamento, offsets, clearance e emissão diferida, preservando rebase e decoração.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.

## P1292 amendment-9 — inserção float e recomposição da região

### Medição anterior à decisão

Em páginas 100pt × 100pt, o vanilla ratificado posiciona um bottom float de
80pt na página 2, reserva o seu `clearance` relativo e só então deixa o flow
posterior competir pela região restante. Com o default `1.5em`, o flow
posterior de 10pt inicia na página 3; com `clearance: 0pt`, ele cabe na página
2. Um caso boundary com `clearance: 5pt` conserva duas páginas nos dois
renderers, mas o candidato refutado colocou o token do float em `y=65.166`
contra `y=90.166` no vanilla. Portanto contagem isolada não prova placement.

No upstream ratificado, `flow/collect.rs:281-331` resolve a ocorrência de
Place sem a emitir; `flow/compose.rs:298-377` calcula fitting e pede relayout
quando uma inserção passa a ocupar a região; `:737-785` aplica reservas e
ancora top/bottom na composição final. Relayout, checkpoints e structs são
mecânica; a obrigação de linguagem é recompor o flow contra a região efetiva
já reservada pelo float.

### Decisão owner-correct

Para `float: true`, este owner produz e realiza uma inserção do distribuidor
ativo, nunca um frame empurrado diretamente no cursor corrente. A ocorrência
conserva identidade, ordem, alinhamento, offsets, scope, frame medido e o
`clearance` resolvido contra o estilo efetivo. O default relativo chega como
`1.5em` do owner `compiler/stdlib/layout.md`; não é convertido em pt na eval.

O fitting usa a dimensão da região efetiva. Quando já existe flow na região,
a necessidade inclui `frame.height + clearance`; numa região vazia, o
clearance não impede que o próprio float seja admitido. Uma vez admitido,
porém, a reserva top/bottom inclui o clearance adjacente ao futuro flow. Um
float que não cabe e pode progredir permanece pendente, sem descarte,
reordenação ou sobreposição forçada.

Adicionar ou alterar uma inserção invalida a composição especulativa da
região corrente e devolve ao owner `compiler/layout/cursor.md` um pedido
interno de recomposição estabilizada. Na nova composição, top floats são
ancorados no topo e acumulados para baixo; bottom floats são ancorados no
fundo e acumulados para cima; clearance separa a área reservada do flow, não
desloca o frame para fora da região. O mesmo mecanismo atende Place comum e o
prefixo liberado por `place.flush`; `flush.rs` não contém fitting, reserva ou
posicionamento.

A recomposição é idempotente por identidade de ocorrência: uma inserção já
aceita não é duplicada ao rever o flow. Floats posteriores à fronteira do
marker continuam fora do prefixo. Em sub-layout, somente inserções e região do
Layouter ativo participam.

Refutam este contrato: emitir no cursor sem reserva; aplicar clearance duas
vezes ou ignorá-lo no flow posterior; resolver `em` na eval; posicionar bottom
relativamente ao cursor usado em vez do fundo da região; duplicar ocorrência
em replay; ou fazer este owner avançar página/coluna por conta própria.

### Aceitação focal

Omitido equivale a `1.5em` no estilo efetivo, inclusive 15pt a 10pt e 30pt a
20pt; explícitos `0pt`, `5pt` e `1.5em` permanecem distintos e soberanos.
Prefix/suffix, top+bottom e boundary preservam páginas, ordem e posições
ratificadas; no-floats e nested não ganham inserções ou vazamento.

## P1292 amendment-11 — Place não-float segue a origem corrente do flow

### Medição anterior à decisão

No controle sem Flush e no controle sem float, o candidato posicionou
`AFTER_MARKER` em `yMin=-9.834pt`, no topo do sub-layout, embora a mesma
unidade já contivesse `AFTER_FLOW`. O vanilla ratificado produziu
`AFTER_FLOW yMin=40.604pt` e `AFTER_MARKER yMin=47.842pt` nos dois controles.

No upstream, Place não-float vira um item distinto em
`flow/distribute.rs:504-510`; na composição final,
`flow/distribute.rs:630-662` avança `offset` pela altura do frame in-flow e
posiciona o Place sem alinhamento vertical em `offset + ruler.position(free)`.
Assim, a semântica observável é relativa à origem corrente da unidade de flow,
não ao topo físico da página. A estrutura `Item` é mecânica informativa, não
obrigação de cópia.

### Decisão owner-correct

Para `float: false` sem alinhamento vertical explícito, este owner ancora o
frame de Place na origem ordinária já alcançada pelo flow. Se existe linha
pendente na mesma unidade, a origem é a baseline corrente combinada com a
aresta superior semântica dessa linha; o Place aparece depois do conteúdo
in-flow já composto, preservando `dx`/`dy`, alinhamento horizontal e rebase do
container. Não se ancora no topo por a linha ainda não ter sido commitada.

O owner `compiler/layout/block.md` fornece fim/gap/referencial do Block; Place
consome esse referencial para sua própria coordenada. Cursor pode transportar
estado e executar rollback, mas não corrige esta fórmula varrendo ou
traduzindo genericamente itens recém-criados.

### Aceitação e refutadores

No controle focal, `AFTER_MARKER` fica em `47.842±0.002pt`, exatamente uma
vez, tanto sem Flush quanto sem float. No vetor transacional principal após
progresso para p3, preserva `4.642±0.002pt`; floats prefixo/sufixo conservam
seus anchors já selados.

Refutam: anchor no topo; constante do token/fixture; translação global em
Cursor; depender de marker/replay para o caminho ordinário; transformar
Place não-float em flow que consome altura; ou alterar o contrato público de
Place.

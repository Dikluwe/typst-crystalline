# Prompt L0 — layout da ilha `html.frame`
Hash do Código: f0ebc1b7

**Estado:** APROVADO NO GATE ADR-0127 P1424 EM 2026-09-16  
**Camada:** L1  
**Ficheiro alvo exclusivo:** `01_core/src/compiler/layout/html_frame.rs`  
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0128, ADR-0129

## Medição anterior à decisão

`typst-html/src/rules.rs:88-91` torna `FrameElem` transparente no target
paginado. `typst-html/src/convert.rs:140-154` relayouta seu body sob target
`Paged`, região ilimitada, e entrega um frame intrínseco ao encoder HTML.
Sondas P1424 mediram `rect(width: 10pt, height: 5pt)` com `viewBox="0 0 10 5"`
e nesting de frame byte-idêntico ao frame simples.

## Obrigação

Esta unidade contém somente a lógica de layout da variante
`Content::HtmlFrame`, em forma B:

- no dispatcher paginado, `layout(layouter, elem)` delega uma vez ao body e
  não emite wrapper, margem ou item próprio;
- a entry point intrínseca cria Layouter isolado com chain, métricas e image
  sizer injetados, executa o body numa região de largura/altura ilimitadas e
  devolve `Page` transparente recortada às dimensões reais;
- nesting colapsa naturalmente porque o braço paginado de um frame interno é
  transparente;
- posições são normalizadas ao referencial local sem cortar tinta suportada;
- callbacks/erros usam os canais existentes e nunca executam Func em layout.

O dispatcher central conserva somente o braço magro e o entry point público de
fachada. Não importar L3, não criar renderer SVG, não paginar em A4 e não usar
`Content::HtmlElem` ou tag sentinela.

## Aceitação

Texto simples produz dimensões intrínsecas positivas; rect 10pt×5pt produz
page 10×5 transparente; frame aninhado é geometricamente idêntico; no layout
paginado ordinário o body ocupa exatamente a posição que teria sem wrapper.
IDs/bytes SVG não pertencem a esta unidade.
## P1486 — número da página intrínseca

### Medição anterior à decisão

`01_core/src/compiler/layout/html_frame.rs:74-86` constrói uma
`Page` isolada para o encoder HTML, fora da sequência paginada
global; `entities/layout_types.md` P1486 exige número lógico
explícito em toda `Page`. A ilha não participa em `/PageLabels`.

### Decisão

A única página intrínseca recebe `logical_number: 1`.
Nenhum contador global, callback ou rótulo PDF é criado.

## P1493 — posições causais da ilha intrínseca (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

O layout intrínseco usa um Layouter isolado, mas `runtime.positions` grava o
ponto local antes de sublayouts terminarem. `place` translada somente `FrameItem`s
ao regressar ao pai; portanto a posição bruta do marker fica na origem e não
prova os 30pt/20pt observados. A hipótese inicial de extrair o runtime foi
refutada pela descida `layout_sub_frame` -> `placement`.

Sonda adicional no vanilla pinado, em 2026-09-24T16:41:36-03:00, manteve
`position() == (x: 30pt, y: 20pt)` quando o próprio contexto colocado também
emitiu texto; diretamente na ilha, o ponto permaneceu 0pt/0pt. Isso refuta
aplicar ao `HtmlPosition` o recorte vertical final usado pela Page cristalina.
A medição partiu do HEAD `b5abe9b288ce168c494a4c5172830e325b9aedae` com
working tree não commitado enumerado no diagnóstico P1493.

### Contrato ratificado

Somente a variante detalhada substitui cada `ContextBlock` por um carrier
privado zero-size identificável pelo id, usando a árvore existente de
`FrameItem` sem nova variante pública. O carrier atravessa `place`, box, grid,
grupos, links e transformações como item geométrico. Após essas composições, o
owner extrai em ordem determinística os pares
`ContextBlock id -> HtmlPosition` no referencial causal da ilha, **antes** do
recorte vertical privado da Page. Em seguida calcula o topo ignorando carriers,
normaliza somente a Page visual e remove integralmente os carriers antes de
medir largura ou devolver a Page.

Frames aninhados continuam uma só ilha: o resultado do frame externo contém as
posições dos markers internos no referencial do externo. Não expor número
lógico, `Position` paginada, PageStore, SVG ou World. Falta de correspondência
id/Location não fabrica ponto; L3 a trata como incapacidade. A superfície
detalhada foi confirmada pelo dono em 2026-09-24.

### P1493 — classificação carrier-only

Somente o sentinel direto é imaterial. `Semantic`, `Link` e `Group`
preexistentes conservam a materialidade da fachada legada mesmo quando seu
único filho é o carrier; classificá-los recursivamente como vazios faria o
passe detalhado divergir do legado. A composição não remove nem achata
wrappers, e translações reais continuam a alcançar o ponto causal.

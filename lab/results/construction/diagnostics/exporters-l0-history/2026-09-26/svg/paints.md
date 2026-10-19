---
Hash do Código: 836c88f5
prompt: infra/export/svg/paints
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/export/svg/paints

## Medição anterior à decisão

Fonte anterior `03_infra/src/export/svg.rs:239-706`, HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, capturada em `2026-09-26T09:29:07-03:00`. Proveniência exata: `00_nucleo/diagnosticos/svg-atomization.md`. O texto histórico conserva medições e gates anteriores; não é nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo `03_infra/src/export/svg/paints.rs`. Possuir PaintDefs, SvgPaintDef, PaintReference, predicate de suporte, reasons de fallback, deduplicação e writers de linear/radial/conic/tiling e stops. Manter exatamente as promoções vigentes, budgets, offsets, alpha, IDs locais, caixas de paint, sampler svg_adaptive_stops e Ratio::repr. Não mover algoritmos do owner de gradients nem promover Unknown nesta extração.

Atomização estrutural reversível em L3: fluxo contínuo ADR-0127, sem mudança de contrato público, comportamento por defeito, fase ou compatibilidade. Referências ao monólito no texto histórico abaixo indicam a fonte anterior; a implementação específica passa a pertencer somente a este consumer. Não ampliar alegações de paridade nem interpretar mecanicamente novos IDs/bytes como paridade de linguagem.

### P1227 — paint servers morfológicos

Medição: `03_infra/src/export/svg.rs:459` converte fill somente por
`color_to_css`, e `write_stroke_attrs` chama `stroke.paint.to_color()`; portanto
linear, radial e tiling são hoje achatados numa cor. Depois da aprovação do
contrato `FrameItem::Shape.fill: Option<Paint>`, o exporter deve:

- recolher paints de fill e stroke, atribuir IDs locais determinísticos e emitir
  `<defs>`; igualdade morfológica pode reutilizar uma definição, sem fazer da
  identidade/ortografia do ID um observável;
- emitir Gradient Linear como `<linearGradient>` e Radial como
  `<radialGradient>`, com `gradientUnits="objectBoundingBox"`, coordenadas
  derivadas dos campos da entidade, focal radial independente e stops na ordem
  efetiva, sem ordenar, deduplicar ou colapsar stops coincidentes;
- preservar alpha como `stop-opacity`, separado da cor, e aplicar o servidor
  ao papel correto por `fill="url(#...)"` ou `stroke="url(#...)"`;
- emitir Tiling modelável como `<pattern>`, preservando tamanho, spacing,
  relative e corpo suportado. Corpo opaco/imagem sem geometria/tamanho
  resolvível não deve ser falsamente declarado equivalente;
- manter Gradient Conic fora da alegação morfológica SVG nativa: sem modelo de
  erro geométrico/cromático validado, não aproximar nem chamar pattern equivalente;
- não converter espaços de cor não modelados para obter igualdade aparente.

Quando um paint permanece fora da alegação (`Conic`, espaço de interpolação
não modelado ou tiling opaco), o exportador conserva o fallback de primeira cor
pré-P1227 para não apagar a geometria e anota a operação com
`data-crystalline-fill-fallback` ou `data-crystalline-stroke-fallback`. Essa
anotação é diagnóstica: a lente deve classificar o paint como `Unknown`, nunca
como servidor morfologicamente preservado.

Os testes públicos do owner devem cobrir linear em fill, radial em fill com
focal point, gradient em stroke, reutilização determinística e tiling de cor.
Conic e conversões de espaço não modeladas permanecem explicitamente fora da
alegação, nunca reduzidas silenciosamente a solid.


### P1229 — Linear/Radial multi-space por stops adaptativos

Retificação P1235: o contrato expandido não fechou os 30 controles e P1234
não demonstrou causa mecânica. Por isso Linear/Oklab, Linear/LinearRgb e
Radial/Hsv regressam a `Unknown` com fallback explícito
`gradient-color-space`; nenhuma das nove candidatas P1231 é promovida. Somente
os controles Linear/Radial sRGB reutilizam a geometria nativa dos servidores
P1227 neste contrato. Para cada par de
stops originais, o exportador preserva a primeira extremidade e insere os stops
produzidos pelo owner `export/gradients/adaptive.rs`; a última extremidade é
emitida uma vez ao final.

Intervalos coincidentes não são subdivididos, deduplicados ou reordenados. Cada
cor emitida conserva alpha, serializado separadamente como `stop-opacity`.
Uma combinação Linear/Radial deixa de receber `gradient-color-space` somente
quando satisfaz um envelope validado vigente. As três promoções invalidadas,
CMYK Linear/Radial e tiling opaco continuam fallback explicitamente `Unknown`.
CMYK não pode ser promovido enquanto a
conversão ICC registrada no L0 de cor permanecer fora de escopo. Conic segue o
contrato vetorial P1230.

IDs, bytes e contagem exata de stops não são paridade. São obrigatórios:
referência local resolvida, variante e geometria corretas, papel fill/stroke,
offsets originais, ordem, descontinuidades, alpha e erro dentro do orçamento.

#### P1261 — representação localizada do último intervalo adaptativo

Medição que precede a decisão: `export/gradients/adaptive.rs:84-147` produz,
para os quatro witnesses `two-wide-stroke` Linear/Radial × Oklab/LinearRgb,
o mesmo número de stops, as mesmas cores do penúltimo stop e o mesmo endpoint
final que o vanilla ratificado. O penúltimo offset pré-serialização é
`0.984375` e o endpoint `1.0`. A primeira divergência aparece somente em
`export/svg.rs:542-583`: `write_gradient_stops` passa o offset por `fmt_num` e
emite `0.984375`, enquanto
`lab/typst-original/crates/typst-svg/src/paint.rs:251-281` passa cada offset
intermediário por `Ratio::repr`; `typst-library/src/layout/ratio.rs:135-138`
usa precisão de duas casas percentuais e emite `98.44%` (numericamente
`0.9844`). A simulação congelada P1261 manteve os outros 20 fixtures
byte-idênticos e reduziu os quatro máximos de `0.009071938982` para
`0.008998699509` em Oklab e de `0.020785701027` para `0.020706225661` em
LinearRgb, dentro dos envelopes P1237.

Decisão: somente o último intervalo dos stops adaptativos Linear/Radial usa a
representação percentual de duas casas do `Ratio::repr` ratificado: o último
stop intermediário e o endpoint terminal. Os stops nativos, `fmt_num`, os
intervalos adaptativos anteriores, as cores, alpha, cap 64 e decisão de
subdivisão permanecem inalterados. O endpoint terminal continua emitido uma
única vez, na ordem original e com cor exata. Linear e Radial compartilham o
mesmo helper de escrita; fill/stroke não participam da decisão.

O teste focal `p1261_endpoint_near_serializa_penultimo_offset_com_repr_vanilla`
congela os quatro casos, o penúltimo stop pré-serialização, a representação
serializada e a unicidade/cor do endpoint. A revalidação numérica usa os
quatro `worst_t` P1260 e exige os envelopes P1237 sem regressão dos demais
máximos, p95, alpha ou quantidade/cap de stops.

#### P1262 — `Ratio::repr` para todo offset adaptativo

Medição que precede a decisão: depois de igualar no owner adaptativo os
clamps e a precisão Oklab do vanilla, os oito stopsets interiores saturados
Linear/Radial passaram a ter as mesmas contagens e cores do baseline. A
fronteira restante era SVG: `typst-svg/src/paint.rs:251-281` aplica
`Ratio::repr` a **cada** offset intermediário, enquanto a regra localizada do
P1261 o aplicava apenas aos dois últimos stops. Nos witnesses interiores, essa
diferença de posição reparsada ainda excedia os budgets congelados, embora o
sample público, a quantização de cor e alpha coincidissem.

Decisão: todo stop Oklab produzido por `svg_adaptive_stops` em Linear/Radial
serializa o offset com a representação percentual de duas casas do
`Ratio::repr` ratificado. A ordem aritmética é parte da fronteira medida:
primeiro `offset * 100` e depois o arredondamento dessa percentagem a duas
casas, pela regra standard de afastamento de zero do vanilla. Assim, os
valores binários efetivos reproduzem simultaneamente `13.87%`, `32.38%`,
`60.62%` e `76.38%`; não se pode colapsar as duas multiplicações numa só nem
converter previamente o offset a `f32`. Esta regra generaliza e substitui a localidade do
P1261 somente para Oklab. LinearRgb conserva a regra localizada do último
intervalo P1261: a varredura de 24 fixtures mostrou que generalizá-la neste
passo reabriria os dois `alpha-first` já preservados, que pertencem ao cluster
seguinte. Stops nativos sRGB continuam no writer histórico `fmt_num`. Cores,
`stop-opacity`, ordem, descontinuidades, endpoint único, cap 64 e decisões de
subdivisão não mudam. A aceitação é o envelope de linguagem P1231/P1237 e
não igualdade textual nem contagem, ainda que estas possam coincidir.

#### P1263 — `Ratio::repr` completo também para LinearRgb adaptativo

Medição: a reconstrução com a ordem aritmética exata consolidada no P1262
refuta a limitação provisória então registada para LinearRgb. Nos gradientes
Linear e Radial, serializar por `Ratio::repr` todos os offsets adaptativos
LinearRgb fecha os quatro alvos interiores não saturados de cor e os seis de
alpha, preserva os controlos `alpha-first` e mantém os quatro pares elegíveis
com `6/6` no decalque P1237. Os gradientes `alpha-mid` e `alpha-last` continuam
com, respetivamente, 36 e 40 stops: não existe deficit causal de refinamento.

Decisão: todo offset produzido pela amostragem adaptativa de gradientes Linear
ou Radial em Oklab ou LinearRgb é serializado diretamente a partir do `f64`
normalizado por `Ratio::repr`, sem conversão intermédia para `f32`. Stops nativos
sRGB continuam no writer histórico `fmt_num`. A mudança não acrescenta stops nem
decisões adaptativas e não altera limiar, cap, cores, alpha, ordem, coincidência
ou continuidade à direita; `adaptive.rs`, o PDF P274 e `Color` permanecem fora
do âmbito.

Aceitação: os dez alvos P1260 devem fechar simultaneamente `max` e `p95`, com
cor premultiplicada e alpha observados separadamente em ponto flutuante antes de
qualquer quantização `u8`; as 24 fixtures P1237 não podem regredir; execução
direta e inversa deve ser determinística; o custo incremental deve permanecer
em zero stops e zero decisões; nenhum target é promovido por esta correção.

#### P1278 — `Ratio::repr` completo nos espaços polares

Medição antes da decisão: P1277 preservou a fronteira P1261 para
Oklch/Hsl/Hsv e serializou offsets adaptativos anteriores ao último intervalo
como `f32` decimal. A fonte ratificada `typst-svg/src/paint.rs:246-277` aplica
`Ratio::repr` a todo stop original e intermediário independentemente do espaço.
O contrafactual P1278 sobre os budgets validados melhorou Hsl de 14/24 para
19/24 e Hsv de 13/24 para 18/24 por geometria, sem tocar em cor, alpha,
subdivisão ou budget. Oklch isoladamente não fechou, demonstrando que a
serialização é necessária, mas não suficiente sem o sampler preciso do owner
adaptativo.

Decisão: todo offset adaptativo Linear/Radial em Oklch, Hsl ou Hsv usa a mesma
representação percentual de duas casas já consolidada para Oklab/LinearRgb,
diretamente a partir do `f64`. Stops nativos sRGB conservam `fmt_num`. Luma e
CMYK ficam fora deste passo. A mudança não altera cap, limiar, quantidade ou
decisão de stops, geometria, alpha, ordem ou descontinuidades e não amplia
`paint_is_svg_native`: os seis pares continuam fallback até gate ADR-0127
posterior e explícito.

Aceitação: os 144 casos polares P1277 são reexecutados contra os mesmos
oráculos; bytes, IDs e contagem de stops não são gate, enquanto max/p95 de cor
premultiplicada e alpha antes de `u8`, raster, grafo e determinismo continuam
obrigatórios. Qualquer falha mantém o par `Unknown`.

#### P1280 — promoção dos seis pares polares validados

**Gate ADR-0127: L0 confirmado explicitamente pelo dono em 2026-08-29.** A
mudança é comportamento por defeito: substitui fallback sólido por servidor
SVG adaptativo somente em Linear/Radial × Oklch/Hsl/Hsv.

Medição anterior à proposta: a campanha P1279 foi registrada no commit
`ad93af9e80213fc410ffeda1f4916b031b088474`.
Sobre a população congelada P1276/P1277, cada um dos seis pares fechou grafo,
envelope numérico, raster e custo em 24/24, totalizando 144/144; os 42/42
inválidos polares foram rejeitados. O transporte lossless do harness eliminou
os 10 falsos negativos de P1278 sem alterar código produtivo, budget, limiar ou
cap. O predicate atual ainda admite Linear/Radial apenas em sRGB, Oklab e
LinearRgb, portanto os 144 casos polares preservam fallback e nenhuma promoção
foi aplicada.

Decisão aprovada: `paint_is_svg_native` admite
Oklch, Hsl e Hsv individualmente para Gradient Linear e Radial. Esses seis
pares reutilizam sem alteração a geometria existente e os stops de
`svg_adaptive_stops`, incluindo o sampler preciso P1278 e a serialização
`Ratio::repr` completa. sRGB, Oklab e LinearRgb conservam as rotas vigentes.
Luma e CMYK permanecem `Unknown` com fallback explícito
`gradient-color-space`; Conic, Tiling, algoritmo PDF, limiar `0.001`, cap 64,
budgets, ordem, descontinuidades, alpha, IDs e política de `Unknown` não mudam.

Esta decisão substitui somente a proibição P1273/P1278 de
promover Oklch/Hsl/Hsv em Linear/Radial. A proibição de promover Luma, CMYK ou
pares não validados permanece normativa. A medição cobre apenas este
fragmento e não prova equivalência SVG geral.

Os testes do owner exigem, para os seis pares, referência local resolvida,
variante Linear/Radial correta, ausência do marcador de fallback,
stops/offsets/ordem/descontinuidades/alpha e papéis fill/stroke preservados.
Testes negativos
exigem que Luma e CMYK continuem fallback e que nenhum wildcard promova Conic
ou Tiling. A aceitação cobre subpromoção, sobrepromoção, referência pendente,
solid disfarçado, bypass do sampler e mudança de budget; bytes e IDs não são
critério de paridade.

#### P1273 — promoção dos quatro pares validados

**Gate ADR-0127:** proposta L0 confirmada explicitamente pelo dono em
2026-08-29. A mudança é deliberadamente classificada como comportamento por
defeito: substitui fallback sólido por servidor SVG nativo somente nos quatro
pares nomeados abaixo.

Medição anterior à proposta, sobre o commit
`9f0bed3633f3245cf27609751a3743968d5956b0`: o owner continua biunívoco com
`03_infra/src/export/svg.rs`; `paint_is_svg_native` admite Linear/Radial apenas
em sRGB, enquanto `write_svg_gradient_stops` já possui a rota adaptativa para
Oklab e LinearRgb. Consequentemente, os quatro pares abaixo ainda recebem
`gradient-color-space` antes de alcançar o writer adaptativo.

A campanha P1272 classificou individualmente como
`Generalization-Preserved`, 24/24 cada:

- Linear/Oklab;
- Radial/Oklab;
- Linear/LinearRgb;
- Radial/LinearRgb.

Decisão aprovada: esses quatro pares deixam o fallback `gradient-color-space`
e usam o servidor SVG Linear/Radial já existente com os stops de
`svg_adaptive_stops`. sRGB conserva a rota nativa histórica. Não se altera
geometria, orçamento, cap 64, fórmula de subdivisão, offsets, cores, alpha,
ordem, multiplicidade, descontinuidades, reutilização de definição nem papel
fill/stroke.

Hsv, Oklch, Hsl, Luma e CMYK continuam `Unknown` para Linear e Radial e mantêm
o fallback explícito `gradient-color-space`. Qualquer espaço ou par não
validado também permanece `Unknown`; opacidade nunca é convertida em
`Preserved` por default. Conic e Tiling conservam os seus contratos e reasons
próprios, sem promoção por arrasto.

O contrato limita-se aos quatro pares identificados acima; não prova
equivalência SVG geral, não cobre espaços não listados e não autoriza ampliar
budgets ou reinterpretar casos fora do envelope.

Plano RED→GREEN autorizado: primeiro substituir os testes de regressão
P1231/P1235 dos quatro pares por testes que exijam referência local resolvida,
variante correta, ausência do fallback, offsets/stops/alpha preservados e
fill/stroke; confirmar RED com o predicate produtivo ainda restrito a sRGB;
depois ampliar apenas esse predicate para Oklab e LinearRgb em Linear/Radial e
confirmar GREEN. Ataques devem sobreviver como gates negativos para promoção
indevida de Hsv/Oklch/Hsl/Luma/CMYK, remoção silenciosa de fallback e bypass da
amostragem adaptativa. Por fim, repetir integralmente o envelope P1272, os
controles sRGB/P1234/P1236/P1237/P1264 e os gates V1/V5/V15/V26; qualquer
regressão ou `Unknown` necessário cancela a promoção.


### P1230 — Conic vetorial por pattern de cunhas

Conic aprovado no SVG permanece paint vetorial. O exportador materializa um
`pattern` local composto por cunhas angulares; cada cunha referencia um
`linearGradient` local de duas extremidades amostradas no espaço declarado.
O grafo `paint -> pattern -> path -> linearGradient -> stops` deve fechar sem
URL pendente, imagem raster ou fallback solid disfarçado.

A geometria preserva centro, ângulo inicial, progressão horária e correção
recíproca do aspect ratio da caixa pintada. Fill e stroke podem reutilizar uma
definição somente quando paint e aspect ratio forem morfologicamente iguais;
transformações do item são aplicadas uma vez pelo grafo SVG existente.

O número `360` observado no vanilla é heurística mecânica, não obrigação.
A implementação pode usar outra cardinalidade, mas cada combinação promovida
deve passar o contrato P1230: no interior angular, erro máximo RGB sRGB
codificado premultiplicado `<= 4/255`, p95 `<= 2/255` e alpha máximo
`<= 2/255`; na máscara local completa, p95 RGBA premultiplicado `<= 4/255` e
no máximo `0.5%` dos pixels podem exceder `16/255`. A máscara e a sua caixa
devem ser derivadas e publicadas a partir do grafo e da geometria local
declarada antes da comparação; incluem costuras, fronteiras e stroke e não
podem ser ajustadas por crop de cor/erro, erosão, fitting ou realinhamento do
conteúdo. Fronteiras coincidentes e a costura `0/1` preservam o lado medido
pelo vanilla, nunca uma média implícita.

Posicionamento produzido por layout, grid, flow e gutters fora dessa geometria
local permanece `Unknown` deste owner. Uma fixture isolada ou a sua máscara
local que falhe o budget viola o paint; somente excesso demonstravelmente
exclusivo fora da máscara pode permanecer `Unknown` de integração externa.

sRGB, Oklab, LinearRgb e Hsv só são promovidos individualmente quando grafo,
geometria e budget passarem. CMYK permanece fallback `conic-gradient`
explicitamente `Unknown` por ADR-0097; nenhum perfil ICC, download ou crate
externa é incorporado por este owner. Espaços não validados também permanecem
`Unknown`. O helper PDF `export/gradients/conic.rs` não é alterado nem
compartilhado por arrasto.


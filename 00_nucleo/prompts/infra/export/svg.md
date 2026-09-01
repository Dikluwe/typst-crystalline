# Prompt L0 — `infra/export/svg` — Exportação SVG
Hash do Código: c5024fd4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c

## P1140.20.2 — canvas SVG

render_bleed inclui bleed e translada TrimBox; sem ele recorta. Fill auto é
branco, none transparente, Paint pelo consumer. Ordem: background/body/foreground.

**Camada**: L3  
**Ficheiro alvo**: `03_infra/src/export/svg.rs`  
**Origem**: P870  
**ADRs**: ADR-0033 (paridade observable)

---

## Contexto

Exportador de páginas Typst para SVG. Recebe uma `Page` cristalina e devolve uma string SVG. Baseia-se na estrutura do `typst-svg` do vanilla (`lab/typst-original/crates/typst-svg/`), mas adaptado aos tipos cristalinos.

## Interface pública

```rust
pub struct SvgOptions {
    /// Se true, formata o SVG com indentação.
    pub pretty: bool,
}

impl Default for SvgOptions { ... }

/// Exporta uma página para SVG (texto sem fontes resolvidas → scope-out/tofu).
pub fn export_svg(page: &Page, opts: &SvgOptions) -> String;

/// Exporta uma página para SVG com fontes resolvidas.
pub fn export_svg_with_fonts(
    page: &Page,
    opts: &SvgOptions,
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
) -> String;
```

## Dependências

- `xmlwriter` — geração de XML.
- `base64` — embutir imagens no SVG.
- `itoa` / `ryu` — serialização de inteiros/floats.
- `ttf-parser` — extração de outlines de glifos para texto como path (P871).
- `flate2` — já em `03_infra/Cargo.toml`; compressão de imagens embutidas se necessário.

## Estratégia de porte

- Copiar a estrutura de `typst-svg/src/lib.rs` (`SVGRenderer`, `State`, render recursivo).
- Mapear `FrameItem` cristalino para elementos SVG:
  - `TextShaped` → glifos como paths (`<symbol>` + `<use>`), replicando o vanilla.
  - `Shape` → `<path>`, `<rect>`, `<circle>`, etc.
  - `Image` → `<image>` com `href="data:..."`.
  - `Group` → `<g transform="...">` com recursão.
  - `Line` → `<line>`.
  - `Link` → `<a>` envolvendo filhos.
- Texto (P871): para cada `ShapedGlyph`, extrair o outline via
  `ttf_parser::Face::outline_glyph`, construir um path SVG relativo e
  emitir `<defs><symbol id="..."><path d="..."/></symbol></defs>`;
  cada ocorrência do glifo referencia o símbolo com `<use xlink:href="#...">`.
  O grupo de texto aplica `matrix(1 0 0 -1 x y)` para inverter o eixo Y
  (fontes usam Y-up; SVG usa Y-down).

## Restrições

- L3 — sem I/O directo; recebe `Page` e devolve `String`.
- Usar tipos cristalinos.
- Foco em SVG standalone válido; `svg_in_bundle`/`svg_in_html` ficam fora do escopo.
- P1133: raios de `ShapeKind<Pt>::RoundedRect` chegam como `Corners<Pt>` já resolvidos.
  O SVG serializa esses pontos diretamente, sem `resolve_pt(0.0)` e sem
  projeção de parcela absoluta de `Length`.
- P1222: em rounded rect com stroke, o raio público descreve o contorno
  externo. O path central do stroke usa `max(radius - thickness/2, 0)` por
  canto. Fill e stroke são operações SVG separadas sobre essa geometria,
  preservando a morfologia do vanilla e evitando que o stroke aumente o raio
  exterior. O fill fecha o path; o stroke termina no ponto inicial sem `Z`,
  preservando cap/join do emissor de referência. Sem stroke, o fill usa os
  raios resolvidos sem inset.

## Critérios de verificação

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
  erro geométrico/cromático selado, não aproximar nem chamar pattern equivalente;
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
não selou causa mecânica. Por isso Linear/Oklab, Linear/LinearRgb e
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
quando satisfaz um envelope selado vigente. As três promoções invalidadas,
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
O contrafactual P1278 sobre os budgets selados melhorou Hsl de 14/24 para
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

#### P1280 — promoção dos seis pares polares certificados

**Gate ADR-0127: L0 confirmado explicitamente pelo dono em 2026-08-29.** A
mudança é comportamento por defeito: substitui fallback sólido por servidor
SVG adaptativo somente em Linear/Radial × Oklch/Hsl/Hsv.

Medição anterior à proposta: o certificado P1279 foi commitado em
`ad93af9e80213fc410ffeda1f4916b031b088474`. O certificado
`00_nucleo/diagnosticos/typst-p1279-offset-identity-certificate.md` tem SHA-256
`be1e16cfb002c70b8d7af83421e7027f2abdb32324a1e8ff94ace7152ee1e3ed`; o
resumo final tem SHA-256
`c0628bc0e2723e664c587a6b5077c8c9a59bea70d2ffe93909a44efdb5b5c80a` e o
manifesto de evidência tem SHA-256
`8f44e3ac8da2b9f75ee075f0656a7917f8039c39734e1cef12951ca21916c1a3`.
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
pares não certificados permanece normativa. O certificado cobre apenas este
fragmento e não prova equivalência SVG geral.

Plano RED→GREEN pós-confirmação: antes do predicate, testes do owner exigem,
para os seis pares, referência local resolvida, variante Linear/Radial correta,
ausência do marcador de fallback, stops/offsets/ordem/descontinuidades/alpha e
papéis fill/stroke preservados; com o predicate atual devem falhar. Depois,
amplia-se somente os dois braços Linear/Radial do predicate. Testes negativos
exigem que Luma e CMYK continuem fallback e que nenhum wildcard promova Conic
ou Tiling. A matriz completa é readjudicada com 144/144 polares nativos, 48/48
Luma ainda fallback, 56/56 inválidos rejeitados e 384/384 recibos
determinísticos; numeric/raster/grafo/custo polares permanecem 144/144 sem
alargar nenhum envelope. Ataques devem discriminar subpromoção, sobrepromoção,
referência pendente, solid disfarçado, bypass do sampler, mudança de budget e
aceitação por bytes/IDs; `mutation_score` exigido é `1.0`.

#### P1273 — promoção dos quatro pares certificados

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

O certificado P1272
`00_nucleo/diagnosticos/typst-p1272-generalization-certificate.md`
(SHA-256 `d07786a848af1778a02bdff7d468fd9db47a499c565281b36df02c21e76359ec`),
pinado pelo manifesto final
`00_nucleo/diagnosticos/p1272-final-manifest.tsv`
(SHA-256 `0c711546ab246becb1468a61e859ea9e6202e79f0379658ff8eea7d7d699619c`),
classificou individualmente como `Generalization-Preserved`, 24/24 cada:

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
certificado também permanece `Unknown`; opacidade nunca é convertida em
`Preserved` por default. Conic e Tiling conservam os seus contratos e reasons
próprios, sem promoção por arrasto.

A evidência P1272 limita-se ao envelope congelado de 96 fixtures, com 24 por
par: 96/96 no grafo e raster, 384/384 métricas numéricas, 28/28 entradas
inválidas rejeitadas, 192/192 recibos determinísticos e 24/24 mutantes
rejeitados (`mutation_score=1.0`). Esse envelope autoriza somente a mudança de
rota identificada acima; não prova equivalência SVG geral, não cobre espaços
não listados e não autoriza ampliar budgets ou reinterpretar `Unknown`.

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
externa é incorporado por este owner. Espaços não selados também permanecem
`Unknown`. O helper PDF `export/gradients/conic.rs` não é alterado nem
compartilhado por arrasto.

### P1224 — stroke complexo preservado no SVG

Quando `FrameItem::Shape` carrega `Stroke`, o exportador preserva tardiamente
`cap`, `join`, `miter_limit`, `dash.array` e `dash.phase`. Emite
`stroke-linecap`, `stroke-linejoin`, `stroke-miterlimit`, `stroke-dasharray` e
`stroke-dashoffset`; `DashLength::LineWidth` resolve para a espessura efetiva
sem reordenar o array. Phase negativa e alpha do paint são observáveis.

Os atributos podem ser omitidos somente quando a omissão for semanticamente
igual ao default SVG. Transform não é assado nos comprimentos de stroke por
este módulo. A entidade L1 é a fonte; L3 não reconstrói informação perdida.

Critério focal: cap round, join bevel, miter 2, dash `[3pt, LineWidth]`, phase
`-0.5pt`, thickness `2pt` e paint com alpha devem aparecer separadamente e na
ordem declarada no SVG.

### P1226 — fill rule de paths

Medição pública: o vanilla emite `fill-rule="evenodd"` para
`curve(fill-rule: "even-odd")`; o cristalino anterior ao contrato emitia o
default `nonzero`. O exportador deve consumir `FrameItem::Shape.fill_rule` e
emitir `evenodd` ou `nonzero` sem inferência pelo winding. Atributo omitido só
quando semanticamente igual ao default SVG e ao valor transportado.

- Documento "Hello" produz SVG parseável com texto renderizado como paths de glifo.
- Documento com formas produz elementos SVG correspondentes.
- Documento com imagem produz `<image>` base64.
- Estrutura SVG comparável à do vanilla (não byte-exact).
- SVG renderiza correctamente mesmo sem a fonte instalada no sistema.
- `radius: 1em` preserva o raio absoluto resolvido pelo layout.
- `rect(radius: 4pt, stroke: 2pt + blue)` emite centro de stroke com raio 3pt,
  fill e stroke separados, mantendo raio exterior 4pt.

### P1248 — imagens web, SVG embutido e fallback observável

Medição anterior à decisão: `FrameItem::Image` já entrega a este owner bytes,
posição, dimensões resolvidas, `clip_rect` de cover e orientação EXIF. Em
`03_infra/src/export/svg.rs`, o braço de imagem consumia apenas posição,
bytes, largura, altura e orientação (esta última ignorada); `render_image`
embutia PNG/JPEG/GIF/WebP, fixava `preserveAspectRatio="none"` e retornava sem
nó para formato `Unknown`. A referência ratificada prepara formatos web,
incluindo `ImageKind::Svg`, e emite data URL; o layout já decide fit/box antes
do exporter. Logo a obrigação é preservar a morfologia visual e os carriers
resolvidos, não copiar `WebImage`, bytes base64 ou estrutura Rust.

Decisão interna de paridade (fluxo contínuo ADR-0127):

- PNG, JPEG, GIF e WebP continuam embutidos com MIME coerente, data URL e
  caixa física exata. Alpha permanece nos bytes originais; não recodificar.
- Bytes cujo primeiro elemento XML significativo é `<svg` são embutidos como
  `image/svg+xml`. A identificação é local a L3 e conservadora: BOM UTF-8,
  whitespace, declaração XML e comentários iniciais podem ser ignorados;
  qualquer construção ambígua, comprimida ou sem raiz SVG fica `Unknown`.
  Este owner não altera o enum público `ImageFormat` nem promete parser SVG.
- O SVG aninhado permanece um recurso de imagem; não se reescrevem IDs,
  scripts, texto, formas ou alpha internos. A preservação alegada limita-se a
  um SVG finito e autossuficiente demonstrado por landmarks visuais.
- `clip_rect` finito emite clip local no espaço da página e recorta cover sem
  assar novamente posição ou orientação. Carrier ausente/malformado é
  `Unknown`, nunca ausência silenciosamente equivalente.
- Orientação EXIF 1–8 é aplicada exatamente uma vez por transformação SVG
  sobre a caixa já resolvida pelo layout. A ortografia da matriz não é
  observável; marcadores assimétricos discriminam ignorar e aplicar duas vezes.
- Formato desconhecido em `FrameItem::Image` emite um marcador diagnóstico
  não visual `data-crystalline-image-fallback="unknown-format"`; não emite
  `<image>`, não inventa MIME e permanece `Unknown`.
- Fit/aspect ratio não é redescoberto no exporter: as dimensões e posição do
  carrier já representam stretch/contain/cover; `preserveAspectRatio="none"`
  apenas aplica essa transformação resolvida. O exporter não recalcula layout.

Testes do owner cobrem os quatro MIME raster, alpha conservado no payload,
caixa/posição, cover clip, orientação assimétrica, SVG aninhado vermelho/azul
com alpha, desconhecido marcado e repetição determinística. Aceitação é
morfologia/semântica; igualdade byte/base64 serve somente para provar
determinismo do mesmo input, nunca paridade com o vanilla.

## P1140.5-A — SVG visualmente transparente

SVG recursa nos filhos de `FrameItem::Semantic` sem desenhar `alt`. Expor
acessibilidade SVG requer medição própria; nesta fase o contrato é preservar o
render e não transformar a descrição de PDF em `<text>` ou tooltip inventado.

## P1246 — clip geométrico local proposto

**Gate ADR-0127:** arquitetura aprovada pelo dono em 2026-08-28. L0 pré-código;
implementação somente após preseal segregado válido.

### Medição que precede a proposta

`FrameItem::Group` transporta `pos`, `matrix`, `clip_mask: Option<ShapeKind<Pt>>`,
`inner_width`, `inner_height` e filhos. O exporter SVG recebe `clip_mask` em
`render_group`, mas atualmente o ignora. A referência ratificada cria uma
definição `clipPath`, referencia-a no grupo e aplica a geometria no espaço do
grupo. A obrigação é o recorte observável, não a ortografia do ID, a ordem de
`defs` ou a estrutura Rust da deduplicação.

### Contrato proposto

- Quando `clip_mask` é `None`, o grupo mantém exatamente a semântica atual sem
  recorte adicional.
- Para `Rect`, `RoundedRect`, `Ellipse` e `Path` com geometria finita e
  representável, emitir um `clipPath` local e uma referência resolvida no grupo
  dono. `Rect`/`Ellipse` usam `inner_width` e `inner_height`; `RoundedRect`
  conserva os quatro raios; `Path` conserva subpaths e curvas no espaço local.
- A transformação e a posição do grupo são aplicadas uma única vez. O clip e
  os filhos compartilham a mesma base local; não assar novamente a transformação
  na geometria do clip.
- Grupos aninhados compõem clips por interseção sem perder o clip ancestral.
- IDs são locais, resolvidos e determinísticos. Renomear IDs ou reordenar
  definições sem alterar o grafo é mecânica. Deduplicação só é permitida quando
  geometria e base espacial observável forem equivalentes.
- O contrato atual não transporta fill-rule no `clip_mask`; paths cuja região
  dependa de even-odd permanecem `Unknown` até existir carrier legítimo. Uma
  linha aberta sem área não é promovida como clip preservado.
- Máscara alpha, `mask`, `filter`, `opacity` ou rasterização não substituem
  clip geométrico e permanecem fora desta alegação.
- Falha de representação, geometria não finita ou informação ausente conserva
  `Unknown`/`CONTRACT-GAP`; nunca remove silenciosamente o conteúdo nem promove
  ausência de clip a equivalência.

### Verificação exigida após confirmação

Contrato e oráculos independentes devem cobrir rect, rounded rect, ellipse,
path, transform, nesting, referência pendente, chave de deduplicação e casos
opacos. Somente mutações semanticamente negativas executadas entram no mutation
score. Nenhum score ou preseal é declarado por esta proposta documental.

## P1247 — destinos internos SVG na página corrente

**Gate ADR-0127:** arquitetura aprovada pelo dono em 2026-08-28. L0 pré-código;
implementação somente após contrato/oráculos/ataques segregados e preseal válido.

### Medição anterior à decisão

`FrameItem::Link` já transporta `LinkTarget::Destination(Label)` até L3, e
`PagedDocument` já conserva `extracted_label_pages` e
`extracted_label_positions`. A API page-only do exporter não recebe esses
mapas; por isso o consumer atual conhece a aresta, mas não consegue materializar
o nó de destino nem fechar a referência.

### Contrato do owner SVG

- Preservar `export_svg` e `export_svg_with_fonts` como wrappers compatíveis que
  usam contexto vazio. Adicionar variantes explícitas que recebem
  `&SvgDestinationContext`; não alterar silenciosamente callers unitários.
- `SvgDestinationContext` é dado L3 imutável e page-local: associa `Label` a
  posição e identidade SVG local. Não executa layout, não consulta filesystem e
  não conhece nomes de ficheiro.
- Para um destino presente, emitir exatamente um nó local identificável na
  posição medida e ligar `<a>` a ele. IDs podem ser renomeados, desde que o grafo
  permaneça fechado, determinístico e sem colisões.
- Para destino ausente ou pertencente a outra página sem rota fornecida, manter
  os filhos visuais e classificar o link como `Unknown`; não emitir fragmento
  pendente nem fabricar posição.
- URL externa permanece preservada e não depende do contexto interno.
- Escaping, nesting e transform preservam o conteúdo e a área clicável. Bytes,
  uso de `href` versus `xlink:href` e ortografia do ID são mecânica quando o
  grafo semântico é equivalente.

Cross-page e bundle permanecem fora deste owner até o caller fornecer uma rota
explícita. A expansão pública de `link()` para label, location ou page/x/y é
outro contrato e não é autorizada por P1247.

## P1249 — glifos matemáticos diretos SVG proposto

**Gate ADR-0127:** arquitetura aprovada pelo dono em 2026-08-28. L0 pré-código;
implementação exige posterior preseal segregado válido.

### Medição anterior à proposta

`FrameItem::Glyph` já transporta `pos`, `glyph_id`, `x_advance`, `size`,
`style` e `base_char`; `style.fill` já contém o paint. A pipeline resolve a
fonte efetiva com `FallbackFontMetrics::resolve_font_combo(base_char, style)` e
inclui o `FontKey` correspondente. O exporter recebe os bytes selecionados, mas
não a associação exata entre o glifo direto e a fonte resolvida. Como
`glyph_id` é relativo à face, sondar a primeira fonte que contém o número não
preserva identidade.

### Contrato proposto do owner SVG

- Estender o contexto explícito do exporter com uma associação imutável entre
  `GlyphFontRequest` normalizado e o `FontKey` completo resolvido pela pipeline.
  A chave inclui `base_char`, família solicitada, variante, variações e estado
  matemático relevante à resolução; não usa índice posicional, endereço, ordem
  de travessia ou identidade da ocorrência. Não alterar o variant público L1.
- Quando a associação e os bytes existirem, extrair o outline do `glyph_id` na
  face exata, aplicar escala `size / units_per_em`, posição e inversão Y uma
  única vez, e preservar `style.fill`.
- Reutilização em `GlyphDefs` inclui a identidade da fonte e o `glyph_id`; ids
  XML e ordem de `defs` são mecânica se a morfologia permanecer igual.
- O exporter confirma que o `FontKey` retornado existe na coleção fornecida e
  usa a posição encontrada somente localmente; reordenar a coleção não muda a
  fonte escolhida.
- Associação ausente, ambígua, face inválida, `units_per_em = 0` ou outline
  inexistente permanece `Unknown` e não produz glifo falso.
- `x_advance` é geometria de layout já consumida pelo posicionamento externo;
  o exporter não refaz shaping nem altera o cursor.
- Os wrappers sem fontes preservam o scope-out vigente. P1249 não autoriza
  fallback `<text>`, fonte do sistema, rede ou síntese a partir de `base_char`.

Contrato e ataques posteriores devem cobrir variante simples, assembly com
várias peças, duas fontes com o mesmo `glyph_id`, fill/alpha, transform, fonte
ausente e wrapper fontless.

## P1286 — morfologia SVG de paths abertos de dois pontos

### Medição anterior à decisão

O oráculo congelado mede `line(start:, end:)` como um `<path>` aberto cuja
origem é o `transform="translate(start)"` e cujo `d` é `M 0 0 l delta`. O
carrier L1 deliberadamente continua `ShapeKind::Path([MoveTo(start),
LineTo(end)])`; igualdade textual de SVG não é o objetivo, mas início e delta
são a morfologia observada pela sonda.

### Decisão

Ao emitir exatamente um path aberto `MoveTo + LineTo`, o SVG normaliza a
representação para translate no primeiro ponto e segmento relativo até o
segundo. A geometria, stroke, fill, bbox e ordem de pintura não mudam. Paths
fechados, cúbicos ou com outra cardinalidade continuam no emissor genérico.
É correção de paridade interna sem API pública, default ou fase nova.

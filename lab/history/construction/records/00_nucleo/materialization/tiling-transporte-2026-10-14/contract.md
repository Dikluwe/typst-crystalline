# Proposta de contrato — transporte de tiling

Status: aguardando confirmação ADR-0127. Estes arquivos são rascunhos; nenhum é L0 canônico nem autoriza código antes do aceite.

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/tiling-transporte-2026-10-14/{parent-before,solid-controls}/results.json.gz registra UTC, diffstat/status, fontes/owners, argv e hashes de binários. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8; SHA-256 do oráculo 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, árvore 6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489.
Layout tiling.rs:124–127 devolve None acima de 10000 células; shape.rs:151–185 só materializa fill, conservando stroke declarativo; export/stream/shape.rs:151/201 reduz Tiling a cor. layout_types.rs:505/531 não transporta célula nem raiz de pintura. render.rs:217 reduz fill a cor e :579 ignora clip_mask. Esses limites são do código/contrato de transporte, não só da fórmula do offset.
A medição anterior tiling-cobertura-2026-10-14 registra over-limit200×100pitch1 sólido e stroke sem motivo. Fontes respectivas permanecem idênticas; esse registro não é uma nova medição de binário.
No SVG atual 240×192 branco/librsvg, place-parent diverge em1194 pixels >16, box-fixed/auto-parent em1192; seus controles Solid coincidem pixel a pixel. block-parent tem2 pixels >16, máximo23. Os SVGs vanilla explicitam T(offset)*R e inversa de transformações herdadas. Fonte pinada typst-svg/src/lib.rs:346–358 e typst-layout/src/inline/box.rs:35–49 confirma raízes locais antes do inset final; FrameKind em typst-library/src/layout/frame.rs:459–474 distingue Hard/Soft. É semântica geométrica, não obrigação de copiar FrameKind, Packed ou igualdade Rust.
Controles Solid também refutam geometria geral em box-place e rotate: respectivamente3200/5296/3645 pixels >16. Portanto o transporte não será usado para declarar essas diferenças automaticamente resolvidas. Registrar a separação e corrigi-las com seus owners. No controle anterior quarter, Tekt PDF/SVG coincide com vanilla SVG, mas vanilla PDF diverge; não copiar essa diferença conhecida.

## API pública proposta

Criar `01_core/src/entities/frame_paint.rs`, owner exclusivo
`00_nucleo/prompts/entities/frame_paint.md`, e declarar `pub mod frame_paint`
na fachada entities. Tipos com `Debug, Clone`:

```rust
pub enum FramePaint {
    Solid(Color),
    Gradient(Gradient),
    Tiling(Arc<TilingFrame>),
}
pub struct TilingFrame {
    pub frame: Frame,
    pub spacing: Size,
    pub offset: Point,
    pub angle: Angle,
    pub relative: RelativeTo,
}
pub struct FrameStroke {
    pub paint: FramePaint,
    pub thickness: f64,
    pub cap: LineCap,
    pub join: LineJoin,
    pub dash: Option<DashPattern>,
    pub miter_limit: f64,
    pub specified: StrokeFields,
    pub overhang: bool,
}
```

Os tipos referidos já existem: Color; Gradient/RelativeTo em entities::gradient
(Self_/Parent, sem Auto); Frame/Size/Point/Angle em layout_types; os campos
geométricos em geometry. O frame.size é a dimensão física da célula, spacing
é distância entre células (assinada), offset é resolvido contra size+spacing,
angle permanece em radianos via Angle. Nenhum tipo tem dependência de exporter,
I/O ou runtime dinâmico. A indireção Arc quebra a recursão física.
FrameStroke mantém todos os campos de Stroke, trocando somente paint pelo
carrier resolvido; Default, se necessário, replica exatamente Stroke::default.
São permitidos From<Color> e From<Gradient> para FramePaint. É proibida uma
conversão total From<Paint> que esconda o layout de Tiling ou reduza a cor.
Não derivar igualdade de linguagem para FramePaint/FrameStroke/TilingFrame.

Em `FrameItem::Shape`, substituir `fill: Option<Paint>` por
`fill: Option<FramePaint>` e `stroke: Option<Stroke>` por
`stroke: Option<FrameStroke>`. Demais campos preservados, inclusive
parent_bbox_at_emit usado pelo caminho legado de gradientes.
Em `FrameItem::Group`, adicionar `paint_root: bool`: false herda a raiz;
true define origem local e dimensão inner_width/inner_height como raiz de
pintura. Page e TilingFrame são raízes implícitas. Não acrescentar variante
a FrameItem nem mudar assinatura de FrameVisitor.

Isso quebra código Rust externo que constrói/desestrutura Shape/Group ou
espera Paint/Stroke nesses campos. Todos os consumidores internos, inclusive
fixtures, terão migração explícita. `Paint`, `Stroke`, `Tiling`, `Value`,
TextStyle e APIs públicas de exportação mantêm seus tipos/assinaturas/defaults.
O tipo declarativo Tiling continua sem Frame. A restrição histórica P1245
contra carrier público resolvido é supersedida somente pelo novo FramePaint.

## Comportamento e migração

Layout prepara cada célula uma vez por ocorrência de paint, conserva cell clip,
tamanho automático/expresso, transparência, spacing, offset e angle. Auto
resolve para Self_ nos shapes; não permanece Auto no TilingFrame. Prepara ambos
os papéis, fill e stroke, incluindo produtores Shape de caixas/blocos/outros
módulos que já aceitam Paint. Não transportar AST/Content declarativo ao exporter.
O máximo10000 deixa de ser estratégia de repetição do layout. Validações de
finitude/pitch positivo precedem loops; célula válida vazia continua transparente.
Erros de materialização usam o SourceResult/diagnóstico já existente; sua mensagem
causal deve ser medida e especificada no owner antes do código. Cor de fallback
não conta como render correto. Casos legados Color/Image/Gradient exigem ponte
explícita no layout ou diagnóstico documentado; não inventar frame em L3.

Box/Block estabelecem paint_root no frame completo, com inset incluído. Place,
Rotate/Scale/Move e grupos técnicos usam false. No paint do próprio container,
preservar a distinção entre seu pai e a raiz que cria para os filhos; medir
fill/stroke próprios antes de atribuir root indistintamente ao backing shape.
Para cada consumidor, usar transformação local→raiz completa: Parent cancela
a transformação acumulada e a origem da shape; Self usa a origem da shape.
T(offset)*R(angle) e redução periódica na base rotacionada ficam invariantes.
Singularidade/infinito não legitima loop ou mudança silenciosa de cor.

PDF usa PatternType1 de célula única; SVG usa pattern userSpaceOnUse de célula
única; raster repete textura/célula preparada no viewport com máscara real de
fill/stroke. Não voltar a gerar N clones no Frame nem aumentar artificialmente
o limite antigo. Spacing negativo com pitch positivo requer sobreposição e
ordem de pintura corretas, não corte ingênuo à dimensão do pitch. Densidade
subpixel exige amostragem coerente; não sumir com o motivo por nearest arbitrário.
Clips aninhados e fill-rule devem ser respeitados no raster. Manter vetores nos
targets vetoriais e isolamento de estado. Não alterar modos Compact/Verbose,
PDF tags, padrões PDF, assinatura dos exporters, flags ou fase eval↔layout.

Shaping e coletores de fontes/imagens/gradientes/spot/patterns percorrem os
recursos da célula recursivamente, uma vez por recurso/ocorrência apropriada.
FrameVisitor genérico e plain_text não passam a interpretar célula como
conteúdo principal da página. Não multiplicar texto extraível, MCIDs, labels,
destinos ou links por repetição. Recurso de padrão é decorativo no PDF marcado.
Recursos aninhados têm identidade fechada/contexto correto, IDs determinísticos
por travessia e referências válidas; endereços não são serializados como IDs.

## Etapas explícitas — nenhuma é certificação isolada de produção

T1 instala carrier/owners/núcleos e migra produtores/consumidores, preservando
Solid/Gradient e sem gerar temporariamente fallback de produção como conclusão.
T2 materializa fill/stroke e roots e implementa PDF/SVG/raster, recursos/shaping,
clipping e repetição limitada pelo viewport. T3 fecha integrações e controles de
geometria de Place/Rotate nos respectivos owners, quando a medição os refutar.
O pacote fica incompleto até T1–T3 e todos os critérios abaixo passarem.
TextStyle.fill ainda Color, tiling em texto, novo fill Paint em Box/Block e
linhas matemáticas Color-only são etapas futuras explícitas (T4, contrato próprio);
não fazem parte desta migração pública nem da alegação de fechamento de tiling
universal. O objetivo geral de todas as correções permanece integralmente aberto.

## Aceitação

RED antes de implementação; fontes declarativas e checks não apenas estruturais.
Denso acima de10000, contorno simples/complexo, path/ellipse/rounded rect, fill-rule,
empty, texto/imagem/células aninhadas, offset relativo/absoluto/grande, angle,
spacing/gap/sobreposição; Self/Auto/Parent, Box/Block/inset/Place/Rotate/Scale e
reuso em páginas/tamanhos distintos. Recursos de fonte/imagem precisam chegar a
todos os targets, com PDF verificável por qpdf e sem perda de texto fora do padrão.
Comparar visual geométrico/linguístico com pin e controles analíticos; não elevar
tolerância, recortar pelo erro ou copiar a diferença vanilla PDF de quarter.
Demonstrar custo de transporte independente da contagem de repetições e medir
performance/memória com estado exato; contagem de objeto é evidência mecânica
de boundedness, não definição de paridade. Controles Solid/Gradient, texto fora
do padrão, destinos e PDF marcado preservados. Build/workspace/rustdoc/fmt,
linhagem 1:1/pins e inventário Clippy/arquitetura completos. Sem publicação.

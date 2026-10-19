# L0 — entities/frame_paint
Hash do Código: d09b66a9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/frame_paint.rs`

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/tiling-transporte-2026-10-14/{parent-before,solid-controls}/results.json.gz registra UTC, diffstat/status, fontes/owners, argv e hashes de binários. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8; SHA-256 do oráculo 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, árvore 6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489.
Layout tiling.rs:124–127 devolve None acima de 10000 células; shape.rs:151–185 só materializa fill, conservando stroke declarativo; export/stream/shape.rs:151/201 reduz Tiling a cor. layout_types.rs:505/531 não transporta célula nem raiz de pintura. render.rs:217 reduz fill a cor e :579 ignora clip_mask. Esses limites são do código/contrato de transporte, não só da fórmula do offset.
A medição anterior tiling-cobertura-2026-10-14 registra over-limit200×100pitch1 sólido e stroke sem motivo. Fontes respectivas permanecem idênticas; esse registro não é uma nova medição de binário.
No SVG atual 240×192 branco/librsvg, place-parent diverge em1194 pixels >16, box-fixed/auto-parent em1192; seus controles Solid coincidem pixel a pixel. block-parent tem2 pixels >16, máximo23. Os SVGs vanilla explicitam T(offset)*R e inversa de transformações herdadas. Fonte pinada typst-svg/src/lib.rs:346–358 e typst-layout/src/inline/box.rs:35–49 confirma raízes locais antes do inset final; FrameKind em typst-library/src/layout/frame.rs:459–474 distingue Hard/Soft. É semântica geométrica, não obrigação de copiar FrameKind, Packed ou igualdade Rust.
Controles Solid também refutam geometria geral em box-place e rotate: respectivamente3200/5296/3645 pixels >16. Portanto o transporte não será usado para declarar essas diferenças automaticamente resolvidas. Registrar a separação e corrigi-las com seus owners. No controle anterior quarter, Tekt PDF/SVG coincide com vanilla SVG, mas vanilla PDF diverge; não copiar essa diferença conhecida.

## Contrato ratificado — ADR-0127


Criar `01_core/src/entities/frame_paint.rs`, owner exclusivo
`00_nucleo/prompts/core/entities/frame_paint.md`, e declarar `pub mod frame_paint`
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


## Invariantes e aceitação

Materializar tipos puros e invariantes do núcleo; nenhum layout/render ocorre nesta unidade. T1–T3 de lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/contract.md completam produtores e targets; T4 é contrato futuro. Este L0 sozinho não certifica padrões produtivos. Aguardar aprovação do manifesto antes de instalar owner e escrever código.

## Ratificação e fronteira da etapa T1a — 2026-10-14

O humano aprovou “Aprovar a proposta” para o manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`.
O recibo está em `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A exigência de aguardar aprovação acima está satisfeita, sem alterar o pacote
aprovado. T1a instala os tipos puros e sua declaração na fachada. A migração
dos campos de FrameItem e produtores/consumidores completa T1b; PDF/SVG/raster,
recursos e clipping completam T2; integrações geométricas completam T3.
Este estado é explicitamente incompleto e não comprova correção do render.
Nenhum fallback legado é legitimado pelo novo carrier.

Nesta unidade, testar transporte integral de Solid/Gradient, retenção da célula
preparada por Arc sem cópia proporcional à repetição, geometria completa de
contorno e equivalência dos defaults com Stroke::default. Não oferecer conversão
total de Paint ou Tiling declarativo. Ausência de PartialEq nos novos carriers
não altera igualdade da linguagem. Testes de compilação verificam a fronteira.

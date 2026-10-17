# Revisão: tiling de produção em shapes

**Aguardando aprovação.** [Contrato completo](contract.md), [manifesto](manifest.json).
SHA-256 do manifesto: `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`.

A proposta envia uma célula preparada pelo layout para PDF/SVG/PNG e permite
que fill e stroke usem o padrão sem transformar alta densidade em cor sólida.
Também identifica quais grupos definem a origem de `parent`.

Mudanças públicas exatas:

- Novo módulo `entities::frame_paint`, com `FramePaint`, `TilingFrame` e `FrameStroke`.
- `FrameItem::Shape.fill`: Option<Paint> → Option<FramePaint>.
- `FrameItem::Shape.stroke`: Option<Stroke> → Option<FrameStroke>.
- Novo campo `paint_root: bool` em `FrameItem::Group`.

Construções/desestruturações Rust externas de Shape/Group precisam migrar.
O compilador conserva Paint/Stroke/Tiling declarativos e as assinaturas de exportação.
O layout continua responsável pela célula; exportadores recebem recursos prontos.
Não há flag ou modo padrão novo. Recursos de célula (fontes/imagens/padrões aninhados)
precisam ser preservados; texto/anotações não podem ser duplicados na página.

Aprovar autoriza T1–T3 do contrato, incluindo migração e testes de PDF/SVG/raster,
clipping e validação de geometria. Não autoriza publicar release. Tiling em texto,
fill Paint de caixas/blocos e linhas Color-only permanecem T4, explícito e separado.
A obrigação geral de todas as correções não é reduzida a este pacote.

As medições atuais isolam erro real de parent em place/box de defeitos gerais de
box-place/rotate, presentes também com cor sólida. Estes últimos não serão chamados
de corrigidos só porque o novo transporte compilou. block é controle positivo.

A parada é exigida por [ADR-0127 §2.1/§2.4](../../adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md):
“adicionar, remover ou alterar campo público” e mudança de “Compatibilidade”.
Os candidatos ainda não são L0 vigente; nenhum código produtivo foi alterado.

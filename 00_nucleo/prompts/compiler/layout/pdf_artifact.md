# Prompt L0 — `compiler/layout/pdf_artifact`
Hash do Código: ca36cee8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Estado do hash:** sentinela pós-gate; consumer ainda não materializado. Não é
selo e não autoriza `--fix-hashes` antes de existir o source proprietário.

**Camada:** L1 render  
**Consumer exclusivo:** `01_core/src/compiler/layout/pdf_artifact.rs`  
**Contrato predecessor:** `00_nucleo/diagnosticos/p1286-contract-receipt.md`
v2, SHA-256
`16597543965ca13d37fdabf9be184f3477df92da4f6702ba14f0a4ca4918f9bb`  
**Gate humano:** confirmado em 2026-08-30 — “Siga a engenharia da refatoração
e pode implementar”.  
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129.

## Medição anterior à decisão

O vanilla mostra o body sem mudar visual/texto, mas preserva `ArtifactElem`
até o tagging. No PDF medido, `Other` usa `/Artifact BMC`; Header e Background
usam BDC com propriedades e nenhum MCID/StructElem. O cristalino já envolve
Formula em `FrameItem::Semantic`; reutilizar `SemanticKind::Formula` seria
semanticamente falso.

## Forma B e contrato da free function

Toda a lógica de layout desta feature vive neste módulo descendente. O braço
central permanece uma delegação estática de uma linha. A função recebe
`&PdfArtifactElem`, realiza o body exatamente uma vez pelo Layouter e envolve
os items visuais produzidos em `FrameItem::Semantic` com
`SemanticKind::Artifact(elem.kind)`, `alt: None`:

```rust
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &PdfArtifactElem,
);
```

O envelope é transparente para bounds, cursor, baseline, posição, quebra,
ordem de pintura e plain-text. Não cria MCID, location ou nó estrutural; essas
exclusões são consumidas por builder/stream. Se o layout normal fragmentar o
body entre frames, cada fragmento comprometido conserva o mesmo kind sem
perder ou duplicar item; não forçar subframe único nem impedir paginação.

## Fronteiras e aceitação

- Não usar vtable, `dyn`, wildcard, PropMap, import `entities→layout` ou
  visibilidade `pub(crate)`; o pin Forma B é normativo.
- Não colapsar para body antes do stream e não reutilizar Formula.
- Com tags disabled, o envelope ainda pode existir internamente, mas o
  consumer PDF deve desenhar os filhos sem BMC/BDC/EMC. Verbose/Compact não
  muda a semântica.
- O layout nunca decide property list PDF, MCID, StructElem ou comportamento
  de AT; estes pertencem aos owners de export e permanecem limitados ao
  fragmento medido.
- Nested artifacts, multi-page real e interação com running matter não foram
  medidos end-to-end e permanecem `Unknown`, embora nenhum possa autorizar
  perda/duplicação visual.
- Este prompt não legitima `PdfArtifactElem`, `SemanticKind`, builder ou
  stream.

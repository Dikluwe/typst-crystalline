# Prompt L0 — `compiler/layout/pdf_attach`
Hash do Código: 8bf0dd23

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Estado do hash:** sentinela pós-gate; consumer ainda não materializado. Não é
selo e não autoriza `--fix-hashes` antes de existir o source proprietário.

**Camada:** L1 render  
**Consumer exclusivo:** `01_core/src/compiler/layout/pdf_attach.rs`  
**Contrato predecessor:** `00_nucleo/diagnosticos/p1286-contract-receipt.md`
v2, SHA-256
`16597543965ca13d37fdabf9be184f3477df92da4f6702ba14f0a4ca4918f9bb`  
**Gate humano:** confirmado em 2026-08-30 — “Siga a engenharia da refatoração
e pode implementar”.  
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129.

## Medição anterior à decisão

O vanilla não gera frame para `AttachElem` e o coleta globalmente. O contrato
aprovado escolheu side-channel de layout até
`PagedDocument.attachments: Vec<Arc<PdfAttachElem>>`; criar `FrameItem`
visível ou nova fase seria divergência. O dispatcher central permanece
exaustivo, estático e magro.

## Forma B e contrato da free function

Toda a lógica desta feature vive neste módulo descendente, nunca no struct de
domínio. O braço central apenas delega. A operação recebe o `Arc` do elemento,
clona esse handle O(1) para o side-channel privado do Layouter e retorna sem
emitir item:

```rust
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &Arc<PdfAttachElem>,
);
```

Uma invocação registra uma ocorrência em ordem documental. Não altera cursor,
largura/altura, baseline, regiões, página, plain-text, locator nem items. A
composição final move o vetor para `PagedDocument.attachments`; não relê bytes
e não decide Filespec.

## Fronteiras e aceitação

- Não usar vtable, `dyn`, wildcard, PropMap, import `entities→layout` ou
  visibilidade `pub(crate)`; o pin Forma B é normativo.
- Não deduplicar aqui. A pipeline, após layout e antes de export, compara paths
  virtuais em ordem e produz o erro global canônico.
- A mesma ocorrência semântica deve ser registrada uma vez por travessia
  comprometida. Re-layout especulativo não pode criar attachment fantasma;
  qualquer mecanismo transacional necessário pertence ao owner do motor.
- Repetição deliberada do elemento conserva duas ocorrências para que o erro
  de path duplicado seja observável.
- Headers repetidos, floats refluídos e outros casos não medidos permanecem
  `Unknown`; não convertê-los em sucesso implícito.
- Este prompt não legitima `PdfAttachElem`, o campo de `PagedDocument`, a
  validação global nem a serialização PDF.

# Prompt L0 — layout (infra)
Hash do Código: f8888a93

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4bf17f1455eef032ab3e30ea038edabed721e8378b913aaecf2b544bf288a917

## Módulo
`03_infra/src/layout.rs`

## Propósito
Ponte entre `FontBookMetrics` (L3) e `Layouter<M>` (L1 genérico).
Expõe `layout_with_font()` para uso por L4 ou exportadores.

## Interface
```rust
pub fn layout_with_font(
    content:   &Content,
    font_data: &[u8],
    font_size: f64,
) -> PagedDocument;
```

## Comportamento
- Se `font_data` é válido → usa `FontBookMetrics` (métricas reais)
- Se inválido → fallback para `typst_core::compiler::layout::layout()`
- O caminho de fonte válida entra pelo mesmo entrypoint de passagem de layout
  que instala o transcript math. Como esta API não possui `Engine`/`World`
  para realizar callbacks, uma request pendente falha fechada antes do retorno;
  jamais devolve `PagedDocument` provisório a L4 ou a um exporter.
- A assinatura pública permanece `PagedDocument`; o erro fechado de callback
  é uma pré-condição violada do helper legado, não um documento vazio nem uma
  geometria parcial. A pipeline produtiva capaz de callbacks usa
  `infra/pipeline.md`, que realiza e estabiliza as requests.

## Critérios de verificação
- Bytes inválidos → não panic, retorna documento via fallback
- Bytes válidos (com fixture) → documento não vazio para texto não vazio
- Conteúdo com callback math e fonte válida não retorna documento provisório

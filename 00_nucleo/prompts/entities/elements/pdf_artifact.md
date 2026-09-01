# Prompt L0 — `entities/elements/pdf_artifact` — `PdfArtifactElem`
Hash do Código: 67b7a3a0

**Estado do hash:** consumer materializado; o valor de `Hash do Código` é o
selo calculado pelo `crystalline-lint --fix-hashes`.

**Camada:** L1 domínio  
**Consumer exclusivo:** `01_core/src/entities/elements/pdf_artifact.rs`  
**Contrato predecessor:** `00_nucleo/diagnosticos/p1286-contract-receipt.md`
v2, SHA-256
`16597543965ca13d37fdabf9be184f3477df92da4f6702ba14f0a4ca4918f9bb`  
**Gate humano:** confirmado em 2026-08-30 — “Siga a engenharia da refatoração
e pode implementar”.  
**ADRs:** ADR-0029, ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O vanilla pinado define `ArtifactElem { kind, body }` e doze kinds em
`pdf/accessibility.rs:36-93`; o layout conserva visual/texto e o PDF mantém a
identidade `/Artifact` sem MCID/StructElem. O receipt predecessor mediu
`other`, Header e Background, e registrou AT real como `Unknown`. A árvore
cristalina já possui Formula P1140.6; passthrough que perca o kind é incorreto.

## Contrato do consumer

Este ficheiro é o owner único dos dados abaixo:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactKind {
    Header,
    Footer,
    Watermark,
    PageNumber,
    LineNumber,
    Redaction,
    Bates,
    Page,
    PaginationOther,
    Layout,
    Background,
    Other,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PdfArtifactElem {
    pub kind: ArtifactKind,
    pub body: Content,
}
```

`ArtifactKind::Other` é o default de linguagem. O enum é fechado e preserva
os doze nomes kebab-case validados pela nativa; não aceitar string arbitrária
nem colapsar kinds distintos.

## `Element` e morfologia

- É contentor não-locatável no fragmento aprovado:
  `element_kind`/`to_payload`/`dyn_kind_name` usam defaults. Não criar
  `ElementKind` ou payload de query sem nova medição/gate.
- `plain_text()` e `is_empty()` delegam ao body.
- `map_content` transforma o body exatamente uma vez e reembrulha o mesmo
  kind; `map_text` desce no body e também preserva o kind.
- `get_field("body")` devolve o body e `get_field("kind")` devolve o nome
  kebab-case. Outros campos devolvem `None`. A representação pública omite o
  kind quando `Other` e inclui `kind` antes de `body` nos demais casos,
  conforme a morfologia P1286 medida; `kind:"other"` explicitamente assente
  não foi distinguido do default e permanece `Unknown`.
- Igualdade/hash são estruturais sobre `kind` e `body`.

## Restrições e aceitação

- L1 puro; não importar Layouter, stream ou exporter.
- Nunca devolver apenas o body: a identidade semântica precisa sobreviver até
  o layout/PDF.
- Visual e plain-text são os do body; isso não implica comportamento provado
  em AT, reflow ou copy-paste.
- Nested artifacts, fragmentação multipágina e fallbacks dos doze kinds por
  versão PDF permanecem `Unknown` fora do fragmento medido.
- O consumer mantém testes co-localizados que provam a bijeção dos doze nomes,
  o default `Other` e a preservação de `kind`/morfologia ao mapear o body.
- Ownership é 1:1. Este prompt não legitima content, layout ou export.

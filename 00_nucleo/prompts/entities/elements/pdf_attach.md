# Prompt L0 — `entities/elements/pdf_attach` — `PdfAttachElem`
Hash do Código: a32148c9

**Estado do hash:** consumer materializado; o valor de `Hash do Código` é o
selo calculado pelo `crystalline-lint --fix-hashes`.

**Camada:** L1 domínio  
**Consumer exclusivo:** `01_core/src/entities/elements/pdf_attach.rs`  
**Contrato predecessor:** `00_nucleo/diagnosticos/p1286-contract-receipt.md`
v2, SHA-256
`16597543965ca13d37fdabf9be184f3477df92da4f6702ba14f0a4ca4918f9bb`  
**Gate humano:** confirmado em 2026-08-30 — “Siga a engenharia da refatoração
e pode implementar”.  
**ADRs:** ADR-0029, ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O vanilla pinado preserva path virtual, bytes, relationship, MIME e descrição
em `pdf/attach.rs:31-86`; coleta attachments globalmente e não gera conteúdo
visual (`typst-layout/src/rules.rs:824`, `typst-pdf/src/attach.rs:13-67`). O
receipt predecessor mediu path obrigatório, bytes como segundo posicional
opcional, extração byte-idêntica e erro por path duplicado. O contrato público
aprovado adiciona `Content::PdfAttach(Arc<PdfAttachElem>)` sem `Value`, trait ou
fase novos.

## Contrato do consumer

Este ficheiro é o owner único dos dados abaixo:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttachedFileRelationship {
    Source,
    Data,
    Alternative,
    Supplement,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PdfAttachElem {
    pub path: EcoString,
    pub data: Arc<Vec<u8>>,
    pub relationship: Option<AttachedFileRelationship>,
    pub mime_type: Option<EcoString>,
    pub description: Option<EcoString>,
}
```

`path` é o nome virtual já resolvido usado tanto como identidade de
deduplicação quanto no Filespec. `data` contém o snapshot lido/fornecido em
eval; este elemento nunca consulta `World`, filesystem ou L3. A validação de
path, MIME, tipos e relationship pertence à nativa `pdf.attach`; o elemento
não aceita nem corrige payload inválido silenciosamente.

## `Element` e morfologia

- É leaf invisível e não-locatável no fragmento aprovado:
  `element_kind`/`to_payload`/`dyn_kind_name`/`get_field` usam os defaults. Não
  acrescentar `ElementKind`, query payload ou location sem nova medição/gate.
- `plain_text()` devolve string vazia, mas `is_empty()` devolve `false`: o
  marker não pode ser podado antes da coleta.
- `map_content` e `map_text` são terminais e reconstroem
  `Content::PdfAttach(Arc::new(self.clone()))`, preservando todos os campos.
- Igualdade/hash são estruturais sobre os cinco campos; identidade de erro por
  duplicado, contudo, usa `path`, não igualdade Rust do struct inteiro.

## Restrições e aceitação

- L1 puro; `Arc<Vec<u8>>` é estado de RAM permitido, não I/O.
- Não criar frame, placeholder, texto, metadata aproximada ou fallback.
- Dois elementos distintos com o mesmo path continuam distintos aqui; a
  pipeline global é quem emite `attempted to attach file {path} twice`.
- Compressão, todos os standards PDF e variantes não medidas de `PathOrStr`
  permanecem `Unknown`; `Unknown` nunca é sucesso.
- O consumer mantém testes co-localizados que provam parsing fechado de
  relationship, invisibilidade textual sem poda e preservação byte a byte dos
  cinco campos nas operações terminais de `Element`.
- Ownership é 1:1. Este prompt não legitima content, layout, pipeline ou
  exporter e não pode ser reutilizado por outro source.

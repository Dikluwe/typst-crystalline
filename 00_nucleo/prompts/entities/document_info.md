# Prompt L0 — `entities/document_info`
Hash do Código: 77aa0cfc

## Camada
L1

## Ficheiro alvo
`01_core/src/entities/document_info.rs`

## Propósito
Transporte puro de metadados do documento definidos por `#set document(...)`
desde a avaliação (`EvalContext`) até ao módulo (`Module`) e, via pipeline, até
ao documento paginado (`PagedDocument`) e ao exportador PDF (`/Info`).

> **Fonte de paridade (P1031)** — doc comment `#[elem]` do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/model/document.rs:14-34`, publicado em
> `typst.app/docs/reference/model/document/`:
>
> *"The document element is the single source of truth for document metadata. With it, you
> can specify the document's title, authors, date, etc. in one place. Typically, the element
> is used with a set rule like this: `#set document(title: [My doc])`. […] By default, the
> metadata is embedded into the output, but not visibly rendered in the document."*
>
> **Citação literal** para as duas afirmações do propósito: a forma de superfície é
> `#set document(...)` e o destino é a metadata do output (o `/Info` do PDF), sem render
> visível.
>
> **Campos — cobertura parcial declarada.** O vanilla define cinco
> (`document.rs:157-179`): `title: Option<Content>`, `author: OneOrMultiple<EcoString>`,
> `description: Option<Content>`, `keywords: OneOrMultiple<EcoString>`,
> `date: Smart<Option<Datetime>>`. O `DocumentInfo` do cristalino cobre três (`title`,
> `author`, `keywords`) e usa `Option<EcoString>` onde o vanilla usa `OneOrMultiple` /
> `Content`. Ausentes: `description` e `date` — **lacuna documentada**, não afirmação
> incorrecta. Notas do vanilla com consequência prática, ainda não registadas aqui:
> `date` por defeito é `{auto}` = data e hora actuais, e *"If you want to create
> byte-by-byte reproducible PDFs, set this to something other than `{auto}`"*
> (`document.rs:168-178`); `title` é requisito de PDF/UA (`document.rs:151-153`).

## Tipo

```rust
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocumentInfo {
    pub title: Option<EcoString>,
    pub author: Option<EcoString>,
    pub keywords: Option<EcoString>,
}
```

- Campos opcionais — só os argumentos efectivamente fornecidos em
  `#set document(...)` são preenchidos.
- `author` aceita tanto `str` como `array` de strings (convertido para uma
  única string separada por vírgula, paridade vanilla simplificada).
- `Default` e `empty()` produzem metadados vazios.
- `is_empty()` é true quando todos os campos são `None`.

## Semântica

Não contém I/O nem lógica de render. É um contentor de valores já avaliados.
A decisão de formato (ex.: data de criação, creator) fica no exportador PDF
(L3), não aqui.

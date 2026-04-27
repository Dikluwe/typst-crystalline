# Prompt L0 — BibEntry
Hash do Código: 763d8bd8

## Módulo
`01_core/src/entities/bib_entry.rs`

## Propósito

`BibEntry` é entrada bibliográfica minimal — vanilla
`hayagriva::Entry` reduzido a 4 fields universais (key, author,
title, year). Adicionado em P159A (Model Bibliography + Cite
par acoplado, ADR-0060 §"Decisão 2" Fase 2 sub-passo 1) como
suporte a `Content::Bibliography { entries: Vec<BibEntry>, ... }`.

## Divergência do original (subset minimal per ADR-0054 graded)

Vanilla `hayagriva::Entry` integra parsing CSL profundo + dezenas
de fields tipográficos. Cristalino reduz a **4 fields universais**
em todas as styles bibliográficas:

- `key`: identificador único (paridade vanilla `Label`).
- `author`: campo universal; usado por todas as styles.
- `title`: campo universal idem.
- `year`: campo universal; `u32` para anos positivos (0 = "no
  year").

Fields vanilla **diferidos** per ADR-0054 graded (extensível
sem breaking change via adição de `Option<String>` fields):
`volume`, `pages`, `journal`, `publisher`, `url`, `doi`, etc.

Refino futuro candidato a integração `hayagriva` via ADR-0062
promovida (NÃO reservado per política P158 estabelecida em P158).

## Representação

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibEntry {
    pub key:    String,
    pub author: String,
    pub title:  String,
    pub year:   u32,
}
```

## Interface pública

```rust
impl BibEntry {
    pub fn new(
        key:    impl Into<String>,
        author: impl Into<String>,
        title:  impl Into<String>,
        year:   u32,
    ) -> Self;
}
```

`PartialEq` derivado — equivalência por todos os 4 fields.
`Clone`/`Debug` derivados — entry é dados puros sem métodos
próprios neste passo.

## Uso

`Content::Bibliography { entries: Vec<BibEntry>, title: ... }`
em P159A. Stdlib `native_bibliography` parseia `Value::Array<
Value::Dict>` para `Vec<BibEntry>` via helper privado
`extract_bib_entries` (validação hard de fields obrigatórios).

## Critérios de verificação

- `BibEntry::new("k", "A", "T", 2024)` produz entry com 4 fields
  acessíveis.
- `PartialEq`: cada field divergente quebra equivalência.
- `Debug` formatting inclui os 4 fields legíveis.

## ADRs aplicadas

- **ADR-0034**: diagnóstico cumprido em P159A §1.3.
- **ADR-0054**: graded scope-out de fields adicionais
  (volume/pages/journal/etc.).
- **ADR-0065 critério #2** (escolha de tipo): primeira aplicação
  isolada concreta — decisão de 4 fields minimais documentada
  em diagnóstico P159A §1.3.
- **ADR-0037** (coesão por domínio): ficheiro novo
  `entities/bib_entry.rs` (paridade padrão P156C `sides.rs`).

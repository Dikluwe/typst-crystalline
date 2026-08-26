# Prompt L0 — Parser BibTeX minimal (P450)
Hash do Código: a1887dad

**Camada**: L1 (parser puro; exposto para L3 via `SystemWorld::load_bibliography`).
**Ficheiro alvo**: `01_core/src/compiler/eval/bibtex.rs`
**Ficheiro consumidor L3**: `03_infra/src/world.rs` (`load_bibliography`)

---

## Propósito

Parser recursivo descendente para um subset BibTeX usado pelo Typst, evitando
uma dependência externa pesada para o caso mais comum (ficheiros `.bib`).
Integra-se com a pipeline de bibliografia já existente: `native_bibliography`
usa-o para `.bib`; `.yaml`/`.yml` continuam a usar `hayagriva::io`.

---

## API pública

```rust
pub struct BibTeXError { pub message: String }

pub fn parse_bibtex(src: &str) -> Result<Vec<BibEntry>, BibTeXError>
```

`BibEntry` é a entidade cristalino em `01_core/src/entities/bib_entry.rs`.

---

## Gramática suportada

```text
file       ::= entry*
entry      ::= '@' type '{' key ',' field_list? '}'
type       ::= 'article' | 'book' | 'inproceedings' | 'misc'
             | 'phdthesis' | 'techreport'
key        ::= texto sem espaço/virgula/chaveta
field_list ::= field (',' field)* ','?
field      ::= name '=' value
name       ::= identificador
value      ::= '{' braced '}' | '"' quoted '"' | number
braced     ::= qualquer coisa com chavetas balanceadas
quoted     ::= qualquer coisa até à próxima aspa não escapada
number     ::= sequência de dígitos (opcionalmente precedida de '-')
```

- Tipos de entrada são **case-insensitive** na prática (normalizados para
  minúsculas).
- Campos obrigatórios por entrada: `title`, `author`, `year`.
- Campos opcionais reconhecidos: `doi`, `url`, `journal`, `booktitle`,
  `volume`, `pages`.
- `booktitle` é mapeado para `BibEntry.journal` quando `journal` está ausente.
- O campo `author` usa o formato BibTeX canónico:
  `Last, First and Last2, First2`.
  - Cada autor é separado por ` and ` (minúsculas).
  - Se houver vírgula, a parte antes é o apelido e a parte depois é o nome.
  - O resultado canónico é `"Last, First and Last2, First2"`.

---

## Escaping

- Strings entre `{}` suportam aninhamento: o conteúdo é tudo entre as chavetas
  exteriores; chavetas interiores são preservadas (protecção de
  capitalização BibTeX).
- Strings entre `""` terminam na próxima aspa não escapada.
- Não há processamento de macros LaTeX (`\"{o}`, `\emph`, etc.) — são
  guardados literalmente.

---

## Comentários

- Linhas que comecem por `%` são ignoradas (comentário estilo TeX).
- Qualquer texto fora de uma entrada `@type{...}` é considerado erro de sintaxe.

---

## Limitações / scope-outs

- Apenas os 6 tipos de entrada listados; outros (ex.: `@online`, `@proceedings`)
  produzem erro.
- Não suporta `@string`, `@preamble`, `@comment` com semântica.
- Não suporta concatenação de strings BibTeX (`#`).
- Encoding apenas UTF-8; ficheiros ISO-8859-1 devem ser rejeitados a montante.
- LaTeX macros arbitrários não são expandidos.

---

## Testes canónicos

```
parse_bibtex("@article{a, author={A}, title={T}, year={2020}}") -> Ok([BibEntry { key:"a", author:"A", title:"T", year:2020 }])
parse_bibtex("@book{b, author={Doe, Jane and Smith, John}, title={B}, year={2021}}") -> author: "Doe, Jane and Smith, John"
parse_bibtex("@article{x, author={A}, title={The {Crystal} Math}, year={2022}}") -> title: "The {Crystal} Math"
parse_bibtex("@misc{y, author={A}, title={T}, year={2020}}") -> Ok; campos opcionais None
parse_bibtex("@online{z, author={A}, title={T}, year={2020}}") -> Err "tipo de entrada não suportado"
parse_bibtex("isto não é bibtex") -> Err "conteúdo inesperado"
```

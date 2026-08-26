# Prompt L0 — `entities/elements/title` — entidade `TitleElem`

Hash do Código: ef2b41ab

**Camada:** L1  
**Ficheiro proprietário:** `01_core/src/entities/elements/title.rs`  
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Medição vigente

O consumer proprietário define a entidade pública `TitleElem { body: Content }`, o
constructor `TitleElem::new(body)` e a implementação de `Element`. A entidade não faz
I/O, não consulta contexto e não escolhe defaults.

## Contrato

- `TitleElem` conserva o `Content` recebido no campo público `body`.
- `TitleElem::new(body)` constrói a entidade com esse mesmo body.
- `plain_text()` delega ao body.
- `map_content` e `map_text` transformam o body e preservam a identidade
  `Content::Title`.
- `get_field("body")` devolve `Value::Content` com o body; qualquer outro campo
  devolve `None`.

## Aceitação em nível de linguagem

Um título conserva a sua identidade semântica e o seu body através das
transformações de conteúdo. Os testes focais verificam a projeção textual e o acesso
ao campo `body`, incluindo a ausência de campos desconhecidos.

## Fora de escopo

Não pertencem a este owner: parsing e constructor de linguagem `title(...)`, fallback
de `document.title`, layout em 1.7em/negrito, introspecção, HTML ou decisões futuras
de apresentação. O constructor de linguagem pertence a
`compiler/stdlib/structural/title.md`; o layout pertence a
`compiler/layout/title.md`.

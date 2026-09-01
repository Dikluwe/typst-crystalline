# Prompt L0 — entidade de conteúdo HTML
Hash do Código: 453c2796

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9

**Estado:** APROVADO NO GATE ADR-0127 EM 2026-08-25  
**Camada:** L1  
**Owner candidato:** `01_core/src/entities/html.rs`

## Medição anterior à decisão

`typst-html/src/lib.rs:47-93` mede `HtmlElem` com tag obrigatória, attrs,
propriedades internas e body opcional. `01_core/src/entities/content.rs:135`
não possui variante capaz de preservar tag + atributos + body; o exporter
`03_infra/src/export/html.rs:30-84` só converte elementos Typst existentes.

## Contrato inicial incompleto (P1166)

Criar entidade pública pura `HtmlElem` com tag validada, atributos ordenados e
body opcional, e variante pública correspondente em `Content`. A igualdade
Rust/estrutura de armazenamento é mecânica; `repr`, fields assentes e DOM
serializado são observáveis. Nomes e valores são preservados semanticamente;
escape ocorre somente no exporter.

O contrato deve distinguir body omitido de body vazio e attrs omitidos de
dictionary vazio quando a reflexão vanilla distinguir. A medição fina de
`.fields()` precede os testes RED de P1166. CSS interno, parent/location,
frame, DOM expandido, MathML e typed attrs ficam fora.

## Invariantes e aceitação

- entidade sem I/O e sem dependência de L3 ou `lab`;
- `Content` continua enum fechado e despacho exaustivo;
- map/repr/plain-text/reflexão declaram braços explícitos;
- `html.elem` round-trip preserva tag, attrs e body;
- serialização valida/escapa nomes e valores sem assar estilo de render.

## P1167 — efeito do primeiro lote tipado (APROVADO EM 2026-08-25)

A medição mostrou que os 12 constructors propostos em P1168 convertem os 76
atributos globais para strings antes de criar o mesmo `HtmlElem` do P1166.
Logo não há novo campo, variante ou tipo público nesta entidade. `HtmlAttrs`
continua preservando a sequência de pares já convertidos; a tabela de
assinaturas/casts pertence ao owner `compiler/stdlib/html`, não ao dado.

Classificação void/raw também não entra neste contrato no P1168: todas as 12
tags aprováveis são normais. P1171 deve nuclear a classificação declarativa
antes de materializar `br`; não acrescentar booleanos ad hoc a `HtmlElem`.

## P1168 — estado triádico de body (APROVADO NO GATE ADR-0127 EM 2026-08-25)

Medição binária posterior ao primeiro GREEN focado refutou a hipótese P1167
de que a entidade existente preservava todos os observáveis:

```text
vanilla html.div()        -> elem(tag: "div", body: none)
vanilla html.elem("div") -> elem(tag: "div")
cristalino, ambos         -> elem(tag: "div")
```

`Option<Box<Content>>` distingue apenas conteúdo de ausência e não consegue
representar simultaneamente `unset`, `none` definido e conteúdo. Substituir o
campo público `body` por um estado fechado explícito:

```rust
pub enum HtmlBody {
    Unset,
    None,
    Content(Box<Content>),
}

pub struct HtmlElem {
    pub tag: EcoString,
    pub attrs: Option<HtmlAttrs>,
    pub body: HtmlBody,
}
```

`html.elem` produz `Unset` quando body é omitido e `Content` quando fornecido.
Os constructors normais tipados produzem `None` quando body é omitido e
`Content` quando fornecido. `repr` omite somente `Unset`, imprime `body: none`
para `None` e o conteúdo para `Content`. Exporter/plain-text/walkers tratam
`Unset` e `None` como corpo vazio e descem somente em `Content`.

Esta substituição foi aprovada pelo dono antes da materialização. Não usar
tag, conteúdo vazio nem booleano oculto como codificação implícita.

## P1288 — retirar ownership do set de features (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`01_core/src/entities/html.rs:12-42` contém hoje `Feature`/`Features` antes de
`HtmlAttrs`/`HtmlBody`/`HtmlElem`, e `:88-92` testa o set no mesmo consumer.
A fonte vanilla pinada mede `A11yExtras` como feature irmã de `Html` em
`typst-library/src/lib.rs:272-305`; portanto a coleção deixou de ser uma
responsabilidade HTML.

### Decisão proposta

Após confirmação humana, mover a definição canônica para o owner
`entities/compiler_features.md`. Este consumer continua dono somente de
`HtmlAttrs`, `HtmlBody`, `HtmlElem` e suas operações puras. Preservar
`entities::html::{Feature, Features}` como re-export explícito da identidade
canônica enquanto o path público anterior exigir compatibilidade; não manter
estado, implementação ou teste duplicado em HTML.

O binding público `html` e o exporter permanecem sujeitos ao Núcleo pinado;
esta entidade não ativa feature, target nem formato.

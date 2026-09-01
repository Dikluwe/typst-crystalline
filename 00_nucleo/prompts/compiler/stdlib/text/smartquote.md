# Prompt L0 — `compiler/stdlib/text/smartquote` — `smartquote`
Hash do Código: dc4246cd

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/text/smartquote.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/text.md` — dono de `text/mod.rs`
e da história por marco (`a1fe997d2`, `077792dfa`). Este L0 especifica **a superfície do
nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/text/smartquote.rs` — `SmartQuoteElem`, ficheiro próprio.

**Fronteira medida**: nó de uma nativa só. `smartquote` é a função com mais commits de
corpo do módulo (9 de 30 que tocaram o ficheiro), e nenhum desses commits a liga a outra
nativa fora de lotes transversais: os pares `overline`+`smartquote` de `a1fe997d2`
(2026-05-19) e `077792dfa` (2026-06-11) são artefacto de atribuição de fronteira (corpo de
`overline` intacto), e `lorem`+`regex`+`smartquote` de `6e29fbaba` e `c98ffc8ac` são
commits de lote. Critério 3 em vácuo → a fronteira vanilla fica.

---

## Contexto

`smartquote(...)` é a face de função da aspa tipográfica que o markup `"foo"` / `'bar'`
também produz. Não é uma decoração nem um constructor de estilo: é um **leaf** de
conteúdo cujo glifo final não é decidido aqui — depende da língua corrente e do estado
open/close do parágrafo, que vivem no Layouter (`localize_quotes`,
`Layouter.smartquote_*_open`).

Daí a assimetria que este nó tem de tornar visível: com `enabled: true` emite o variant
`Content::SmartQuote` e **delega** a escolha do glifo; com `enabled: false` resolve na
hora, emitindo `Content::Text` com o glifo ASCII literal, sem passar pelo variant — o
estado open/close do Layouter fica intacto porque o Layouter nunca vê este caso.

## Instrução

`smartquote(double: ?, enabled: ?, alternative: ?, quotes: ?)` — **zero**
posicionais; contrato proposto condicionado ao gate P1286.

| Nomeado | Tipo | Default | Semântica |
|---|---|---|---|
| `double` | `Bool` | `true` | aspa dupla (`"`) vs simples (`'`) |
| `enabled` | `Bool` | `true` | `false` → glifo ASCII literal, sem alternância lang-aware |
| `alternative` | `Bool` | `false` | seleciona o par alternativo localizado quando disponível |
| `quotes` | `Auto`, string, array ou dicionário | `auto` | override explícito single/double |

- `enabled: true` → `Content::SmartQuote` com `double` e os overrides
  explicitamente assentes; o leaf não é embrulhado em `Content::Styled`.
- `enabled: false` → `Content::text("\"")` se `double`, `Content::text("'")` caso
  contrário.
- Posicionais → `smartquote() não aceita argumentos posicionais (recebeu {n})`.
- Tipo errado em `double`/`enabled`/`alternative` é erro de bool; tipos e
  cardinalidades de `quotes` seguem as mensagens P1286 abaixo.
- Nomeado desconhecido → `smartquote(): argumento nomeado inesperado '{nome}'`.

O consumer de layout (glifo lang-aware, alternância open/close per documento) é
propriedade de `entities/content.md` e do L0 do layout de texto.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados. Em especial:
  a língua **não** é lida aqui, apesar de o glifo final depender dela. Se um passo futuro
  precisar da língua nesta função, isso é mudança de fase do pipeline (eval ↔ layout) e
  exige paragem no gate de L0 (ADR-0127).
- O estado open/close é do Layouter, nunca desta função — não introduzir contador,
  `static` ou campo de `EvalContext` para o simular (V13).

## Critérios de Verificação

```
#smartquote()                          → Content::SmartQuote { double: true }
#smartquote(double: false)             → Content::SmartQuote { double: false }
#smartquote(enabled: false)            → Content::Text("\"")
#smartquote(enabled: false, double: false) → Content::Text("'")
#smartquote(double: 1)                 → Err "smartquote(double:) espera bool, recebeu integer"
#smartquote(true)                      → Err "não aceita argumentos posicionais (recebeu 1)"
#smartquote(alternative: true)         → Content::SmartQuote com override true
#smartquote(quotes: "()")              → Content::SmartQuote com par double `(`, `)`
#smartquote(quotes: "x")               → Err "expected 2 characters, found 1 character"
#smartquote(foo: 1)                    → Err "argumento nomeado inesperado 'foo'"
```

## P1286 — `alternative` e `quotes` com carrier explícito (GATE ADR-0127)

### Medição anterior à decisão

No vanilla ratificado, `text/smartquote.rs:51-89,201-317,335-419` define
`alternative: bool = false` e `quotes: auto|string|array|dictionary`. String
conta grapheme clusters Unicode; string e array devem conter exatamente dois
itens; o dicionário aceita apenas `single` e `double`, cada qual `auto`, string
ou array. O receipt P1286 mediu alemão default `„Default“`, alternativo
`»Alt«`, override explícito `(Explicit)`, array `[[Array]]`, override
parcial de simples e as três mensagens negativas canônicas.

O cristalino já transporta deltas de `#set` em `Styles::push_custom`. Porém a
auditoria de morfologia pública refutou usar `Content::Styled` também para os
argumentos da chamada direta: `repr_content` é transparente ao wrapper em
`compiler/eval/repr.rs:681-739`, mas `content.func()` escolhe pelo
`elem_name()` externo em `compiler/eval/bindings/field_access.rs:648-677` e
exporia `styled` em vez de `smart.quote`. Não se infere transparência total de
uma só operação que por acaso elimina o wrapper.

### Decisão

- Remover o scope-out de `alternative` e `quotes`.
- Manter `double`/`enabled` e seus defaults. `alternative` aceita somente
  `Bool`, default `false`; `quotes` aceita `Auto`, string, array ou dicionário,
  default `Auto`.
- A validação/canonização pertence ao owner de quotes localizado. A chamada
  direta produz `Content::SmartQuote` e guarda somente os argumentos
  explicitamente assentes nos novos campos públicos opcionais definidos em
  `entities/elements/smartquote.md`; não cria `Content::Styled` local.
- `#set smartquote` continua a usar o carrier `Styles` existente. No layout,
  campo explícito do leaf prevalece sobre a chain, e a chain prevalece sobre o
  default. `quotes: auto` explícito precisa permanecer distinguível de campo
  omitido para poder apagar quotes herdadas.
- `enabled: false` continua a emitir ASCII imediatamente e ignora
  `alternative`/`quotes`, sem alterar o estado do quoter.
- Quotes explícitas têm precedência sobre `alternative`; membros ausentes ou
  `auto` no dicionário usam o par localizado correspondente.
- Erros observáveis: string fora de dois graphemes →
  `expected 2 characters, found {n} character{s}`; array fora de dois itens →
  `expected 2 quotes, found {n} quote{s}`; chave alheia →
  `unexpected key "{key}", valid keys are "double" and "single"`.

Esta proposta altera os campos públicos de `SmartQuoteElem`; é
**PARAGEM OBRIGATÓRIA** por contrato público no ADR-0127. Não altera default,
ordem de pipeline ou compatibilidade de chamadas já aceites. Região BCP47,
morfologia de `content.fields()` não medida e comportamento real fora da
matriz permanecem `Unknown`, nunca sucesso implícito.

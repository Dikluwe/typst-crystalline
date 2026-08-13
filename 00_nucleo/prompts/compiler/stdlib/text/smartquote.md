# Prompt L0 — `compiler/stdlib/text/smartquote` — `smartquote`
Hash do Código: 3ce868d8

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

`smartquote(double: ?, enabled: ?)` — **zero** posicionais.

| Nomeado | Tipo | Default | Semântica |
|---|---|---|---|
| `double` | `Bool` | `true` | aspa dupla (`"`) vs simples (`'`) |
| `enabled` | `Bool` | `true` | `false` → glifo ASCII literal, sem alternância lang-aware |

- `enabled: true` → `Content::smartquote(double)` (variant leaf, 1 campo `bool`).
- `enabled: false` → `Content::text("\"")` se `double`, `Content::text("'")` caso
  contrário.
- Posicionais → `smartquote() não aceita argumentos posicionais (recebeu {n})`.
- Tipo errado num nomeado → `smartquote({nome}:) espera bool, recebeu {tipo}`.
- Nomeado desconhecido → `smartquote(): argumento nomeado inesperado '{nome}'`.

**Scope-out com erro educacional** (ADR-0054 graded): `alternative` (aspas alternativas
DE/FR) e `quotes` (`Smart<SmartQuoteDict>`) respondem com mensagem que cita o scope-out —
literalmente `"smartquote({key}:) não suportado neste passo (P287 §A.2 scope-out /
ADR-0054 graded)"` — e não com "argumento inesperado" genérico. A distinção é observável na
mensagem, e por isso é aceitação ao nível da mecânica: **a mecânica é o observável**
(ADR-0108). A citação acima reproduz a mensagem visível ao autor do documento; não é
referência de legitimação (ver a nota do hub sobre estas duas mensagens).

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
#smartquote(alternative: true)         → Err com a mensagem de scope-out literal (acima)
#smartquote(quotes: "()")              → Err com a mensagem de scope-out literal (acima)
#smartquote(foo: 1)                    → Err "argumento nomeado inesperado 'foo'"
```

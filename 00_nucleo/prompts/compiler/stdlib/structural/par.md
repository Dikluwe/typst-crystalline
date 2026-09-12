# Prompt L0 — `compiler/stdlib/structural/par` — `par` e `parbreak`
Hash do Código: 7568a8bb

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/par.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/par.rs` — ficheiro próprio.

**Fronteira medida**: nó das nativas de parágrafo provenientes do mesmo arquivo
vanilla `model/par.rs`. Antes de P1140.17, `native_par` tinha **1** commit de corpo real
(`c98ffc8ac`, 2026-07-21 — o commit que a criou); o único commit que a liga a `quote` ou
`footnote` é o rename de módulo `0f5575cd0`. Critério 3 mudo, critério 4 separa — e é o
critério 4 que decide, porque o 3 está em **silêncio, não em contradição**.

> **Origem desta fronteira**: `par` vinha de um nó `flow` que agrupava `par`, `quote` e
> `footnote`. Esse agrupamento estava sustentado por um cluster que **não existia** —
> artefacto de atribuição de fronteira da ferramenta de co-mudança (em `c98ffc8ac` o único
> bloco de corpo é `@@ -655,0 +656,79 @@ pub fn native_quote(`: inserção pura de `par`
> depois de `quote`, que não mudou). Medido e desfeito em 2026-08-13.

---

## Contexto

`par(...)` é a face de função do parágrafo. O ponto que este nó tem de tornar visível é
que **não emite elemento próprio**: no cristalino o parágrafo é implícito no fluxo, e a
nativa devolve o body como está — só embrulha quando há estilo para transportar.

`parbreak()` é a face de função da quebra de parágrafo já representada por
`Content::Parbreak`. Não cria outro elemento nem outro algoritmo: devolve o mesmo marker
que `SyntaxKind::Parbreak` produz a partir de uma linha vazia em markup.

## Instrução

`par(body, leading: ?, justify: ?)` — 1 posicional `Content | Str` obrigatório.

- `Content` → devolvido **directamente** (parágrafo implícito); `Str` → convertido em
  `Content`.
- `leading: Length` → embrulha o body em `Content::Styled` com o custom de leading.
- `justify` → **aceite e ignorado**: o alinhamento justificado é decidido no layout, não
  aqui. Aceitar-e-ignorar é deliberado (paridade de superfície sem paridade de efeito), não
  omissão.
- Sem body → erro com a **mensagem literal do vanilla**; tipo errado →
  `expected content`; nomeado desconhecido → erro.

### `parbreak()` — P1140.17

**Medição anterior à decisão** — vanilla ratificado `a51e02804`,
`2026-08-24T13:20:36-03:00`, HEAD cristalino
`45b547073d7686cdd5d3e3030c82de3e22ec395f`, working tree não commitado:

```text
repr(type(parbreak))       → "function"
repr(parbreak())           → "parbreak()"
parbreak(1)                → error: unexpected argument
parbreak(foo: true)        → error: unexpected argument: foo
```

No mesmo estado, o cristalino devolve `unknown variable parbreak`. A fonte vanilla é
`lab/typst-original/crates/typst-library/src/model/par.rs:697-728`; `ParbreakElem` não
tem campos nem parâmetros.

**Decisão:** adicionar `native_parbreak() -> Content`, sem posicionais e sem nomeados.
A nativa retorna exactamente `Value::Content(Content::Parbreak)`. O binding global e
`std.parbreak` têm kind `function`. A linha vazia em markup continua pelo caminho
sintático existente; múltiplas quebras continuam a colapsar no consumer vigente.

Esta adição é o contrato público e o comportamento por defeito vigente.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados.
- A mensagem de `par()` sem body é byte-a-byte a do vanilla: aqui **a mecânica é o
  observável** (ADR-0108), logo a string conta como contrato.
- Nenhum helper privado. Se um passo futuro precisar de um parser de argumentos aqui, o L0
  reescreve-se antes do código.
- `native_parbreak` usa a validação normal de `Args`, mas não partilha parsing com
  `native_par`: as assinaturas não têm campos em comum.
- Nenhuma mudança na estrutura de `Content`, realização de parágrafos ou layout é
  autorizada por P1140.17. O `repr` canônico muda de `parbreak` para `parbreak()`
  conforme o observável vanilla medido, sem campo de origem.

## Critérios de Verificação

```
#par[x]                → Content devolvido directamente (sem elemento novo)
#par("x")              → string convertida em Content
#par(leading: 2pt)[x]  → Styled com leading custom
#par(justify: true)[x] → aceite, sem efeito
#par()                 → Err (mensagem literal do vanilla)
#par(1)                → Err "expected content"
#par(zz: 1)            → Err (nomeado desconhecido)
repr(type(parbreak))   → "function"
repr(parbreak())       → "parbreak()"
parbreak(1)            → Err "unexpected argument"
parbreak(foo: true)    → Err "unexpected argument: foo"
parbreak(); parbreak() → mesmo colapso semântico de duas linhas vazias
```

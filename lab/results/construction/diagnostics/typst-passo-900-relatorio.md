# Relatório — Passo 900: bug `Str + Content` (causa real do crash 2 catalogado em P894)

**Data:** 2026-07-25
**Commit de partida:** `013a1b951` (P899)

---

## Fase A — diagnóstico

### Caso mínimo (reconfirmado a partir de `typst-passo-894-relatorio.md`)

```typst
#let bra(x) = "⟨" + x + "|"
$ bra(phi) $
```

Erro: `cannot add string and content`. Vanilla compila sem erro, produz `⟨𝜑|`.

### Causa exacta

`eval_binary_op` (`01_core/src/engine/eval/operators.rs`), braço `BinOp::Add`: só existiam
`(Str, Str)` e `(Content, Content)`. Não é um braço com lógica errada (unwrap/panic) — é
simplesmente **falta o braço**, caindo no fallback genérico de erro de tipo (`format!("cannot add
{a} and {b}")`, linha 588) — comportamento **controlado**, não panic real de processo (mesma
observação de género já feita em P894: "crash" no vocabulário destes passos significa "não produz
PDF", não pânico).

**Achado incidental relevante**: `join()` (P728, mesmo ficheiro, usado para concatenação de
expressões dentro de `{ ... }`/`for`/`while`) **já tinha exactamente esta lógica** —
`(Str, Content)`, `(Content, Str)`, `(Symbol, Symbol)`, `(Str, Symbol)`, `(Symbol, Str)`,
`(Symbol, Content)`, `(Content, Symbol)` — nunca portada para `eval_binary_op`. `join` e o operador
`+` são funções distintas no cristalino (paralelo à distinção vanilla `ops::join` vs `ops::add`).

### Vanilla (`lab/typst-original/crates/typst-library/src/foundations/ops.rs::add`, linhas 129-138)

Confirmado por leitura directa do código-fonte vanilla: **coerção implícita**, não erro. `Str`/
`Symbol` convertem para `Content` (`TextElem::packed`/`SymbolElem::packed`) e somam como
`Content + Content`; `Symbol + Symbol`/`Symbol ± Str` produzem `Str`
(`format_str!("{a}{b}")`). As tabelas de `add` e `join` do vanilla são **idênticas** para estas 7
combinações (comparação linha a linha confirmada).

### Outras combinações com o mesmo problema (Fase A ponto 4)

Grep e comparação directa da tabela `BinOp::Add` de `eval_binary_op` contra a tabela `add()` do
vanilla confirmou 7 combinações em falta, todas envolvendo `Str`/`Symbol`/`Content` cruzados:
`Symbol+Symbol`, `Str+Symbol`, `Symbol+Str`, `Str+Content`, `Content+Str`, `Symbol+Content`,
`Content+Symbol`. Nenhuma outra família de tipos (`Int`/`Length`/`Ratio`/etc.) tem gaps
equivalentes — essas já estavam completas.

---

## Fase B — Implementação

**Decisão de âmbito**: as 7 combinações foram corrigidas juntas, não só a reportada
(`Str+Content`) — "mais consistente" per a opção que a própria materialização oferecia, escolhida
porque o risco é mínimo (lógica já existente e comprovada em `join()`, apenas portada para
`eval_binary_op`, não inventada de novo).

Novos braços em `eval_binary_op`/`BinOp::Add`, espelhando `join()`:

```rust
(BinOp::Add, Value::Symbol(a), Value::Symbol(b)) => Ok(Value::Str(format!("{}{}", a.ch, b.ch).into())),
(BinOp::Add, Value::Str(a), Value::Symbol(b))     => Ok(Value::Str(format!("{}{}", a, b.ch).into())),
(BinOp::Add, Value::Symbol(a), Value::Str(b))     => Ok(Value::Str(format!("{}{}", a.ch, b).into())),
(BinOp::Add, Value::Str(a), Value::Content(b))    => Ok(Value::Content(Content::sequence(vec![Content::text(a), b]))),
(BinOp::Add, Value::Content(a), Value::Str(b))    => Ok(Value::Content(Content::sequence(vec![a, Content::text(b)]))),
(BinOp::Add, Value::Symbol(a), Value::Content(b)) => Ok(Value::Content(Content::sequence(vec![Content::text(a.ch.to_string()), b]))),
(BinOp::Add, Value::Content(a), Value::Symbol(b)) => Ok(Value::Content(Content::sequence(vec![a, Content::text(b.ch.to_string())]))),
```

### TDD

Teste `p900_add_str_content_symbol_combinacoes` (`operators.rs`), cobrindo as 7 combinações
directamente via `eval_binary_op`, incluindo o caso mínimo exacto de P894/P900 (`Str("⟨") +
Content("x")` → `Content` com `plain_text() == "⟨x"`). Vermelho confirmado antes da implementação
(`Err("cannot add string and content")`, mensagem idêntica à catalogada em P894); verde depois.

### Suíte completa

```
typst-core:    4733 passed; 0 failed; 3 ignored
typst-infra:    734 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed
```

Zero regressões.

### `crystalline-lint`

`--fix-hashes .`: 1 ficheiro (`operators.rs`, mesmo L0 `eval/ops.md`). `crystalline-lint .`: 0
drift, só o warning pré-existente V7 (`package_version_resolution.md`, não relacionado).

### Confirmação visual

Reprodução exacta do caso mínimo original (`#let bra(x) = "⟨" + x + "|"` + `$ bra(phi) $`,
ANTES do workaround que o dono aplicou ao `.typ` de 30 secções para contornar este crash):

```
exit=0 (antes: erro de compilação)
Output: ⟨φ|  (paridade com o vanilla, que produz ⟨𝜑|)
```

**Nota sobre a secção 26 do `.typ` de 30 secções**: o dono já tinha contornado este crash
externamente (`#let bra(x) = [⟨#x\|]`, interpolação de markup em vez de concatenação de string —
registado em `typst-passo-896-relatorio.md`). A secção 26 já não exercita o caminho `Str + Content`
— por isso a confirmação visual usou o caso mínimo original directamente, reproduzido à parte,
para validar a correcção real. Hash do `.typ` de 30 secções confirmado inalterado
(`9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`) e recompilação integral
confirmada sem regressão (`exit=0`, sequência `(1)`...`(44)` completa).

## Benchmark (Fase C)

7 cenários, `hyperfine --warmup 5 -N -m 20`. Todas as leituras dentro ou abaixo da baseline
estabelecida em P897/898 (`01-hello` 94.5ms, `04-math` 156.4ms) — sem regressão, sem necessidade de
remedição isolada desta vez.

## Fora de âmbito

Nenhum achado incidental novo neste passo — a investigação foi directa e o âmbito (7 combinações de
tipo) já estava completamente delimitado pela comparação com a tabela vanilla.

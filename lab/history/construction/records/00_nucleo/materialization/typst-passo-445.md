# P445 — Smart quotes

> **Passo:** 445  
> **Data:** 2026-06-24  
> **Foco:** Materializar smart quotes (tipográficas) no lexer e no eval, convertendo `"` e `'` em `\u{201C}`/`\u{201D}` (curly double) e `\u{2018}`/`\u{2019}` (curly single) conforme contexto.  
> **Pré-requisitos:** P444 (text decorações fechado).  

---

## Contexto

O Typst vanilla converte `"` e `'` em tipográficas (curly quotes) automaticamente no lexer, com base no contexto de abertura/fechamento. O cristalino hoje trata `"` e `'` como literais rectos (straight quotes) no texto. Este passo materializa o **smart quoting** como quick win tipográfico.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| Smart quotes existem no cristalino? | Não — `"` e `'` são literais rectos | ❌ |
| Vanilla faz smart quoting no lexer? | Sim — `typst-syntax/src/lexer.rs` detecta contexto | ✅ |
| `Scanner`/`Lexer` já processa quotes? | Sim — `"` é delimitador de string em code; `'` é identificador de math | ✅ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** XS (~15 min; 2-3 arms de lexer + 1 teste + spec L0).

---

## ADR-0107 — Paridade linguagem

Contrato comportamental: `"Hello"` em markup produz `\u{201C}Hello\u{201D}` (curly double); `'Hello'` em markup produz `\u{2018}Hello\u{2019}` (curly single). Em code mode, `"` continua a ser delimitador de string (não smart quote). Em math mode, `'` continua a ser identificador de math (primes, não smart quote).

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**

1. **`rules/lexer/markup.rs`** — No braço de texto/markup, ao encontrar `"`:
   - Se o caractere anterior for whitespace ou início de texto → `"` vira `\u{201C}` (opening double).
   - Caso contrário → `"` vira `\u{201D}` (closing double).
   - Análogo para `'` → `\u{2018}`/`\u{2019}` (opening/closing single).
   - O token emitido é `Token::Text` com o caractere Unicode tipográfico em vez do literal ASCII.

2. **`rules/lexer/markup.rs`** — Edge cases:
   - `"` seguido de pontuação (`.`, `,`, `!`, `?`) → closing double.
   - `"` precedido de pontuação → opening double (raro, mas possível após `(`).
   - `'` seguido de `s` (possessivo) → `\u{2019}` (closing single/apostrophe).
   - `'` no início de palavra (`'tis`) → `\u{2019}` (apostrophe, não opening single).

3. **`rules/lexer/tests.rs`** — Teste unitário: `smart_quotes_duplas` e `smart_quotes_simples`.

4. **Spec L0** — Adicionar nota em `00_nucleo/prompts/engine/lexer/markup.md` sobre smart quoting.

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Local da conversão | Lexer (markup mode) | Paridade com vanilla; eval/layout recebem texto já tipográfico |
| Estado do lexer | `prev_was_whitespace: bool` | Flag simples no `Lexer` para decidir opening vs closing; O(1) por caractere |
| Code mode | Inalterado | `"` continua delimitador de string; smart quotes não aplicam |
| Math mode | Inalterado | `'` continua identificador de math; smart quotes não aplicam |

---

## Scope-out explícito

- Smart quotes em code mode (strings) — continua scope-out; strings são literais.
- Smart quotes em math mode — continua scope-out; math tem semântica própria para `'`.
- Configuração de locale para quotes (ex: `\u{00AB}`/`\u{00BB}` em francês) — continua scope-out; vanilla usa `"`/`'` universalmente.
- Nested quotes (`"He said 'hello' to me"`) — continua scope-out; vanilla trata cada nível independentemente (single dentro de double → `\u{2018}`/`\u{2019}`).

---

## Critério de fecho

- [ ] Lexer markup converte `"` → `\u{201C}`/`\u{201D}` conforme contexto.
- [ ] Lexer markup converte `'` → `\u{2018}`/`\u{2019}` conforme contexto.
- [ ] Code mode inalterado (`"` = delimitador de string).
- [ ] Math mode inalterado (`'` = identificador de math).
- [ ] 2 testes unitários verdes (smart quotes duplas e simples).
- [ ] Spec L0 atualizado (lexer markup).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero novas violações.

---

**Próximo passo:** Com P445 fechado, continuamos com **smallcaps** (P446, XS) ou **sub/superscript** (P447, XS-S) ou **outras features de paridade**. Indique se quer ajustar o escopo do P445.

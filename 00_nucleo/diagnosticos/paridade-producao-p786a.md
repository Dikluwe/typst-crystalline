# P786a — Erros de sintaxe descartados silenciosamente: reconfirmação, correção e validação

> **Passo:** 786a
> **Data:** 2026-07-20, medições entre ~17:55Z e ~18:23Z
> **Commit-base:** `0774275fe1b340d823e624958f66e5b6b344a51a` + **working tree não commitado** (a própria correção)
> **Ficheiros alterados (`git diff HEAD --stat`):** substantivos — `01_core/src/engine/eval/mod.rs`, `engine/eval/tests.rs` (+202), `engine/parse/markup.rs` (+37), `engine/parse/parser.rs` (+17), `entities/syntax_node.rs` (+7), `03_infra/src/integration_tests.rs` (+4), e os L0s `prompts/engine/eval.md`, `prompts/engine/parse.md`, `prompts/entities/syntax-node.md`; mais ~15 ficheiros com só a linha `@prompt-hash` actualizada por `crystalline-lint --fix-hashes`
> **Binários:** vanilla `lab/typst-original/target/release/typst` (0.15.0, rev `969087ec`); cristalino `./target/release/typst` (rebuild pós-correção)
> **ADRs:** ADR-0107 (mensagens de erro são observáveis), ADR-0108 (reconfirmar antes de aceitar), regra de proveniência P569.

---

## Resumo em uma linha

**T2 e T1 reconfirmados, corrigidos e validados: todo erro sintático do parser agora aborta a compilação (exit 1, sem PDF) e os warnings `no text within stars/underscores` chegam ao stderr com hint — paridade vanilla verificada posição a posição. As 18 falhas de teste que a propagação integral expôs eram todas documentos de teste com sintaxe inválida (o vanilla rejeita-as com as mesmas mensagens) — corrigidas as fontes, não a propagação.**

---

## 1. Passo 0 — Reconfirmação independente (sem herdar o relatório externo)

| Teste | Comando | Vanilla | Cristalino (antes) |
|---|---|---|---|
| T2 | `#let x = (` → `compile` | `error: unclosed delimiter` 1:9, exit 1 | **exit 0, PDF de 1897 bytes gerado** |
| T1 | `**` → `compile` | `warning: no text within stars` 1:0 + hint, exit 0 | exit 0, silêncio |

Achados confirmados. Bónus da sonda: `#let x = { #set text(..) [x] }` → vanilla exit 1 (`the character '#' is not valid in code` + 2 hints); cristalino exit 0 — mesma família de T2.

## 2. Mecanismo exacto (leitura de código)

`01_core/src/engine/eval/mod.rs:308-321` (pré-correção): `root.errors()` filtrado a `SyntaxErrorKind::InvalidHexNumber | InvalidUnicodeCodepoint`; todo o resto descartado. O comentário P648/P649 explicava a filtragem: P634 tentou propagar tudo e 17 testes quebraram porque "o parser assinala construções válidas como erro".

**Verificação empírica desse receio** (ADR-0108 — não aceitar o comentário de 2026 sem medir): probe com 20 construções válidas → **zero** falsos positivos no parser actual. O único caso suspeito (`#` em bloco de código) é erro genuíno — o vanilla rejeita-o igualmente. Os falsos positivos da era P634 já não existem.

## 3. Correção

1. **`eval/mod.rs`**: propagação integral — todo `root.errors()` vira `SourceDiagnostic::error` fatal (com hints via `with_hint`), **excepto** `NoTextWithinStars`/`NoTextWithinUnderscores`, que seguem para o `sink` como warning (não abortam).
2. **`entities/syntax_node.rs`**: `SyntaxErrorKind` ganha as 2 variantes de warning.
3. **`engine/parse/markup.rs`** (`strong`/`emph`): port da regra vanilla (`parser.rs:137-168`): delimitador fechado + conteúdo vazio → error node zero-width com kind de warning + hint vanilla. **Bug encontrado e corrigido em validação**: a medição de comprimento tem de acontecer *antes* de comer o delimitador de fecho, porque o `eat` arrasta a trivia seguinte (medido: `**\n` ficava com len 3 e o warning nunca disparava em ficheiros reais).
4. **`engine/parse/parser.rs`**: helpers `text_len_since` e `warn_at`.
5. **L0s actualizados primeiro** (Protocolo de Nucleação): `eval.md` §P786a, `parse.md` §P786a, `syntax-node.md` §P786a + `--fix-hashes`.

## 4. As 18 falhas — investigadas uma a uma

A propagação integral quebrou 17 testes em `typst-core` + 1 em `03_infra` — o mesmo número de P634, mas a causa verificada é outra. **Todas** as fontes foram submetidas ao vanilla 0.15.0:

| Grupo | Testes | Fonte | Vanilla 0.15.0 | Resolução |
|---|---|---|---|---|
| p635 | 7 | `{ return "a" "b" }`, `{ break } str(i)`, etc. | exit 1 `expected semicolon or line break` | fontes corrigidas com `;`/newline (validadas: exit 0); asserções inalteradas |
| smart quotes | 3 | `#set text(lang:"en") "Hello,"` | exit 1 `expected semicolon or line break` | quebra de linha após o `#set`; asserções inalteradas |
| set scoping | 5+1 | `#{ #set .. }`, `[#set .. x]` | exit 1 `` `#` not valid in code `` / `expected semicolon or line break` | `set` sem `#`; forma validada no vanilla; asserções inalteradas |
| show rule | 1 | `#{ #show "A": "B" }` | exit 1 `` `#` not valid in code `` | `show` sem `#` |
| p648 hex | 1 | `#let x = 0xZZ` | 1 erro: `invalid hexadecimal number` | cristalino emite 2 erros (cascata "expected expression" + hex); asserção alargada de `first()` para `any()` — **divergência de contagem de erros em cascata registada** (vanilla suprime; candidata a passo próprio) |

Nenhuma falha era regressão: eram documentos que "funcionavam" só porque o bug descartava os erros — exactamente o caso previsto no passo.

## 5. Validação final (binário release, comandos do passo)

```text
$ ./target/release/typst /tmp/p786a-unclosed.typ /tmp/out.pdf
/tmp/p786a-unclosed.typ:1:9: error: unclosed delimiter        exit 1 — nenhum PDF gerado ✓
$ ./target/release/typst /tmp/p786a-stars.typ
warning: no text within stars @1:0 + hint "using multiple consecutive stars..."  exit 0 ✓
```

Paridade posicional verificada contra o vanilla (todas com hint idêntico): `**`/EOF → 1:0; `**\n` → 1:0; `** texto` → 1:0; `** e **` → 1:0 + 1:5 (vanilla: idem, 2 warnings); `***txt***` → 1:0 + 1:7 (vanilla: idem); `* *` → sem warning em ambos; `__\n` → `no text within underscores`; `#` em código → erro + **2 hints do vanilla propagados** (`you are already in code mode`, `try removing the '#'`).

- `cargo test --workspace`: **verde** (4266 testes typst-core — 4259 + 7 novos P786a — e restantes suites), exit 0.
- `crystalline-lint .`: **zero violações** (2 warnings V7 pré-existentes).

## 6. Registos para passos futuros

1. **Cascata de erros**: `0xZZ` emite "expected expression" + "invalid hexadecimal number" (vanilla: 1 erro). Supressão de cascata (`after_error`/`trim_errors` não cobrem o caso do token léxico inválido).
2. **Wording**: mensagem de hex sem backticks (`` `0xZZ` `` no vanilla).
3. **Span zero-width dos warnings**: o warning de `**` aponta 1:0 sem largura; o vanilla cobre `^^` (1:0–1:2). Posição idêntica, largura difere.
4. **T4/T6 do relatório de reverificação P785** (fora deste passo): mensagem enganadora para main não-UTF8; spans `<detached>` em `eval()`.

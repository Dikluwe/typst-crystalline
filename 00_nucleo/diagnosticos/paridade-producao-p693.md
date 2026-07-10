# Paridade em produção — P693 — `str.match()` aceita `str | regex`

**Passo:** P693 (`00_nucleo/materialization/typst-passo-693.md`)
**Data:** 2026-07-10
**Commit deste relatório:** `7a8eaf2c6cbc4d1f0a980b52518a7e39ae0c0717` (preenchido no 2.º commit)
**ADRs:** ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir),
regra P662-P664 (nome igual ao vanilla obriga a comportamento igual).
**Dependências:** P689 (`match()` original), P692 (`matches()` e o helper `match_dict`).

---

## 0. Proveniência das medições (regra de P569)

- **Commit em que os testes/sonda foram corridos:** `cc0ef7edcf9e27816ff90f8012863116187d22f7`
  (HEAD de P692; trabalho em detached HEAD sobre este commit).
- **Working tree na medição:** alterações não commitadas em 2 ficheiros tracked:
  `00_nucleo/prompts/rules/stdlib/collections.md`, `01_core/src/rules/stdlib/collections.rs`.
  `git diff --stat` no momento da medição: `2 files changed, 58 insertions(+), 8 deletions(-)`.
- **Hora da validação final:** 2026-07-10T23:27:23Z (`date -u`).
- **Binários:** vanilla 0.15.0 (`lab/typst-original/target/release/typst`, commit 969087ec);
  cristalino (`target/debug/typst`, recompilado com `cargo build` após a edição); extracção
  com `pdftotext -layout`.
- **Scratch:** `temp_p693/` (dentro do repo), apagado no fim.

---

## 1. Resumo

`str.match()` (singular, P689) só aceitava `regex`; o vanilla aceita `str | regex`.
Alargado para `str | regex`, reaproveitando o helper `match_dict` e o padrão de
`str_matches` (P692). Fecha a cadeia de correcções de `str` (P689–P693): todos os métodos
oficiais implementados, sem variação de assinatura conhecida por corrigir.
`cargo test --workspace` 4378 passed / 0 failed; `crystalline-lint .` 0 violations.

---

## 2. Sonda mínima (medida antes de decidir — ADR-0108)

Vanilla 0.15.0 (`temp_p693/sonda_match.typ`):

| expr | vanilla |
|------|---------|
| `("abcabc").match("bc")` | `(start: 1, end: 3, text: "bc", captures: ())` |
| `("abc").match("z")` | `none` |
| `("abcabc").match(regex("bc"))` | `(start: 1, end: 3, text: "bc", captures: ())` |
| `("abc").match("")` | `(start: 0, end: 0, text: "", captures: ())` |

Confirma: com `str`, `match` devolve o mesmo dict de `match(regex)` e `captures` fica
**vazio** (como em `matches()` com string, P692); string vazia matcheia no início
(0..0); não encontrado → `none`. Concorda com a fonte (`foundations/str.rs:456`:
`StrPattern::Str(pat) => self.0.match_indices(pat).next().map(match_to_dict)`).

---

## 3. Implementação

`01_core/src/rules/stdlib/collections.rs::str_match`: acrescentado o braço `Value::Str`
(espelho de `str_matches`), reaproveitando `match_dict` (P692):

```rust
[Value::Str(pat)] => Ok(s
    .match_indices(pat.as_str())
    .next()
    .map(|(i, m)| match_dict(i, i + m.len(), m, vec![]))
    .unwrap_or(Value::None)),
[Value::Regex(re)] => Ok(re
    .captures_first(s.as_str())
    .map(|m| match_dict(m.start, m.end, &m.text, m.captures))
    .unwrap_or(Value::None)),
[other] => Err(... "str.match() espera str ou regex, recebeu {other}"),
_       => Err(... "str.match() requer 1 argumento posicional"),
```

`00_nucleo/prompts/rules/stdlib/collections.md`: linha de `match` actualizada
(`str | regex`; nota P693; débito P692 marcado como resolvido); cabeçalho actualizado
(fecho da cadeia). `crystalline-lint --fix-hashes .` → "Nothing to fix".

---

## 4. Testes

2 testes unitários novos em `collections.rs::tests` (P693) + 1 teste P689 actualizado:

- `p693_str_match_str_literal` — `"abcabc".match("bc")` → dict (start 1, end 3, "bc",
  captures vazio); `"abc".match("z")` → `none`; `"abc".match("")` → (0, 0, "").
- `p693_str_match_regex_sem_regressao` — regex com capturas continua a funcionar;
  `Int` (tipo errado) → erro.
- `p689_str_position_match_tipo_errado` (P689) **actualizado**: o assert "match com Str →
  erro" deixou de ser válido (match agora aceita str); substituído por "match com Int →
  erro", preservando o propósito (tipos errados rejeitados) sob a nova assinatura.

`cargo test --workspace` (2026-07-10T23:27:23Z): **4378 passed / 0 failed**
(3709 typst-core + 610 + 28 + 2 + 27 + 2; +2 relativamente a P692).
`crystalline-lint .`: **0 violations**.

---

## 5. Validação end-to-end

`temp_p693/sonda_match.typ` compilado nos dois motores após a mudança — saídas idênticas:

| expr | vanilla | cristalino depois |
|------|---------|-------------------|
| `match("bc")` | `(1, 3, "bc", ())` | `(1, 3, "bc", ())` |
| `match("z")` | `none` | `none` |
| `match(regex("bc"))` | `(1, 3, "bc", ())` | `(1, 3, "bc", ())` |
| `match("")` | `(0, 0, "", ())` | `(0, 0, "", ())` |

`matches()` (P692) e `match(regex)` (P689) sem regressão (workspace verde).

---

## 6. Fecho da cadeia P689–P693

Com este passo, a revisão de paridade dos métodos de `str` fica **fechada**:

| Passo | Correcção |
|-------|-----------|
| P689 | `codepoints`, `position`, `match` (regex) — índices em bytes |
| P690 | `len`/`at`/`slice` → bytes; `char-len`/`char-at`/`char-slice` (extensão) |
| P691 | `find` → substring/`none` (tipo de retorno) |
| P692 | `matches` e `normalize` (ausentes); `unicode-normalization` em L1 |
| P693 | `match` → `str \| regex` (assinatura) |

Estado final: todos os métodos de instância oficiais de `str` implementados e paritários
com o vanilla; as únicas diferenças remanescentes são extensões cristalinas não-portáveis
registadas (`char-*`, `repeat`, `to-upper`, `to-lower`) e a diferença de apresentação do
`repr` para combining characters (P692) — nenhuma delas é uma variação de assinatura por
corrigir.

---

## 7. Critério de fecho do passo

- [x] Sonda mínima completa.
- [x] `match()` aceita `str | regex`, testado contra o vanilla.
- [x] `matches()` e `match(regex)` sem regressão.
- [x] Sem regressão em `cargo test --workspace` (4378/0).
- [x] `crystalline-lint .` limpo (0 violations).
- [x] Cadeia P689–P693 declarada fechada (§6).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p693.md`, com hash do commit.

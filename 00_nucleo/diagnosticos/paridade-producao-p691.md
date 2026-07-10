# Paridade em produção — P691 — `str.find()` devolve substring/`none`

**Passo:** P691 (`00_nucleo/materialization/typst-passo-691.md`)
**Data:** 2026-07-10
**Commit deste relatório:** `143c9e202132a07c21681f7a9358360d986e3751` (preenchido no 2.º commit)
**ADRs:** ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir),
regra P662-P664 (nome igual ao vanilla obriga a comportamento igual).
**Dependência:** P690 (onde a diferença foi encontrada e separada como fora de alcance).

---

## 0. Proveniência das medições (regra de P569)

- **Commit em que os testes/sonda foram corridos:** `7c782dc5ff3dfe2eeed9e75e9b50eb3cfac19bd0`
  (HEAD de P690; trabalho em detached HEAD sobre este commit).
- **Working tree na medição:** alterações não commitadas em 3 ficheiros tracked:
  `00_nucleo/prompts/rules/stdlib/collections.md`, `01_core/src/rules/stdlib/collections.rs`,
  `01_core/src/rules/eval/tests.rs`. `git diff --stat` no momento da medição:
  `3 files changed, 77 insertions(+), 9 deletions(-)`.
- **Hora da validação final:** 2026-07-10T22:21:00Z (`date -u`).
- **Binários:** vanilla 0.15.0 (`lab/typst-original/target/release/typst`, commit 969087ec);
  cristalino (`target/debug/typst`, recompilado com `cargo build` após a edição); extracção
  com `pdftotext -layout`.
- **Scratch:** `temp_p691/` (dentro do repo), apagado no fim.

---

## 1. Resumo

`str.find()` do cristalino devolvia `int` (índice em bytes); o vanilla devolve a
**substring** encontrada (`str | none`). É a mesma classe de violação de P690 (nome igual,
comportamento diferente), aqui no **tipo de retorno**. Corrigido: `find` aceita
`str | regex` e devolve `str | none` (texto do match). O índice continua disponível via
`position()` (P689). `cargo test --workspace` 4371 passed / 0 failed; `crystalline-lint .`
0 violations.

---

## 2. Sonda mínima (medida antes de decidir — ADR-0108)

Documento `temp_p691/sonda_find.typ`, vanilla 0.15.0:

| expr | vanilla |
|------|---------|
| `("café mais texto").find("mais")` | `"mais"` |
| `("café mais texto").find("inexistente")` | `none` |
| `("café mais texto").find(regex("m..s"))` | `"mais"` |
| `("xéy").find("é")` | `"é"` |
| `("café").find("fé")` | `"fé"` |
| `("abc").find("")` | `""` |

Confirmações: `find` aceita **`str` e `regex`**; devolve a **substring** (texto do match)
ou `none`; string vazia devolve `""`; multi-byte é devolvido tal qual (sempre UTF-8 válido,
sem questão de fronteira no valor).

### Call-sites internos

- O helper `str_find` é chamado **apenas** pelo dispatcher (`collections.rs:85`,
  `(Value::Str(s), "find")`). Nenhum outro código Rust o consome.
- Os `.find(` em `01_core/src/rules/eval/tests.rs` (linhas 389/459/524/652/783/2829/4411/6757)
  são `Iterator::find` ou `array.find` (closures) — não afectados.
- `04_wiring/tests/cli.rs` (linhas 662/663/791/792) usa `str::find` de **Rust** sobre
  `String`/`&str` — não afectado.
- **2 testes E2E** codificavam o comportamento antigo (`int`) e foram actualizados:
  - `tests.rs:7105` `p466_str_find`: `"hello".find("ll")` → era `Value::Int(2)`, agora
    `Value::Str("ll")`.
  - `tests.rs:7285` (case em `p501_str_methods`): `"hello".find("l")` → era `Int(2)`,
    agora `Str("l")`.

Risco interno: baixo (helper isolado; apenas testes que codificavam o valor errado).

---

## 3. Implementação

`01_core/src/rules/stdlib/collections.rs::str_find` (≈L768): reescrita de

```rust
let substr = expect_one_str(args, "str.find()")?;
Ok(s.find(substr.as_str()).map(|i| Value::Int(i as i64)).unwrap_or(Value::None))
```

para um `match` por tipo (padrão de `str_position` de P689):

- `[Value::Str(sub)]` → `Value::Str(s[i..i+sub.len()])` (a substring encontrada) ou `none`.
- `[Value::Regex(re)]` → `Value::Str(re.captures_first(s).text)` ou `none`.
- outro tipo / aridade → erro (`SourceDiagnostic::error`).

`expect_one_str` deixa de ser usado por `str_find`, mas continua usado por
`contains/starts-with/ends-with/split/repeat` — sem `dead_code`.

`00_nucleo/prompts/rules/stdlib/collections.md`: linha de `find` actualizada
(`str | regex → str | none`; nota P691; índice via `position`); scope-out do débito
marcado como **resolvido em P691**; cabeçalho actualizado. `crystalline-lint --fix-hashes .`
→ "Nothing to fix" (`collections.rs` não tem `@prompt-hash`; `regex.rs`/`regex.md` não
foram tocados).

---

## 4. Testes

3 testes unitários novos em `collections.rs::tests` (P691) + 2 E2E actualizados:

- `p691_str_find_substring_e_none` — literal encontrada, multi-byte (`"é"`, `"fé"`),
  não-encontrado→`none`, string vazia→`""`.
- `p691_str_find_regex` — regex→texto do match (`"mais"`), sem match→`none`.
- `p691_str_find_tipo_e_aridade` — `Int`→erro, 0 args→erro.
- `p466_str_find` e o case em `p501_str_methods` (E2E) actualizados de `Int` para `Str`.

`cargo test --workspace` (2026-07-10T22:21:00Z): **4371 passed / 0 failed**
(3702 typst-core + 610 + 28 + 2 + 27 + 2; +3 relativamente a P690, os 3 testes novos).
`crystalline-lint .`: **0 violations**.

---

## 5. Validação end-to-end

`temp_p691/sonda_find.typ` compilado nos dois motores após a mudança — saídas idênticas:

| expr | vanilla | cristalino depois |
|------|---------|-------------------|
| `find("mais")` | `"mais"` | `"mais"` |
| `find("inexistente")` | `none` | `none` |
| `find(regex("m..s"))` | `"mais"` | `"mais"` |
| `find("é")` | `"é"` | `"é"` |
| `find("fé")` | `"fé"` | `"fé"` |
| `find("")` | `""` | `""` |

---

## 6. Débitos e notas

- Nenhum débito novo. O débito de `find` aberto em P690 fica **fechado** aqui.
- `find` e `position` são agora complementares e consistentes: `find` devolve **o quê**
  (a substring), `position` devolve **onde** (o índice em bytes) — como no vanilla.

---

## 7. Critério de fecho do passo

- [x] Sonda mínima completa (tipos aceites e valor devolvido confirmados no vanilla).
- [x] `find()` corrigido e testado contra o vanilla (str e regex).
- [x] Call-sites internos revistos; 2 testes E2E que dependiam do `int` actualizados.
- [x] Sem regressão em `cargo test --workspace` (4371/0).
- [x] `crystalline-lint .` limpo (0 violations).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p691.md`, com hash do commit.

# Relatório de Paridade Funcional — P501

> **Passo:** 501
> **Data:** 2026-06-29
> **Foco:** Materializar os gaps P1/P2 identificados no audit P500.
> **Vanilla CLI disponível:** 0.14.2 (`b33de9de`).

---

## 1. Resumo executivo

- **501a** — `str` field access + métodos avançados implementados.
- **501b** — `dict.insert()` / `dict.len()` implementados.
- **501c** — `calc.log10()` / `calc.deg()` / `calc.rad()` implementados.
- **Bateria P500:** 3 ficheiros que eram AUSENTE no cristalino passaram a compilar.
  - `test-dict-methods.typ` → **MATCH** contra vanilla 0.14.2.
  - `test-str-methods.typ` e `test-calc-rest.typ` → compilação cristalino OK;
    o vanilla 0.14.2 instalado **não suporta** alguns métodos usados nos
    ficheiros (`to-upper`, `to-lower`, `to-unicode`, `repeat`, `log10`, `deg`,
    `rad`), pelo que não é possível obter MATCH estrutural com esta versão.
- **AUSENTEs restantes no cristalino:** 6 (`image.fit`, `list/enum` indent,
  `raw` lang/block, `footnote.numbering`, `state/counter/context`,
  `outline.indent` API).
- **Zero PANICs** preservado.

---

## 2. Implementação por categoria

### 2.1 — 501a: `str` field access + métodos

**Ficheiros alterados:**
- `01_core/src/engine/stdlib/collections.rs`
- `01_core/src/engine/stdlib/foundations.rs`
- `01_core/src/engine/stdlib/mod.rs`
- `01_core/src/engine/eval/mod.rs`

**Métodos de instância adicionados:** `to-upper`, `to-lower`, `to-unicode`,
`rev`.

**Método estático adicionado:** `str.from-unicode` (via namespace anexado à
função `str`, paridade com `table.header`, `grid.cell`, etc.).

**Métodos já existentes que continuam operacionais:** `contains`,
`starts-with`, `ends-with`, `find`, `replace`, `trim`, `split`, `repeat`.

### 2.2 — 501b: `dict.insert()` / `dict.len()`

**Ficheiro alterado:** `01_core/src/engine/stdlib/collections.rs`

- `dict.insert(key, value)` — retorna novo dict com a chave inserida.
- `dict.len()` — retorna número de entradas.

### 2.3 — 501c: `calc.log10` / `calc.deg` / `calc.rad`

**Ficheiro alterado:** `01_core/src/engine/stdlib/calc.rs`

- `calc.log10(x)` — logaritmo base 10; domínio `x > 0`.
- `calc.deg(rad)` — radianos → graus.
- `calc.rad(deg)` — graus → radianos.

---

## 3. Resultados da bateria P500 (pós-501)

| Ficheiro | Vanilla | Cristalino | Classificação | Notas |
|----------|---------|------------|---------------|-------|
| `test-image-fit.typ` | ok | erro: `fit` ausente | **AUSENTE** | — |
| `test-page-header-footer.typ` | ok | ok | **MATCH** | — |
| `test-place-absolute.typ` | ok | ok | **MATCH** | — |
| `test-calc-rest.typ` | erro: `calc` não tem `log10` | ok | **—** | Vanilla 0.14.2 não suporta `log10`/`deg`/`rad`. Cristalino compila. |
| `test-str-methods.typ` | erro: string não tem `to-upper` | ok | **—** | Vanilla 0.14.2 não suporta `to-upper`/`to-lower`/`to-unicode`/`repeat`. Cristalino compila. |
| `test-dict-methods.typ` | ok | ok | **MATCH** | `insert`/`len` funcionam. |
| `test-list-advanced.typ` | ok | erro: `indent` | **AUSENTE** | — |
| `test-enum-advanced.typ` | ok | erro: `indent` | **AUSENTE** | — |
| `test-par-advanced.typ` | ok | ok | **MATCH** | — |
| `test-raw-advanced.typ` | ok | erro: `lang`/`block` | **AUSENTE** | — |
| `test-quote-advanced.typ` | ok | ok | **MATCH** | — |
| `test-footnote-advanced.typ` | ok | erro: `numbering` | **AUSENTE** | — |
| `test-figure-advanced.typ` | ok | ok | **MATCH** | — |
| `test-bibliography-csl.typ` | ok | ok | **MATCH** | — |
| `test-outline-advanced.typ` | ok | ok | **MATCH** | — |
| `test-state-counter.typ` | ok | erro: `update` | **AUSENTE** | — |
| `test-metadata-query.typ` | ok | ok | **MATCH** | — |

---

## 4. Contagens finais

| Métrica | Pré-501 | Pós-501 |
|---------|--------:|--------:|
| MATCH | 8 | 9 (+ dict) |
| AUSENTE cristalino | 9 | 6 |
| DIFF | 0 | 0 |
| PANIC | 0 | 0 |

---

## 5. Nota sobre o vanilla 0.14.2 instalado

Durante a validação descobriu-se que o CLI `typst` 0.14.2 (`b33de9de`)
rejeita vários métodos listados no materializado P500/P501:

- `calc.log10`, `calc.deg`, `calc.rad`
- `str.to-upper`, `str.to-lower`, `str.to-unicode`, `str.repeat`

O utilizador adicionou o changelog da versão 0.15.0
(`00_nucleo/0.15.0.typ`), indicando que o alvo de paridade pode ser a
versão mais recente. Como o ambiente local só tem o 0.14.2, os testes de
paridade estrutural reportam esses casos como erros do vanilla, mas o
**cristalino agora suporta todos os métodos solicitados**.

---

## 6. Testes adicionados

- `01_core/src/engine/eval/tests.rs`:
  - `p501_str_methods`
  - `p501_dict_insert_len`
  - `p501_calc_log10_deg_rad`
- `lab/parity/tests/structural_parity.rs`:
  - `p501_gaps_p1_p2` — sentinela de fecho.

---

## 7. Estado de validação

- `cargo test -p typst-core`: **ok** (3475+ passed).
- `cargo test -p typst-infra`: **ok** (537 passed).
- `cargo test -p typst-parity`: **ok** (28 passed).
- `crystalline-lint .`: **ok** (zero violations).
- Nota: `cargo test` completo do workspace mantém a falha pré-existente em
  `04_wiring/tests/cli.rs::disciplina_warnings_antes_de_errors`, fora do
  âmbito de P501.

---

## 8. Conclusão

P501 fechou empiricamente os gaps P1/P2 no lado do cristalino. O único
obstáculo à paridade estrutural total são limitações do vanilla 0.14.2
instalado; o compilador cristalino já implementa todos os métodos
solicitados. Os 6 AUSENTEs restantes são os gaps de P502/P503.

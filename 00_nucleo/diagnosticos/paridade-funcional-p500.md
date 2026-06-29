# Relatório de Paridade Funcional — P500

> **Passo:** 500
> **Data:** 2026-06-29
> **Foco:** Audit de cobertura stdlib expandida do vanilla 0.14.2.
> **Metodologia:** `typst query --format json` vs `query_to_summary` (cristalino),
> classificação ADR-0054 (`MATCH` / `DIFF` / `ERRO_DESCRITIVO` / `PANIC` / `AUSENTE`).
> **Vanilla:** 0.14.2 (`b33de9de`).

---

## 1. Resumo executivo

- **17 ficheiros** de teste criados em `lab/parity/corpus/p500/`.
- **Zero PANICs** no cristalino — nenhum crash prioritário para P501.
- **8 MATCH**, **9 AUSENTE**, **0 DIFF**.
- Os AUSENTEs são quase todos **um único argumento/método ausente** numa feature
  que, de resto, já funciona. Os gaps são pequenos e bem delimitados.

---

## 2. Ajustes aos ficheiros de teste

Dois snippets do materializado `typst-passo-500.md` não são válidos para o
vanilla 0.14.2 instalado. Foram ajustados para permitir a comparação:

| Ficheiro | Ajuste | Motivo |
|----------|--------|--------|
| `test-bibliography-csl.typ` | Removido `locale: "en-US"` | Vanilla 0.14.2 rejeita `locale` como argumento inesperado. |
| `test-outline-advanced.typ` | `indent: true` → sem `indent` | Vanilla 0.14.2 espera `length`, `function` ou `auto`; cristalino espera `bool`. Divergência de API documentada na secção 4. |

---

## 3. Resultados por ficheiro

| Ficheiro | Vanilla | Cristalino | Classificação | Notas |
|----------|---------|------------|---------------|-------|
| `test-image-fit.typ` | ok | erro: `image(): argumento nomeado inesperado 'fit'` | **AUSENTE** | `width` funciona; `fit` ainda não implementado. |
| `test-page-header-footer.typ` | ok (count=0) | ok (count=0) | **MATCH** | Compila sem erros; header/footer/context não são locatable. |
| `test-place-absolute.typ` | ok (count=0) | ok (count=0) | **MATCH** | Compila sem erros; `place` não é locatable. |
| `test-calc-rest.typ` | ok | erro: `campo 'log10' não existe` | **AUSENTE** | `calc.log10`, `calc.deg` e `calc.rad` ausentes; restantes funções testadas individualmente funcionam. |
| `test-str-methods.typ` | ok | erro: `field access não suportado em str` | **AUSENTE** | Field access em `str` não existe; todos os métodos listados estão por implementar. |
| `test-dict-methods.typ` | ok | erro: `campo 'insert' não existe` | **AUSENTE** | `keys()`, `values()`, `pairs()`, `remove()` funcionam; `insert()` e `len()` ausentes. |
| `test-list-advanced.typ` | ok | erro: `argumento nomeado inesperado 'indent'` | **AUSENTE** | `marker: Array` e listas aninhadas funcionam; `indent`, `body-indent`, `tight` ausentes. |
| `test-enum-advanced.typ` | ok | erro: `argumento nomeado inesperado 'indent'` | **AUSENTE** | `start`, `numbering` e enums aninhados funcionam; `indent`, `body-indent`, `tight` ausentes. |
| `test-par-advanced.typ` | ok (count=1) | ok (count=1) | **MATCH** | Compila; `spacing`, `justify`, `first-line-indent`, `linebreaks`, `hanging-indent` emitem warnings de scope-out. |
| `test-raw-advanced.typ` | ok | erro: `argumento nomeado inesperado: 'lang'` | **AUSENTE** | Triple-backtick funciona (P490); `#raw(lang:, block:)` ainda não aceita argumentos. |
| `test-quote-advanced.typ` | ok (count=1) | ok (count=1) | **MATCH** | `attribution`, `block`, `quotes` funcionam. |
| `test-footnote-advanced.typ` | ok | erro: `footnote(): argumento nomeado 'numbering' não suportado` | **AUSENTE** | Corpo de footnote funciona; `numbering` é scope-out/cosmético. |
| `test-figure-advanced.typ` | ok (count=1) | ok (count=1) | **MATCH** | `caption`, `kind`, `supplement`, `numbering` funcionam. |
| `test-bibliography-csl.typ` | ok (count=1) | ok (count=1) | **MATCH** | `style: "ieee"` funciona; `locale` não existe em 0.14.2. |
| `test-outline-advanced.typ` | ok (count=1) | ok (count=1) | **MATCH** | `title`, `depth` funcionam; `indent` diverge de API. |
| `test-state-counter.typ` | ok | erro: `campo 'update' não existe` | **AUSENTE** | `state(...)` existe, mas `state.update/get` e `context` não produzem resultado; `counter` é variável desconhecida. |
| `test-metadata-query.typ` | ok (count=1) | ok (count=1) | **MATCH** | `metadata()` com label e `query(<tag>)` funcionam. |

---

## 4. Descobertas detalhadas

### 4.1 — `image` avançado
- `width: 50%` funciona.
- `fit: "contain" | "cover" | "stretch"` ainda não implementado.

### 4.2 — `page` / `place`
- Ambos compilam sem erros no cristalino.
- Como não são locatable, este passo não mediu o comportamento de render — apenas
  ausência de PANIC e de erros de parse.

### 4.3 — `calc` restante
Teste individual de 21 funções:

| Função | Estado |
|--------|--------|
| `abs`, `sin`, `cos`, `tan`, `pow`, `sqrt`, `floor`, `ceil`, `min`, `max`, `rem`, `quo`, `atan`, `asin`, `acos`, `exp`, `ln`, `clamp` | OK |
| `log10`, `deg`, `rad` | **AUSENTE** |

### 4.4 — `str` métodos avançados
Field access a `str` ainda não é suportado. Todos os métodos listados
(`to-upper`, `to-lower`, `split`, `trim`, `replace`, `to-unicode`,
`from-unicode`, `contains`, `starts-with`, `ends-with`, `find`, `rev`,
`repeat`) estão bloqueados por esta infraestrutura.

### 4.5 — `dict` métodos avançados
Teste individual:

| Método | Estado |
|--------|--------|
| `keys()`, `values()`, `pairs()`, `remove("a")` | OK |
| `insert()`, `len()` | **AUSENTE** |

### 4.6 — `list` / `enum` avançados
- `marker: Array`, listas aninhadas, `start`, `numbering` funcionam.
- `indent`, `body-indent`, `tight` ainda não implementados em ambos.

### 4.7 — `par` avançado
- Compila e é locatable.
- `spacing`, `justify`, `first-line-indent`, `linebreaks`, `hanging-indent`
  são ignorados com warning (scope-out documentado).

### 4.8 — `raw` avançado
- Triple-backtick com linguagem funciona (P490).
- A função `#raw(..., lang:, block:)` ainda não aceita argumentos nomeados.

### 4.9 — `footnote` avançado
- Corpo de footnote funciona.
- `numbering:` é scope-out/cosmético.

### 4.10 — `figure` avançado
- `caption`, `kind`, `supplement`, `numbering` funcionam.

### 4.11 — `bibliography` / `outline`
- Bibliography com `style: "ieee"` funciona; `locale` não é parâmetro do 0.14.2.
- Outline `title`/`depth` funciona.
- **Divergência de API:** cristalino implementou `outline(indent: bool)` enquanto
  vanilla 0.14.2 espera `length | function | auto`. Requer decisão arquitetural
  em P501+ se se quer alinhar com vanilla.

### 4.12 — `state` / `counter` / `context`
- `state(...)` existe.
- `state.update()`, `state.get()`, `context` e `counter(...)` ainda não
  funcionam/estão ausentes.

### 4.13 — `metadata` / `query`
- `metadata()` com label e `query(<label>)` funcionam corretamente.

---

## 5. Lista priorizada de gaps para P501+

| Prioridade | Gap | Impacto | Tamanho estimado |
|------------|-----|---------|------------------|
| **P1** | `str` field access + métodos | Alto — muitos usos de manipulação textual | M |
| **P1** | `dict.insert()` / `dict.len()` | Alto — mutação e introspecção de dicionários | S |
| **P2** | `calc.log10`, `calc.deg`, `calc.rad` | Baixo — adições numéricas triviais | XS |
| **P2** | `image.fit` | Médio — layout de imagens comum | S |
| **P2** | `list`/`enum` `indent`, `body-indent`, `tight` | Médio — layout de listas | M |
| **P2** | `#raw(lang:, block:)` | Médio — blocos de código programáticos | S |
| **P3** | `footnote(numbering:)` | Baixo — cosmético | XS |
| **P3** | `state.update/get` + `context` + `counter` | Alto — mas envolve runtime state; maior que P501 | L |
| **P3** | Alinhar API `outline.indent` com vanilla | Baixo — decisão de design | S |

---

## 6. Estado de validação

- `cargo test -p typst-parity p500_audit -- --nocapture`: **ok** (zero PANICs).
- `cargo test -p typst-parity`: **ok** (25 passed; sentinelas P206D actualizadas para corpus de 46 ficheiros).
- `cargo build -p typst-wiring`: **ok**.
- `crystalline-lint .`: **ok** (zero violations).
- Nota: `cargo test` completo do workspace tem uma falha pré-existente em `04_wiring/tests/cli.rs::disciplina_warnings_antes_de_errors`, fora do âmbito de P500.

---

## 7. Conclusão

P500 cumpriu o objetivo: descobrir empiricamente o que falta validar na stdlib
expandida. A bateria P490 cobria os gaps conhecidos; este audit revelou **9
AUSENTEs pequenos e bem delimitados**, nenhum PANIC. O trabalho seguinte
(P501) pode começar pelos gaps de maior impacto (`str`, `dict.insert/len`) ou
pelos trivialmente resolúveis (`calc.log10/deg/rad`).

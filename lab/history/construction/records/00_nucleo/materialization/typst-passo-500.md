---

# P500 — Audit de Cobertura Stdlib Expandida

> **Passo:** 500
> **Data:** 2026-06-29
> **Foco:** Descobrir empiricamente que funcionalidades stdlib do vanilla 0.14.2 ainda não foram validadas pela bateria P490. Não declarar conclusão — medir o que falta.
> **Tipo:** Diagnóstico empírico via novos ficheiros `.typ` de teste.
> **Tamanho:** M (~45 min de sondas).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P498 (bateria P490 fechada em 20/20 MATCH).

---

## 1. Contexto

A bateria P490 validou 20 ficheiros `.typ` e atingiu **20/20 MATCH**. No entanto, esses 20 ficheiros foram desenhados para cobrir os **gaps identificados no diagnóstico inicial** (D1–D5), não para cobrir a **stdlib completa** do vanilla 0.14.2.

Este passo cria um **segundo corpus** de ficheiros `.typ` para funcionalidades que **nunca foram testadas** na bateria P490, descobrindo novos gaps antes de declarar o projeto completo.

---

## 2. Metodologia

Para cada categoria de funcionalidade não coberta, criar um ficheiro `.typ` de teste, correr contra vanilla 0.14.2 e contra o cristalino, e classificar o resultado.

```bash
# Vanilla:
typst query --format json documento.typ "selector"

# Cristalino (via helper L3):
cargo run -p typst-wiring -- query documento.typ "selector"
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## 3. Categorias Não Cobertas pela Bateria P490

### 3.1 — `image` com atributos avançados

```typst
// test-image-fit.typ
#image("test.png", width: 50%, fit: "contain")
#image("test.png", width: 50%, fit: "cover")
#image("test.png", width: 50%, fit: "stretch")
```

- `fit: "contain"` / `fit: "cover"` / `fit: "stretch"` — implementados?
- `width: 50%` — `Rel<Length>` ligado a image?

### 3.2 — `page` com `header`, `footer`, `numbering`

```typst
// test-page-header-footer.typ
#set page(
  header: [_Cabeçalho_],
  footer: context [Página #counter(page).display()],
  numbering: "1",
)
#lorem(50)
```

- `header:` e `footer:` — implementados?
- `context [...]` — delayed evaluation block. Implementado?
- `counter(page).display()` — page counter funciona?
- `numbering: "1"` — page numbering implementado?

### 3.3 — `place` com posicionamento absoluto

```typst
// test-place-absolute.typ
#place(top + right)[Canto superior direito]
#place(bottom + left)[Canto inferior esquerdo]
#place(center)[Centro]
#lorem(30)
```

- `place(top + right)` — posicionamento absoluto. Implementado?
- `place(bottom + left)` — combinação de alignments. Implementado?

### 3.4 — `calc` restante (funções não testadas em P495)

```typst
// test-calc-rest.typ
#calc.abs(-5)
#calc.sin(calc.pi / 2)
#calc.cos(0)
#calc.tan(calc.pi / 4)
#calc.pow(2, 10)
#calc.sqrt(2)
#calc.floor(3.7)
#calc.ceil(3.2)
#calc.min(1, 2, 3)
#calc.max(1, 2, 3)
#calc.rem(17, 5)
#calc.quo(17, 5)
#calc.atan(1)
#calc.asin(1)
#calc.acos(1)
#calc.exp(1)
#calc.ln(2)
#calc.log10(100)
#calc.deg(180)
#calc.rad(180)
#calc.clamp(5, 0, 10)
```

- P433 materializou calc. Verificar cobertura real das funções restantes.
- `calc.atan`, `calc.asin`, `calc.acos`, `calc.exp`, `calc.ln`, `calc.log10`, `calc.deg`, `calc.rad`, `calc.clamp` — implementados?

### 3.5 — `str` métodos avançados

```typst
// test-str-methods.typ
#"Hello".to-upper()
#"HELLO".to-lower()
#"Hello, World!".split(", ")
#"  hello  ".trim()
#"hello".replace("l", "r")
#"abc".to-unicode()
#str.from-unicode(97)
#"hello".contains("ell")
#"hello".starts-with("he")
#"hello".ends-with("lo")
#"hello".find("l")
#"hello".rev()
#"hello".repeat(3)
```

- `to-upper()`, `to-lower()`, `split()`, `trim()`, `replace()`, `to-unicode()`, `from-unicode()`, `contains()`, `starts-with()`, `ends-with()`, `find()`, `rev()`, `repeat()` — implementados?

### 3.6 — `dict` métodos avançados

```typst
// test-dict-methods.typ
#let d = (a: 1, b: 2, c: 3)
#d.keys()
#d.values()
#d.pairs()
#d.remove("a")
#d.insert("d", 4)
#d.len()
```

- `keys()`, `values()`, `pairs()` — já implementados como colateral P495?
- `remove()`, `insert()`, `len()` — implementados?

### 3.7 — `list` com atributos avançados

```typst
// test-list-advanced.typ
#list(
  marker: ([•], [–], [·]),
  indent: 1.5em,
  body-indent: 0.5em,
  tight: false,
  [Item A],
  list([Sub-item 1], [Sub-item 2]),
  [Item B],
)
```

- `marker: Array` — P470 scope-out, mas P494 validou selector. Implementar?
- `indent:`, `body-indent:`, `tight:` — implementados?
- Listas aninhadas — funcionam?

### 3.8 — `enum` com atributos avançados

```typst
// test-enum-advanced.typ
#enum(
  start: 5,
  numbering: "(a)",
  indent: 1.5em,
  body-indent: 0.5em,
  tight: false,
  [Quinto],
  [Sexto],
  enum([Sub-item 1], [Sub-item 2]),
  [Sétimo],
)
```

- `start:`, `numbering:`, `indent:`, `body-indent:`, `tight:` — implementados?
- Enums aninhados — funcionam?

### 3.9 — `par` com atributos avançados

```typst
// test-par-advanced.typ
#set par(
  leading: 1.5em,
  spacing: 2em,
  justify: true,
  first-line-indent: 1em,
  linebreaks: "optimized",
  hanging-indent: 2em,
)
Parágrafo um com texto suficiente para testar justificação e espaçamento.
Parágrafo dois.
```

- `leading`, `spacing`, `justify`, `first-line-indent` — implementados?
- `linebreaks: "optimized"` — scope-out declarado?
- `hanging-indent:` — implementado?

### 3.10 — `raw` com atributos avançados

```typst
// test-raw-advanced.typ
```rust
fn main() { println!("Hello!"); }
```
#raw("fn main() {}", lang: "rust", block: true)
#raw("inline", lang: "python", block: false)
```

- `lang` attribute aceito? Highlighting activo ou fallback monocromo?
- `block: true` / `block: false` — implementados?
- Triple-backtick vs `#raw(...)` — paridade?

### 3.11 — `quote` com atributos avançados

```typst
// test-quote-advanced.typ
#quote(
  attribution: [Einstein],
  block: true,
  quotes: false,
)[A imaginação é mais importante que o conhecimento.]
```

- `attribution` — implementado (P155)?
- `block:`, `quotes:` — implementados?

### 3.12 — `footnote` com atributos avançados

```typst
// test-footnote-advanced.typ
Texto com nota.#footnote[Esta é a nota de rodapé.]
Texto com nota numerada.#footnote(numbering: "1")[Outra nota.]
```

- `numbering:` — implementado?
- Footnote numbering automático — funciona?

### 3.13 — `figure` com atributos avançados

```typst
// test-figure-advanced.typ
#figure(
  image("test.png"),
  caption: [Legenda da figura.],
  kind: "image",
  supplement: [Fig.],
  numbering: "1",
)
```

- `caption:`, `kind:`, `supplement:`, `numbering:` — implementados?

### 3.14 — `bibliography` com estilos CSL

```typst
// test-bibliography-csl.typ
#bibliography(
  "refs.bib",
  style: "ieee",
  locale: "en-US",
)
```

- `style: "ieee"`, `style: "apa"` — implementados?
- `locale:` — implementado?

### 3.15 — `outline` com atributos avançados

```typst
// test-outline-advanced.typ
#outline(
  title: [Índice],
  depth: 2,
  indent: true,
)
= Capítulo 1
== Secção 1.1
= Capítulo 2
```

- `title:`, `depth:`, `indent:` — implementados?

### 3.16 — `state` / `counter` / `context`

```typst
// test-state-counter.typ
#let s = state("key", 0)
#s.update(5)
#context s.get()

#counter(heading).update(1)
#context counter(heading).get()
```

- `state()`, `state.update()`, `state.get()` — implementados?
- `counter()`, `counter.update()`, `counter.get()` — implementados?
- `context` block — implementado?

### 3.17 — `metadata` / `query` avançado

```typst
// test-metadata-query.typ
#metadata("info") <tag>
#context query(<tag>)
```

- `metadata()` com label — implementado?
- `query(<label>)` — implementado?
- `context query(...)` — implementado?

---

## 4. Formato do Relatório de Resultados

Para cada ficheiro de teste, produzir uma linha:

```
| test-image-fit.typ | Vanilla: [ok] | Cristalino: [erro descritivo] | DIFF | fit: scope-out |
```

Colunas: `ficheiro | vanilla | cristalino | classificação | notas`

**Classificações:**
- `MATCH` — output estruturalmente equivalente.
- `DIFF` — ambos produzem resultado mas diferente (investigar).
- `ERRO_DESCRITIVO` — cristalino retorna erro com mensagem clara (aceitável se scope-out).
- `PANIC` — cristalino crasha (bug prioritário para P501).
- `AUSENTE` — funcionalidade não reconhecida (avaliar se novo work ou scope-out).

---

## 5. Critério de Fecho

- [ ] Todos os 17 ficheiros de teste criados e executados contra vanilla 0.14.2.
- [ ] Todos os 17 ficheiros de teste executados contra cristalino.
- [ ] Tabela de resultados com classificação para cada teste.
- [ ] PANICs identificados como P501 bugs prioritários (deve ser zero).
- [ ] DIFFs avaliados: gap real vs divergência aceitável (ADR-0054 graded).
- [ ] AUSENTEs avaliados: novo work vs scope-out declarado.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p500.md` produzido.
- [ ] Lista priorizada de gaps para P501+ produzida.

---

## 6. Próximo Passo (P501)

Dependendo dos resultados:

- **Se há PANICs:** P501 é bug-fix prioritário.
- **Se há DIFFs inesperados:** P501 materializa o gap mais impactante.
- **Se tudo é MATCH ou ERRO_DESCRITIVO aceitável:** P501 é performance benchmark (DEBT-42) ou relatório de publicação (P502).

---

## A. Apêndice — Lista de Ficheiros de Teste P500

```
test-image-fit.typ
test-page-header-footer.typ
test-place-absolute.typ
test-calc-rest.typ
test-str-methods.typ
test-dict-methods.typ
test-list-advanced.typ
test-enum-advanced.typ
test-par-advanced.typ
test-raw-advanced.typ
test-quote-advanced.typ
test-footnote-advanced.typ
test-figure-advanced.typ
test-bibliography-csl.typ
test-outline-advanced.typ
test-state-counter.typ
test-metadata-query.typ
```

Total: **17 ficheiros**.

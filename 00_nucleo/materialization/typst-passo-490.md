---

# P490 — Bateria de testes de paridade funcional cristalino vs vanilla

> **Passo:** 490
> **Data:** 2026-06-29
> **Foco:** Descobrir empiricamente onde o cristalino diverge do vanilla 0.14.2 em comportamento observável. Não declarar conclusão — medir o que falta.
> **Tipo:** Diagnóstico empírico via `typst query` comparado com output do cristalino.
> **Tamanho:** M (~45 min de sondas).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.

---

## Metodologia

Para cada categoria de funcionalidade, criar um ficheiro `.typ` de teste, correr contra vanilla 0.14.2 e contra o cristalino, e comparar o output estrutural.

```bash
# Vanilla:
typst query --format json documento.typ "heading"

# Cristalino (via helper L3):
cargo run -p typst-wiring -- query documento.typ "heading"
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## Categoria 1 — Funcionalidades `parcial` do inventário 148

### 1.1 — `list` com marker por nível (array)

```typst
// test-list-marker-array.typ
#list(
  marker: ([•], [–], [·]),
  [Item A],
  [Item B],
)
```

P470 implementou `marker: Str` mas declarou `marker: Array` como scope-out. Confirmar que o cristalino retorna erro descritivo (não panic).

### 1.2 — `enum` com `start:` e numbering pattern com parênteses duplos

```typst
// test-enum-start.typ
#enum(start: 5, numbering: "(a)", [Quinto], [Sexto], [Sétimo])
```

- Vanilla: produz `(e)`, `(f)`, `(g)`.
- Cristalino: `start:` implementado? `"(a)"` com parênteses duplos — cobre o subset P470?

### 1.3 — `par` com atributos de spacing e justify

```typst
// test-par.typ
#set par(leading: 1.5em, spacing: 2em, justify: true, first-line-indent: 1em)
Parágrafo um com texto suficiente para testar justificação e espaçamento.
Parágrafo dois.
```

- `leading`, `spacing`, `justify`, `first-line-indent` — implementados ou scope-out?
- `linebreaks: "optimized"` — scope-out declarado?

### 1.4 — `show link` com estilo

```typst
// test-show-link.typ
#show link: it => underline(text(blue, it))
Visita #link("https://example.com")[este sítio].
```

P452/P463 materializou links. `#show link: ...` funciona?

---

## Categoria 2 — Funcionalidades de alto impacto

### 2.1 — `table` com header e footer

```typst
// test-table.typ
#table(
  columns: (1fr, 1fr, 1fr),
  table.header[Nome][Idade][Cidade],
  [Ana], [25], [Lisboa],
  [Bob], [30], [Porto],
  table.footer[Total][2][—],
)
```

- `table.header` / `table.footer` — implementados?
- Query: `typst query test-table.typ "table"` — conta 1?

### 2.2 — `raw` com lang e highlighting

```typst
// test-raw.typ
```rust
fn main() { println!("Hello!"); }
```
```

- `lang` attribute aceito? Highlighting activo ou fallback monocromo?
- `block: true` implícito para triple-backtick?

### 2.3 — `quote` com attribution

```typst
// test-quote.typ
#quote(attribution: [Einstein])[A imaginação é mais importante que o conhecimento.]
```

P155 materializou `Content::Quote`. `attribution` funciona?

### 2.4 — `footnote` body no rodapé

```typst
// test-footnote.typ
Texto com nota.#footnote[Esta é a nota de rodapé.]
Mais texto.
```

P304/P305 fechou footnotes. Confirmar que body aparece no rodapé e query conta 1.

### 2.5 — `page` com header, footer, counter

```typst
// test-page.typ
#set page(
  header: [_Cabeçalho_],
  footer: context [Página #counter(page).display()],
)
#lorem(50)
```

- `header:` e `footer:` — implementados?
- `context [...]` — delayed evaluation block. Implementado?
- `counter(page).display()` — page counter funciona?

### 2.6 — `place` elemento

```typst
// test-place.typ
#place(top + right)[Canto superior direito]
#lorem(30)
```

Posicionamento absoluto via `place`. Implementado?

---

## Categoria 3 — stdlib `calc` com args nomeados

```typst
// test-calc.typ
#calc.sqrt(2)
#calc.pow(2, 10)
#calc.sin(calc.pi / 2)
#calc.log(100, base: 10)
#calc.floor(3.7)
#calc.ceil(3.2)
#calc.round(3.567, digits: 2)
#calc.min(1, 2, 3)
#calc.max(1, 2, 3)
#calc.rem(17, 5)
#calc.quo(17, 5)
```

- `calc.log(100, base: 10)` — arg nomeado `base:` implementado?
- `calc.round(3.567, digits: 2)` — arg `digits:` implementado?
- P433 materializou calc. Verificar cobertura real.

---

## Categoria 4 — `array`/`str`/`dict` métodos avançados

### 4.1 — Array métodos

```typst
// test-array.typ
#let arr = (3, 1, 4, 1, 5, 9, 2, 6)
#arr.sorted()
#arr.dedup()
#arr.enumerate()
#arr.zip((10, 20, 30))
#arr.chunks(3)
#arr.windows(3)
#arr.flatten()
#arr.fold(0, (acc, x) => acc + x)
```

- `dedup()`, `windows(n)`, `chunks(n)` — implementados?
- P466 materializou métodos de array. Quais destes cobriu?

### 4.2 — Str métodos

```typst
// test-str.typ
#"Hello".to-upper()
#"HELLO".to-lower()
#"Hello, World!".split(", ")
#"  hello  ".trim()
#"hello".replace("l", "r")
#str(255, base: 16)
#"abc".to-unicode()
```

- `str(255, base: 16)` — conversão de base. Implementado?
- `to-unicode()` — implementado?

### 4.3 — Dict com `default:` e mutação

```typst
// test-dict.typ
#let d = (a: 1, b: 2, c: 3)
#d.at("z", default: 99)
#d.keys()
#d.values()
#d.pairs()
```

- `d.at("z", default: 99)` — arg `default:` implementado?

---

## Categoria 5 — Show/set edge cases

### 5.1 — `#show` com regex e texto misturado

```typst
// test-show-regex.typ
#show regex("\d+"): it => text(red, it)
O número 42 e o número 100 aparecem a vermelho.
```

P393/P473 materializou show regex. O texto "42" e "100" ficam red?

### 5.2 — `#set` local dentro de bloco

```typst
// test-set-local.typ
#set text(size: 12pt)
Texto a 12pt.
#[
  #set text(size: 24pt)
  Texto a 24pt.
]
Texto de volta a 12pt?
```

`#set` local em `#[...]` respeitado?

### 5.3 — `#show` multi-field where (scope-out declarado)

```typst
// test-show-where-multi.typ
#show heading.where(level: 1, outlined: true): it => upper(it.body)
= Heading nível 1
```

P474 declarou multi-field como scope-out. Confirmar erro descritivo (não panic).

---

## Categoria 6 — Math

```typst
// test-math.typ
$ sum_(i=0)^n i = (n(n+1))/2 $
$ vec(1, 2, 3) $
$ mat(1, 0; 0, 1) $
$ cases(x + y = 1, x - y = 0) $
$ integral_0^1 x^2 dif x = 1/3 $
```

- `vec`, `mat`, `cases` — implementados?
- Query: `typst query test-math.typ "math.equation"` — conta 5?

---

## Categoria 7 — Layout visual

### 7.1 — `image` com `fit:`

```typst
// test-image.typ
#image("test.png", width: 50%, fit: "contain")
```

- `fit: "contain"` implementado?
- `width: 50%` — `Rel<Length>` ligado a image?

### 7.2 — `stroke` como dict com sides

```typst
// test-stroke-sides.typ
#rect(stroke: (left: 3pt + red, right: 1pt + blue, top: none, bottom: 2pt + green))
```

P247 materializou stroke. Sides dict funciona?

### 7.3 — `columns` e `colbreak`

```typst
// test-columns.typ
#set page(columns: 2)
#lorem(100)
#colbreak()
#lorem(50)
```

P216–P221 materializou columns. `colbreak()` funciona?

---

## Formato do relatório de resultados

Para cada ficheiro de teste, produzir uma linha:

```
| test-list-marker-array.typ | Vanilla: [ok] | Cristalino: [erro descritivo] | ERRO_DESCRITIVO | P470 scope-out confirmado |
```

Colunas: `ficheiro | vanilla | cristalino | classificação | notas`

**Classificações:**
- `MATCH` — output estruturalmente equivalente.
- `DIFF` — ambos produzem resultado mas diferente (investigar).
- `ERRO_DESCRITIVO` — cristalino retorna erro com mensagem clara (aceitável se scope-out).
- `PANIC` — cristalino crasha (bug prioritário para P491).
- `AUSENTE` — funcionalidade não reconhecida (avaliar se gap real ou scope-out).

---

## Critério de fecho

- [ ] Todos os testes (~25) executados contra vanilla 0.14.2 e contra cristalino.
- [ ] Tabela de resultados com classificação para cada teste.
- [ ] PANICs identificados como P491 bugs prioritários (deve ser zero).
- [ ] DIFFs avaliados: gap real vs divergência aceitável (ADR-0054 graded).
- [ ] AUSENTEs avaliados: novo work vs scope-out declarado.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p490.md` produzido.
- [ ] Lista priorizada de gaps para P491+ produzida.

---

## Próximo passo (P491)

Dependendo dos resultados:

- **Se há PANICs:** P491 é bug-fix prioritário.
- **Se há DIFFs inesperados:** P491 materializa o gap mais impactante.
- **Se tudo é MATCH ou ERRO_DESCRITIVO aceitável:** P491 é audit de cobertura stdlib (args nomeados em `calc`, `str.to-unicode`, etc.) ou expansão do corpus de paridade.

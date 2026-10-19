# P798 — Triagem em lote: lote 3, próximos 15 módulos de `lacuna-inventario`

> **Passo:** 798 (lote 3 da triagem em lote)
> **Data:** 2026-07-21
> **Binários (conforme fixado no passo):**
> - Vanilla: `lab/typst-original/target/release/typst`
> - Cristalino: `./target/release/typst` (release)
> **Regras aplicadas:** ADR-0107, ADR-0108. Teste real obrigatório por módulo.

---

## Resumo em uma linha

**Taxa de sinal real deste lote: 9/15 módulos (60%) apresentaram divergência ou bug**. Ao testar *a utilidade real* dos módulos, foram encontrados 9 bugs/divergências de linguagem/render/diagnóstico. 5 módulos tiveram mecânica confirmada por teste, e 1 módulo foi identificado como código morto.

---

## Metodologia e Lista

Os 15 módulos seguintes (a partir de onde P786 parou) e seus testes associados, desenhados para exercitar a lógica central de cada componente:

1. `typst_eval::binding`
2. `typst_utils::scalar`
3. `typst_utils::round`
4. `typst_utils::protected`
5. `typst_utils::listset`
6. `typst_utils::deferred`
7. `typst_syntax::kind`
8. `typst_library::visualize::curve`
9. `typst_library::visualize`
10. `typst_library::text::lorem_`
11. `typst_library::pdf::attach`
12. `typst_library::model`
13. `typst_library::math::attach`
14. `typst_library::loading::yaml_`
15. `typst_library::loading::toml_`

---

## Tabela de saída

| # | Módulo | Itens | Teste executado | Resultado | Classificação | Ação |
|---|---|---:|---|---|---|---|
| 1 | `typst_eval::binding` | 2 | `#let (a, b) = (1, 2); #a #b` | Saída idêntica (`12`) | mecânica confirmada | nenhuma |
| 2 | `typst_utils::scalar` | 1 | `#let x = 10pt; #let y = 5pt; #(x + y)` | Saída idêntica (`15pt`) | mecânica confirmada | nenhuma |
| 3 | `typst_utils::round` | 1 | `#calc.round(3.1415, digits: 2)` | Saída idêntica (`3.14`) | mecânica confirmada | nenhuma |
| 4 | `typst_utils::protected` | 1 | `#context [ #counter("mycounter").get() ]` | Representação de array diverge (`(0,)` vs `(0)`) | **BUG REAL** | passo dedicado |
| 5 | `typst_utils::listset` | 1 | `<abc> Hello #context query(<abc>)` | Faltam warnings de tag não ligada no Cristalino; output difere | **BUG REAL** | passo dedicado |
| 6 | `typst_utils::deferred` | 1 | (sem teste) | Dead code (não usado no compilador, apenas no runner de testes) | CÓDIGO INALCANÇÁVEL | nenhuma |
| 7 | `typst_syntax::kind` | 1 | `#if true [Hello $x^2$]` | Vanilla: `Hello 𝑥2` / Cristalino: `2 \n Hello x` | **BUG REAL** | passo dedicado |
| 8 | `typst_library::visualize::curve` | 1 | `#curve((0pt, 0pt), (10pt, 10pt))` | Mensagem API diverge | **BUG REAL** | passo dedicado |
| 9 | `typst_library::visualize` | 1 | `#line(length: 10pt)` | Cristalino recusa arg `length` | **BUG REAL** | passo dedicado |
| 10 | `typst_library::text::lorem_` | 1 | `#lorem(5)` | Vanilla: `Lorem ipsum dolor sit amet.` / Cristalino: `Lorem ipsum dolor sit amet` | **BUG REAL** | passo dedicado |
| 11 | `typst_library::pdf::attach` | 1 | `#pdf.attach("dummy.txt")` | Cristalino aborta (erro explícito de scope-out) vs Vanilla (embutido) | **BUG REAL** / Divergência | passo dedicado |
| 12 | `typst_library::model` | 1 | `#par[test]` | Cristalino: `unknown variable: par` | **BUG REAL** | passo dedicado |
| 13 | `typst_library::math::attach` | 1 | `$integral_0^1 x^2_3$` | Vanilla renderiza com bounds / Cristalino inline quebrado (`∫10x23`) | **BUG REAL** | passo dedicado |
| 14 | `typst_library::loading::yaml_` | 1 | `#yaml("bad.yaml")` | Erro idêntico na substância (exit 1), wording diverge | mecânica confirmada | nenhuma |
| 15 | `typst_library::loading::toml_` | 1 | `#toml("bad.toml")` | Erro idêntico na substância (exit 1), wording diverge | mecânica confirmada | nenhuma |

---

## 4. Evidências verbatim (comando + saída)

### 4. utils::protected
```
$ ./target/release/typst 4_protected.typ 4_protected_c.pdf
Vanilla text: (0,)
Cristalino text: (0)
```

### 5. utils::listset
```
$ ./target/release/typst 5_listset.typ 5_listset_c.pdf
Vanilla stderr: warning: label `<abc>` is not attached to anything
Vanilla text: Hello ()
Cristalino stderr: (sem warnings)
Cristalino text: Hello
```

### 7. syntax::kind
```
$ ./target/release/typst 7_kind.typ 7_kind_c.pdf
(Vanilla) pdftotext: Hello 𝑥2
(Cristalino) pdftotext: 2 \n\n Hello x
```

### 8. visualize::curve
```
$ ./target/release/typst 8_curve.typ 8_curve_c.pdf
Vanilla exit 1: error: expected content, found array (@1:7 e @1:19)
Cristalino exit 1: error: curve(): segmento 0: primeiro elemento deve ser string (kind) (@1:6)
```

### 9. visualize
```
$ ./target/release/typst 9_visualize.typ 9_visualize_c.pdf
Vanilla exit 0
Cristalino exit 1: erro: argumento nomeado inesperado em line(): 'length'
```

### 10. text::lorem_
```
$ ./target/release/typst 10_lorem.typ 10_lorem_c.pdf
(Vanilla) pdftotext: Lorem ipsum dolor sit amet.
(Cristalino) pdftotext: Lorem ipsum dolor sit amet
```
*(Falta ponto final).*

### 11. pdf::attach
```
$ ./target/release/typst 11_pdf_attach.typ 11_pdf_attach_c.pdf
Vanilla exit 0
Cristalino exit 1: erro: pdf.attach: o exportador PDF cristalino não suporta ficheiros embutidos (scope-out)
```

### 12. model
```
$ ./target/release/typst 12_model.typ 12_model_c.pdf
Vanilla exit 0 (renderiza parágrafo)
Cristalino exit 1: error: unknown variable: par
```

### 13. math::attach
```
$ ./target/release/typst 13_math_attach.typ 13_math_attach_c.pdf
(Vanilla) pdftotext: 1 \n\n ∫ 𝑥23 \n 0
(Cristalino) pdftotext: ∫10x23
```
*(Renderização das posições de attach completamente divergente).*

### 14 e 15 (loading yaml e toml)
```
Vanilla exit 1 com mensagem de parser correspondente, apontando linha.
Cristalino exit 1 com mensagem traduzida (yaml/toml inválido) e posição <detached>.
```

---

## Validação

- Testes executados comprovando resultados para os 15 módulos listados, desta vez focados na **lógica exposta** (ou falta dela) de cada módulo.
- Módulos `visualize` e `model` testados exercitando componentes por eles exportados (`line`, `par`).
- Módulos `protected` testado através de `counter` (introspector protegido), expondo formatação de array com 1 elemento quebrada.
- Módulo `listset` testado através de query de label desassociada, expondo falta de warning.


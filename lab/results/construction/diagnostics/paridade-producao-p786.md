# P786 — Triagem em lote: próximos 15 módulos de `lacuna-inventario` (teste real obrigatório por módulo)

> **Passo:** 786 (lote 2 da triagem em lote)
> **Data:** 2026-07-20, medições entre ~17:15Z e ~17:50Z
> **Commit:** `0774275fe1b340d823e624958f66e5b6b344a51a`
> **Estado git:** sem alterações tracked; untracked: este relatório, `reverificacao-p785-15-modulos.md`, `typst-passo-786.md`, `typst-passo-786a.md`, artefactos de teste em `temp/temp_p786/`
> **Binários (conforme fixado no passo):** vanilla `lab/typst-original/target/release/typst` = typst 0.15.0 (rev `969087ec`); cristalino `./target/release/typst` (build 2026-07-20 11:09, mesmo commit)
> **Regras aplicadas:** ADR-0107 (paridade com a língua; mensagens de erro são observáveis), ADR-0108 (**nenhuma classificação sem teste real**), regra de proveniência de P569.

---

## Resumo em uma linha

**Taxa de sinal real deste lote: 12/15 módulos (80%)** — muito acima dos 27% do lote P785. 3 módulos com mecânica confirmada por teste (`text::case`, `loading::xml_`, `eval::ops`), 12 com divergência observável confirmada. Nenhum bug é pequeno/isolado — todos são superfície de língua ausente ou comportamento silenciado, logo **nenhuma correção feita neste passo; tudo registado para passos dedicados** (§5).

---

## Sonda e desempate (declarado)

Sonda = pipeline do passo com a lista consolidada de exclusões (P765a–P784 + os 17 módulos triados em P785/reverificação). Resultado: 3 módulos com 3 itens + 13 módulos com 2 itens concorrendo a 12 vagas. Pela ordem do próprio pipeline, ficam de fora deste lote apenas `typst_eval::binding` (2 itens) — fica para o lote 3. **Os 15 deste lote (33 itens):** `text::smartquote`(3), `symbols`(3), `model::numbering_`(3), `utils::version_`(2), `utils::bitset`(2), `text::item`(2), `text::case`(2), `model::reference`(2), `loading::xml_`(2), `loading::csv_`(2), `layout::layout_`(2), `foundations::selector`(2), `foundations::context`(2), `eval::rules`(2), `eval::ops`(2).

---

## Tabela dos 15 módulos

| # | Módulo | Itens | Teste executado (doc em `temp/temp_p786/`) | Resultado | Classificação | Ação |
|---|---|---:|---|---|---|---|
| 1 | `text::smartquote` | 3 | `a_smartquote_min.typ`, `a_smartquote.typ`, `a_smartquote_err_str/arr.typ` | aspas duplas retas vs curvas; `#set smartquote` ignorado; erros de validação ausentes | **BUG REAL** | passo dedicado |
| 2 | `symbols` | 3 | `a_symbols.typ`, `a_symbols_neq.typ`, `a_symbols_math1/2/3.typ` | `#sym.alpha`/`arrow.r` markup OK; `#sym.subset.neq` erro; `$ arrow.r $`/`$ dif $` erro | **BUG REAL** | passo dedicado |
| 3 | `model::numbering_` | 3 | `a_numbering.typ`, `a_numbering_enum.typ` | `#numbering` inexistente; enum `+` sem numeração; warning hebrew-zero ausente | **BUG REAL** | passo dedicado |
| 4 | `utils::version_` | 2 | `--version`, `c_version_sys.typ`, `c_version_sys2.typ` | `#sys.version` renderiza `version(0, 15, 0)`; `.at()` inexistente; CLI sem commit e 0.1.0 ≠ sys.version 0.15.0 | **BUG REAL** | passo dedicado |
| 5 | `utils::bitset` | 2 | `c_bitset_header.typ`, `c_bitset_conflict.typ` | `table.header` não repete (pág. 2+); conflito célula↔header não detetado (exit 0) | **BUG REAL** | passo dedicado |
| 6 | `text::item` | 2 | `a_textitem.typ`, `a_textitem_emph.typ` | `*negrito*`/`_itálico_` sem efeito visual (faces bold/italic inexistentes) | **BUG REAL** | passo dedicado (infra de fontes) |
| 7 | `text::case` | 2 | `a_case.typ` | `#lower`/`#upper` string e conteúdo: pdftotext **idêntico** | mecânica confirmada | nenhuma |
| 8 | `model::reference` | 2 | `a_reference.typ`, `a_reference_num.typ`, `a_reference_cite.typ` | erros de ref inválida silenciados; refs fora do fluxo (fundo da página); suplemento "Secção" vs "Section" | **BUG REAL** | passo dedicado |
| 9 | `loading::xml_` | 2 | `b_xml.typ`, `b_xml_err.typ` | parsing e acesso idênticos; erro de XML inválido com mesma substância/exit (wording diverge) | mecânica confirmada | nenhuma (wording registado) |
| 10 | `loading::csv_` | 2 | `b_csv.typ`, `b_csv_err.typ`, `b_csv_delim_err.typ` | parsing OK; **CSV malformado aceite com exit 0**; razão do erro de delimitador enganadora; `row-type` API diverge | **BUG REAL** | passo dedicado |
| 11 | `layout::layout_` | 2 | `b_layout.typ` | `#layout` inexistente (`unknown variable: layout`) | **BUG REAL** | passo dedicado |
| 12 | `foundations::selector` | 2 | `b_selector_show.typ`, `b_selector.typ`, `b_selector_query.typ` | show por elemento OK; query por label OK; **`#show <lbl>: ...` rejeitado** | **BUG REAL** | passo dedicado |
| 13 | `foundations::context` | 2 | `b_context.typ`, `b_context_here.typ`, `b_context_err.typ` | `#context text.lang` e `#context here().position()` falham; wording de `require` diverge | **BUG REAL** | passo dedicado |
| 14 | `eval::rules` | 2 | `c_rules_showpage.typ`, `c_rules_showpar.typ`, `c_rules_probe_show.typ` | `page`/`par` desconhecidos (warnings vanilla inalcançáveis); **show rule por string aceite e não aplicada em silêncio** | **BUG REAL** | passo dedicado |
| 15 | `eval::ops` | 2 | `c_ops_values.typ`, `c_ops_error.typ`, `c_ops_error2.typ` | valores **byte-idênticos** (arith, concat, cmp, short-circuit, assign); wording de erro de tipo diverge | mecânica confirmada | nenhuma (wording registado) |

---

## 4. Evidências verbatim (comando + saída)

### smartquote (A1, A2)
`a_smartquote_min.typ` = `"duplas" e 'simples'.` — ambos exit 0:
```
vanilla pdftotext:   “duplas” e ‘simples’.
cristalino pdftotext: "duplas" e ‘simples’.
```
`a_smartquote.typ` (com `#set smartquote(enabled: false)` e `quotes: "«»"`) — cristalino exit 0 com:
```
a_smartquote.typ:3:5: warning: set: target 'smartquote' ainda não suportado
  hint: targets suportados: heading, page, figure, text, par
```
`a_smartquote_err_str.typ` = `#set smartquote(quotes: "abc")`:
```
vanilla exit 1: error: expected 2 characters, found 3 characters  (@1:24)
cristalino exit 0 (só o warning "ainda não suportado")
```

### symbols (A3, A4)
```
$ ./target/release/typst a_symbols_neq.typ ...   # #sym.subset.neq
error: unknown symbol modifier 'neq'        (vanilla: exit 0, render ⊊)
$ ./target/release/typst a_symbols_math2.typ ... # $ arrow.r $
error: variável desconhecida: arrow         (vanilla: exit 0, render →)
$ ./target/release/typst a_symbols_math3.typ ... # $ integral x dif x $
error: unknown variable: dif + 2 hints      (vanilla: exit 0)
```
Contraste: `#sym.alpha #sym.arrow.r` em markup → idêntico (`α→`).

### numbering_ (A5, A6)
```
$ ./target/release/typst a_numbering.typ ...
error: unknown variable: numbering          (vanilla: exit 0 → "3. b) (IV.7)" + warning hebrew-zero + fallback "0")
a_numbering_enum.typ (+ primeiro / + segundo):
vanilla → "1. primeiro 2. segundo 3. terceiro" ; cristalino → "-primeiro -segundo -terceiro"
```

### version_ (B5–B7)
```
pdftotext vanilla:     "version: 0.15.0 / major: 0 / minor: 15 / patch: 0"
pdftotext cristalino:  "version: version(0, 15, 0)"   +  c_version_sys.typ → error: version does not contain field "at" (exit 1)
--version: vanilla "typst 0.15.0 (969087ec)" ; cristalino "typst 0.1.0"  (inconsistente com o próprio sys.version=0.15.0)
```

### bitset (B8, B9)
```
c_bitset_header.typ (tabela 20 linhas, header repeat default):
pdfinfo vanilla → Pages: 7, "H1 H2" no topo de TODAS ; cristalino → Pages: 5, "H1 H2" só na página 1
c_bitset_conflict.typ (table.cell rowspan: 2 sobre header):
vanilla exit 1: error: cell would conflict with header also spanning row 0 (@4:2 + hint)
cristalino exit 0 (silêncio; PDF com sobreposição)
```

### text/item (A7) — ênfase sem efeito
`a_textitem_emph.typ` = `normal *negrito* _italico_` — ambos exit 0, pdftotext idêntico. `pdffonts`:
```
vanilla:    LibertinusSerif-Regular + LibertinusSerif-Bold + LibertinusSerif-Italic (3 faces)
cristalino: AAAAAA+CrystallineFont1/2/3 (subsets da mesma face sintética; render mutool sem bold/italic)
```
**Inferência:** shaping separa os runs (3 items ↔ 3 subsets), mas as faces bold/italic não existem na fonte embutida — causa na infra de fontes/PDF, não em `TextItem`.

### reference (A8–A10)
```
a_reference.typ (heading sem numbering + @ref):
vanilla exit 1 ×3: error: cannot reference heading without numbering (+ hint `#set heading(numbering: "1.")`)
cristalino exit 0 (silêncio)
a_reference_num.typ (happy path): ambos exit 0
vanilla pdftotext: "Ver Section 1 e tambem Inicial 1." / "Nova referencia: Section 1.1."
cristalino: refs em branco no fluxo; "Secção 1.1"/"Secção 1" renderizados no FUNDO da página (mutool, y≈1030–1100)
a_reference_cite.typ = "Isto cita @knuth1984.":
vanilla exit 1: error: label `<knuth1984>` does not exist in the document
cristalino exit 0 → "Isto cita ." + "?" solto
```

### csv_ (B1) — CSV malformado aceite
`b_csv_err.typ` = `#csv("b_data_bad.csv")` com `a,b\n1,2,3`:
```
vanilla exit 1: error: failed to parse CSV (found 3 instead of 2 fields in line 2) (@b_data_bad.csv:2:0)
cristalino exit 0 (silêncio; PDF com (("a", "b"), ("1", "2", "3")))
```

### layout_ (B2)
```
$ ./target/release/typst b_layout.typ ...
error: unknown variable: layout   (vanilla exit 0 → "W=180pt H=130pt" / "BW=80pt BH=40pt")
```

### selector (B3)
`b_selector.typ` com `#show <sp>: it => [LBL=#it]`:
```
vanilla exit 0 ("HEAD=Alpha [origLBL=] found=1")
cristalino exit 1: error: selector inválido para show rule: label (@2:6)
```
Contraste OK: `#show heading: it => ...` idêntico; `#context query(<a1>).len()` idêntico.

### context (B4, B5)
```
#context [lang=#text.lang]        → cristalino exit 1: cannot access fields on type function
#context [page=#here().position().page] → cristalino exit 1: cannot access fields on type location
(vanilla exit 0: "lang=en" / "page=1")
```

### rules (C-B1–B3)
```
#show page: it => [WRAPPED: #it]  → vanilla: warning "`show page` is not supported and has no effect" + hint, exit 0
                                    cristalino: exit 1 "error: unknown variable: page"
#show par: set block(spacing: 4em) → vanilla: warning + 2 hints, exit 0 ; cristalino: exit 1 "unknown variable: par"
#show "world": [W]                 → ambos exit 0; vanilla render "Hello W.", cristalino render "Hello world." (não aplicada, em silêncio)
```

### ops — divergência de wording registada
```
#("a" - 1): vanilla "cannot subtract integer from string" ; cristalino "cannot apply Sub to str and int" (ambos exit 1, span 1:2)
#(1 / 0): "cannot divide by zero" idêntico em ambos
```

### Divergências de mensagem (mesmo exit/substância — registadas, sem ação)
- `format_xml_error`: `found closing tag 'a' instead of 'b' @1:6` vs `xml inválido: expected 'b' tag, not 'a' at 1:7` (span `<detached>`).
- `Delimiter` não-ASCII: vanilla `delimiter must be an ASCII character` vs cristalino `delimiter deve ser um único carácter` — razão enganadora ("é" é um só carácter).
- `require` fora de contexto: vanilla `can only be used when context is known` + 2 hints vs cristalino `cannot access fields on type function` / `here() chamado fora de contexto locatable...`.
- `csv(row-type:)`: vanilla espera o **tipo** `dictionary`; cristalino espera a **string** `"dictionary"` — API divergente nos dois sentidos.

---

## 5. Decisão sobre os achados

Nenhum bug deste lote é "pequeno e isolado" — são superfícies de língua ausentes (`#numbering`, `#layout`, `page`/`par` como valores, `#sys.version.at`, `#show <lbl>`), comportamento silenciado onde o vanilla erra (CSV malformado, refs inválidas, conflito de header, erros de `smartquote`), ou features de render (ênfase bold/italic, repetição de header, refs fora do fluxo). **Nenhuma correção feita neste passo; todos registados para passos dedicados.** Candidatos naturais de agrupamento (decisão do dono; existe `typst-passo-786a.md` já rascunhado em materialization):

1. **Numbering** (`#numbering`, enum `+`, warning de fallback) — módulos 3, e adjacente ao bug "heading nível 2 `1.` vs `1.1.`".
2. **References/citations** (módulo 8 — erros silenciados, posição no fluxo, suplemento lang).
3. **Context/layout eval** (módulos 11, 13 — `#layout`, `text.lang`, `here().position()`).
4. **Selectors/show rules** (módulos 12, 14 — `#show <lbl>`, show por string, `page`/`par` bindings).
5. **Smartquote** (módulo 1 — duplas default + `#set smartquote` + validações).
6. **Math/symbol scope** (módulo 2 — `subset.neq`, `arrow.r`, `dif` em math).
7. **Table header** (módulo 5 — repetição + deteção de conflito).
8. **Ênfase/fontes** (módulo 6 — faces bold/italic na fonte embutida).
9. **CSV strictness + API** (módulo 10 — rejeitar linhas com campos a mais; `row-type`; mensagem do delimitador).
10. **sys.version/CLI** (módulo 4 — Display de `Version`, `.at()`, `--version` com commit).

---

## 6. Spot-checks independentes (re-executados por mim fora do swarm, ADR-0108)

8 casos re-corridos dos artefactos dos agentes, todos confirmados: `a_smartquote_min` (aspas duplas), `a_numbering` (unknown variable), `b_layout` (unknown variable), `b_csv_err` (exit 0 silencioso), `c_rules_probe_show` (`Hello W.` vs `Hello world.`), `b_context_here` (cannot access fields on location), `a_symbols_neq` (unknown symbol modifier), `a_textitem_emph` (pdffonts: 3 faces Libertinus vs 3 subsets CrystallineFont idênticos).

## 7. Observações adjacentes (fora do escopo dos itens)

- `#text(size:)` não suportado (`argumento nomeado desconhecido: 'size'`).
- Fontes embutidas do cristalino falham em poppler (`Couldn't create a font`); verificação visual depende de mutool/fallback.
- Math italic: `$ alpha beta $` extrai `αβ` no cristalino vs `𝛼𝛽` no vanilla (inferência: styling, não scope).
- Mensagens cristalinas misturam pt/en (`variável desconhecida: arrow` vs `unknown variable: dif`).
- Paginação diverge no teste de header (5 vs 7 páginas) — métricas de inset/altura; observação, não classificado.

## 8. Validação

- `cargo test --workspace`: **4975 passed, 0 failed**, exit 0.
- `crystalline-lint .`: **zero violações**; 2 warnings V7 pré-existentes (prompts órfãos `engine/eval/field-access.md`, `infra/package_version_resolution.md`).

## 9. Taxa de sinal e consequência para os próximos lotes

**12/15 (80%)** neste lote vs 27% no lote P785. A fronteira de "mecânica pura" acabou: a partir daqui os módulos restantes são maioritariamente superfície de língua (stdlib/render). Recomendação para P787+: manter lote de 15 e teste obrigatório, mas esperar que a maioria vire passo dedicado — o gargalo deixa de ser a triagem e passa a ser a fila de correções (§5).

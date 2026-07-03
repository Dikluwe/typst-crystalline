# Auditoria do Projecto — Passo 550

**Data:** 2026-07-03  
**Foco:** Varrer relatórios do projecto à procura de (a) itens marcados fechados sem comparação directa contra o vanilla, (b) scope-outs sem razão escrita, e (c) passos próximos no tempo que tocam o mesmo ficheiro sem se referirem.  
**Binários:** `./target/release/typst` (cristalino), `/usr/local/bin/typst` (vanilla 0.14.2).

---

## 1. Resumo executivo

- **Lista bruta:** 279 ficheiros com ocorrências das palavras-chave; 1308 ocorrências de "scope-out / fora de escopo / adiado / fechado / concluído".
- **Categoria A (fechado sem número):** 5 casos. Têm evidência concreta noutras secções do relatório, mas a conclusão final é qualitativa.
- **Categoria B (scope-out sem razão):** ~17 ocorrências, maioritariamente em tabelas resumo onde a justificativa foi reduzida a "Fora de escopo". O caso mais forte é `paridade-funcional-p514.md`.
- **Categoria C (passos próximos, mesmo ficheiro):** 9 ficheiros identificados. A maioria cruza referências; dois (`cursor.rs`, `builder.rs`) merecem atenção documental.
- **Re-testados:** 13 itens de maior risco. Todos confirmaram fecho, excepto o ponto em aberto já conhecido de colunas multi-página.
- **Inventário criado:** `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md` (não existia anteriormente).

---

## 2. Grupo 1 — Lista bruta

Comandos usados:

```bash
grep -rln "scope-out\|fora de escopo\|adiado\|não é objectivo\|aceite\|fechado\|concluído" \
  00_nucleo/diagnosticos/ --include="*.md" | sort > /tmp/p550-lista-bruta-ficheiros.txt
# → 279 ficheiros

grep -n "scope-out\|fora de escopo\|adiado\|não é objectivo\|aceite\|fechado\|concluído" \
  00_nucleo/diagnosticos/*.md > /tmp/p550-lista-bruta-linhas.txt
# → 1308 ocorrências
```

---

## 3. Grupo 2 — Filtragem por risco

### 3.1 Categoria A — "fechado" sem número/comando na conclusão

| Ficheiro:linha | Conclusão | O que falta na conclusão |
|---|---|---|
| `paridade-producao-p538d.md:99` | "Não houve regressões." | Comando/número do teste manual |
| `paridade-producao-p540.md:126` | "`#for ...` funciona no cristalino." | Referência ao teste de corpus / `pdftotext` |
| `paridade-producao-p541.md:151` | "Padrões compostos funcionam no cristalino." | Tabela `pdftotext` vs vanilla |
| `paridade-producao-p543.md:84` | "Caso comum permanece sem regressão." | Teste manual contra vanilla |
| `paridade-producao-p547.md:126` | "Validação final verde." | Comparação vanilla e artefactos que desapareceram |

**Nota:** todos estes relatórios apresentam evidência nas secções anteriores. O risco é documental, não funcional.

### 3.2 Categoria B — scope-out sem razão escrita

Casos mais representativos:

| Ficheiro:linha | Item | Problema |
|---|---|---|
| `paridade-funcional-p514.md:211-215` | HTML/SVG/PNG/IDE/Plugin | "Fora de escopo" sem razão (cópia descuidada do handoff) |
| `paridade-gap-p508.md:152` | HTML/SVG/raster export | "Fora de escopo" sem razão |
| `sondagem-vanilla-p531.md:197` | IDE/LSP | Tabela resumo perde a justificativa da linha 44 |
| `paridade-producao-p515.md:187` | CFF subsetting | "Scope-out do subsetting" — justificativa circular |
| `regressao-subsetting-p519.md:231-232` | `smcp` / `calt` | "Scope-out" sem razão |
| `paridade-producao-p534.md:64` | Fusão de runs | "Scope-out" sem explicar que é optimização de export |
| `paridade-producao-p537.md:24` | Overflow nota em colunas | "scope-out conhecido" sem referenciar a explicação completa |

### 3.3 Categoria C — ficheiros com muitos passos próximos

| Ficheiro | Passos próximos | Cruzaram referências? |
|---|---|---|
| `01_core/src/rules/layout/mod.rs` | P537–P547 | Sim |
| `01_core/src/rules/layout/cursor.rs` | P537–P545 | **Parcial** — P545 não referencia P544 directamente |
| `01_core/src/rules/layout/text.rs` | P544, P547 | Sim |
| `01_core/src/rules/layout/bib_csl.rs` | P533, P547 | Sim |
| `01_core/src/rules/eval/bibliography.rs` | P497–P540 | Sim |
| `03_infra/src/shaper.rs` | P534–P548 | Sim |
| `03_infra/src/font_metrics.rs` | P544, P546, P548 | Sim |
| `03_infra/src/export/stream.rs` | P520, P530, P548, P549 | Sim |
| `03_infra/src/export/builder.rs` | P520–P548 | **Parcial** — P535/P536 adjacentes, focos separados |

---

## 4. Grupo 3 — Re-verificação directa (13 itens)

| # | Passo | Teste | Resultado | Veredicto |
|---|---|---|---|---|
| 1 | P540 | `#for (i, x) in items.enumerate()` | Output `pdftotext` idêntico ao vanilla | Fechado |
| 2 | P541 | Numeração `"1 / 1"` em 3 páginas | `1/3`, `2/3`, `3/3` idênticos | Fechado |
| 3 | P543 | Fallback global `font: "Helvetica"` | Texto extraído sem duplicação | Fechado |
| 4 | P538d | Numeração automática simples | Texto extraído idêntico; `style.font` preenchido | Fechado |
| 5 | P545 | Interpolação `#{expr}` em markup | Funciona; gap de espaço em listas é pré-existente | Fechado |
| 6 | P534 | Texto misto latim+CJK+árabe | Texto extraído idêntico | Fechado |
| 7 | P537 | Notas de rodapé em `columns(2)` | Notas no fundo da coluna; vanilla 0.14.2 difere | Fechado com scope-out documentado |
| 8 | P535 | Bookmarks PDF | Estrutura idêntica; `zoom=nan` scope-out | Fechado |
| 9 | P536 | Metadados `/Info` | `Title`/`Author` correctos | Fechado |
| 10 | P547 | CSL IEEE | Artefactos tipográficos desapareceram; conteúdo correcto | Fechado |
| 11 | P533 | Citação inline `@key1` | `See [1].` idêntico | Fechado |
| 12 | P532 | Numeração `"i"` | `i`, `ii` idênticos | Fechado |
| 13 | P520/P548 | Kerning / delta `TJ` | Posições reais idênticas (ver P549) | Fechado |

---

## 5. Grupo 4 — Inventário

Criado `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md` com:

- Lista completa das Categorias A, B e C.
- Tabela dos 13 itens re-testados.
- Itens que continuam abertos (colunas multi-página, fusão de blocos `BT...ET`, zoom `/XYZ`, XMP, etc.).

---

## 6. Tabela final

| Categoria | Quantidade encontrada | Quantidade re-testada | Reclassificados como abertos |
|---|---:|---:|---:|
| A — fechado sem número | 5 | 5 | 0 |
| B — scope-out sem razão | ~17 | 0* | 0 |
| C — ficheiro com passos próximos | 9 | 2** | 0 |

\* Scope-outs são decisões, não funcionalidades testáveis; o risco é documental.  
\** Re-testámos os dois casos de maior risco (`cursor.rs` via P545/P540; `builder.rs` via P535/P536).

---

## 7. Itens que continuam abertos

| Item | Passo | Por que continua aberto |
|---|---|---|
| Documento multi-página em `#set page(columns: 2)` | P537 | 5 páginas vs 2 do vanilla para `#lorem(1200)` |
| Fusão de blocos `BT...ET` consecutivos | P534 | Scope-out de optimização de export |
| Zoom explícito em `/XYZ` de bookmarks | P535 | Scope-out; cristalino usa `null` |
| Stream XMP de metadados | P536 | Scope-out; apenas `/Info` |
| Ordenação sofisticada de fallback | P534 | Scope-out; ordem linear do FontBook |

---

## 8. Conclusão

A auditoria P550 mostra que o projecto está, em geral, bem documentado: a maioria dos passos recentes cruza referências e apresenta evidências. Os problemas encontrados são predominantemente **documentais** (conclusões que não reproduzem o número/comando, scope-outs com justificativa mínima). O único risco funcional significativo que permanece é o **layout multi-página em colunas** (P537). A lista bruta de 279 ficheiros e 1308 ocorrências foi reduzida a um inventário gerenciável.

---

## 9. Nota sobre testes

`cargo test --workspace` reportou 572 passed / 1 failed / 6 ignored. A falha é `p307b_snapshot_tests::p307b_snapshot::p307b_07_multi_feature` (`actual=2130B expected=2130B`), uma flakiness pré-existente de snapshot binário já documentada em P541/P548; não é introduzida por este passo diagnóstico.

`crystalline-lint .`: zero violations.

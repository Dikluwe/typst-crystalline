# Passo 999 — Relatório: Auditoria de todos os prompts (Fase 1)

**Tipo**: Auditoria — catálogo, sem correcções aplicadas.
**Data**: 2026-08-12
**Âmbito**: `00_nucleo/prompts/` (305 ficheiros `.md`). ADRs fora de âmbito (decisão do dono).

## Proveniência da medição (ADR-0121)

- **Commit**: `00dc94966` (HEAD no momento da auditoria).
- **Working tree**: não commitados apenas `00_nucleo/materialization/typst-passo-999.md`
  (o próprio passo), `00_nucleo/diagnosticos/typst-dsm.html` e 3 PDFs de teste na raiz
  (`test_crystalline.pdf`, `test_oracle.pdf`, `test_vanilla.pdf`) — nenhum ficheiro de
  `00_nucleo/prompts/` nem de código estava alterado.
- **Método**: agregação mecânica por script (`temp/p999/audita_prompts.py` e variantes,
  conservados em `temp/p999/`); dados brutos em `temp/p999/auditoria.json` e
  `temp/p999/tabela-p999.csv`. Regex do Critério B exactamente como prescrito no passo.
  Cruzamento com a linhagem real medido pelos headers `@prompt` nos `.rs` de
  `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/`.

## Resumo executivo

| Critério | Resultado medido |
|----------|------------------|
| **A — Atomização (1 prompt : 1 ficheiro)** | **19 prompts violam** (cobrem >1 ficheiro de código). Os 3 candidatos de ADR-0104 **continuam por fatiar**. **10 prompts órfãos** (nenhum código os cita). |
| **B — Referências a passo** | **295 de 305 prompts** têm referências a passos (**5086 ocorrências**). **281 têm ocorrências graves** (4794 occ. em secções substantivas); só **14** são exclusivamente leves (292 occ. em `Criado em`/`Histórico de Revisões`); só **10** estão limpos. **140 prompts têm passos nos próprios títulos de secção** (738 títulos). |
| **C — Completude estrutural (template)** | Só **19 de 305** têm as 5 secções canónicas. Faltas: Instrução 283, Resultado Esperado 258, Restrições 221, Contexto 162, Critérios de Verificação 150. **70 prompts** têm `Histórico de Revisões` (secção descontinuada por esta auditoria — marcar para remoção). |
| **D — Ambiguidade (workflow)** | **Não aplicável — meta-achado**: o workflow `.agents/workflows/auditar-spec.md` **não existe** (a pasta `.agents/` não existe). Critério D não pôde ser aplicado sem inventar critérios próprios (proibido pelo passo). |
| **E — Cruzamento doc (só math)** | ver secção «Fase C» abaixo. |

Leitura contra a espinha dorsal: o ponto 3 («passos são mecanismo, descartável — não
deixam resíduo») está violado de forma quase universal (Critério B); o ponto 5
(«1 prompt : 1 ficheiro») tem 19 violações concentradas nos hubs históricos; o ponto 4
(«fazer sentido sozinhos») falha nos 140 prompts cuja estrutura de secções é cronológica
(títulos = números de passo), não temática.

---

## Critério A — Atomização

Medição: para cada prompt, contagem de ficheiros `.rs` cujo header `@prompt` o cita
(linhagem real, não o campo «Arquivos gerados», que só 11 prompts preenchem no formato
do template).

### A.1 — Candidatos de ADR-0104: continuam por resolver (confirmado)

ADR-0104 lista `rules/eval.md`, `rules/parse.md`, `engine/layout.md` como candidatos a
fatiar. O commit `6636c5ea6` («renomeia rules para engine em L1, L0 e consumidores»)
confirma que `rules/` de hoje é `engine/`. Estado actual medido:

| Prompt | Ficheiros de código que o citam (todos citam **só** este) |
|--------|-----------------------------------------------------------|
| `engine/layout.md` | **13** — `engine/layout/{mod,cursor,dynamic,grid,grid_placement,helpers,hyphenation,metrics,placement,sequence,slicing,sub_frame,tests}.rs` |
| `engine/eval.md` | **11** — `engine/eval/{mod,bibliography,bindings,closures,control_flow,flow,markup,math,modules,rules,tests}.rs` |
| `engine/parse.md` | **7** — `engine/parse/{mod,code,markup,math,parser,patterns,rules}.rs` |

**Conclusão**: não fatiados até hoje. O achado de ADR-0104 mantém-se válido e é a
violação A mais gravosa pela combinação com o Critério B (estes 3 prompts estão também
no top de ocorrências graves — ver tabela).

### A.2 — Achados novos de atomização (não listados em ADR-0104)

| Prompt | Nº fich. | Nota |
|--------|---------|------|
| `engine/atomizacao_elementos.md` | **30** | Citado como único L0 por 30 ficheiros de layout por elemento (`engine/layout/{text,block,boxed,cite,colbreak,curve,decorations,divider,footnote,grid_cell,grid_footer,grid_header,h_space,hide,pad,pagebreak,place,quote,repeat,smartquote,stack,table_cell,table_footer,table_header,term_item,terms,transform,v_space}.rs` + `introspect/{heading,labelled}.rs`). É o L0 do padrão ADR-0109 (forma B); funciona na prática como prompt colectivo dos elementos sem L0 de layout próprio. **Registado como violação estrita de 1:1; se é intencional (spec de padrão partilhado) é decisão a tomar no passo de correcção.** |
| `entities/f_fronteira_e1.md` | 5 | `element_registry.rs`, `elements/{dynamic,emph,strong,test_callout}.rs` |
| `engine/lexer/mod.md` | 4 | `lexer/{mod,code,markup,math}.rs` |
| `engine/lang.md` | 3 | `lang/{mod,figure_supplement,quotes}.rs` |
| 12 prompts com 2 ficheiros | 2 | `infra.md`, `infra/shaper.md`, `infra/package_downloader.md`, `infra/export/builder.md`, `engine/layout_references.md`, `engine/stdlib/{foundations,state,counter,context}.md`, `engine/stdlib_audit_methodology.md`, `engine/math/layout/_comum.md`, `testing/math_oracle.md` |

### A.3 — Prompts órfãos de linhagem (achado colateral de A)

10 prompts existem mas **nenhum ficheiro de código os cita** via `@prompt`
(`template-prompts.md` excluído por ser o próprio template):

`engine/eval/table.md`, `engine/eval/decimal-arithmetic.md`, `engine/show-regex.md`,
`engine/stdlib/grid_hline.md`, `engine/stdlib/grid_vline.md`, `engine/stdlib/table_hline.md`,
`engine/stdlib/table_vline.md`, `engine/style/font-dict.md`,
`infra/package_version_resolution.md`, `package-spec-dto.md`.

Isto quebra o ponto 1 da espinha dorsal na direcção inversa (prompt que não gera código
— ou gera mas a linhagem aponta para o prompt-pai grosso). Cada um precisa de decisão:
ligar a linhagem, fundir no pai, ou remover.

---

## Critério B — Zero referência a passo

Regex aplicada (a do passo): `\b[Pp]asso\s*[0-9]|\bP[0-9]{2,4}[A-Za-z]?\b`.
Classificação: **leve** = ocorrência só em `Histórico de Revisões` ou em linha de
metadados `**Criado em**`/`**Atualizado em**`; **grave** = qualquer outra secção.

### B.1 — Totais

- 5086 ocorrências em 295 prompts (de 305). 4794 graves, 292 leves.
- **10 prompts limpos**: `entities/document_info.md`, `engine/layout_counters.md`,
  `infra/layout.md`, `engine/math/mod.md`, `engine/mod.md`, `engine/scanner.md`,
  `entities/lang.md`, `entities/source-result.md`, `package-spec-dto.md`,
  `template-prompts.md`.
- **14 prompts só com ocorrências leves** (remoção mecânica e segura):
  `engine/eval/cast.md`, `entities/{counter_format,rel,ptr-eq-arc}.md`,
  `infra/export/gradients/{function_dict,linear}.md`, `entities/ast/{code,expr,markup,math}.md`,
  `entities/{file-id,package-spec,syntax-kind,syntax-text}.md`.
- **140 prompts têm números de passo em títulos de secção (738 títulos)** — a forma mais
  grave: a estrutura do documento é cronológica, não temática (ex. confirmado por leitura:
  `engine/math/layout/accent.md` tem TODAS as 8 secções intituladas por passo,
  `## P906 …` a `## P989 …`). O exemplo já conhecido `entities/content.md` confirma-se
  (66 títulos com passo).

### B.2 — Top 15 por ocorrências graves

| B grave | B total | Prompt |
|--------:|--------:|--------|
| 322 | 322 | `engine/eval.md` |
| 320 | 320 | `entities/content.md` |
| 224 | 224 | `engine/layout.md` |
| 213 | 213 | `entities/gradient.md` |
| 207 | 207 | `engine/introspect.md` |
| 131 | 131 | `entities/f_fronteira_e1.md` |
| 121 | 121 | `engine/math/layout/_comum.md` |
| 103 | 103 | `engine/stdlib/foundations.md` |
| 95 | 95 | `engine/stdlib/structural.md` |
| 93 | 101 | `infra/font_metrics.md` |
| 82 | 94 | `infra/shaper.md` |
| 72 | 75 | `entities/region.md` |
| 71 | 80 | `engine/eval/ops.md` |
| 67 | 83 | `entities/introspector.md` |
| 58 | 71 | `infra/export/builder.md` |

### B.3 — Achado de nome de ficheiro

`00_nucleo/prompts/passo-537b-set-page-columns.md` — o **próprio nome do ficheiro**
referencia um passo, e o conteúdo é organizado por passos (P537, P552, P537b, P750).
Caso extremo: mesmo limpo de referências internas, o nome viola a regra nova.

### B.4 — Nota sobre o template

`template-prompts.md` (ele próprio limpo de B) ainda **prescreve a secção
`Histórico de Revisões`** e o campo `Criado em` — ou seja, o template vigente legitima
exactamente o que a regra nova proíbe. Corrigir o template é pré-requisito de qualquer
correcção em massa (senão os prompts novos nascem a violar).

---

## Critério C — Completude estrutural

Verificação estrita das 5 secções canónicas de `template-prompts.md` (Contexto,
Restrições Estruturais, Instrução, Critérios de Verificação, Resultado Esperado),
case-insensitive, aceitando prefixos de numeração (`## 1. Contexto` conta).

- **Só 19 de 305 prompts têm as 5 secções.**
- Faltas por secção: Instrução 283 · Resultado Esperado 258 · Restrições Estruturais 221 ·
  Contexto 162 · Critérios de Verificação 150.
- **94 prompts não têm nenhuma das 5** (estrutura inteiramente fora do template);
  **~190 têm estrutura parcial ou vocabulário alternativo**.

**Qualificação importante (medição, não decisão)**: a ausência do nome canónico não
implica ausência do papel. Inventário dos títulos `##` em uso mostra vocabulários
alternativos consolidados — `Struct` (72 prompts), `Interface pública` (53),
`Propósito` (43), `Invariantes` (40), `Semântica` (35), `Critério` (40),
`Contexto e Objetivo` (25), `Não-objectivos`/`Scope-out` (18)… — ou seja, há pelo menos
dois dialectos de estrutura em vigor (template canónico e estrutura por elemento/tipo),
mais o anti-padrão cronológico (secções por passo, Critério B). A decisão — impor o
template, ou actualizar o template para o dialecto de facto e marcar o resto — fica para
o passo de correcção. **O que é objectivo**: 94 prompts sem nenhuma das 5 secções sob
qualquer nome, e os 140 com estrutura cronológica, não têm estrutura temática estável.

**`Histórico de Revisões`**: presente em **70 prompts** — descontinuada por esta
auditoria; todas as instâncias marcadas para remoção no passo de correcção (as suas 292
ocorrências leves de B desaparecem com ela).

---

## Critério D — Meta-achado (workflow inexistente)

O passo manda aplicar os critérios de `.agents/workflows/auditar-spec.md` **sem presumir
nem inventar critérios próprios**. Medição: a pasta `.agents/` não existe no repositório
(`Glob .agents/**` → 0 resultados; `Glob **/auditar-spec*` → 0 resultados). O único
directório de configuração de agente presente é `.claude/` (só `settings.local.json`).

**Consequência**: Critério D não foi aplicado a nenhum prompt. O workflow precisa de ser
escrito (ou o passo de correcção decidirá critérios de ambiguidade outra forma) antes de
qualquer auditoria de ambiguidade — registado como achado de processo, separado dos
achados por prompt.

---

## Fase C — Cruzamento com o corpus de documentação (só `math/`)

Subagente de leitura cruzou os 20 ficheiros de `00_nucleo/corpus-docs/math/` com os 16
prompts L0 de math do âmbito (`engine/math/**`), verificando as ausências com greps
exaustivos sobre os prompts adjacentes (`engine/eval.md`, `engine/stdlib/math_style.md`,
`engine/stdlib/sym.md`, `engine/stdlib/structural.md`, `engine/layout/equation.md`,
`entities/ast/math.md`, `entities/elements/math_*.md`) para não declarar omissões falsas.
Resultado: **3 contradições + 13 omissões**.

### Contradições (prompt contradiz citação verbatim da doc)

- **C1 — `class("large", …)`**: a doc atribui à classe efeitos de layout (centragem no
  eixo math + stretch vertical em `display`, `corpus-docs/math/class.typ:37`) e de
  limites (`class.typ:17`); o L0 fixa que o override é **só espaçamento**, "não afecta o
  layout" (`entities/elements/math_class_override.md:15-20`, confirmado em
  `engine/stdlib/structural.md:1005` e `engine/math/layout/spacing.md:201-204`). No
  cristalino o stretch em display existe só via `is_large_operator` por carácter
  (`engine/math/layout/_comum.md:449-480`), nunca via `class("large", …)`.
- **C2 — `is_math_function` com nomes que a doc não reconhece como operadores**:
  `engine/math/symbols.md:73-79` lista `div`, `rank`, `Var`, `Cov`, `E`, `sqrt`, `root`
  como operadores predefinidos (upright); a lista verbatim da doc
  (`corpus-docs/math/op.typ:14`, 42 operadores) não os contém — no vanilla `$ Var(x) $`,
  `$ Cov $`, `$ rank $` são variáveis em itálico. Divergência de língua assada no L0 sem
  decisão registada.
- **C3 — "`\` em math é só quebra de linha"**: afirmação em `engine/eval.md:3251`
  (atribuída a confirmação do dono) **refutada pelo corpus**: escape de delimitador
  (`lr.typ:44`) e escape de separador em math calls (`00-index.typ:36`). Ver «Os três
  usos de `\`» abaixo.

### Omissões (doc define explicitamente; nenhum prompt de math documenta)

| # | Tópico | Corpus (file:line) | Estado nos prompts |
|---|--------|--------------------|--------------------|
| O1 | 13 dos 17 acentos nomeados (macron, breve, caron, harpoon, arrow.l…) | `accent.typ:36-39` | só 4 cobertos (`accent.md:71-73`, `eval.md:3043-3046`); sem scope-out |
| O2 | 11 operadores predefinidos (`arg`, `coth`, `csch`, `ctg`, `deg`, `hom`, `id`, `lg`, `sech`, `sinc`, `tg`) | `op.typ:14` | ausentes de `is_math_function` (`symbols.md:76-79`) — reverso de C2 |
| O3 | `cases`: parâmetros `delim`, `reverse`, `gap` | `cases.typ:23-41` | struct só tem `rows` (`math_cases.md:22`); sem scope-out |
| O4 | `frac`: parâmetro `style` + regra de absorção de parênteses | `frac.typ:14,24-32,78` | `frac.md` só cobre estilo vertical |
| O5 | `mid()` inexistente; `set math.lr(size: 1em)` | `lr.typ:8-9,44,50` | `mid(`: zero ocorrências em todos os prompts; named arg `size:` tem scope-out (`eval.md:3014-3018`); set-rule omisso |
| O6 | `mat`: `augment`, `align`, `gap` | `mat.typ:27-50` | só `delim` coberto (`eval.md:3129-3134`) |
| O7 | `vec`: `align`, `gap` | `vec.typ:17-25` | idem O6 |
| O8 | função de utilizador `stretch(body, size:)` + default `100% + 0pt` | `stretch.typ:8-34` | mecanismo interno coberto (`stretchy.md`); a função da linguagem não existe em L0 nenhum |
| O9 | `underparen`/`overparen`/`undershell`/`overshell` | `underover.typ:31-45` | só brace/bracket cobertos (`eval.md:3094-3125`); sem scope-out |
| O10 | `equation`: `number-align`, `supplement`, `alt`, `breakable` | `equation.typ:29-57` | `layout/equation.md:66-84` cobre `numbering`/`block`; os 4 parâmetros omissos |
| O11 | `primes(n)`: forma-função (o vanilla rejeita; corpus mediu) | `primes.typ:15-19` | sintaxe por apóstrofes coberta (`_comum.md:216-222`); comportamento da forma-função não fixado |
| O12 | regra dos dois pontos em math calls | `00-index.typ:36-39` | nenhum prompt documenta a sintaxe especial de argumentos de math calls |
| O13 | modificadores de variantes (`phi.alt`) | `00-index.typ:17` | **coberto fora do conjunto** (`stdlib/sym.md:15-22`) — registado para não ser lido como lacuna |

### Os três usos de `\` em matemática (caso obrigatório do passo)

1. **Quebra de linha** (`00-index.typ:23`) — **documentado**: `_comum.md:672-686` (§P991),
   `eval.md:3248-3294` (§P996/P997).
2. **Escape de delimitador** contra matching/auto-scaling (`lr.typ:44,49`;
   `stretch.typ:12`) — **não documentado** em nenhum prompt (greps por
   `escape|backslash` em `engine/math/` só devolvem a quebra de linha).
3. **Escape de separador em math calls** (`f(a\, b)`, `mat(1\; 2)` — `00-index.typ:36-38`)
   — **não documentado** em nenhum prompt.

Nota adicional: `accent.typ:11` usa um quarto padrão relacionado, `\u{0303}` (escape de
codepoint), também sem documentação. E `engine/eval.md:3251` afirma explicitamente que o
uso 1 é o único — contradição C3.

### Ficheiros do corpus sem achados novos

`binom`, `roots`, `sizes`, `styles`, `variants` — cobertos; `cancel`, `attach`,
`underover` (parcial), `accent` (parcial) — lacunas com **scope-out declarado** (não
contadas como omissões silenciosas; a declaração de scope-out é o comportamento
correcto que O1/O3/O4/O6/O7/O9 deviam ter tido).

### Tabela-resumo do cruzamento

| Corpus | Contradições | Omissões |
|--------|---:|---:|
| 00-index | 1 (C3) | 2 (O12, uso 3 de `\`) |
| accent | 0 | 1 (O1) |
| attach / binom / cancel / roots / sizes / styles / variants | 0 | 0 |
| cases | 0 | 1 (O3) |
| class | 1 (C1) | 0 |
| equation | 0 | 1 (O10) |
| frac | 0 | 1 (O4) |
| lr | 0 | 2 (O5, uso 2 de `\`) |
| mat | 0 | 1 (O6) |
| op | 1 (C2) | 1 (O2) |
| primes | 0 | 1 (O11) |
| stretch | 0 | 1 (O8) |
| underover | 0 | 1 (O9) |
| vec | 0 | 1 (O7) |
| **Total** | **3** | **13** |


---

## Fora de âmbito (cumprido)

- Nenhum prompt foi corrigido — só catalogação.
- ADRs não auditadas. Nota colateral registada sem acção: ADR-0104 cita paths de
  `rules/` que hoje são `engine/` (rename confirmado em `6636c5ea6`) — o conteúdo da ADR
  continua correcto por correspondência, mas os paths estão obsoletos.
- Corpus de documentação não expandido além de `math/`.

## Anexos

- **Tabela completa por prompt (305 linhas, ordenada por gravidade combinada)**:
  secção «Tabela por prompt» abaixo. Dados brutos: `temp/p999/tabela-p999.csv`
  (por ocorrência: `temp/p999/auditoria.json`).
- Ordenação: violação grave de B primeiro; a seguir nº de secções C em falta; depois
  violação de A; depois volume de B.

## Tabela por prompt

| # | Prompt | A (nº fich.) | B total | B grave | C (faltam) | Hist. |
|---|--------|---|---|---|---|---|
| 1 | `entities/f_fronteira_e1.md` | 5 ⚠ | 131 | 131 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 2 | `engine/stdlib/foundations.md` | 2 ⚠ | 103 | 103 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 3 | `engine/atomizacao_elementos.md` | 30 ⚠ | 48 | 48 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 4 | `engine/stdlib/state.md` | 2 ⚠ | 25 | 25 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 5 | `testing/math_oracle.md` | 2 ⚠ | 9 | 9 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 6 | `engine/stdlib/counter.md` | 2 ⚠ | 7 | 7 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 7 | `engine/stdlib/context.md` | 2 ⚠ | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 8 | `infra.md` | 2 ⚠ | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 9 | `engine/math/layout/frac.md` | 1 | 39 | 39 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 10 | `shell/cli.md` | 1 | 39 | 39 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 11 | `entities/style.md` | 1 | 38 | 38 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 12 | `engine/math/layout/assembly.md` | 1 | 37 | 37 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 13 | `engine/stdlib/shapes.md` | 1 | 37 | 37 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 14 | `engine/math/layout/accent.md` | 1 | 35 | 35 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 15 | `engine/math/layout/underover.md` | 1 | 35 | 35 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 16 | `engine/stdlib/collections.md` | 1 | 36 | 34 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 17 | `engine/math/layout/attach.md` | 1 | 32 | 32 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 18 | `engine/stdlib/layout.md` | 1 | 32 | 32 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 19 | `infra/pipeline.md` | 1 | 32 | 32 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 20 | `engine/math/layout/stretchy.md` | 1 | 30 | 30 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 21 | `engine/math/layout/matrix.md` | 1 | 29 | 29 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 22 | `engine/math/layout/root.md` | 1 | 26 | 26 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 23 | `entities/show.md` | 1 | 26 | 22 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado | sim |
| 24 | `infra/layout_bidi.md` | 1 | 19 | 19 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 25 | `engine/math/layout/cases.md` | 1 | 17 | 17 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 26 | `engine/stdlib/text.md` | 1 | 17 | 17 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 27 | `wiring.md` | 1 | 15 | 15 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 28 | `entities/elements/bibliography.md` | 1 | 17 | 14 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado | sim |
| 29 | `engine/layout/table.md` | 1 | 14 | 14 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 30 | `entities/layouter_runtime_state.md` | 1 | 14 | 14 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 31 | `engine/stdlib/figure_image.md` | 1 | 13 | 13 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 32 | `entities/elements/table.md` | 1 | 11 | 11 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 33 | `infra/export/oracle.md` | 1 | 10 | 10 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 34 | `engine/layout/bibliography.md` | 1 | 12 | 9 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado | sim |
| 35 | `engine/math/layout/delimited.md` | 1 | 9 | 9 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 36 | `entities/elements/outline.md` | 1 | 9 | 9 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 37 | `engine/eval/table.md` | 0 (órfão) | 8 | 8 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 38 | `engine/layout/list_item.md` | 1 | 8 | 8 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 39 | `engine/stdlib/math_style.md` | 1 | 8 | 8 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 40 | `engine/layout/enum_item.md` | 1 | 7 | 7 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 41 | `engine/stdlib/assert.md` | 1 | 6 | 6 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 42 | `engine/stdlib/gradients.md` | 1 | 6 | 6 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 43 | `entities/elements/cite.md` | 1 | 6 | 5 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado | sim |
| 44 | `engine/math/layout/cancel.md` | 1 | 5 | 5 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 45 | `engine/math/layout/op.md` | 1 | 5 | 5 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 46 | `entities/elements/equation.md` | 1 | 5 | 5 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 47 | `entities/elements/footnote.md` | 1 | 5 | 5 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 48 | `shell/diagnostic.md` | 1 | 5 | 5 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 49 | `entities/sink.md` | 1 | 4 | 4 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 50 | `engine/layout/bib_csl.md` | 1 | 5 | 3 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado | sim |
| 51 | `core.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 52 | `engine/stdlib/transforms.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 53 | `entities/elements/columns.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 54 | `entities/elements/image.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 55 | `entities/elements/metadata.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 56 | `entities/elements/raw.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 57 | `entities/shaped_glyph.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 58 | `engine/stdlib/grid_hline.md` | 0 (órfão) | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 59 | `engine/stdlib/grid_vline.md` | 0 (órfão) | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 60 | `engine/stdlib/table_hline.md` | 0 (órfão) | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 61 | `engine/stdlib/table_vline.md` | 0 (órfão) | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 62 | `entities/elements/align.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 63 | `entities/elements/block.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 64 | `entities/elements/counter_display_callback.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 65 | `entities/elements/counter_update.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 66 | `entities/elements/grid.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 67 | `entities/elements/hide.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 68 | `entities/elements/ref.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 69 | `entities/elements/repeat.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 70 | `entities/elements/state.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 71 | `entities/elements/state_display.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 72 | `entities/elements/state_update.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 73 | `entities/engine.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 74 | `entities/mod.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 75 | `shell.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 76 | `engine/layout/link.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 77 | `engine/stdlib/label.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 78 | `engine/stdlib/numbering.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 79 | `engine/stdlib/ref.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 80 | `entities/elements/boxed.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 81 | `entities/elements/counter_display.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 82 | `entities/elements/figure.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 83 | `entities/elements/grid_cell.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 84 | `entities/elements/label.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 85 | `entities/elements/pad.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 86 | `entities/elements/place.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 87 | `entities/elements/quote.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 88 | `entities/elements/shape.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 89 | `entities/elements/smartquote.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 90 | `entities/elements/stack.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 91 | `entities/elements/table_cell.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 92 | `entities/elements/transform.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 93 | `infra/bibtex.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 94 | `infra/shaper.md` | 2 ⚠ | 94 | 82 | Contexto,Restrições,Instrução,ResultadoEsperado | sim |
| 95 | `engine/layout_references.md` | 2 ⚠ | 10 | 10 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 96 | `engine/parse.md` | 7 ⚠ | 7 | 7 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 97 | `engine/lang.md` | 3 ⚠ | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 98 | `entities/gradient.md` | 1 | 213 | 213 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 99 | `engine/introspect.md` | 1 | 207 | 207 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 100 | `engine/stdlib/structural.md` | 1 | 95 | 95 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 101 | `entities/layout_types.md` | 1 | 57 | 57 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 102 | `engine/stdlib/plugin.md` | 1 | 50 | 50 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 103 | `engine/stdlib/color.md` | 1 | 45 | 45 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 104 | `entities/geometry.md` | 1 | 40 | 40 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 105 | `entities/style_chain.md` | 1 | 38 | 38 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 106 | `engine/stdlib/calc.md` | 1 | 26 | 26 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 107 | `engine/stdlib/primitives-constructors.md` | 1 | 25 | 25 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 108 | `engine/stdlib/eval.md` | 1 | 22 | 22 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 109 | `engine/stdlib/loading.md` | 1 | 24 | 21 | Contexto,Restrições,Instrução,ResultadoEsperado | sim |
| 110 | `entities/color.md` | 1 | 18 | 18 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 111 | `entities/corners.md` | 1 | 14 | 14 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 112 | `entities/version.md` | 1 | 13 | 13 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 113 | `engine/layout/shape_block_behaviour.md` | 1 | 12 | 12 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 114 | `engine/layout_outline.md` | 1 | 12 | 12 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 115 | `entities/paint.md` | 1 | 12 | 12 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 116 | `entities/citation_form.md` | 1 | 11 | 11 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 117 | `entities/elements/list_item.md` | 1 | 11 | 11 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 118 | `engine/stdlib/_comum.md` | 1 | 10 | 10 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 119 | `entities/elements/enum_item.md` | 1 | 10 | 10 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 120 | `entities/dir.md` | 1 | 8 | 8 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 121 | `entities/elements/h_space.md` | 1 | 8 | 8 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 122 | `entities/elements/heading.md` | 1 | 8 | 8 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 123 | `passo-537b-set-page-columns.md` | 1 | 8 | 8 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 124 | `engine/layout-image.md` | 1 | 7 | 7 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 125 | `entities/bib_entry.md` | 1 | 7 | 7 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 126 | `engine/stdlib/sym.md` | 1 | 5 | 5 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 127 | `entities/parity.md` | 1 | 5 | 5 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 128 | `engine/layout_figure.md` | 1 | 4 | 4 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 129 | `engine/show-regex.md` | 0 (órfão) | 4 | 4 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 130 | `entities/axes.md` | 1 | 4 | 4 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 131 | `entities/decimal.md` | 1 | 4 | 4 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 132 | `entities/duration.md` | 1 | 4 | 4 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 133 | `entities/elements/link.md` | 1 | 4 | 4 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 134 | `entities/enum_numbering.md` | 1 | 4 | 4 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 135 | `entities/sides.md` | 1 | 4 | 4 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 136 | `entities/tiling.md` | 1 | 4 | 4 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 137 | `engine/stdlib/panic.md` | 1 | 3 | 3 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 138 | `engine/stdlib/tiling-stdlib.md` | 1 | 3 | 3 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 139 | `entities/bytes.md` | 1 | 3 | 3 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 140 | `entities/elements/divider.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 141 | `entities/elements/grid_hline.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 142 | `entities/elements/grid_vline.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 143 | `entities/elements/math_accent.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 144 | `entities/elements/math_cancel.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 145 | `entities/elements/math_op.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 146 | `entities/elements/overline.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 147 | `entities/elements/pagebreak.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 148 | `entities/elements/table_hline.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 149 | `entities/elements/table_vline.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 150 | `entities/image-sizer.md` | 1 | 3 | 3 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 151 | `entities/list_marker.md` | 1 | 3 | 3 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 152 | `entities/citation_style.md` | 1 | 4 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado | sim |
| 153 | `engine/eval/decimal-arithmetic.md` | 0 (órfão) | 2 | 2 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 154 | `entities/elements/_comum.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 155 | `entities/elements/colbreak.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 156 | `entities/elements/grid_footer.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 157 | `entities/elements/grid_header.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 158 | `entities/elements/linebreak.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 159 | `entities/elements/math_align_point.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 160 | `entities/elements/math_attach.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 161 | `entities/elements/math_cases.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 162 | `entities/elements/math_delimited.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 163 | `entities/elements/math_frac.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 164 | `entities/elements/math_matrix.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 165 | `entities/elements/math_root.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 166 | `entities/elements/math_styled.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 167 | `entities/elements/math_underover.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 168 | `entities/elements/strike.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 169 | `entities/elements/table_footer.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 170 | `entities/elements/table_header.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 171 | `entities/elements/term_item.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 172 | `entities/elements/terms.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 173 | `entities/elements/underline.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 174 | `entities/elements/v_space.md` | 1 | 2 | 2 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 175 | `entities/font_variations.md` | 1 | 2 | 2 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 176 | `engine/layout/raw_highlight.md` | 1 | 1 | 1 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 177 | `entities/elements/curve.md` | 1 | 1 | 1 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 178 | `infra/image-sizer.md` | 1 | 1 | 1 | Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 179 | `engine/eval.md` | 11 ⚠ | 322 | 322 | Restrições,Instrução,ResultadoEsperado |  |
| 180 | `engine/math/layout/_comum.md` | 2 ⚠ | 121 | 121 | Contexto,Instrução,ResultadoEsperado |  |
| 181 | `engine/lexer/mod.md` | 4 ⚠ | 3 | 3 | Restrições,Instrução,ResultadoEsperado |  |
| 182 | `entities/content.md` | 1 | 320 | 320 | Contexto,Instrução,ResultadoEsperado |  |
| 183 | `infra/font_metrics.md` | 1 | 101 | 93 | Restrições,Instrução,ResultadoEsperado | sim |
| 184 | `entities/region.md` | 1 | 75 | 72 | Instrução,CritériosVerif,ResultadoEsperado | sim |
| 185 | `engine/layout/equation.md` | 1 | 45 | 45 | Restrições,Instrução,ResultadoEsperado |  |
| 186 | `entities/math_constants.md` | 1 | 42 | 42 | Restrições,Instrução,ResultadoEsperado |  |
| 187 | `infra/fonts.md` | 1 | 47 | 37 | Restrições,Instrução,ResultadoEsperado | sim |
| 188 | `engine/math/layout/spacing.md` | 1 | 30 | 30 | Restrições,Instrução,ResultadoEsperado |  |
| 189 | `entities/func.md` | 1 | 29 | 29 | Restrições,Instrução,ResultadoEsperado |  |
| 190 | `infra/plugin_host.md` | 1 | 25 | 25 | Restrições,Instrução,ResultadoEsperado |  |
| 191 | `infra/measurements.md` | 1 | 23 | 23 | Instrução,CritériosVerif,ResultadoEsperado |  |
| 192 | `infra/system-world.md` | 1 | 27 | 22 | Restrições,Instrução,ResultadoEsperado | sim |
| 193 | `entities/page_store.md` | 1 | 23 | 22 | Instrução,CritériosVerif,ResultadoEsperado |  |
| 194 | `infra/query-helpers.md` | 1 | 24 | 20 | Instrução,CritériosVerif,ResultadoEsperado | sim |
| 195 | `contracts/plugin_host.md` | 1 | 20 | 20 | Restrições,Instrução,ResultadoEsperado |  |
| 196 | `entities/math_style.md` | 1 | 20 | 20 | Restrições,Instrução,ResultadoEsperado |  |
| 197 | `entities/sealed-positions.md` | 1 | 17 | 16 | Instrução,CritériosVerif,ResultadoEsperado |  |
| 198 | `engine/math/symbols.md` | 1 | 15 | 13 | Restrições,Instrução,ResultadoEsperado | sim |
| 199 | `entities/position.md` | 1 | 13 | 13 | Instrução,CritériosVerif,ResultadoEsperado |  |
| 200 | `engine/style/font-dict.md` | 0 (órfão) | 10 | 10 | Restrições,Instrução,ResultadoEsperado |  |
| 201 | `entities/plugin_func.md` | 1 | 10 | 10 | Restrições,Instrução,ResultadoEsperado |  |
| 202 | `contracts/world.md` | 1 | 9 | 9 | Restrições,Instrução,ResultadoEsperado |  |
| 203 | `entities/elements/math_limits_override.md` | 1 | 9 | 9 | Restrições,Instrução,ResultadoEsperado |  |
| 204 | `wiring/eviction.md` | 1 | 9 | 9 | Instrução,CritériosVerif,ResultadoEsperado |  |
| 205 | `entities/regex.md` | 1 | 9 | 8 | Instrução,CritériosVerif,ResultadoEsperado |  |
| 206 | `entities/args.md` | 1 | 8 | 7 | Restrições,Instrução,ResultadoEsperado | sim |
| 207 | `entities/glyph_variants.md` | 1 | 7 | 7 | Restrições,Instrução,ResultadoEsperado |  |
| 208 | `engine/stdlib/sys.md` | 1 | 6 | 6 | Restrições,Instrução,ResultadoEsperado |  |
| 209 | `entities/math-class.md` | 1 | 4 | 4 | Restrições,Instrução,ResultadoEsperado |  |
| 210 | `entities/module.md` | 1 | 4 | 4 | Restrições,Instrução,ResultadoEsperado |  |
| 211 | `entities/source.md` | 1 | 4 | 4 | Restrições,Instrução,ResultadoEsperado |  |
| 212 | `entities/symbol.md` | 1 | 4 | 4 | Restrições,Instrução,ResultadoEsperado |  |
| 213 | `engine/layout/heading.md` | 1 | 4 | 3 | Restrições,Instrução,CritériosVerif |  |
| 214 | `engine/stdlib/emoji.md` | 1 | 3 | 3 | Restrições,Instrução,ResultadoEsperado |  |
| 215 | `engine/stdlib/pdf.md` | 1 | 3 | 3 | Restrições,Instrução,ResultadoEsperado |  |
| 216 | `entities/elements/math_class_override.md` | 1 | 3 | 3 | Restrições,Instrução,ResultadoEsperado |  |
| 217 | `entities/font-list.md` | 1 | 3 | 3 | Restrições,Instrução,ResultadoEsperado |  |
| 218 | `engine/utils.md` | 1 | 2 | 2 | Restrições,Instrução,ResultadoEsperado |  |
| 219 | `entities/scope.md` | 1 | 2 | 1 | Restrições,Instrução,ResultadoEsperado | sim |
| 220 | `engine/layout.md` | 13 ⚠ | 224 | 224 | Instrução,ResultadoEsperado |  |
| 221 | `infra/export/builder.md` | 2 ⚠ | 71 | 58 | Instrução,ResultadoEsperado | sim |
| 222 | `engine/stdlib_audit_methodology.md` | 2 ⚠ | 4 | 3 | Instrução,ResultadoEsperado |  |
| 223 | `infra/package_downloader.md` | 2 ⚠ | 3 | 2 | Instrução,ResultadoEsperado |  |
| 224 | `engine/eval/ops.md` | 1 | 80 | 71 | Restrições,Instrução | sim |
| 225 | `entities/introspector.md` | 1 | 83 | 67 | Instrução,CritériosVerif | sim |
| 226 | `infra/export/stream.md` | 1 | 40 | 36 | Instrução,ResultadoEsperado | sim |
| 227 | `entities/element_payload.md` | 1 | 37 | 31 | Instrução,CritériosVerif | sim |
| 228 | `entities/element_kind.md` | 1 | 36 | 30 | Instrução,CritériosVerif | sim |
| 229 | `entities/selector.md` | 1 | 37 | 29 | Instrução,CritériosVerif | sim |
| 230 | `engine/columns.md` | 1 | 29 | 29 | Instrução,ResultadoEsperado |  |
| 231 | `entities/bib_store.md` | 1 | 29 | 24 | Instrução,CritériosVerif | sim |
| 232 | `entities/resolved_label_store.md` | 1 | 23 | 21 | Instrução,CritériosVerif | sim |
| 233 | `entities/label_registry.md` | 1 | 22 | 18 | Instrução,CritériosVerif | sim |
| 234 | `engine/introspect/from_tags.md` | 1 | 28 | 17 | Instrução,CritériosVerif | sim |
| 235 | `entities/counter_registry.md` | 1 | 20 | 16 | Instrução,CritériosVerif | sim |
| 236 | `infra/export/mod.md` | 1 | 16 | 15 | Instrução,ResultadoEsperado |  |
| 237 | `engine/introspect/extract_payload.md` | 1 | 18 | 13 | Instrução,CritériosVerif | sim |
| 238 | `world-types.md` | 1 | 14 | 13 | Instrução,ResultadoEsperado | sim |
| 239 | `infra/export/images.md` | 1 | 12 | 10 | Instrução,ResultadoEsperado |  |
| 240 | `engine/introspect/locatable.md` | 1 | 15 | 9 | Instrução,CritériosVerif | sim |
| 241 | `entities/counter_update.md` | 1 | 12 | 9 | Instrução,CritériosVerif | sim |
| 242 | `entities/image-format.md` | 1 | 12 | 9 | Instrução,ResultadoEsperado | sim |
| 243 | `entities/font_book.md` | 1 | 11 | 8 | Instrução,ResultadoEsperado | sim |
| 244 | `entities/state_registry.md` | 1 | 11 | 8 | Instrução,CritériosVerif | sim |
| 245 | `entities/locator.md` | 1 | 10 | 8 | Instrução,CritériosVerif | sim |
| 246 | `entities/state_update.md` | 1 | 10 | 8 | Instrução,CritériosVerif | sim |
| 247 | `entities/tag.md` | 1 | 10 | 8 | Instrução,CritériosVerif | sim |
| 248 | `entities/counter_state_legacy.md` | 1 | 12 | 7 | Instrução,CritériosVerif | sim |
| 249 | `entities/content_hash.md` | 1 | 8 | 6 | Instrução,CritériosVerif | sim |
| 250 | `engine/introspect/fixpoint.md` | 1 | 8 | 5 | Instrução,CritériosVerif | sim |
| 251 | `entities/location.md` | 1 | 7 | 5 | Instrução,CritériosVerif | sim |
| 252 | `infra/export/gradients/adaptive.md` | 1 | 6 | 5 | Instrução,ResultadoEsperado |  |
| 253 | `engine/introspect/convergence.md` | 1 | 6 | 4 | Instrução,CritériosVerif | sim |
| 254 | `engine/footnote_overflow_columns.md` | 1 | 4 | 4 | Instrução,ResultadoEsperado |  |
| 255 | `infra/font_variant.md` | 1 | 4 | 4 | CritériosVerif,ResultadoEsperado |  |
| 256 | `entities/element_info.md` | 1 | 5 | 3 | Instrução,CritériosVerif | sim |
| 257 | `entities/metadata_store.md` | 1 | 5 | 3 | Instrução,CritériosVerif | sim |
| 258 | `infra/export/fonts.md` | 1 | 4 | 3 | Instrução,ResultadoEsperado |  |
| 259 | `infra/export/tests.md` | 1 | 4 | 3 | Instrução,ResultadoEsperado |  |
| 260 | `infra/export/svg.md` | 1 | 3 | 3 | Instrução,ResultadoEsperado |  |
| 261 | `infra/export/gradients/cmyk.md` | 1 | 3 | 2 | Instrução,ResultadoEsperado |  |
| 262 | `infra/export/gradients/mod.md` | 1 | 3 | 2 | Instrução,ResultadoEsperado |  |
| 263 | `infra/package_version_resolution.md` | 0 (órfão) | 3 | 2 | Instrução,ResultadoEsperado |  |
| 264 | `infra/export/gradients/conic.md` | 1 | 2 | 1 | Instrução,ResultadoEsperado |  |
| 265 | `infra/export/gradients/radial.md` | 1 | 2 | 1 | Instrução,ResultadoEsperado |  |
| 266 | `infra/export/gradients/relative.md` | 1 | 2 | 1 | Instrução,ResultadoEsperado |  |
| 267 | `infra/export/render.md` | 1 | 1 | 1 | Instrução,ResultadoEsperado |  |
| 268 | `world.md` | 1 | 1 | 1 | Instrução,ResultadoEsperado | sim |
| 269 | `entities/value.md` | 1 | 45 | 34 | Instrução | sim |
| 270 | `infra/export-fixtures.md` | 1 | 27 | 26 | ResultadoEsperado |  |
| 271 | `entities/ast/mod.md` | 1 | 3 | 1 | ResultadoEsperado | sim |
| 272 | `engine/scopes.md` | 1 | 21 | 19 | — | sim |
| 273 | `infra/export/font_subset.md` | 1 | 23 | 13 | — | sim |
| 274 | `infra/embedded_fonts.md` | 1 | 16 | 11 | — | sim |
| 275 | `entities/syntax-node.md` | 1 | 7 | 4 | — | sim |
| 276 | `infra/fontdb.md` | 1 | 8 | 3 | — | sim |
| 277 | `entities/label.md` | 1 | 3 | 1 | — | sim |
| 278 | `entities/span.md` | 1 | 3 | 1 | — | sim |
| 279 | `entities/syntax-mode.md` | 1 | 3 | 1 | — | sim |
| 280 | `entities/syntax-set.md` | 1 | 3 | 1 | — | sim |
| 281 | `entities/operators.md` | 1 | 2 | 1 | — | sim |
| 282 | `entities/document_info.md` | 1 | 0 | 0 | Contexto,Restrições,Instrução,CritériosVerif,ResultadoEsperado |  |
| 283 | `engine/eval/cast.md` | 1 | 2 | 0 | Contexto,Restrições,Instrução,ResultadoEsperado | sim |
| 284 | `engine/layout_counters.md` | 1 | 0 | 0 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 285 | `infra/layout.md` | 1 | 0 | 0 | Contexto,Restrições,Instrução,ResultadoEsperado |  |
| 286 | `entities/counter_format.md` | 1 | 1 | 0 | Restrições,Instrução,CritériosVerif |  |
| 287 | `engine/math/mod.md` | 1 | 0 | 0 | Restrições,Instrução,ResultadoEsperado |  |
| 288 | `engine/mod.md` | 1 | 0 | 0 | Restrições,Instrução,ResultadoEsperado |  |
| 289 | `engine/scanner.md` | 1 | 0 | 0 | Restrições,Instrução,ResultadoEsperado |  |
| 290 | `entities/lang.md` | 1 | 0 | 0 | Restrições,Instrução,ResultadoEsperado |  |
| 291 | `entities/source-result.md` | 1 | 0 | 0 | Restrições,Instrução,ResultadoEsperado |  |
| 292 | `entities/rel.md` | 1 | 2 | 0 | Restrições,Instrução | sim |
| 293 | `entities/ptr-eq-arc.md` | 1 | 1 | 0 | Instrução,ResultadoEsperado |  |
| 294 | `infra/export/gradients/function_dict.md` | 1 | 1 | 0 | Instrução,ResultadoEsperado |  |
| 295 | `infra/export/gradients/linear.md` | 1 | 1 | 0 | Instrução,ResultadoEsperado |  |
| 296 | `package-spec-dto.md` | 0 (órfão) | 0 | 0 | Instrução,ResultadoEsperado | sim |
| 297 | `entities/ast/code.md` | 1 | 2 | 0 | — | sim |
| 298 | `entities/ast/expr.md` | 1 | 2 | 0 | — | sim |
| 299 | `entities/ast/markup.md` | 1 | 2 | 0 | — | sim |
| 300 | `entities/ast/math.md` | 1 | 2 | 0 | — | sim |
| 301 | `entities/file-id.md` | 1 | 2 | 0 | — | sim |
| 302 | `entities/package-spec.md` | 1 | 2 | 0 | — | sim |
| 303 | `entities/syntax-kind.md` | 1 | 2 | 0 | — | sim |
| 304 | `entities/syntax-text.md` | 1 | 2 | 0 | — | sim |
| 305 | `template-prompts.md` | 0 | 0 | 0 | — | sim |

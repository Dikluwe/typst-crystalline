# Passo 343 — relatório: inventário do bake (medição; o de-bake decide-se sobre os números)

> **Veredito.** Existem **4 campos assados** num nó de `Content` (medidos da fonte,
> não da narrativa): o `TextStyle` de `Content::Text` (10 sub-campos) + 3 de
> numbering (`heading`, `equation`, `figure`). **Vaza** para a linguagem: o
> `TextStyle` (medido, Achado 2 — via `==` **e** `it.body`); os 3 de numbering só via
> `==` (estreito; `equation`/`figure` nem têm `get_field`). O `TextStyle` **decompõe**
> em duas naturezas: o **bold de heading** é redundante (o layout re-deriva em
> `:1293`) e é a raiz do Achado 2 → **des-assável já**; as props de **`#set text`**
> (size/fill/weight/…) são **load-bearing** (o `#set` só muta a chain, que morre na
> fronteira — o campo assado é o **único** transporte) → **interseção com o F-5,
> esperam**. Os 3 de numbering são bakes **intencionais** (F-2, fecharam canais
> globais) — **não** são alvos de de-bake. Inventário content-preserving: zero
> produto tocado, suíte **2719 / 3238**, lint **0/0**.

## Setup / pré-condição

- HEAD = `04383f433` (commit do relatório P342). O passo pediu `3a02223f9`; a
  diferença é **só o `.md` do P342**. `git diff --name-only 03619dc93 HEAD` não toca
  `.rs`/`.toml` → produto **content-idêntico** ao P340. Pré-condição substantiva ok.
- Suíte: `typst-core --lib` = **2719**; workspace = **3238** (P342, intactas).
- `crystalline-lint .` = **0/0**.
- Árvore: zero modificação rastreada.
- Vanilla `lab/typst-original` (`typst 0.14.2`, binário do P341) como referência;
  `lab` só lido/executado.

---

## Fase A — a tabela do inventário do bake (da fonte, `file:line`)

| # | Campo assado | Onde assa (`file:line`) | Consumidor (`file:line`) | Nó ou chain? | Vaza p/ linguagem? | Testes que asseveram (`file:line`) | Interseção c/ multi-passe | Vanilla (onde a prop vive) |
|---|---|---|---|---|---|---|---|---|
| **1** | **`Content::Text` → `TextStyle`** (10 sub-campos: bold, italic, size, fill, heading_level, weight, tracking, leading, lang, font) | `content.rs:122` (decl); assado em `eval/mod.rs:310,320,368`, `eval/markup.rs:118-119` via `TextStyle::from(&StyleChain)` (`style_chain.rs:320`) | `layout/mod.rs:610-625` — **combina** o `node_style` (assado) com a chain `self.style`: `bold: node||chain` (`:610`), `size: chain se maior senão node` (`:612-616`), `fill: chain.or(node)` (`:617`)… | **ambos** (lê o nó **e** a chain) | **SIM — render→linguagem** (Achado 2, medido). Via `==` (arm `Text`, `content.rs:1711`) **e** acesso a campo `it.body` (`heading.rs:77` devolve o body cru) | `eval/tests.rs:1627` (`eval_set_text_bold`), `:1655`,`:1664`,`:2380`,`:2393` (escopo `#set text(bold)`); PartialEq de `Text` em `content.rs` | **divide** (ver decomposição abaixo): heading-bold **não** intersecta; props de `#set text` **intersectam** o F-5 | styling é `#[ghost]` (chain-resolved): `text/mod.rs:75-76` (`font`), só `pub text: EcoString` (`:755`) é conteúdo — **nunca assa** (`repr([a])=[a]`, P342) |
| **2** | **`HeadingElem.numbering_active: bool`** | `heading.rs:33` (decl); assado em `eval/markup.rs:90-98` de `engine.styles.custom("heading.numbering")` (`Content::heading_numbered`, `content.rs:1055`) | `layout/mod.rs:714` (`h.numbering_active`); `introspect.rs:817` (gate auto-TOC, `:457-463`) | **nó** | **estreito — só `==`** (PartialEq de `HeadingElem`); **não** via `get_field` (expõe só `body`/`level`, `heading.rs:77-79`) | `eval/tests.rs:248-268` (`find_heading_numbered`, asserção `=true`), `:279` (`=false`), `:282-284` | — (bake **intencional** F-2 S1/S5, fechou o canal global `SetHeadingNumbering`/StateRegistry; valor do contador segue via Introspector `formatted_counter_at`) | vanilla resolve numbering da chain no realize; o bake crist foi escolha que **substituiu estado global pior** |
| **3** | **`EquationElem.numbering_active: bool`** | `equation.rs:29` (decl); assado em `eval/mod.rs:524-528` de `engine.styles.custom("equation.numbering")` (`Content::equation_numbered`, `content.rs:1184`) | `layout/mod.rs:812` (`e.numbering_active`); `introspect.rs:657` (`*block && *numbering_active`) | **nó** | **estreito — só `==`**; `equation.rs` **não** tem `get_field` (campos não expostos a `#show`) | `eval/tests.rs` (família numbering, `:282-284`); `introspect`/layout gates | — (bake intencional F-2 S2, idem #2) | idem #2 |
| **4** | **`FigureElem.numbering: Option<String>`** | `figure.rs:26` (decl); assado em `eval/closures.rs:79-83` de `engine.styles.custom("figure.numbering")`, armazenado em `stdlib/figure_image.rs:90` (`Content::figure`) | `layout/mod.rs:860-862` (`caption_prefix`); `introspect.rs:412-414` (`is_counted`) | **nó** | **estreito — só `==`**; `figure.rs` **não** tem `get_field` | `figure.rs:94`,`:105-147` (`is_counted` Some/None) | — (bake intencional F-2 S3, removeu o campo global `engine.figure_numbering`, carona C2 F-3) | vanilla guarda o **pattern** na chain (`#set figure(numbering:)`), resolve no realize |

**Total: 4 campos assados** (o #1 decompõe em 10 sub-valores). Os 65 outros módulos de
elemento **não têm** campo assado — todos os seus campos são estruturais/autorais
(body, level, block, width, colspan, url, stroke explícito…), confirmado por varredura
da fonte dos 68 módulos.

### Caso de fronteira (valor resolvido na criação, não é campo de struct)
- **Glyph de SmartQuote** (`eval/mod.rs:317-338`): o glyph (`«»`/`""`/…) é escolhido do
  `engine.styles.lang()` **no eval** e assado como o **conteúdo** de um `Content::Text`
  (+ o `TextStyle` do #1). Não é um campo de struct — é um valor resolvido-na-criação.
  Em vanilla, `SmartQuoteElem` resolve no realize a partir do `lang` da chain. Vaza só
  na medida em que o texto resultante difere; nota, não linha da tabela.

### Decomposição do #1 (o que torna o fatiamento possível)
O `node_style` assado é lido **junto** com a chain do layout (`:610-625`). Logo:
- **bold de heading** (delta `eval/markup.rs:85`): o layout **re-deriva** independente
  (`:1293` `push_styles(Bold)`, combinado `:610` `||`) → o assado é **transporte
  redundante**. É **a raiz medida do Achado 2** (`it.body`=`Text("a",{bold:true})` vs
  `[a]`=`{bold:false}`). **Des-assável já.**
- **bold/italic de `*…*`/`_…_`** (deltas `:58`,`:71`): emitem `Content::Styled([Bold/Italic])`
  e o layout empurra do `Styled` (`:1248-1251`) → o bold no `Text` interno é redundante;
  **não** adiciona vazamento novo (`Styled` vs `Text` já é cross-variant em `==`).
- **props de `#set text`** (size/fill/weight/tracking/leading/lang/font): o `eval_set_rule`
  **só muta a chain** (`rules.rs:362,503`) e **não emite nó** — a chain **morre** na
  fronteira eval→layout (P342, A1). O campo assado é o **único** transporte destas
  props para o layout. **De-bake exige o `#set text` emitir um nó de transporte
  primeiro** → **interseção com o F-5/F-realização** → **esperam.**

---

## TRAVA ARQUITETURAL — checkpoint com a tabela

### Quantos / quais vazam / quais esperam (vereditos medidos)
- **Campos assados: 4** (#1 com 10 sub-valores). Sem "~".
- **Vazam para a linguagem: 4.** Faces: #1 via `==` **e** `it.body` (medido, render→linguagem);
  #2/#3/#4 só via `==` (estreito; #3/#4 nem expõem `get_field`).
- **Des-assável com segurança já:** o **bold de heading** dentro do #1 (redundante +
  raiz do Achado 2).
- **Esperam pela interseção (F-5):** as **props de `#set text`** dentro do #1 (transporte
  único; precisam do `#set` emitir nó antes).
- **Fora do escopo de de-bake:** #2/#3/#4 (numbering) — bakes **intencionais** (F-2)
  que fecharam canais globais; de-assar **reabriria** estado global morto. Revisitar só
  se a fidelidade de `==` sobre numbering virar requisito.

### Proposta de fatiamento do de-bake (opções; o dono decide)
- **Fatia 1 — `TextStyle`/bold de heading (resolve o Achado 2; caminho crítico da recursão P341b).**
  Parar de assar o delta bold do heading (`eval/markup.rs:85`) no `Text` do corpo (ou
  excluí-lo do `==`). Redundante para o render (`:1293` re-deriva). Efeito: `it.body == [a]`
  passa a **casar** (e desbloqueia parcialmente o `m1` do P341b). **Não é
  content-preserving** — muda o `==` de propósito; roda com a regra do P340 (paridade
  vs vanilla compilado; testes contraditos evoluídos **um a um** com a saída do vanilla
  colada). Testes a rever: os que fixam bold assado no corpo do heading; o
  `it.body==[a]` vira verde. Risco a verificar no lote: o `:1293` cobre **toda** a
  subárvore do heading, não só o topo.
- **Fatia 2 — props de `#set text` (espera o F-5).** De-assar size/fill/weight/tracking/
  leading/lang/font exige o `#set text` emitir um nó de transporte (estilo `Styled`)
  para o layout ler da chain. Interseção com o F-realização restante; superfície maior.
  Testes a rever: `eval/tests.rs:1627,1655,1664,2380,2393` (escopo `#set text`) +
  layout de size/fill — evoluídos contra o vanilla.
- **Fatia 0 (não-fatia) — numbering (#2/#3/#4).** Deixar como está; intencional. Só
  entra se a fidelidade de `==`/`it.numbering` for pedida — e aí **na chain**, não
  reabrindo canal global.

### Caráter do de-bake declarado
**Não é content-preserving** — muda comportamento de propósito (`it.body == [a]` passa a
casar). Regra do P340: paridade contra o vanilla compilado; cada teste contradito
evoluído **um a um** com a saída do vanilla colada como justificativa; os que o vanilla
**confirma**, intactos.

Parar aqui. Nenhum código de produção, nenhum teste alterado.

---

## Mapa de filtro (campo novo — lugar lógico na versão destilada)

> **O inventário do bake vem junto com a introdução do `#set`** — é onde a forma
> **assado-vs-chain** se decide. Quem materializa o `#set text`/`#set heading` tem de
> escolher, ali, se a prop vive no nó (assado) ou na chain (resolvida no realize); essa
> escolha é a origem de todos os 4 campos desta tabela.

**Onde o conhecimento entrou de verdade (rastro):**
- **Passo 22** — a raiz marcada (rich text / estilo em markup).
- **Passo 30** — o bake construído (`Content::Text(_, TextStyle)`; `TextStyle::from(chain)`).
- **~Passo 102** — o rastro do bake perdeu-se na narrativa de continuidade (a deriva que
  este inventário existe para não repetir — por isso a regra "ler a fonte, não o relatório").
- **P335 (F-2)** — os 3 bakes de numbering introduzidos **de propósito** (fecharam canais
  globais); distintos do bake acidental de estilo.
- **P342** — a **face de igualdade** do vazamento medida pela primeira vez (Achado 2), e a
  redundância do bold de heading (layout re-deriva).
- **P343 (aqui)** — o inventário completo: 4 campos, a decomposição load-bearing vs
  redundante, a ordem segura de de-bake.

---

## Item aberto carregado

`content→elements → 0` — **fora da fila, sem dono**. Baseline da lente `content→elements = 66`,
`target = 0`, que nenhum lote entrega. Três saídas (decisão, não bloqueio):
**reconciliar o baseline** (Modelo D tem `≠ 0` por desenho) / **nomear marco pós-F-6** /
**registrar lacuna** do plano.

---

## Verificação (gates)

```
content-preserving: zero código de produção, zero teste alterado, zero ficheiro de
  produto tocado. Suíte 2719 / 3238 inalterada.
lint: crystalline-lint . = 0 violations, 0 warnings.
medição reproduzível: greps/leituras registrados (file:line na tabela); vanilla
  typst 0.14.2 (binário do P341, lab só lido). Zero "~" na tabela.
lente: não corrida (read-only; nada de produto muda) — inalterada vs P340.
```

Nenhum commit de código. Entregável = tabela do inventário + vereditos + proposta de
fatiamento (opções), para decisão do dono na TRAVA.

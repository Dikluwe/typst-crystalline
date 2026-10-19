# Passo 342 — relatório: auditoria da separação linguagem/render (medição)

> **Veredito.** A fronteira **eval→layout** é limpa (só a árvore `Content` cruza;
> `engine.styles` morre). A **igualdade de conteúdo VAZA** (`render→linguagem`): o
> `==` do `Content` observa o **`TextStyle` assado eager** dentro de `Content::Text`
> (Passo 30). O **Achado 2** (`it.body == [a]` → crist NOMATCH, vanilla MATCH) é
> **esse vazamento medido** — e **não** é o wrapper β1 `Styled` (a hipótese-líder do
> passo) nem proveniência de parse (o `Content` não tem span). O conserto cai na
> **camada de linguagem (igualdade/bake do estilo)**, fora do render e fora do
> transporte. A recursão do P341b só se remede **depois**. Auditoria
> content-preserving: zero produto tocado, suíte **2719 / 3238**, lint **0/0**.

## Setup / pré-condição

- HEAD = `3a02223f9`. O passo pediu `03619dc93` (P340), mas os commits desde então
  (`1b9b8ad1c` P341, `3a02223f9` P341b) **só adicionam `.md` de relatório** —
  `git diff --name-only 03619dc93 HEAD` não toca nenhum `.rs`/`.toml`. Produto
  **content-idêntico** ao P340. Pré-condição substantiva satisfeita.
- Suíte: `typst-core --lib` = **2719 passed**; workspace = **3238 passed**.
- `crystalline-lint .` = **No violations found** (0/0).
- Árvore: zero modificação rastreada (só `.md` de materialização untracked + `lab/`
  build-artifacts pré-existentes).
- Binários: cristalino `target/debug/typst`; vanilla
  `lab/typst-original/target/debug/typst` (`typst 0.14.2`, do P341 — `lab` só
  executado, não editado).

---

## Fase A — o mapa da fronteira (ponto × veredito × direção × `file:line`)

| # | Ponto da fronteira | O que cruza / quem lê | Veredito | Direção | `file:line` |
|---|---|---|---|---|---|
| **A1** | **eval → layout** | só a árvore `Content`; `engine.styles` **morre**; o layout reconstrói a sua própria chain a partir do tree | **LIMPO** | — | `layout/mod.rs:2644` (`layout(content: &Content)`); chain reconstruída em `:1248-1251` (`Styled`→`push_styles`) e `:1293` (bold de heading re-derivado) |
| **A2** | **igualdade de `Content`** | `PartialEq` manual; arm `Text` compara `(texto, TextStyle)`; arm `Styled` compara `(body, Styles)` | **VAZAMENTO** | **render→linguagem** | `content.rs:1707` (impl), `:1711` (arm `Text` → `a==b && sa==sb`), `:1790` (arm `Styled`), `:1833` (`_ => false`) |
| **A2′** | **igualdade de `Value`** | `#[derive(PartialEq)]` → delega ao `==` de `Content` | **VAZAMENTO** (herdado de A2) | render→linguagem | `value.rs:17` |
| **A3** | **`query` / introspecção (walk)** | `Content::Styled` é **transparente** — desce no `body`, ignora os `styles` (`_`) | **LIMPO** | — | `introspect.rs:1204` |
| **A4** | **matching de `#show`** | por `Selector::NodeKind`/`DynKind` (por kind, **não** por igualdade de árvore); inspeciona `Styled.delta().bold/italic` para casar `strong`/`emph` (modelo ADR-0038 `*bold*`→`Styled`, **semântico, por desenho**); **ignora** o custom de numbering do β1 | **LIMPO** | — | `rules.rs:104` (guard por `RuleId`), `:113-115` (`is_bold/italic_styled`), `:119-127` (selector match) |
| **A5** | **acesso a campo `it.body`** | devolve `self.body.clone()` **cru** — carrega o `TextStyle` assado; **sem stripping/normalização** | **CONDUTA do vazamento A2** | render→linguagem | `heading.rs:77` (`"body" => Some(Value::Content(self.body.clone()))`) |

### Cross-check estrutural (lente) — não corrido
Opcional; não muda nada (read-only). O acoplamento `content↔style` já é conhecido
(megaciclo-90, baseline P340 `219|676|[90,4]|66|0`) e é **estrutural**, não o
vazamento **comportamental** que esta auditoria mede — não confundir (caveat do
passo). Baseline da lente inalterado (nada de produto mudou).

---

## Fase B — testes de vazamento (medidos)

### B1 — igualdade de conteúdo depende de artefato de render? **SIM — VAZAMENTO render→linguagem.**

Par mínimo do Achado 2 (`= a`):
- `it.body` = corpo do heading. `eval_heading` empurra `StyleDelta { bold: Some(true) }`
  (`markup.rs:85`) antes de avaliar o corpo; o `Content::Text` **assa o estilo ativo**
  (`mod.rs:310-311`, `TextStyle::from(&*engine.styles)`, Passo 30). Resultado:
  `Content::Text("a", TextStyle{ bold: true, … })`.
- `[a]` = bloco fresco. `Expr::ContentBlock` avalia com `engine.styles.clone()` (sem
  bold) (`mod.rs:503-515`). Resultado: `Content::Text("a", TextStyle{ bold: false, … })`.
- **Wrapping idêntico nos dois lados**: ambos passam por `eval_markup_body` →
  `Content::sequence(parts)`, que **desembrulha o singleton** (`content.rs`:
  `1 => parts.into_iter().next().unwrap()`). Logo os dois são `Content::Text` puro —
  **a única diferença é o campo `bold`** do `TextStyle`.
- `==`: arm `Text` (`content.rs:1711`) faz `a==b && sa==sb`. `"a"=="a"` true; `sa==sb`
  **false** (`bold: true` ≠ `bold: false`) → **NOMATCH**.

Medição (binários):

| `.typ` | vanilla | cristalino |
|---|---|---|
| `#show heading: it => if it.body == [a] {[EQ]} else {[NE]}` · `= a` | **EQ** | **NE** |
| `#show heading: it => [#repr(it.body)]` · `= a` | **`[a]`** | `repr` inexistente |
| `#repr([a])` (topo) | **`[a]`** | — |

O `repr` do vanilla é **`[a]` dos dois lados** — o vanilla **não assa estilo** no
elemento (o estilo vive na style-chain, aplicado no realize), então a igualdade de
conteúdo não o observa → EQ. O cristalino assa-o no nó e o `==` (e `it.body`) o
observam.

**Reforço:** o layout cristalino **re-deriva** o bold do heading independentemente
(`layout/mod.rs:1293`), logo o `TextStyle` assado no `Text` é **transporte
redundante** que mesmo assim **polui a igualdade da linguagem**. Isto fixa a direção
como `render→linguagem`.

### B2 — o wrapper β1 (`Styled`) é observável pela linguagem?

`Content::Styled` é **sobrecarregado**: (i) `*bold*`/`_italic_` → `Styled(body,[Bold/Italic])`
(semântico, ADR-0038, **observável por desenho** via `#show strong/emph`); (ii)
transporte β1 do `#set …(numbering:)` local (`mod.rs:402`). Por operação de
linguagem, só o **β1-numbering** interessa aqui:

| operação | β1-numbering observável? | `file:line` |
|---|---|---|
| `==` | **SIM** (arm `Styled` compara `body && styles`; embrulhado ≠ desembrulhado por cross-variant) — vazamento estreito | `content.rs:1790` |
| `query`/introspect | **NÃO** (transparente) | `introspect.rs:1204` |
| `#show` matching | **NÃO** (só reage a bold/italic; ignora o custom numbering) | `rules.rs:113-127` |
| `plain_text` | **NÃO** (transparente) | `content.rs:1668` |
| `map_content`/`map_text` | preserva o wrapper, recursa o body (não expõe styles como conteúdo) — neutro | `content.rs:2028,2198` |

**Veredito B2:** o wrapper β1-numbering é observável pela linguagem **apenas via `==`
estrutural**, e só no caso estreito de um `#set …(numbering:)` local mid-body. **Não
é** a causa do Achado 2 (o `= a` não tem nó `Styled`). É uma superfície de vazamento
**segunda e mais estreita**, no mesmo lugar (a definição de igualdade).

### B3 — a fronteira eval→layout não realimenta. **CONFIRMADO LIMPO.**
`layout(content: &Content)` é unidirecional: consome `&Content`, devolve
`PagedDocument`/frames; nada do render volta a influenciar decisão de avaliação. A
interceção eager (`intercept_content`) corre **durante** o eval, antes do layout.

### B4 — a realização eager é transformação de conteúdo, não leitura de render. **CONFIRMADO LIMPO.**
`intercept_content(content: Content, …) -> SourceResult<Content>` (`rules.rs:193`) →
`apply_show_rules` opera puramente sobre `Content`; **não lê estado de layout**.

---

## Diagnóstico do Achado 2

**Vazamento `render→linguagem` — mas não o suspeito nomeado pelo passo.**

- **Causa medida:** o **`TextStyle` assado eager** dentro de `Content::Text` (Passo 30,
  `mod.rs:310-311`), induzido pelo delta `bold:true` do heading (`markup.rs:85`),
  observado pelo arm `Text` da igualdade (`content.rs:1711`) e exposto cru por
  `it.body` (`heading.rs:77`).
- **Refuta a hipótese-líder do passo (§B.1):** **não** é o wrapper β1 `Styled` (o `= a`
  não produz `Styled`) **nem** proveniência de parse (`span`/origem — o enum
  `Content` **não tem** campo de span; grep zero). É o **campo de estilo** da variante
  `Text`.
- **É vazamento, não semântica de linguagem (§B.1 alternativa):** os dois lados são
  **markup analisado** com **estrutura idêntica** (vanilla `repr` = `[a]` nos dois;
  desembrulho de singleton igual nos dois). Não há divergência analisado≡construído
  nem de wrapping — só o estilo de render assado. Logo **não** é o modelo de valor a
  não-normalizar como o vanilla; é o `==` a observar transporte de render.
- **Comparação com o vanilla (referência do observável-na-linguagem):** vanilla mantém
  o estilo na style-chain (aplicado no realize, fora da árvore); `it.body` ≡ `[a]` →
  **MATCH**. Cristalino assa → **NOMATCH**.

---

## Recomendação de camada para o conserto (sem executar)

**O conserto cai na camada de linguagem — tirar o artefato de render da igualdade.**
Duas miras, ambas fora do render e fora do transporte:

1. **A igualdade de `Content` (e a superfície `it.body`) não observar o estilo de
   render assado.** Forma fiel ao vanilla: o estilo não pertence ao elemento — vive na
   chain e aplica-se no realize. Conserto estreito hoje: o arm `Text` comparar texto +
   só o estilo **observável-na-linguagem** (se algum), ou — alinhado ao F-5 — **não
   assar** o estilo no nó (chain-authoritative). Isto alinha o `==` cristalino ao
   vanilla. Mira: `content.rs:1711` + o ponto de bake `mod.rs:310-311` / o delta
   `markup.rs:85`.
2. **Reabre o gatilho do wrapper β1 (P339), na camada certa — mas como item separado.**
   O β1-numbering é uma **segunda** superfície de vazamento, **só via `==`** (`content.rs:1790`),
   estreita. Tratar junto **se/quando** o dono reabrir, no mesmo lugar (igualdade
   ignora wrappers de transporte). **Não** é a causa do Achado 2 — não confundir.

**NÃO é conserto de semântica de linguagem** (o modelo de valor comparar como o vanilla
promete): medido que as estruturas **são** idênticas módulo o estilo assado — a falha
é o `==` observar o transporte, não o modelo não-normalizar.

### O que o mapa diz sobre a recursão (P341b)
O `m1` do P341b (`it.body == [a]` como guard auto-limitante) falha **por este
vazamento de igualdade**, não pelo guard. Logo:
- O **guard por-regra** (mecanismo de render) só se remede **depois** do conserto da
  igualdade. Um mini-fixpoint por-conteúdo **sem** consertar a igualdade ainda daria
  `m1` = "a" (NOMATCH → `else { it }` → identidade).
- **Ordem:** primeiro a igualdade (linguagem); a fidelidade de recursão (P341b opção B)
  é downstream e só observável depois de `it.body == [a]` casar.

---

## Item aberto carregado

`content→elements → 0` — **fora da fila, sem dono**. Baseline da lente mede
`content→elements = 66`, espera `target = 0`, que nenhum lote entrega. Três saídas
(decisão, não bloqueio): **reconciliar o baseline** (Modelo D tem `≠ 0` por desenho) /
**nomear marco pós-F-6** / **registrar lacuna** do plano.

---

## Verificação (gates)

```
content-preserving: zero código de produção, zero teste alterado, zero ficheiro de
  produto tocado (lab só executado, não editado). Suíte 2719 / 3238 inalterada.
lint: crystalline-lint . = 0 violations, 0 warnings.
medição reproduzível:
  crist:    target/debug/typst <probe>.typ <out>.pdf ; pdftotext
  vanilla:  lab/typst-original/target/debug/typst compile <probe>.typ <out>.pdf
  probes (em /tmp, não-produto):
    p342probe.typ:  #show heading: it => if it.body == [a] {[EQ]} else {[NE]} / = a
    p342repr.typ:   #show heading: it => [#repr(it.body)] / = a / #repr([a])
  binários do P341; vanilla 0.14.2.
lente: não corrida (read-only; nada de produto muda) — inalterada vs P340.
```

Nenhum commit de código. Entregável = mapa da separação + diagnóstico do Achado 2 +
recomendação de camada, para decisão do dono na TRAVA.

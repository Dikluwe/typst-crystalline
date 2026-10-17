# Prompt L0 — Style e Styles
Hash do Código: a312cea9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:f10f7afaabd1b97f3048736842df89d0ba48d4daa0ae3cd453c9be317f32ef91

## Módulo
`01_core/src/entities/style.rs`

## Propósito

Definir o vocabulário tipado de propriedades de estilo para o motor
Typst cristalino em L1. Fundação para `#set`/`#show` e para
`Content::Styled` (ADR-0038).

## Contrato

### Enum `Style`

Variantes obrigatórias (Passo 99.A + **P288**):

- `Bold(bool)` — forma booleana interna de `text.weight` (**não** existe `text.bold`)
- `Italic(bool)` — forma booleana interna de `text.style` (**não** existe `text.italic`)
- `Size(Length)` — propriedade `text.size` declarada como abs + em
- `Fill(Color)` — propriedade `text.fill` (forward-compat)

> **Correcção P1031 — duas variantes nomeavam propriedades que a linguagem não tem.**
>
> A tabela dizia `Bold(bool)` → *"propriedade `text.bold`"* e `Italic(bool)` →
> *"propriedade `text.italic`"*. Não existem. O vanilla ratificado (`e0e8ca4d`) expõe
> `crates/typst-library/src/text/mod.rs`:
>
> | Propriedade da linguagem | `file:line` | Default | Superfície |
> |---|---|---|---|
> | `text.style` (`FontStyle`) | `text/mod.rs:223-226` | `Normal` | `#text(style: "italic")`, `"oblique"` |
> | `text.weight` (`FontWeight`) | `text/mod.rs:249-252` | `Regular` | `#text(weight: 500)`, `#text(weight: "bold")` |
> | `text.size` (`TextSize`) | `text/mod.rs:292` | `#[default(TextSize(Abs::pt(11.0).into()))]` = **11pt** | `#set text(size: …)` |
> | `text.fill` (`Paint`) | `text/mod.rs:314` | `#[default(Color::BLACK.into())]` = **preto** | `#set text(fill: …)` |
> | `text.lang` (`Lang`) | `text/mod.rs:473` | `#[default(Lang::ENGLISH)]` = **`en`** | `#set text(lang: "pt")` |
>
> **Medição de confirmação (2026-08-13)** — `#set text(bold: true)`:
>
> | Binário | Saída |
> |---|---|
> | Vanilla `/usr/local/bin/typst` (`typst 0.15.1 (e0e8ca4d)`) | `error: unexpected argument: bold` (col. 10) |
> | Cristalino `target/release/typst` (fonte em HEAD `4f64e4e69`) | `error: unexpected argument: bold` (col. 10) |
>
> **Mesma mensagem, mesma coluna** — a *superfície* do cristalino já está correcta; era só a
> etiqueta neste L0 que estava errada. `Bold`/`Italic` são representação booleana interna
> das duas propriedades reais, e a redução de `FontWeight`/`FontStyle` a `bool` é
> simplificação declarada (perde `oblique` e os pesos numéricos, que a variante
> `Weight(u16)` cobre em parte). Não é achado de código; é achado de redacção, corrigido
> aqui.
>
> **Os defaults de `size`, `fill` e `lang` da tabela acima passam a ser a referência deste
> L0.** Nota: o default de `lang` **é `en`** e o cristalino diverge — achado escalado em
> `compiler/layout_figure.md` §P1031 (ACHADO 2).
- `HeadingLevel(u8)` — propriedade `heading.level` (forward-compat)
- **`Lang(Lang)`** — propriedade `text.lang` (Passo 288 — fecha
  assimetria Tabela B.3 vs B.4; `StyleDelta.lang` existia desde
  P130/P131B/P144 mas escrito apenas por parse-driven `eval_set_rule`).
  Activa 2ª fonte de entrada via `Content::Styled(body, Styles::from_iter(
  [Style::Lang(Lang::PT)]))` — desbloqueia testes lang-aware adiados em
  P287. Diagnóstico `diagnostico-style-lang-passo-288.md`.
- **`Weight(u16)`** — propriedade `text.weight` (Passo 289 — fecha
  1/4 da assimetria residual P288 §7 risco terciário; paralelo
  arquitectural absoluto a `HeadingLevel(u8)` e `Lang(Lang)`).
  `StyleDelta.weight: Option<u16>` existia desde P126/P129 mas escrito
  apenas por parse-driven `eval_set_rule` (`#set text(weight: 700)` ou
  nome simbólico via `FontWeight::from_name`). Storage raw `u16` (range
  0-1000 CSS/OpenType; sem validação per spec não-objectivo). Activa 2ª
  fonte de entrada via `Content::Styled(body, Styles::from_iter(
  [Style::Weight(700)]))`. Consumer faux-bold P139
  (`TextStyle::faux_bold_stroke_pt`) reusado sem alteração — **primeira
  aplicação directa de ADR-0098** (single source of truth como invariante
  anti-bug; hash `export.rs 66cb8ac3` preservado pelo 6º passo
  consecutivo). Diagnóstico `diagnostico-style-weight-passo-289.md`.
- **`Tracking(Length)`** — propriedade `text.tracking` (Passo 290 — fecha
  1/3 da assimetria residual P289 §5.6; paralelo absoluto a P288/P289).
  `StyleDelta.tracking: Option<Length>` existia desde P127/P137 mas
  escrito apenas por parse-driven `eval_set_rule` em `eval/rules.rs:374`.
  `Length` preserva `abs + em` (P127). Activa 2ª fonte de entrada via
  `Content::Styled(body, Styles::from_iter(
  [Style::Tracking(Length::em(0.1))]))`. **Distinção crítica vs P288/P289**:
  consumer P137 inclui emit real (`Tc` operator em `export.rs:2139-2146`)
  — mas via paradigma `FrameItem::Text.style.tracking` capturado pelo
  Layouter (`TextStyle::from(&chain)`), **não via `chain.tracking()`
  directo**. P290 confirma **ADR-0098 robusta mesmo com emit consumer
  real**: o critério não é "ausência total em emit" mas "via
  `FrameItem.style` capturado". Hash `export.rs 66cb8ac3` preservado
  pelo **7º passo consecutivo**. Diagnóstico
  `diagnostico-style-tracking-passo-290.md` (5 secções A.0-A.5 com
  A.0 não-trivial).
- **`Leading(Length)`** — propriedade `text.leading` (Passo 291 — fecha
  1/2 da assimetria residual P290 §5.6; resta apenas `font`).
  `StyleDelta.leading: Option<Length>` existia desde P128/P138 mas
  escrito apenas por parse-driven `eval_set_rule` em `eval/rules.rs:298`.
  Activa 2ª fonte de entrada via `Content::Styled(body, Styles::from_iter(
  [Style::Leading(Length::em(0.65))]))`. **Paradigma consumer distinto
  vs Tracking** (per-line, não per-glyph): `cursor.rs:119-128` em
  `flush_line` faz peek do último `FrameItem::Text` da `current_line`
  (`iter().rev().find_map`) — lê `style.leading.resolve_pt(font_size)`
  e adiciona `line_height + leading_pt` a `cursor_y` antes do drain.
  `export.rs` zero hits para leading — ADR-0098 vigente; hash
  `66cb8ac3` preservado pelo **8º passo consecutivo**.
  **Divergência arquitectural consciente**: vanilla typst tem `leading`
  em `par`; cristalino captura em `text` por conveniência temporária
  (Tabela A.3 linha 70 + 184; sem `Content::Par` propriamente). P291
  preserva esta divergência.
  Diagnóstico `diagnostico-style-leading-passo-291.md` com **6 secções**
  A.0-A.5 + A.5' anti-reflexão (mitigação activa do risco "sequência
  reflexa" P290 §7 — elemento estructuralmente novo identificado:
  paradigma peek `current_line` per-line distinto vs tracking per-glyph).
- **`Font(FontList)`** — propriedade `text.font` (Passo 292 — **fecha
  1/1 final da assimetria residual P289 §5.6; pós-P292 série cirúrgica
  P288-P292 termina naturalmente — assimetria 5/5 fechada**).
  `StyleDelta.font: Option<FontList>` existia desde P140B/P141/P146.
  Activa 2ª fonte de entrada via `Content::Styled(body, Styles::from_iter(
  [Style::Font(FontList::single("Inter"))]))`.
  **Distintivo arquitectural vs P288-P291**: `FontList` é
  `pub struct FontList(Vec<FontFamily>)` — **não é `Copy`**. P292 escolhe
  opção (a) `Font(FontList)` apesar disso — **`Style` enum perde `Copy`
  derive** (A.2.0 inventário literal confirma 0 call sites dependem de
  `*style`; perda inofensiva). Cascade arm usa `f.clone()` em vez de
  `*f` por necessidade material.
  **Paradigma consumer "indirect resolution via FontBook"** (2 layers):
  `FrameItem::Text.style.font` capturado pelo Layouter; emit multifont
  (`export.rs:2169-2174`) consulta `style.font.as_ref()` + `fonts.iter()
  .position(|f| match name)` → `/F{i+1} Tf` (PDF font select). ADR-0098
  vigente. Hash `export.rs 66cb8ac3` preservado pelo **9º passo
  consecutivo**.
  Diagnóstico `diagnostico-style-font-passo-292.md` (6 secções A.0-A.5
  + A.5' N=2 anti-reflexão).
- **`Subscript(bool)`** / **`Superscript(bool)`** — Passo 448. Propriedades
  de texto para `#sub[...]` / `#super[...]`. Modeladas como `Style` (não
  variants próprios de `Content`) para reaproveitar a cadeia de estilos;
  o Layouter aplica `size *= 0.6` e `baseline_offset` em `layout/text.rs`.
- **`Highlight(Option<Color>)`** — Passo 449. Propriedade de texto para
  `#highlight[...]`. `Some(color)` activa o fundo colorido; `None`
  desactiva explicitamente (herança cortada por `fill: none`). O Layouter
  emite um `FrameItem::Shape` rectangular por detrás de cada run de texto
  que resolva com `TextStyle.highlight = Some(color)`.

**Marco arquitectural P292**: pós-P292, **série cirúrgica P288-P292
termina naturalmente** — não há mais campos `StyleDelta` sem variant
`Style` correspondente (assimetria 5/5 fechada). O Passo 448 mantém-se
ortogonal (dois novos pares `Style`/`StyleDelta`).

Derive pós-P292: `Debug, Clone, PartialEq`. **`Copy` removido por
`FontList: !Copy`** (`Style::Font(FontList)`). Inventário A.2.0
confirma 0 call sites afectados — perda inofensiva. Clone preserved
para `cascade.push_styles(&Styles)` walk.

### Struct `Styles` — **fachada sobre `StyleDelta`** (Lote F-4, P338)

**Colapso da dualidade de backing** (ADR-0106 fronteira E1; correção do recon
P337 — não era "2ª chain", era backing duplo). Antes do F-4 `Styles` armazenava
`Vec<Style>` e a `StyleChain` armazenava `StyleDelta`, com `push_styles` a
**converter** entre os dois — duas representações da mesma informação. F-4
colapsa: `Styles` passa a **embrulhar um único `StyleDelta`** (o backing único),
de modo que `Content::Styled` e a `StyleChain` carregam **a mesma representação**
(zero conversão dupla).

- `Styles { delta: StyleDelta }` — uma fachada tipada sobre o backing único.
- O enum `Style` permanece como **vocabulário-construtor**: `from_iter`/`push`
  **dobram** cada `Style` no campo correspondente de `StyleDelta` (last-write-wins
  por campo — semântica idêntica à projeção antiga). A projeção `Style→StyleDelta`
  (o `match` exaustivo das 10 variantes) vive em `Styles::from_iter`/`push`.
- Métodos: `new()`, `push(Style)`, `from_iter<I: IntoIterator<Item = Style>>(iter)`,
  `is_empty()` (delegado a `StyleDelta::is_empty`), `delta() -> &StyleDelta`,
  **`from_delta(StyleDelta) -> Self`** (P352, show-set): constrói uma `Styles`
  envolvendo um `StyleDelta` já dobrado — o inverso de `delta()`. Usado pela
  captura do show-set (`#show k: set …`) para carregar o efeito do `set` num
  `Content::Styled` (`f_fronteira_e1.md §3a.8`, `entities/show.md`).
  **`push_custom(key, value) -> Self`** (F-realização fatia 1, P339),
  **`is_semantically_empty() -> bool`** (P345, ADR-0107): nenhum campo tipado
  (bold/italic/size/…) definido, **ignorando** o canal `custom`. Distingue um
  `Content::Styled` semântico (`*bold*` → mantém na morfologia) de um de puro
  transporte β1 (só `custom` → transparente no `==` morfológico).
- **`iter()` e `len()` removidos** — não há mais `Vec<Style>` armazenado; os
  consumidores lêem `delta()` (campos tipados), não uma lista de variantes.
- **Canal `custom` (F-2) disponível por construção**: o `StyleDelta` embrulhado
  já tem `custom: Vec<(EcoString, Value)>`, de modo que `#set` pode viajar no
  `Content::Styled` quando F-realização/F-5 o exigirem (a porta fica aberta).
- **`Styles::push_custom(key, value)` (F-realização fatia 1, P339)**: dobra uma
  entrada `(key, value)` no `StyleDelta.custom` embrulhado — **espelho** de
  `StyleChain::push_custom` (`style_chain.rs:175`), o mecanismo que permite a
  `#set …(numbering:)` embrulhar o escopo num `Content::Styled` carregando o
  custom (transporte aditivo β1, governado por `f_fronteira_e1.md §3a.8`).
  **Não** introduz variante no enum `Style`: o canal custom é `(key, value)` por
  decisão do §3a.8 (resolve o fork "forma de partida" do §3b.1 a favor da
  representação `(key, value)` já implantada no F-2). O baking atual permanece
  autoritativo até o F-5; caminho duplo chain↔assado com paridade testada.

Derive: `Debug, Clone, Default, PartialEq`.

## Invariantes

- Sem I/O (pureza L1).
- Sem dependência de `LazyHash` (ADR-0016 preservada).
- Sem proc macros custom (ADR-0026 como precedente).

## Consumidores

- `Content::Styled(Box<Content>, Styles)` — variante de `Content` (carrega o
  backing único via a fachada; tipo da variante **inalterado** no F-4).
- `StyleChain::push_styles(&Styles)` — pós-F-4 empurra `styles.delta().clone()`
  (a projeção `Style→StyleDelta` mudou-se para `Styles::from_iter`; o método
  permanece como fachada fina, sem o `match`).
- Deteção de `#show` (`eval/rules.rs`) lê `styles.delta().bold/italic` — não
  mais `iter()` sobre variantes.
- Pipeline futuro de `#set`/`#show` — activação fora do Passo 99.


## Traits Rust de formatação/construção — proposta 2026-10-16

### Medição anterior à decisão

O inventário anterior tem should_implement_trait em Styles.from_iter e inherent_to_string nos três tipos. As quatro implementações e os L0 vigentes foram lidos; fontes/hashes/HEAD/working tree/UTC/diffstat estão no base.json.gz deste pacote. Os dois parsers Option e os dois next de domínio foram classificados separadamente; não são alvo de conversão para traits. A sonda duration-before-probe e a correção interna anterior documentam a faixa i128 sem copiar o defeito. É mudança de contrato Rust, não mudança de linguagem Typst ou nova paridade medida.

### Contrato proposto

Implementar std::iter::FromIterator<Style> para Styles. Mover o mesmo fold ordenado de Style para StyleDelta ao trait, sem segundo backing/clones/reordenação. Conservar o método inerente público from_iter<I: IntoIterator<Item=Style>>(iter)->Self como adapter para <Self as FromIterator<Style>>::from_iter(iter). Preservar last-write-wins por campo, vazio, canais custom e todas as outras APIs. A implementação do trait resolve a ambiguidade nominal sem expect neste método.

O contrato só materializa após confirmação humana ADR-0127 §2.1/2.4: existem três novos impls Display, um FromIterator e migração de três métodos inerentes públicos para o blanket ToString. Código genérico pode passar a depender desses traits; rollback pode quebrar esse código. A implementação não deve adicionar FromStr, novas variantes/campos, flags, defaults, fase, dependências ou supressões. Os comentários/exemplos antigos de to_string passam a significar a interface ToString, somente após ratificação.

### Aceitação

T1: usos genéricos Display+ToString e collect::<Styles> devem falhar antes por trait ausente e passar depois. T2: Decimal exibe 1.2300 sem perder zeros, preserva negativo/zero e mantém rejeições Option; Version exibe vazio e 1.2.0 preservando componentes. T3: Duration mantém textos corrigidos de MIN/MAX/MIN+1, zero e limites negativos, via format! e ToString. T4: collect vazio é vazio e Weight(400),Weight(700) produz weight Some(700), com demais campos inalterados; adapter inerente mantém chamadas existentes. T5: suíte existente de linguagem/casts/repr/show/CLI, workspace completo, build, Clippy/expectativas, Rustdoc, formato e linhagem, sob 12 GiB, swap zero e um job, sem relaxar esperados. Sem publicar release.

## Ratificação humana — 2026-10-16

Proposta aprovada pelo humano nesta sessão, manifesto SHA-256 `d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8`. O contrato acima passa a vigente; a aprovação inclui materialização e testes T1–T5, sem release. Registro em `00_nucleo/diagnosticos/clippy-enums-2026-10-16/approval-receipt.json`.

## Adaptador inerente — previsão corrigida após medição 2026-10-16

### Medição anterior à decisão

A primeira validação deste lote, após ratificação e implementação, terminou no gate do inventário Clippy: first-validation/validation-clippy.log em 00_nucleo/diagnosticos/clippy-enums-2026-10-16 contém should_implement_trait em Styles::from_iter mesmo com FromIterator<Style> implementado. Os quatro testes approved_traits passaram, incluindo collect::<Styles> vazio e a dobra ordenada de Weight. traits-green-result.json registra comando, UTC, cgroup e retorno; first-validation/validation-details.json preserva hashes, HEAD, working tree e diffstat. Portanto a previsão anterior de que apenas implementar o trait eliminaria este aviso foi refutada na ferramenta instalada (Clippy/Rust 1.92). Não é ausência do trait nem evidência de ambiguidade na linguagem Typst.

### Decisão e aceitação

Manter o adaptador inerente e sua assinatura exatamente como ratificados. Corrigir somente a previsão anterior de ausência de expect: prescrever #[expect(clippy::should_implement_trait, reason="S005: adapter inerente de compatibilidade; FromIterator<Style> implementado e validado por collect")] no adaptador. O corpo continua delegando integralmente ao trait; nenhum fold duplicado ou supressão no trait/impl/módulo/crate. O atributo deve ser cumprido com unfulfilled_lint_expectations como erro; collect e chamadas inerentes continuam validados. É adendo interno de documentação/metadados em fluxo contínuo ADR-0127; nenhum contrato público ratificado é alterado. A cláusula anterior sem expect e a previsão de eliminação automática ficam substituídas apenas por esta medição/decisão. L0 primeiro e resselo restrito; suíte completa e gates sob 12 GiB, swap zero e um job.



## TS1 — tamanho completo de texto (ratificado pelo humano, 2026-10-17)

### Medição anterior à decisão

`00_nucleo/diagnosticos/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação

O humano aprovou TS1 e os dois contratos públicos da proposta de manifesto
771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba.
Implementação e testes S1–S16 sob 12 GiB/swap zero; sem release/merge/push.
Resselo somente após implementação e grafo válido. RG1 continua independente.

### Responsabilidade exclusiva adicional

Style::Size recebe Length completo; fold_into transfere esse valor para StyleDelta.size. Coleção/backing único, demais variantes, campos e métodos permanecem. Sem um segundo size paralelo ou marcador falso para codificar em. Construtores absolutos antigos migram explicitamente para Length::pt(pt.val()).

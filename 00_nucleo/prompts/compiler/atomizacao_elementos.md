# Prompt L0 — Atomização dos elementos (layout/introspect → arquivo do elemento)

Hash do Código: 734522ea

**Camada**: L1 · **Módulos afetados**: `01_core/src/compiler/layout/mod.rs` (o monólito
`layout_content`), `01_core/src/compiler/introspect.rs` (o walk), e os arquivos dos elementos
`01_core/src/entities/elements/<elem>.rs`.
**Decisão de origem**: **ADR-0109** (atomização — mover a lógica para o arquivo da unidade;
`match` exaustivo + estático + imports ficam; NÃO é desacoplar `content→elements`). Complementa
ADR-0026 (enum fechado sem vtable, **satisfeita**) e ADR-0105 (modelo D, exaustividade cl.3,
**mantida**). Disciplina: ADR-0107 (content-preserving) + ADR-0108 (medir antes de decidir).
**Tipo**: especificação de atomização. **Forward-looking**: descreve o desenho + a medição; a
implementação é **por fatias, pós-Trava**, com **hash humano por fatia**. **Content-preserving**:
a lógica muda de arquivo, **não** muda de comportamento (oráculo: a rede de caracterização +11,
P331).

> **Estatuto: MATERIALIZADO — fatia família-containers (P376).** O dono aprovou **Opção B** (§4) e
> a **fatia containers** (§5); o hash foi sincronizado (`crystalline-lint --fix-hashes`) e o código
> movido para `compiler/layout/{block,boxed,stack,pad}.rs`. Suíte verde (content-preserving), lint 0/0. Os
> lotes seguintes (Figure/Image/… e o resto) ficam para passos futuros, decisão do dono.

---

## §0 — Estatuto e sincronização de hash

Materializado pela fatia containers (§5): os 4 arquivos declaram
`@prompt rules/atomizacao_elementos.md` + `@prompt-hash <hash deste ficheiro>`, sincronizados por
`crystalline-lint --fix-hashes .` (V7 órfão resolvido). Lotes futuros que materializem mais
elementos re-sincronizam se o L0 mudar.

---

## §1 — A medição do monólito (a fonte vence; `file:line`)

### `layout/mod.rs` — `fn layout_content` (`01_core/src/compiler/layout/mod.rs:527-2383`)
- **1857 linhas, 59 arms bespoke, SEM wildcard (exaustivo).** [medido P375 + P376]
- Os arms mais gordos (linhas): `Block` 296 (`:1798`), `Boxed` 198 (`:1600`), `Overline` 93
  (`:2220`), `Stack` 84 (`:1516`), `Place` 66 (`:1266`), `Pad` 57 (`:1418`), `Transform` 49
  (`:1006`), `Heading` 44 (`:758`), `Columns` 44 (`:2118`), `Image` 41 (`:1215`), `Figure` 34
  (`:935`), `Shape` 33 (`:973`). [medido]
- Os arms math (`MathAccent`/`MathCancel`/… `:891-920`) são **agrupados** e descem ao path math
  (`rules/math/layout/`), fora deste monólito — **fora desta fatia.** [medido `:903`]

### `introspect.rs` — o walk (`01_core/src/compiler/introspect.rs:156`)
- **43 arms nativos**; cada elemento aparece também em `materialize_time` (`:176`),
  `extract_payload` (`:422`), e o walk principal (`:828`). [medido]

### O hub (`content.rs`)
- Já é **delegação magra** (os 6 métodos `is_empty/plain_text/eq/get_field/map_content/map_text`
  delegam a `e.metodo()` via a infra que o P375 mediu já existir). **Nada gordo a atomizar no hub
  além da delegação que já está lá.** [medido P375]

### O que os arms leem (a lógica é inteira, não um pedaço) — ex.: `Heading` (`:758-798`)
Lê o **estado privado do `Layouter`**: `self.font_size_pt`, `self.style`, `self.regions`,
`self.page_config.margin`, `self.chain` (`custom("heading.numbering")`), `self.current_location`,
`self.introspector` (`formatted_counter_at`), e chama `self.flush_line()` + `self.layout_content()`.
[medido `:769-797`] **A lógica depende inteiramente do estado local do layouter** — mover exige
passar/expor esse estado (ver §3).

---

## §2 — A forma canónica da delegação (ADR-0109, forma B)

Antes (monólito): `Content::Heading(h) => { /* 44 linhas */ }`.
Depois (magro, exaustivo, estático): `Content::Heading(h) => heading::layout(self, h)`, com a
lógica numa **free function** `pub(super) fn layout<M,S>(layouter, h)` em `compiler/layout/heading.rs`
(camada de render). **O `match` fica magro (1 linha/arm); a jump table, a exaustividade e os
imports ficam.** *(§3/§4 abaixo registam por que a Opção A — método no struct — foi rejeitada.)*

---

## §3 — O custo medido da Opção A (por que foi rejeitada) — o achado para o dono

Mover a lógica para `entities/elements/heading.rs` (a Opção A, que o P376 herdou da ADR antes da
correção P377) **exige três coisas** (medido, marcado para o dono decidir — ADR-0108):

1. **Import reverso `entities → rules::layout::Layouter`.** Hoje **nenhum** elemento chama o
   `Layouter` (as menções em `math_*.rs` são comentário) [medido]. Cria um **ciclo de módulos**
   `entities ↔ rules` **dentro de L1** — mecanicamente permitido (Rust aceita ciclos intra-crate;
   `crystalline-lint` opera na topologia de **camadas**, não em ciclos intra-L1 → fica **0/0**),
   mas é uma inversão estrutural nova.
2. **Alargar a visibilidade do `Layouter`.** Os campos lidos (`style`, `regions`, `font_size_pt`,
   …) e métodos (`flush_line`, `layout_content`) são privados [medido `:84-218`]. O método no
   elemento exige `pub(crate)` neles — **alarga a superfície de API** do layouter.
3. **Threading de genéricos.** `Layouter<'a, M: FontMetrics, S: ImageSizer>` é genérico
   [medido `:84`] → o método vira `fn layout<M: FontMetrics, S: ImageSizer>(&self, lo: &mut
   Layouter<'_, M, S>)` em cada elemento.

**Nenhum desses é violação da ADR-0109** (não exige `dyn` nem wildcard; o despacho fica estático e
exaustivo). Mas os três são **custo real** que o dono deve aceitar conscientemente.

---

## §4 — O fork A/B (decisão do dono na Trava)

- **Opção A — elemento-dono (REJEITADA pela ADR-0109 corrigida no P377).** `impl HeadingElem { fn
  layout(&self, lo) }` em `heading.rs`. **Prós:** "abrindo só o arquivo do elemento" no sentido literal — struct
  + Element + layout + introspect juntos. **Contras:** o custo §3 (ciclo + `pub(crate)` +
  genéricos).
- **Opção B — arquivo de layout por-elemento.** `compiler/layout/heading.rs` com
  `pub(super) fn layout<M,S>(lo, h: &HeadingElem)`. **Prós:** atomiza o monólito de 1857 linhas em
  ~59 arquivos pequenos **sem** o ciclo nem o alargamento de visibilidade (mesmo módulo-árvore;
  segue a separação domínio/render do Typst vanilla). **Contras:** a lógica de layout do elemento
  vive ao lado do layout, **não** no mesmo arquivo da definição do struct (a leitura é por-feature,
  não por-elemento-único).

**Ambas mantêm o `match` exaustivo + estático + os imports** → ambas satisfazem as não-metas da
ADR-0109. A diferença é **onde** o arquivo atomizado mora e o custo §3.

> **ESCOLHA DO DONO (P376): Opção B.** A lógica de layout de cada elemento move para
> `compiler/layout/<elem>.rs` (`pub(super) fn layout<M,S>(lo: &mut Layouter<'_,M,S>, e: &XElem)`);
> o arm no monólito vira `Content::X(e) => elem::x::layout(self, e)`. **Sem** import reverso, **sem**
> `pub(crate)` novo, **sem** ciclo (mesmo módulo-árvore `rules::layout`) — o custo §3 **não se
> paga**. A Opção A (lógica no arquivo do struct) é a **forma rejeitada** pela ADR-0109 (corrigida no
> P377): cria o acoplamento dado→render.

---

## §5 — Escopo e ordem (a fatia aprovada)

> **ESCOLHA DO DONO (P376): fatia família-containers.** O Estágio 1 move os **4 arms mais gordos**:
> `Block` 296 linhas (`:1798`), `Boxed` 198 (`:1600`), `Stack` 84 (`:1516`), `Pad` 57 (`:1418`) =
> **~635 linhas** → `compiler/layout/{block,boxed,stack,pad}.rs`. Maior encolhimento imediato do
> monólito. **Aditivo-neutro**: a rede +11 e a suíte passam **sem asserção virada**.

- **Ordem dentro da fatia**: do maior para o menor (`Block` → `Boxed` → `Stack` → `Pad`), cada arm
  migra independente (a exaustividade fica intacta o tempo todo).
- **Depois** (lotes futuros, decisão do dono): `Figure`/`Image`/`Shape`/`Transform`; depois o resto.
- **Fora desta fatia**: os arms math (path `rules/math/layout/`, agrupados); a varredura do projeto
  inteiro; a decisão de crates (todas decisão do dono, **depois**).

---

## §6 — Não-metas confirmadas (ADR-0109)

- **`match` exaustivo MANTIDO** (sem wildcard) — a garantia do compilador fica. [medido: os 59
  arms são sem wildcard hoje]
- **Despacho ESTÁTICO** — sem `dyn`/vtable/PropMap. A ADR-0026 fica **satisfeita**, não emendada.
- **Imports FICAM** — `content→elements = 68` (P374) **não é gate** e não se mexe; a Opção A
  **adiciona** um import reverso (§3.1), a Opção B não.
- **Content-preserving** — a rede de caracterização +11 e a suíte passam **sem asserção virada**;
  qualquer viragem = a lógica mudou ao mover → **investigar, não mascarar**.
- **INTACTOS**: α/caso 2, `morph_canon`/`==`, caso 4, flag P350c, F-5b (fechado), os 3 numbering,
  o `#set` de props de usuário.

---

## §7 — Fatia P377: elementos visuais (forma B)

> **Aprovado pelo dono (P377): fatia elementos visuais.** Atomizados na forma B (free function em
> `compiler/layout/<elem>.rs`):
> - `Heading` (`:765`, 44 linhas) → `compiler/layout/heading.rs` — lê `chain`/`introspector`/
>   `current_location`/`style`/`regions` (estado diferente dos containers; prova a forma em quem lê
>   o Introspector).
> - `Transform` (`:1013`, 49 linhas) → `compiler/layout/transform.rs`.
> - `Shape` (`:980`, 33 linhas) → `compiler/layout/shape.rs`.
> - `Columns` (`:1544`, 44 linhas) → `compiler/layout/columns.rs`.
>
> Arm magro: `Content::Heading(h) => heading::layout(self, h)` (idem os outros 3). As free functions
> acedem ao estado privado do `Layouter` por **descendência de módulo**; helpers livres
> (`heading_scale`/`resolve_pt`/`measure_content`/`collect_sub_items`) e a const
> `COLUMNS_DEFAULT_GUTTER_RATIO` via `super::`. **Sem** import reverso, **sem** `pub(crate)`.
>
> **Leitura:** `layout_content` 1276 → 1126 linhas (−150; −731 acumulado desde P376).
> **Não-metas (medido):** `match` exaustivo (0 wildcards), despacho estático (0 `dyn`), `entities/`
> não tocado. Suíte verde (rede +11 sem asserção virada). Precedente seguido: `figure.rs`/`image.rs`.

---

## §8 — Fatia P378: visuais/decorações restantes (forma B)

> **Aprovado pelo dono (P378): fatia Place + Image + Figure + Decorações.** Forma B.
> - `Place` (`:1166`, 58 linhas) → **novo** `compiler/layout/place.rs` — lê `cell_align`/
>   `available_width`/`layout_sub_frame_with_width`/`floats_pending`/`cursor_y_*_reserve`/
>   `layout_place`.
> - `Underline`/`Strike`/`Overline` (arm agrupado `:1502`, 75 linhas) → **novo**
>   `compiler/layout/decorations.rs` — free function recebe `&Content` e re-match interno (1.º arm
>   agrupado atomizado); lê `font_size_pt`/`style.fill`/`regions`/`decoration_lines_collector`.
> - `Image` (`:1115`, ~36 linhas) → **completa** `compiler/layout/image.rs` (`pub(super) fn layout`
>   junto do helper `calculate_dimensions` já lá).
> - `Figure` (`:909`, ~31 linhas de cálculo de prefixo) → **completa** `compiler/layout/figure.rs`
>   (dobra o prefixo + chama o `layout_figure` existente).
>
> Arm magro: `Content::Place(e) => place::layout(self, e)` (idem). Estado privado do `Layouter` por
> descendência; helpers via `super::`. **Sem** import reverso, **sem** `pub(crate)`, **sem** `dyn`.

### Inventário — o que falta para fechar o `layout_content` (medido P378)
- **Movidos até aqui** (P376+P377+P378): Block/Boxed/Stack/Pad, Heading/Transform/Shape/Columns,
  Place/Image/Figure/Decorações(3) = **15 unidades**.
- **Restam não-math** (lotes futuros, mesma forma B): Quote 46, Cite 35, Colbreak 33, TermItem 25,
  SmartQuote 25, Repeat 25, Pagebreak 25, VSpace 23, Divider 19, Bibliography 19, Table 18, EnumItem
  18, Hide 15, Raw 15, ListItem 14, TableCell 12, TableFooter 11, Footnote 8, Terms 7, Ref 7, Link 6,
  família Grid (~30), HSpace 4. Plus o **core/infra** (Text 82, Sequence 51, Styled 14, Dynamic 31,
  SetPage 25, família state/counter) — a decidir se atomiza ou fica como motor.
- **Math** (fatia FINAL própria): `Equation` + arm agrupado de 16 variantes (`:865`) = **17
  variantes** → descem a `rules/math/layout/`. Fora de todas as fatias não-math.

---

## §9 — Fatia 1/2 P380: fluxo de bloco e estrutura (forma B, por-elemento)

> **Aprovado pelo dono (P380): granularidade POR-ELEMENTO** (um arquivo por elemento, como os 12
> fat já feitos; o dono aceitou os ficheiros header-dominados dos 1-liners). Forma B.
>
> **Listas** → `compiler/layout/{list_item,enum_item,terms,term_item}.rs` (lê fluxo-texto: `regions`/
> `page_config`/`flush_line`/`font_size_pt`/`style`/`layout_content`; TermItem lê `chain`).
> **Tabelas/Grid** (cluster `layout_grid`) → `Grid` dobra em `compiler/layout/grid.rs` (junto de
> `layout_grid`); `Table` → `table.rs` (chama `lo.layout_grid`, pub(super) visível por descendência);
> `grid_header`/`grid_footer`/`grid_cell`/`table_cell`/`table_header`/`table_footer` → arquivos
> próprios (1-liner `layout_content(&e.body)`).
> **Breaks** → `pagebreak.rs` (lê `regions`/`flush_line`/`new_page`/`pages`), `colbreak.rs`.
> **Spacing** → `h_space.rs`, `v_space.rs`, `repeat.rs`.
>
> Arm magro: `Content::EnumItem(e) => enum_item::layout(self, e)`. Estado privado do `Layouter` e o
> método `layout_grid` (pub(super)) por descendência de módulo. **Sem** import reverso, **sem**
> `pub(crate)`, **sem** `dyn`.

### Fora desta fatia (medido P379) — NÃO tocar
- **Text** (`:635`, 82L) — fatia própria (folha de render, chain-pesado).
- **Máquina do layouter** — `Sequence`/`Styled`/`Dynamic`/`SetPage` (orquestram/reconfiguram; não são
  elementos de domínio).
- **Math** — `Equation` + arm agrupado 16 variantes (`:858`/`:868`) → fatia final, `rules/math/layout/`.
- **Displays counter/state** ([a-decidir], fronteira).

### O que resta após esta fatia (rumo a fechar)
Fatia 2 (refs/citações + avulsos: Quote/SmartQuote/Raw/Hide/Divider/Cite/Ref/Link/Bibliography/
Footnote) + Text + (máquina fica) + math final.

---

## §10 — P381: Fatia 2 (refs/avulsos) + Fatia Text (isolada), forma B por-elemento

> **Fatia 2 (refs/citações + avulsos), commit próprio.** 9 arms inline → `compiler/layout/<elem>.rs`:
> `cite.rs` (lê `introspector`), `bibliography.rs` (usa `super::format_bib_entry`), `footnote.rs`
> (estado dedicado `footnote_counter`/`pending_footnote_bodies` por descendência), `quote.rs`
> (`chain.lang`), `smartquote.rs` (estado dedicado `smartquote_*_open` + `layout_word`), `raw.rs`
> (`layout_word`), `hide.rs`, `divider.rs`, `link.rs`. **`Ref`/`Labelled` já delegam a
> `references.rs`** (atomizados; não se tocam).
>
> **Fatia Text (isolada), commit próprio.** `Text` (`@653`, ~82 linhas) → `compiler/layout/text.rs`.
> Folha de render (decodifica `#set text` da chain, merge top-wins, `split_whitespace`→`layout_word`;
> **não re-entra `layout_content`** — confirmado P379). Caminho quente: oráculo crítico é a rede
> `f_caracterizacao_estilo::*`.
>
> Forma B; estado privado/dedicado e helpers (`format_bib_entry`, `layout_word`) por **descendência
> de módulo**; **sem** import reverso, **sem** `pub(crate)`, **sem** `dyn`.

### FORA (não tocar — medido P379)
- **Máquina do layouter**: `Sequence` (`@742`), `Styled` (`@1045`), `Dynamic` (`@579`), `SetPage`
  (`@998`) — orquestram/reconfiguram; **não** são elementos de domínio.
- **Math**: `Equation` (`@848`) + arm agrupado 16 variantes (`@858`) + `MathAlignPoint`/`Linebreak`
  (`@887`) → fatia final, `rules/math/layout/`.
- **Displays counter/state** ([a-decidir], fronteira).

### Após este passo (o "monólito fechado" no sentido correto)
O `layout_content` fica **só máquina + math** (+ os no-ops e os arms já-magros). Os **elementos de
domínio** do layout estão atomizados. Resta a **fatia math** (final) e depois a atomização do
`introspect.rs`.

---

## §11 — P382: fatia math (final) — consolidar a cola em equation.rs

> **Medição do subsistema (a forma NÃO é assumida):** o math **já está atomizado** em
> `rules/math/layout/` — `MathLayouter` (`mod.rs:223`) + `layout_node` (`:258`, despacha as 16
> variantes) + `layout_equation` (`:246`) + arquivos por-feature (`frac.rs`/`attach.rs`/`matrix.rs`/
> `cases.rs`/`delimited.rs`/`root.rs`/`stretchy.rs`/`assembly.rs`). A ponte do lado-layout é
> `compiler/layout/equation.rs::Layouter::layout_equation` (`:21`) — **convenção `impl Layouter`
> método** (ADR-0037/P96.7), **não** a forma B free-function das fatias anteriores.
>
> **Os 3 arms math no `layout_content` são cola fina** (não lógica de math — essa está no subsistema):
> - `Equation` (`@770`): decode do gate `equation.numbering` da chain (5 linhas) + `layout_equation`.
> - 16-variante agrupado (`@780`): **fallback defensivo** (`plain_text`→`layout_word`) para nós math
>   fora de uma equação — não é o layout real (esse é `layout_node`).
> - `MathAlignPoint`/`Linebreak` (`@809`): **no-op** (`=> {}`).
>
> **Decisão do dono (P382): consolidar a cola em `equation.rs`** (convenção `impl Layouter` do
> subsistema): `layout_equation_arm(&mut self, e: &EquationElem)` (decode+bridge) e
> `layout_math_fallback(&mut self, content)` (o fallback). Arms magros no núcleo; o no-op fica.
> equation.rs mantém a sua linhagem (`layout.md`). **Sem** `dyn`, **sem** wildcard, `entities/`
> intacto.

### Após este passo — a atomização do LAYOUT fecha
O `layout_content` fica **só máquina** (`Sequence`/`Styled`/`Dynamic`/`SetPage`) **+ no-ops/displays**
counter/state. Os **elementos de domínio do layout (incl. math) estão atomizados**. A próxima frente
é o **`introspect.rs`** (43 arms); os displays counter/state ficam [a-decidir].

---

## §12 — P383: introspect.rs — completar a convenção por-elemento

> **Medição (a forma sai do walk, não do layout/math):** o `introspect.rs` é o tronco do walk; o
> subsistema `rules/introspect/` já tem submódulos por-concern (`extract_payload.rs`, `from_tags.rs`,
> `fixpoint.rs`, `convergence.rs`, `locatable.rs`), cada um com **L0 próprio**. A lógica de
> introspeção **por-elemento já foi extraída nas migrações M5/M6** (P178-P200): `extract_payload`
> (payload por elemento), `from_tags::apply_state_displays`/`apply_counter_displays` (os displays — o
> [a-decidir] do P379 **resolve-se aqui**: já estão atomizados em `from_tags.rs`), e os helpers
> `compute_heading_auto_toc`/`compute_heading_for_toc`/`compute_labelled` (compute por-elemento,
> soltos em `introspect.rs`).
>
> **Classificação dos arms do walk (medido):** ~31 são **recursão/descida** (`walk(&e.body…)` —
> máquina); Heading/Labelled fazem **orquestração de emissão de tags** (máquina, chamam os
> `compute_*`); Equation/Figure/CounterUpdate são **puros/no-op** (lógica já em extract_payload/
> populate_intr); Empty/Text/Dynamic/Outline **no-op**. `materialize_time` é **rebuild recursivo**
> (máquina) + a substituição de `CounterDisplay`/`StateDisplay` (adapter). **O walk é máquina; a
> lógica por-elemento já está extraída.**
>
> **Decisão do dono (P383): completar a convenção por-elemento** — mover os helpers `compute_*`
> (ainda soltos em `introspect.rs`) para o submódulo por-elemento:
> - `compute_heading_auto_toc` + `compute_heading_for_toc` → `rules/introspect/heading.rs`.
> - `compute_labelled` → `rules/introspect/labelled.rs`.
> O walk chama via `heading::…`/`labelled::…` (pub(super), descendência). Os ficheiros novos declaram
> `@prompt rules/atomizacao_elementos.md` (convenção da atomização P376+; o submódulo usa L0 próprio
> por ficheiro, mas a atomização governa este movimento). **Sem** `dyn`, **sem** wildcard, `entities/`
> intacto. A **máquina do walk fica** (recursão/tags/rebuild).

### Após este passo — "elementos primeiro" completo
A camada layout (P376-382) e a camada introspect (M5/M6 + este passo) têm a lógica por-elemento
atomizada. O walk e o `layout_content` ficam **máquina**. A frente seguinte é a **varredura do
projeto** (decisão do dono).

---

## §13 — P472: extensão de `cite.rs` — ibid. + last_cited_key

> `cite.rs` materializado em P381 (Fatia 2). P472 adiciona lógica de **ibid.**:
> quando a mesma key é citada consecutivamente em `CitationStyle::Numeric` +
> `CitationForm::Normal`, o layouter emite `ibid.` em vez do número.

### Mudança em `compiler/layout/cite.rs`

```rust
// Antes do match principal de cite:
let is_ibid = style == CitationStyle::Numeric
    && form == CitationForm::Normal
    && layouter.last_cited_key.as_deref() == Some(key.as_str());
layouter.last_cited_key = Some(key.clone());
if is_ibid {
    layouter.layout_content(&Content::text("ibid.".to_string()));
    if let Some(s) = &e.supplement { layouter.layout_content(s); }
    return;
}
```

### Mudança em `compiler/layout/mod.rs` — campo `last_cited_key`

```rust
pub(super) last_cited_key: Option<String>,  // inicializado None em Layouter::new()
```

Actualizado em cada citação (inclusive no caminho CSL cache). Permite detecção de ibid.
sem consultar o Introspector (estado local do Layouter).

### Scope-out explícito (P472)

- ibid. apenas para `Numeric + Normal`; outros styles e forms não usam ibid.
- ~~op. cit. (citação não-consecutiva para key já vista) — futuro~~ → **implementado em P473**.
- reset de `last_cited_key` em pagebreak — não necessário para semântica mínima.

---

## §14 — P473: extensão de `cite.rs` — op. cit. + `previously_cited_keys`

> `ibid.` (P472) cobre citações *consecutivas*. `op. cit.` cobre o caso
> complementar: a key já foi citada antes, mas não consecutivamente.

### Campo `previously_cited_keys` no Layouter

```rust
// Em layout/mod.rs — Layouter struct:
pub(super) previously_cited_keys: std::collections::HashSet<String>,
// Inicializado: previously_cited_keys: std::collections::HashSet::new()
```

### Detecção em `compiler/layout/cite.rs`

Antes de actualizar `last_cited_key`, push `last_cited_key` para `previously_cited_keys`:

```rust
let is_ibid = style == Numeric && form == Normal
    && layouter.last_cited_key.as_deref() == Some(key.as_str());
let is_op_cit = style == Numeric && form == Normal
    && !is_ibid
    && layouter.previously_cited_keys.contains(key.as_str());

// Actualiza previously_cited_keys com a key anterior ANTES de last_cited_key.
if let Some(prev) = layouter.last_cited_key.take() {
    layouter.previously_cited_keys.insert(prev);
}
layouter.last_cited_key = Some(key.clone());
```

**Ordem obrigatória:** (1) computar `is_ibid`/`is_op_cit` com o estado actual; (2) `take()` de `last_cited_key` e insert; (3) set `last_cited_key` com key actual.

### Render de `op. cit.`

```rust
if is_op_cit {
    let n_str = citation_number_for_key(intr, key)
        .or_else(|| bib_number_for_key(intr, key))
        .map(|n| n.to_string())
        .unwrap_or_else(|| key.clone());
    let author_abbrev = entry
        .map(|e| e.author.split(',').next().unwrap_or(&e.key).to_string())
        .unwrap_or_else(|| key.clone());
    let text = format!("[{}] {}, op. cit.", n_str, author_abbrev);
    layouter.layout_content(&Content::text(text));
    // supplement se presente
    return;
}
```

### Invariante de exclusão mútua

```
ibid.    = last_cited_key == current_key (consecutivo)
op. cit. = previously_cited_keys.contains(current_key) AND NOT ibid.
normal   = NOT ibid. AND NOT op. cit.
```

### Scope-out explícito (P473)

- `op. cit.` apenas para `Numeric + Normal`; outros styles/forms sem op. cit.
- `ibid., p. N` / `op. cit., p. N` com page override — scope-out.
- `loc. cit.` — scope-out.
- Reset de `previously_cited_keys` em nova secção — não implementado.

## §15 — P784: `compiler/layout/text.rs` propaga `TextStyle.math`

`resolve_effective_style` (a fatia Text isolada, §10) constrói `effective:
TextStyle` campo-a-campo a partir de `layouter.style` + overrides de `#set
text(...)`. Novo campo `math: bool` (P784, ver `entities/layout_types.md`
§P784) adicionado: `math: layouter.style.math` — herda directo do style
corrente, sem lógica de override (este merge não é math-específico, só
reflecte o valor já activo no layouter).

## §16 — P891: `compiler/layout/text.rs` propaga `TextStyle.math_script`

Mesmo padrão de §15: campo `math_script: bool` (P891, ver
`entities/layout_types.md` §P891) adicionado a `resolve_effective_style`:
`math_script: layouter.style.math_script` — herda directo, sem lógica de
override (este merge não é script-específico).

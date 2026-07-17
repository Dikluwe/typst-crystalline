# F — Inventário 1a: lado cristalino do estilo (StyleChain / Styles / Set*)

> Diagnóstico read-only. Front 1a do dossiê "the F" / StyleChain.
> Cada afirmação factual carrega `file:line` ou o grep exacto.
> NÃO foi lido `lab/`, `00_nucleo/materialization/` nem `00_nucleo/context/`.
> Greps base:
> `grep -rnE "StyleChain|Styles|StyleNode|Style::" 01_core/src/`
> `grep -rnE "SetHeadingNumbering|SetEquationNumbering|SetPage|SetFigureNumbering" 01_core/src/`
> `grep -rnE "Styled" 01_core/src/`

---

## 0. Resumo executivo (mapa mental)

Existem **três mecanismos de estilo independentes e desconexos** no cristalino,
e o ponto central do design é que eles **não convergem**:

1. **`#set text(...)`** → muta a `StyleChain` (`engine.styles`) **em eval-time** e
   **assa (bake-in)** o `TextStyle` resolvido dentro de cada `Content::Text(EcoString, TextStyle)`
   (`eval/mod.rs:282/292/339`). A `StyleChain` **não sobrevive** ao eval — o resultado
   já vem achatado no nó.
2. **`*bold*` / `_italic_` / `strong()` / `emph()`** → produzem `Content::Styled(Box<Content>, Styles)`
   (`content.rs:1050/1057`), que **sobrevive** na árvore e é re-resolvido em layout-time
   via `StyleChain::push_styles` (`layout/mod.rs:1244-1252`). Esta é a **2ª `StyleChain`**,
   reconstruída do zero no Layouter.
3. **`#set heading/equation/page/figure`** → produzem **marcadores `Content::Set*`** (variantes
   dedicadas), que **não tocam em `Styles`/`StyleChain` de todo** — viajam como nós na árvore
   e são consumidos por canais distintos (Introspector para numbering, mutação de `page_config`
   para SetPage).

`Style`/`Styles`/`StyleChain` são portanto um sistema **parcial** que cobre só `text`-like
properties (bold/italic/size/fill/heading_level/lang/weight/tracking/leading/font), com muitas
delas **inertes em layout**. As `Set*` são um sistema paralelo de "config a partir deste ponto"
que não passa por `Styles`. Não existe `PropMap`; não existe vtable; `Style` é um enum fechado
e manual (divergência consciente do vanilla `#[elem]`, ver `style.rs:18-19`).

Há **dois tipos chamados `Styles`**: o real (`style.rs:131`) e um **stub morto**
(`world_types.rs:158 pub struct Styles(())`) usado só pelo seu próprio smoke-test
(`world_types.rs:571`).

---

## 1. Inventário de sites que tocam estilo

### 1.1 `style::Styles` (o real) vs `world_types::Styles(())` (stub)

| Tipo | Definição | Usado por | Estado |
|------|-----------|-----------|--------|
| `style::Styles { inner: Vec<Style> }` | `entities/style.rs:131` | `Content::Styled`, `push_styles`, eval markup, stdlib structural, layouter | **Vivo** — o real |
| `world_types::Styles(())` | `entities/world_types.rs:158` | **só** `world_types.rs:571` (`let _ = Styles::new();` num teste) | **Stub morto** — "NÃO migrar neste passo" (`world_types.rs:155`) |

Verificação de uso do stub:
`grep -rn "Styles" 01_core/src/entities/world_types.rs` → linhas 158/160/164/571 apenas.
Nenhum import qualificado `world_types::Styles` em qualquer camada
(`grep -rnE "world_types::Styles" 01_core 02_shell 03_infra 04_wiring` → 0 hits).
O comentário do stub (`world_types.rs:155-157`) descreve o que o vanilla tem
(`EcoVec<LazyHash<Style>>` com vtable dinâmica) — i.e. o **alvo** que o "F" deverá decidir.

### 1.2 Tabela site × file:line × papel

Papéis: **P**=produz/constrói, **C**=consome/lê (vira efeito), **T**=propaga/transporta.

| Site | file:line | Papel | Nota |
|------|-----------|-------|------|
| `enum Style` (def) | `style.rs:33` | — | 10 variantes (§5.2) |
| `struct Styles` (def) | `style.rs:131` | — | `Vec<Style>` |
| `Styles::from_iter / push / iter` | `style.rs:142/147/152` | P/T | construção do delta |
| `struct StyleDelta` (def) | `style_chain.rs:31` | — | 10 campos `Option<…>` |
| `struct StyleNode { delta, parent }` | `style_chain.rs:79` | T | nó da lista ligada |
| `struct StyleChain(Option<Arc<StyleNode>>)` | `style_chain.rs:95` | T | clone O(1) |
| `StyleChain::default_chain` | `style_chain.rs:105` | P | bold=false, italic=false, size=11.0 |
| `StyleChain::push(delta)` | `style_chain.rs:120` | T | empilha delta |
| `StyleChain::push_styles(&Styles)` | `style_chain.rs:134` | P/T | projecta `Style`→`StyleDelta` |
| `StyleChain::fill/heading_level/bold/italic/size/weight/tracking/leading/lang/font` | `style_chain.rs:196-288` | C | resolvers walk-up-the-chain |
| `From<&StyleChain> for TextStyle` | `style_chain.rs:317` | C | **ponto único de achatamento** (ADR-0039) |
| `Engine.styles: &'a mut StyleChain` | `engine.rs:49` | T | a chain viaja como `&mut` no Engine |
| eval raiz cria `StyleChain::default_chain()` | `eval/mod.rs:212` | P | origem da chain em eval |
| `#set text` empilha na chain | `eval/rules.rs:314` e `:455` | P | `*engine.styles = engine.styles.push(delta)` |
| `*bold*`/`_emph_` empilham scope-local | `eval/markup.rs:37` | P | `engine.styles.push(delta)` (markup `*..*`) |
| eval **assa** `TextStyle::from(&*engine.styles)` em `Content::Text` | `eval/mod.rs:282/292/339` | C | bake-in — chain resolvida vira campo do nó |
| `eval_link` lê chain | `eval/markup.rs:106-108` | C | `TextStyle::from(styles)` |
| closures clonam chain | `eval/closures.rs:131` | T | `engine.styles.clone()` |
| `Content::Styled(Box, Styles)` (def) | `content.rs:511` | T | wrapper que transporta `Styles` |
| `Content::strong/emph` → `Styled([Bold/Italic])` | `content.rs:1050/1057` | P | construtores |
| stdlib `strong()/emph()` | `stdlib/structural.rs:37/53` | P | idem |
| Layouter campo `chain: StyleChain` | `layout/mod.rs:97` | T | 2ª chain, no Layouter |
| Layouter arm `Content::Styled` push/pop | `layout/mod.rs:1244-1252` | C | `push_styles` + `TextStyle::from` + save/restore |
| Layouter arm `Content::TermItem` push bold | `layout/mod.rs:1288-1293` | C | reusa `push_styles([Bold(true)])` |
| Layouter arm `Content::Text` merge | `layout/mod.rs:576-609` | C | funde `node_style` (assado) com `self.style` (chain) |
| Math layouter `MathIdent`/`MathText` decide italic | `math/layout/mod.rs:260-274` | C | estilo derivado de `&TextStyle`, não de `Styles` |
| `TextStyle` (resultado achatado) def | `layout_types.rs:109+` | C | "é **o resultado** de resolver uma `StyleChain`" (`layout_types.rs:115`) |
| introspect `Content::Styled` transparente | `introspect.rs:351`, `:1239` | T | desce no body, preserva `styles` |
| `map_content`/`map_text` `Styled` | `content.rs:2015-2018`, `:2184-2187` | T | recursa body, clona `styles` |
| `plain_text` `Styled` | `content.rs:1657` | T | delega ao body |
| `eq` `Styled` | `content.rs:1780` | C | `ba==bb && sa==sb` |
| `is_empty` — **sem arm `Styled`** | `content.rs:1480-…` | — | cai no fallback (§3) |
| `get_field` — **sem arm `Styled`** | `content.rs:1842-1849` | — | retorna `None` (§3) |
| `Value::Styles` comentado/desactivado | `value.rs:96` | — | `// Styles(Styles), … bloqueia show/set` |

---

## 2. As 4 variantes `Set*` end-to-end

Ponto crítico transversal: **nenhuma das 4 passa por `Styles`/`StyleChain`**. São nós-marcadores
opacos na árvore `Content`, com canais de consumo próprios. Todas têm header de doc com
"DEBT-10: substituir por StyleChain quando o motor de introspecção completo for implementado"
(`content.rs:332/343`).

### 2.1 `SetHeadingNumbering { active: bool }`

```
PRODUZ  eval/rules.rs:213-227   #set heading(numbering:"1.1")
        → Ok(Value::Content(Content::SetHeadingNumbering { active }))
            (active = existe arg named "numbering" do tipo Str)
   def  content.rs:334
TRANSPORTA  nó normal na árvore Content; PartialEq content.rs:1732;
            plain_text "" content.rs:1613; is_empty fallthrough content.rs:1956
CONSOME (numbering)  via INTROSPECTOR, não via Layouter:
  - extract_payload.rs:44-47 → emite ElementPayload::StateUpdate
       key="numbering_active:heading", update=Set(Bool(active))
  - locatable.rs:64 → Content::SetHeadingNumbering => true (locatável P182C)
  - introspect.rs:1003 (walk arm)
  EFEITO  heading numbering activo lido em layout via
          Introspector::is_numbering_active_at("numbering_active:heading", loc)
CONSOME (Layouter)  layout/mod.rs:709-716 → **NO-OP** (comentário: helper eliminado P190G)
```

Diagrama:
`#set heading(numbering)` → `eval/rules.rs:227` → `Content::SetHeadingNumbering` (árvore)
→ `extract_payload.rs:44` `StateUpdate(numbering_active:heading)` → StateRegistry/Introspector
→ heading layout lê via `is_numbering_active_at`. Layouter arm é no-op (`mod.rs:709`).

### 2.2 `SetEquationNumbering { active: bool }`

```
PRODUZ  NÃO há produtor em eval/rules.rs (grep: 0 hits para SetEquationNumbering em eval/).
        Construído directamente em testes (introspect.rs:3234 etc.).
        → i.e. eval-side produtor AINDA NÃO existe (ver Dúvidas §D1).
   def  content.rs:345
TRANSPORTA  igual a Heading; PartialEq content.rs:1733; plain_text "" content.rs:1614
CONSOME  via INTROSPECTOR:
  - extract_payload.rs:58-61 → StateUpdate key="numbering_active:equation"
  - locatable.rs:75 → true
  - introspect.rs:1024 (walk arm)
  EFEITO  equation.rs:33-37 lê
          introspector.is_numbering_active_at("numbering_active:equation", loc)
          → gate de numeração da equação (counter via from_tags arm Equation P186E)
CONSOME (Layouter)  layout/mod.rs:718-722 → **NO-OP**
```

Diagrama:
`Content::SetEquationNumbering` (só em testes hoje) → `extract_payload.rs:58`
`StateUpdate(numbering_active:equation)` → Introspector → `equation.rs:33` gate
`is_numbering_active_at` → counter de equação avança. Layouter arm no-op (`mod.rs:718`).

### 2.3 `SetPage { width: Option<f64>, height: Option<f64>, margin: Option<f64> }`

```
PRODUZ (2 produtores):
  - eval/rules.rs:230-256  #set page(width/height/margin)  → Content::SetPage{...}
  - stdlib/layout.rs:318   Ok(Value::Content(Content::SetPage { width, height, margin }))
   def  content.rs:473-477
TRANSPORTA  nó na árvore; PartialEq content.rs:1773-1774; plain_text "" content.rs:1654
CONSOME  Layouter DIRECTAMENTE (não Introspector):
  - layout/mod.rs:1102-1125 → muta self.page_config (clone+override por campo);
       se changed e página não vazia → flush_line + new_page;
       sincroniza regions.current.width/height/cursor/line_start (P216A)
  EFEITO  REAL e imediato em layout (única das 4 com efeito não-introspectivo directo)
  - layout_types.rs:361 comentário: "Mutável durante o layout — Content::SetPage altera"
locatable.rs:125 → NÃO locatável (no fall-through não-locatável)
```

Diagrama:
`#set page(...)` (`rules.rs:256`) **ou** `page()` builtin (`stdlib/layout.rs:318`)
→ `Content::SetPage` (árvore) → `layout/mod.rs:1102` muta `self.page_config` + regions
→ páginas seguintes mudam de tamanho/margem. **Sem Introspector.**

### 2.4 `SetFigureNumbering { pattern: String }`

```
PRODUZ  eval/rules.rs:259-278  #set figure(numbering:"1")
        → também muta engine.figure_numbering (rules.rs:275)
        → Content::SetFigureNumbering { pattern: new_numbering.unwrap_or_default() }
   def  content.rs:377
TRANSPORTA  PartialEq content.rs:1740; plain_text "" content.rs:1619
CONSOME  Layouter layout/mod.rs:724-726 → **NO-OP**
         comentário: "numeração baked-in em cada nó Figure (Passo 75, DEBT-14)"
         locatable.rs:120 → não-locatável
         introspect.rs:1062 (walk arm)
  EFEITO  NENHUM via este nó — o efeito real está assado em cada Content::Figure
          (engine.figure_numbering capturado em eval-time, DEBT-14)
```

Diagrama:
`#set figure(numbering)` (`rules.rs:276`) → muta `engine.figure_numbering` (eval-time, real)
**+** emite `Content::SetFigureNumbering` (marcador inerte) → Layouter no-op (`mod.rs:724`).
O número aparece porque cada `Figure` já levou o pattern assado.

### 2.5 Síntese das 4

| Variante | Produtor | Canal de consumo | Efeito real onde |
|----------|----------|------------------|------------------|
| SetHeadingNumbering | `rules.rs:227` | Introspector StateUpdate | `is_numbering_active_at` em heading layout |
| SetEquationNumbering | **só testes** (sem eval) | Introspector StateUpdate | `equation.rs:33` gate |
| SetPage | `rules.rs:256` + `stdlib/layout.rs:318` | Layouter directo | `mod.rs:1102` muta page_config |
| SetFigureNumbering | `rules.rs:276` | nenhum (no-op) | assado em `Figure` (eval-time) |

Observação de design: 4 marcadores, **4 canais diferentes**, zero envolvimento de `Styles`.
É o anti-padrão que o "F"/StyleChain deveria unificar (cada um deveria ser um `Style` numa
`StyleChain` propagada, como no vanilla).

---

## 3. `Styled(Box<Content>, Styles)` — o wrapper hoje

Def: `content.rs:511`. Doc `content.rs:503-510`: "Fundação tipada para `#set`/`#show`. Ainda
**não** é consumida pelo Layouter actual" — **comentário desatualizado**: o Layouter consome
desde Passo 100 (`layout/mod.rs:1244`).

### 3.1 Onde é criado
- `Content::strong(body)` → `Styled(body, [Bold(true)])` (`content.rs:1050`)
- `Content::emph(body)` → `Styled(body, [Italic(true)])` (`content.rs:1057`)
- stdlib `strong()`/`emph()` (`stdlib/structural.rs:37/53`)
- markup `*..*` / `_.._` via `eval/markup.rs:61/74` (`Content::strong`/`emph`)
- **Não** é criado por `#set text` (esse caminho usa `StyleChain` directa + bake-in, §1.2).

### 3.2 Onde é aberto/aplicado
- **Layouter** `layout/mod.rs:1244-1252`: `prev = chain.clone(); chain = chain.push_styles(styles);
  self.style = TextStyle::from(&chain); layout_content(body); restore`.
  Save/restore O(1) por `Arc::clone`. É aqui que `Styles` vira efeito visual.
- O delta só ganha vida quando o `Content::Text` interno é layoutado e funde `self.style`
  (chain) com o seu `node_style` assado (`mod.rs:586-603`, regra "qualquer prop activa na
  chain tem prioridade").

### 3.3 O que `Styles` carrega
`Vec<Style>` (10 variantes possíveis, §5.2). Na prática hoje, produzido só com `Bold`/`Italic`
(strong/emph) — as outras 8 variantes existem mas **não têm produtor que crie `Styled` com
elas** fora de testes (§5.4). `Styles` é `Clone + PartialEq + Default` (`style.rs:130`).

### 3.4 Interacção com os 6 hub matches de `content.rs`

| Hub method | Arm `Styled`? | Comportamento | file:line |
|-----------|---------------|---------------|-----------|
| `plain_text` | **Sim** | `body.plain_text()` (transparente) | `content.rs:1657` |
| `is_empty` | **Não** (sem arm explícito) | cai no fallback do match (não está na lista 1480-…); ver Dúvida D2 | `content.rs:1480+` |
| `map_content` | **Sim** | recursa body, **clona styles** | `content.rs:2015-2018` |
| `map_text` | **Sim** | recursa body, **clona styles** | `content.rs:2184-2187` |
| `get_field` | **Não** | match só trata Heading/Figure; `Styled` → `_ => None` | `content.rs:1842-1849` |
| `eq` (PartialEq) | **Sim** | `ba==bb && sa==sb` (compara body E styles) | `content.rs:1780` |

Padrão: `Styled` é **transparente** para travessia (plain_text/map_*) e **opaco-comparável**
para igualdade. `get_field` ignora-o (não expõe os estilos como campos consultáveis) e
`is_empty` não tem tratamento dedicado.

---

## 4. As 3 folhas provisórias: `Text`, `MathText`, `MathIdent`

### 4.1 Campos de estilo que cada folha já leva

| Folha | Def | Campo de estilo | Comentário |
|-------|-----|-----------------|-----------|
| `Text(EcoString, TextStyle)` | `content.rs:121` | **`TextStyle` assado inteiro** | "estilo capturado em eval … reflecte as #set text() activas no momento da produção" (`content.rs:119-120`) |
| `MathText(EcoString)` | `content.rs:169` | **nenhum** | estilo decidido no math layouter |
| `MathIdent(EcoString)` | `content.rs:166` | **nenhum** | estilo decidido no math layouter |

Assimetria central: **`Text` carrega estilo explícito e assado**; **`MathText`/`MathIdent`
carregam só a string** e recebem estilo implícito do contexto de layout.

### 4.2 Onde o layouter decide font/size/weight hoje

**Texto normal (`Content::Text`)** — `layout/mod.rs:576-609`:
funde dois fontes de estilo num `effective: TextStyle`:
- `bold/italic = node_style.X || self.style.X` (OR — chain ou assado);
- `size = if self.style.size > self.font_size_pt { self.style.size } else { node_style.size }`
  (heading/Styled aumentou vs `#set text(size)` assado);
- `fill/heading_level/weight/tracking/leading/lang/font = self.style.X.or(node_style.X)`
  (chain top-wins, `mod.rs:594-602`, DEBT-52 Passo 136).
Depois `for word in text.split_whitespace() { layout_word(word) }`.

**Math (`MathIdent`/`MathText`)** — `math/layout/mod.rs:260-274`:
- `MathIdent`: decide italic por heurística — variável de 1 letra que não é símbolo conhecido
  e não é função → `TextStyle { italic: true, ..style.clone() }`; senão `italic: false`
  (`mod.rs:265-269`). Não consulta `Styles`; deriva do `&TextStyle style` passado.
- `MathText`: `layout_text_node(text, style)` — usa o `style` recebido tal-qual.
- `MathStyled` (não é folha, mas relevante): `math/layout/mod.rs:337-349` aplica `map_glyph`
  + size factor sobre uma cópia do `style` (`size = style.size * size_factor`).

Conclusão: para math, **font/size vêm do `&TextStyle` que desce a recursão** (origem em
`equation.rs:46` `layout_equation(body, &self.style)`), e weight/bold são fundidos por
`MathStyled`, não por `Styles`. Para texto normal, font/size/weight saem do merge
`chain (self.style)` × `assado (node_style)` em `mod.rs:586-603`.

---

## 5. Mecânica da `StyleChain` hoje (leitura integral)

Ficheiros: `entities/style_chain.rs` (583 linhas) e `entities/style.rs` (233 linhas), lidos na íntegra.

### 5.1 Estruturas de dados

- `StyleDelta` (`style_chain.rs:31-64`): struct com **10 campos `Option<…>`** — `bold:Option<bool>`,
  `italic:Option<bool>`, `size:Option<f64>`, `fill:Option<Color>`, `heading_level:Option<u8>`,
  `weight:Option<u16>`, `tracking:Option<Length>`, `leading:Option<Length>`, `lang:Option<Lang>`,
  `font:Option<FontList>`. `None` = herdar do pai. Construtor `StyleDelta::empty()` (`:67`) põe tudo `None`.
- `StyleNode { delta: StyleDelta, parent: Option<Arc<StyleNode>> }` (`style_chain.rs:79`) — nó de
  lista ligada imutável. `#[derive(Debug, Clone)]`.
- `StyleChain(Option<Arc<StyleNode>>)` (`style_chain.rs:95`) — newtype sobre o topo da lista.
  `None` = cadeia vazia (resolve para defaults hard-coded). **Clone O(1)** (só clona o `Arc` do topo);
  **leitura O(N)** percorrendo até ao 1º delta que define a prop (`style_chain.rs:84-88`).
- **Não há `PropMap`**, não há `HashMap`, não há vtable, não há `LazyHash`. É uma lista ligada
  de structs com campos fixos. O delta é "denso por tipo, esparso por valor" (campos fixos, valores `Option`).

### 5.2 Variantes de `Style` (enum, `style.rs:33`)

10 variantes, todas com payload tipado:
`Bold(bool)` `:35`, `Italic(bool)` `:37`, `Size(Pt)` `:39`, `Fill(Color)` `:41`,
`HeadingLevel(u8)` `:43`, `Lang(Lang)` `:50` (P288), `Weight(u16)` `:61` (P289),
`Tracking(Length)` `:74` (P290), `Leading(Length)` `:94` (P291), `Font(FontList)` `:122` (P292).
`#[derive(Debug, Clone, PartialEq)]` — **`Copy` removido em P292** porque `FontList` contém
`Vec<FontFamily>` (`style.rs:27-31`). Teste de catálogo "10 variantes" em `style.rs:209-231`.

`Styles { inner: Vec<Style> }` (`style.rs:131`) — `from_iter`/`push`/`iter`/`is_empty`/`len`.
Invariante (`style.rs:128-129`): ordem = inserção; na resolução, o valor mais recente por variante ganha.

### 5.3 Como a cadeia é construída

- **Raiz**: `StyleChain::default_chain()` (`style_chain.rs:105-116`) → root com
  `bold:Some(false), italic:Some(false), size:Some(11.0)`, resto `None`.
  Alternativa `StyleChain::empty()` (`:99`) → `StyleChain(None)`.
- **Empilhar delta cru**: `push(delta) -> Self` (`style_chain.rs:120-126`) — cria `StyleNode` novo
  com `parent = self.0.clone()`. Usado por `#set text` (`eval/rules.rs:314/455`) e markup (`markup.rs:37`).
- **Empilhar `Styles` tipado**: `push_styles(&Styles) -> Self` (`style_chain.rs:134-191`) — itera
  os `Style`, projecta cada variante no campo correspondente de um `StyleDelta` (match exaustivo,
  `:138-187`), depois `self.push(delta)`. Usado por `Content::Styled` no Layouter (`mod.rs:1247`).
  `Font` usa `f.clone()` (não-Copy) `:187`.

### 5.4 Como é consultada / resolve (`StyleChain::get`-equivalente)

**Não existe um método único `get`.** Há **um resolver por propriedade**, todos com o mesmo
padrão "walk up the chain, primeiro que define ganha":
- Diretos: `fill` `:196`, `heading_level` `:210`, `weight` `:241`, `tracking` `:251`,
  `leading` `:261`, `lang` `:271`, `font` `:281` (este clona — não-Copy).
- Via helpers genéricos: `bold()` `:222`/`italic()` `:227` → `resolve_bool` (`:290`);
  `size()` `:232` → `resolve_f64` (`:301`). Com defaults `unwrap_or(false)` / `unwrap_or(11.0)`.

O padrão de cada resolver (ex. `style_chain.rs:196-205`):
```
let mut node = self.0.as_deref();
while let Some(n) = node {
    if let Some(v) = n.delta.<campo> { return Some(v); }
    node = n.parent.as_deref();
}
None
```

**Ponto único de achatamento**: `impl From<&StyleChain> for TextStyle` (`style_chain.rs:317-335`,
ADR-0039) — chama os 10 resolvers e monta um `TextStyle` plano. É a fronteira entre o mundo
"chain" (source-of-truth no Layouter) e o mundo "achatado" (`FrameItem::Text`).

### 5.5 Inércia / cobertura

`StyleDelta`/`StyleChain` capturam 10 props mas várias são **inertes em layout** (capturadas mas
sem consumer): comentários "Inerte em layout" em `style_chain.rs:42/45/50/56/62` para
weight/tracking/leading/lang/font (estado à data dos respetivos passos). DEBT-52 (`DEBT.md:15-23`)
foi o tracker de "consumer integral de StyleDelta em layout" e está marcado encerrado;
o merge real em `layout/mod.rs:594-602` já propaga os 5 campos top-wins.

---

## 6. ADRs/DEBTs de fundo relevantes

- ADR-0038 (Passo 99): fundação `Style`/`Styles`/`push_styles` (`style_chain.rs:7`).
- ADR-0039 (Passo 100): Layouter mantém `StyleChain` como source-of-truth; `From<&StyleChain>`
  achata em `TextStyle` no emit (`style_chain.rs:314-316`).
- ADR-0026 como precedente: enum fechado manual em vez de `#[elem]` (`style.rs:18`).
- DEBT-52 (`DEBT.md:15`): consumer integral de `StyleDelta` em layout (encerrado).
- DEBT-10 (`content.rs:332/343`): "substituir [Set*] por StyleChain quando o motor de introspecção
  completo for implementado" — declaração explícita de que as `Set*` são dívida que o F deve fechar.
- DEBT 99.E (`DEBT.md:276-278`): "Transferido ao F (1) — `Styled(Box<Content>, Styles)`: carrega
  `Styles`, … resolvido junto das 4 `Set*`". O `DEBT.md` (`:270-278`) já antecipa que `Styled` +
  as 4 `Set*` são o território do "F".

---

## Dúvidas (para o dossiê §perguntas)

- **D1**: `SetEquationNumbering` **não tem produtor em eval** (`grep -rn SetEquationNumbering 01_core/src/engine/eval/` → 0 hits). Só é construído em testes. É intencional (numbering de equação ainda não exposto em `#set`) ou lacuna? O comentário `content.rs:337-342` diz "Materializada em P199B" mas o caminho eval parece ausente.
- **D2**: `Content::Styled` não aparece como arm explícito em `is_empty` (`content.rs:1480+`) — cai num fallback. Não consegui confirmar pela leitura parcial se o fallback é `false` por defeito ou se `Styled` deveria delegar ao body (como faz `plain_text`). Verificar o fim do match `is_empty` (linhas após 1509).
- **D3**: Há **duas `StyleChain` distintas em runtime**: a de eval (`engine.styles`, que é assada e descartada) e a do Layouter (`self.chain`, reconstruída do zero a partir de `Content::Styled`). O `#set text` só afecta a primeira (bake-in em `Content::Text`); `*bold*` só afecta a segunda. Pergunta de design para o F: o achatamento eval-time de `#set text` é uma decisão a preservar ou a unificar com a chain de layout?
- **D4**: `SetPage` tem **dois produtores** (`eval/rules.rs:256` e `stdlib/layout.rs:318`). Confirmar se são caminhos redundantes (markup `#set page` vs builtin `page()`) ou se um é legacy.
- **D5**: `Value::Styles(Styles)` está comentado em `value.rs:96` com nota "bloqueia show/set". Saber se a reactivação desse `Value` faz parte do escopo do F (necessário para `#show`/captura de `Styles` como valor de 1ª classe).
- **D6**: muitas das 10 props de `StyleDelta` (weight/tracking/leading/lang/font) só têm produtor via `#set text` (parse-driven) ou via testes; fora de `strong`/`emph` (Bold/Italic), nenhum `Content::Styled` real é criado com as outras 8 no pipeline de produção. Confirmar se isto limita o escopo do F a Bold/Italic na prática.

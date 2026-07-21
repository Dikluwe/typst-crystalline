# Prompt L0 — Entidade `ShowRule` (Passo 68-70; atualizado P352)
Hash do Código: 59ae1915

## Propósito

Define as entidades de dados para o motor de show rules do cristalino.
Uma show rule (`#show selector: transform`) interceta nós de conteúdo no
momento da sua criação (eager) e aplica uma transformação declarativa.

## Tipos

### `NodeKind`

Tipo de nó de conteúdo para selecção por tipo.
Conjunto completo: `Heading`, `Figure`, `Strong`, `Emph`, `Raw`,
`Equation`, `ListItem`, `Underline`, `Strike`, `Overline`, `Smallcaps`,
`Subscript`, `Superscript`, `Highlight`.
Outros tipos (`EnumItem`, `Link`, etc.) adicionados em passos futuros.

### `Selector`

Selector de uma show rule. Variantes:
- `Text(String)` — substitui ocorrências literais de um texto. Aplicação
  depende da `Transformation` (P790): `Str` via `map_text`; `Content`/`Func`
  via splice em `map_content` (o texto à volta do match é preservado).
- `NodeKind(NodeKind)` — interceta nós nativos de um tipo específico.
- `DynKind(String)` — interceta um elemento **dinâmico** de utilizador
  (fronteira E1) pelo **nome de kind** (`dyn_kind()`). Lote F-3 inc-2: o
  selector `#show callout:` resolve para uma `FuncRepr::Element` sem fn-ptr
  nativo e casa por nome. Viaja pela **mesma** travessia que `NodeKind`.
- **P417** `Where { base: Box<Selector>, field: EcoString, value: Value }` —
  filtra por campo de elemento. Ex.: `heading.where(level: 1)`. O `base` deve
  ser `NodeKind` (nativo) em P417; outros bases são scope-out. Matching: o
  nó casa `base` **e** o campo `field` existe **e** o seu valor é semanticamente
  igual a `value` (via igualdade de `Value`, ADR-0107). Sem vtable — lógica em
  free function na camada de render (`rules/show/where_match.rs`).
- **P791** `Label(Label)` — casa um nó `Content::Label` pelo **nome** do label
  (`#show <sp>: …`). Não viaja pela travessia principal de `apply_show_rules`:
  o wrapper `Content::Label` é criado na associação retroactiva de `<label>`
  (Passo 56) **depois** do corpo já ter sido interceptado, logo a aplicação é
  dedicada (`intercept_labelled` em `rules/eval/rules.rs`) — só regras de label
  casam nesse ponto, evitando dupla aplicação das outras regras. `it` = o
  **corpo** (o elemento rotulado; o label é metadado, não nó — paridade
  vanilla `target.label()`); a saída substitui o wrapper inteiro (label
  consumido). Aplicação única, última-declarada primeiro (innermost-first,
  P358) — sem loop de revisitação (equivalente à `Revocation` do vanilla).
- **P423** `And(Vec<Selector>)` / `Or(Vec<Selector>)` — combinadores de
  selectors em show rules. Ex.: `heading.or(figure)`, `heading.and(figure)`.
  Semântica: `And` casa se **todos** os sub-selectors casarem; `Or` casa se
  **pelo menos um** sub-selector casar. Ambos são recursivos e de curto-circuito
  esquerda-para-direita. O contraditório (`heading.and(figure)`) retorna
  `false` sem erro. N-ário para paridade com `Selector::And(EcoVec)`/
  `Selector::Or(EcoVec)` de query; methods encadeáveis (`a.or(b).or(c)`).
  Sem vtable — lógica na free function `selector_matches` em
  `rules/eval/rules.rs` (forma B, ADR-0109).

### `RuleId`

Tipo alias `pub type RuleId = u64`. Identificador único atribuído a cada
`ShowRule` no momento da sua criação em `EvalContext` (campo `next_rule_id: RuleId`,
incrementado a cada regra registada).

### `Transformation`

A transformação que uma show rule aplica. Materializa o **S5** do spike-2
(`Transformation = Content | Func | Style`, `f_fronteira_e1.md` §3c). Substitui
o antigo `transform: Value` solto por um vocabulário fechado:

- `Func(Func)` — closure `it => …`; recebe o nó e devolve `Content` (ou `Str`,
  promovido a `Content::text`). Consome o passe de show.
- `Content(Content)` — substituição estática direta. Consome o passe.
- `Str(EcoString)` — substituição literal, **apenas** para `Selector::Text`
  (aplicada por `map_text`). `Content`/`Func` sobre `Text` são suportados por
  splice (P790 — ver invariantes); `Style` sobre `Text` falha explicitamente
  no eval (DEBT-19 ENCERRADO).
- `Style(Styles)` — **show-set** (`#show k: set …`, P352). Os styles do `set` são
  **capturados** na declaração (sem mutar `engine.styles` globalmente) e
  **transportados** embrulhando o conteúdo num `Content::Styled(…, styles)` — o
  carregador `StyledElem`-scoped da fatia 1 (P339, `f_fronteira_e1.md` §3a.8).
  **NÃO consome o passe** (espelha `map.apply(transform); continue` do vanilla,
  `typst-realize/src/lib.rs:458-464`): o conteúdo renderiza sob a chain aumentada,
  não é substituído. Paridade: `styles.rs:504` (`content.styled_with_map(styles)`).

  **Ordem show-set-vs-func (P356, paridade `lib.rs:341,357,458-464`).** As show-set
  que casam um elemento dobram com base no **elemento** (o nó que entra na
  realização), **não** no output de uma func que também o transforme. Quando uma
  show-set **e** uma func casam o mesmo elemento, o vanilla dobra a show-set na
  chain (`map`) e aplica a func **sob** a chain aumentada (`chained =
  styles.chain(&map)`); o crystalline espelha isto **embrulhando o output da func**
  no `Styles` da show-set (a show-set fica ativa quando a func realiza). Casa o
  **exemplo canônico** da referência (`docs/reference/language/styling.md`,
  *Show rules*: 4× `#show heading`, "keeping styling composable", "good practice").
  Sem este ordenamento, a show-set seria perdida (o output da func não casa o
  seletor). Múltiplos show-set continuam a compor por `collapse`.

### `ShowRule`

Triplo `(id, selector, transform)` armazenado no `EvalContext` durante a avaliação.
`id` é um `RuleId` único por sessão de avaliação.
`transform` é uma `Transformation` (ver acima).

## Invariantes

- `ShowRule` é `Clone` — o motor clona o `Vec<ShowRule>`/`Arc<[ShowRule]>` antes de
  aplicar (snapshot explícito para evitar borrow conflict durante `apply_show_rules`).
- `Selector::NodeKind` identifica o tipo pelo enum, não por string (DEBT-21 MITIGADO);
  `Selector::DynKind` identifica o elemento dinâmico pelo nome de kind interned.
- `Selector::Text` suporta `Transformation::Str` (via `map_text`) e, desde
  **P790**, `Content`/`Func` por **splice**: o nó `Content::Text` é fatiado nas
  ocorrências do padrão e o replacement é emendado entre as fatias, preservando
  o texto envolvente (paridade vanilla `visit_regex_match`,
  `typst-realize/src/lib.rs:1391`). A travessia é `map_content`, que **não
  reentra** no nó substituído — equivalente à `Style::Revocation` do vanilla
  (o output não é re-varrido pela mesma regra). `Func` recebe o texto do match
  como `Content::Text` e o seu output (Content ou Str) é emendado por
  ocorrência. O match é **por nó de texto individual** — o vanilla agrupa
  elementos textuais adjacentes na mesma style chain antes de casar; match
  cross-node é divergência registada (scope-out). `Style` sobre `Text` falha
  explicitamente no eval (DEBT-19 ENCERRADO).
- **P417** `Selector::Where` baseia-se em `NodeKind` (nativo) em P417; elementos
  dinâmicos (`DynKind`) são scope-out. O matching delega a free function que
  consulta `Content::get_field(field)` e compara com `value` via igualdade
  semântica de `Value`.
- **P423** `Selector::And`/`Or` aceitam sub-selectors de qualquer tipo suportado
  (`NodeKind`, `DynKind`, `Where`, recursivamente `And`/`Or`). O matching é
  recursivo e curto-circuito. `And` vazio retorna `false`; `Or` vazio retorna
  `false`. Os combinadores viajam pela travessia de nós (`is_node_rule`) sse
  todos os sub-selectors forem node-like.
- `Transformation::Style` **não** consome o passe de show e **não** é válida para
  `Selector::Text` (show-set é sobre elementos, não sobre texto literal). Show-set
  **não** muta o estilo global na declaração — captura-o e transporta-o confinado.
- Anti-recursão **durante a chamada** do recipe via `active_guards: Vec<RuleId>` no
  `EvalContext` (DEBT-20): uma regra cujo `id` já está em `active_guards` é saltada —
  impede re-entrada na criação aninhada. A **terminação da revisitação** (output que
  re-casa) é o **ponto-fixo morfológico** (modelo α, P348, `morph_canon`), não o
  `active_guards`; o teto `MAX_SHOW_RULE_DEPTH` é o backstop.
- `apply_show_rules` faz uma única travessia `map_content` para todos os NodeKind +
  DynKind rules (DEBT-23 ENCERRADO): itera o snapshot de regras dentro da closure.
- **Ordem das `func` (P358, innermost-first).** A travessia de regras **func** tenta a
  **última-declarada primeiro** (`node_rules.iter().rev()`), de modo que, no subconjunto
  onde **uma** func é efetiva, a **última-declarada vence** — paridade com o vanilla
  (recipes innermost-first, `styles.rs:835` `next_back`; ex.: `#show heading: it=>[A:]…`
  ⨁ `…[B:]…` → `"B:T"`). É **reordenação**, **não** acumulação: o vanilla **não** acumula
  múltiplos `func` same-kind (aplica um quando o output muda de kind; **erra** no caso
  mesmo-kind) — medido no P357. O fold de **show-set** (`Transformation::Style`) mantém a
  ordem de declaração + `collapse` (a **última-declarada sobrepõe**, como a referência
  promove) — não é invertido.

## Caso 1, lacuna (ii) — FECHADA (P358): ordem, não acumulação

A medição do P357 (`f-recon-lacuna-ii-passo-357.md`, vanilla 0.14.2 oráculo) estabeleceu
que o vanilla **NÃO acumula** múltiplos `func` same-kind (aplica **um** quando o output
muda de kind; **erra** no caso mesmo-kind) e que **não há demanda** na referência (0
exemplos). Logo a única divergência real era de **ordem**, fechada pelo conserto
innermost-first (ver invariante acima): a **última-declarada func vence** — paridade com
o vanilla. **Não há acumulação a reconciliar** — a premissa "vanilla acumula func" (P354,
herdada do B1) era um conflato com o **show-set** (fold de estilo, = lacuna (i), feita no
P356). **A2/A3 declinados** (A2 acumularia onde o vanilla erra; A3 reabriria o α/P348
selado — custo alto, demanda nula).

**Residual (paridade de erro, não buraco).** 2+ `func` same-kind **mesmo-kind-returning**
(o output re-casa o seletor) → **recursão** → ambos (vanilla e crystalline) **erram** no
teto (`maximum show rule depth exceeded`) = **caso 2**, já tratado pelo α/teto. Não é
acumulação; é paridade de erro.

**Gatilho de reabertura** (medido como zero hoje): se surgir um padrão real (pacote/doc)
onde 2+ `func` same-kind **precisem** acumular com ordem exata, os casos viram testes de
paridade contra o vanilla e A2/A3 entram nessa hora — não antes (ADR-0107/0108).

## Tests obrigatórios

- Construção estrutural de `Selector::And`/`Or`.
- Eval de `heading.or(figure)` produz `Value::Selector(Or([Kind(Heading), Kind(Figure)]))`.
- Eval de `heading.and(figure)` produz `Value::Selector(And([Kind(Heading), Kind(Figure)]))`.
- `selector_matches` para `Or` positivo (heading casa, figure casa).
- `selector_matches` para `And` positivo (`heading.where(level: 1).and(heading)` casa em heading nível 1).
- `selector_matches` para `And` negativo/contraditório (`heading.and(figure)` não casa em nenhum nó).
- E2E: `#show heading.or(figure): set text(red)` aplica a heading e figure, não a paragraph.
- E2E: `#show heading.and(figure): set text(red)` não aplica a nada.

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-23 | P417 (M): +variant `Selector::Where { base, field, value }` para filtragem de show rules por campo de elemento. Integração com `Value::Selector`, parsing de `heading.where(level: 1)`, e matching via `Content::get_field`. | `show.rs`, `show.md`, `value.rs`, `selector.rs`, `selector.md`, `eval/closures.rs`, `eval/rules.rs`, `introspector.rs`, `stdlib/foundations.rs` |
| 2026-06-23 | P423 (S-M): +variants `Selector::And(Vec<Selector>)`/`Or(Vec<Selector>)` e methods `.or()`/`.and()` em `Value::Selector`. Matching recursivo com curto-circuito em show rules; query já suporta And/Or (P209C). | `show.rs`, `show.md`, `eval/closures.rs`, `eval/rules.rs` |
| 2026-06-25 | P467 (S — sonda A.0): confirma estado funcional de `Selector::Where` em show rules; inventário de cobertura atualizado; nenhuma alteração de código. | `show.md`, `cobertura-vanilla-vs-cristalino.md` |
| 2026-06-27 | P474 (XS — sonda fecho): confirma wiring E2E completo de `#show elem.where(field: value)` desde P417 — `eval_show_rule` → `query_selector_to_show_selector` → `apply_show_rules` → `selector_matches(Where)`. **Trilha 3: 3/3 fechado.** 2 testes confirmatórios adicionados. | `show.md`, `show-regex.md`, `eval/tests.rs` |

## Layer

L1 — domínio puro. Sem I/O, sem estado global.

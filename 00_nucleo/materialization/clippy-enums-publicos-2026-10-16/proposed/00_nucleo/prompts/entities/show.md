# Prompt L0 — Entidade `ShowRule` (Passo 68-70; atualizado P352)
Hash do Código: cf18af0c

`NativeElement` e o grupo de filtros integram o contrato vigente de show rules.

## Propósito

Define as entidades de dados para o motor de show rules do cristalino.
Uma show rule (`#show selector: transform`) interceta nós de conteúdo no
momento da sua criação (eager) e aplica uma transformação declarativa.

## Tipos

### `NodeKind`

Tipo de nó de conteúdo para selecção por tipo.
Conjunto completo: `Heading`, `Figure`, `Strong`, `Emph`, `Raw`,
`Equation`, `ListItem`, `Underline`, `Strike`, `Overline`, `Smallcaps`,
`Subscript`, `Superscript`, `Highlight`, `Par`.
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
validado — custo alto, demanda nula).

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

## P1339 — base de show por função nativa de elemento

### Medição anterior à decisão

Em HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, sem diff produtivo,
`01_core/src/entities/show.rs:23-86` possui `NodeKind`, mas não uma base
para toda função nativa de elemento; `Text(String)` significa padrão literal,
não a função `text`. `selector_matching.rs:43-65` rejeita identidades que
não consegue converter. As fixtures `diagnosticos/p1339-full-show-fixture-*`
comprovam no vanilla ratificado aplicação de filtros de strong/emph/text;
registros `p1339-full-show-final-*.json` registram proveniência e a limitação
CLI nos perfis cristalinos não-default, sem convertê-la em prova de matching.

### Decisão proposta — somente o carrier

Adicionar ao `entities::show::Selector`:

```rust
NativeElement(crate::entities::func::Func),
```

É uma base de regra sobre identidade nativa, não invocação da função. Não
duplicar o grupo de filtros neste enum: o conversor pode compor a base com
os `Where` já existentes. A apresentação pública do grupo pertence ao
QuerySelector e ao owner de repr, não a essa forma interna de realização.

Preservar `NodeKind`, `Text`, `DynKind`, `Regex`, `Where`, `And`, `Or`,
`Label`, RuleId, Transformation, ShowRule e suas assinaturas. `DynKind`
continua exclusivo de elemento dinâmico; `Text` continua padrão literal.
Não acrescentar variantes a NodeKind nem modificar Content nesta proposta.

O comportamento da nova base é propriedade de
`compiler/eval/selector_matching.md`; a entidade não importa compiler,
não reconhece endereços de builtins, não chama funções nem transforma nós.
Clone/PartialEq continuam mecânicos; a variante não muda a igualdade de Func.

A extensão é pública e afeta matches exaustivos: gate ADR-0127 obrigatório.
Não muda fase, precedência de regras, revogação, guardas de recursão ou
ordenação de show-set. Aplicação ao mesmo nó via caminhos antigos e novos
precisa ser verificada, inclusive particularidades de Par; não basta fazer
a variante compilar. Inferência de suficiência é refutada por um seletor
nativo medido que precise de outra informação para identificar o nó.


## Carriers compactos de introspecção/show — proposta 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz em 00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16 registra UTC, fontes/L0, hashes, status e git diff HEAD --stat. Os owners atuais foram lidos antes de propor esta alteração. probe-details.json registra rustc 1.92, argv, rlib/binário/sonda com SHA-256 e saída; probe-result.json registra teto de 12 GiB, swap zero, um job e eventos. A sonda é sintética de carriers e do algoritmo de registro/cópia, não benchmark de compilação/layout/RSS nem distribuição de workloads reais. Os candidatos são espelhos com tipos reais de domínio; o caso inline usa Tag e ShowRule produtivos.

Tag mede 416 bytes; Start boxed e Arc medem 48. Para 4.096 pares Start/End com payload Heading sem heap, inline solicita 3.407.872 bytes numa alocação; Box solicita 2.031.616 em 4.097; Arc solicita 2.097.152 em 4.097. Clone de Tag boxed também aloca por Start; os caminhos produtivos lidos em introspect/fixpoint.rs:86–136 e entrypoints.rs:44–66 usam Vec<Tag> temporário e leituras emprestadas, sem clonar o vetor. Walk :325–356 popula o índice por referência antes de mover info para Start. A decisão não toma o clone sintético como frequência produtiva.

Transformation mede 328 e ShowRule 376 bytes; candidatos com Box/Arc<Styles> medem 120 e 168. rules/show.rs:464–470 copia to_vec antes de cada registro; apply.rs:108–112,349–352 filtra/clona regras. Em 128 registros 100% show-set simples sem reter snapshots antigos, inline solicita 12.276.960 bytes/384 alocações, Box 8.194.560/8.640, Arc 5.530.624/512. Com snapshots retidos, picos solicitados são 3.203.856, 4.141.744 e 1.477.808. No controle com canal custom, as cópias de 4.096 regras solicitam 4.107 alocações inline, 8.203 Box e 11 Arc (crescimento do Vec incluído). As leituras de estilos são emprestadas em apply/intercept/outline/table_cells; a aplicação ainda clona o delta quando necessário.

Contraexemplo obrigatório: construir uma única coleção de 4.096 regras 100% show-set simples pede 1.540.096 bytes inline, 2.031.616 Box e 2.097.152 Arc. Arc pode reter capacidade sobrante do canal custom; no controle rico de 128 registros sem snapshots antigos, o final vivo foi 78.064 bytes inline e 156.432 Arc. Não afirmar ganho universal ou ignorar custo atômico/indireção. É mecânica de armazenamento sob ADR-0030/0107, não paridade de linguagem ou ganho cognitivo por IA medido. A inferência favorável a Arc no ciclo de cópias seria refutada por regressão de memória/tempo em um workload real relevante; manter visíveis estes controles.

### Responsabilidade exclusiva proposta

Substituir exclusivamente Transformation::Style(Styles) por Transformation::Style(std::sync::Arc<Styles>). Manter Func/Content/Str, Selector/NodeKind/ShowRule/RuleId e os demais campos/assinaturas. Clone de Transformation/ShowRule compartilha os estilos imutáveis; não clona o StyleDelta neste transporte. Leituras via &Styles continuam por deref; aplicar estilos continua fazendo as cópias já necessárias no owner de aplicação, sem alterar sua ordem. Código externo constrói Style(Arc::new(styles)); para editar um estilo compartilhado usa Arc::make_mut antes de chamar push, preservando isolamento por valor. A mudança é mecânica de ownership, sem novo compartilhamento mutável. Não mover fase, capturar outro estado, mudar precedência/guards, tornar style de show-set global ou introduzir vtable. Esta cláusula supersede somente o payload Style anterior e preservações históricas que o congelavam.

### Gate e aceitação

Proposta não vigente até ratificação humana ADR-0127 §2.1/§2.4: há mudança de payload público e compatibilidade de construção/acesso. Não editar código ou owners ativos por este pacote antes da aprovação. Após ratificação instalar estes L0, testes primeiro e RED real, implementação e resselo estritamente dos owners alterados. Não silenciar large_enum_variant, alterar limiar ou adicionar dependências/flags/defaults/fases.

T1: contratos de construção Box<ElementInfo>/Arc<Styles> e orçamento relativo dos carriers (Tag até três Location; Transformation até Content+alinhamento) devem falhar no estado antigo e passar no novo, sem fixar ABI universal. T2: Start/End/Location/payload/label, igualdade e hashes estruturais por valor, clone independente da Box e mutation isolation preservados; hash não vira formato persistido. T3: testes existentes de introspecção/convergência, nesting, heading/label, state/counter/query/metadata continuam com os mesmos esperados e erros. T4: clone de Transformation::Style compartilha o Arc; Arc::make_mut em cópia preserva estilos do original, incluindo custom, e as variantes Func/Content/Str permanecem. T5: controles existentes de show-set/func, precedência, escopo, labels, outline, table.cell e textual/regex mantêm semântica/morfologia/erros; nenhuma regra executa a mais. T6: workspace completo, build, Clippy all-targets e expectativas/lints já fechados como erro, rustdoc -D warnings com quatro índices, fmt, diff, linhagem estrita/reversa e Tekt sem aumento; 12 GiB, swap zero, um job. Registrar HEAD/working tree/hashes/diffstat/UTC e distinguir tamanhos/bytes solicitados de RSS. Não publicar release. Alguma mudança de assinaturas, campos ou fase além destes payloads exige outra proposta.

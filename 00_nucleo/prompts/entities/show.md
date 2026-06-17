# Prompt L0 — Entidade `ShowRule` (Passo 68-70; atualizado P352)
Hash do Código: edc8666b

## Propósito

Define as entidades de dados para o motor de show rules do cristalino.
Uma show rule (`#show selector: transform`) interceta nós de conteúdo no
momento da sua criação (eager) e aplica uma transformação declarativa.

## Tipos

### `NodeKind`

Tipo de nó de conteúdo para selecção por tipo.
Conjunto completo (Passo 69): `Heading`, `Figure`, `Strong`, `Emph`, `Raw`,
`Equation`, `ListItem`.
Outros tipos (`EnumItem`, `Link`, etc.) adicionados em passos futuros.

### `Selector`

Selector de uma show rule. Variantes:
- `Text(String)` — substitui ocorrências literais de um texto (via `map_text`).
- `NodeKind(NodeKind)` — interceta nós nativos de um tipo específico.
- `DynKind(String)` — interceta um elemento **dinâmico** de utilizador
  (fronteira E1) pelo **nome de kind** (`dyn_kind()`). Lote F-3 inc-2: o
  selector `#show callout:` resolve para uma `FuncRepr::Element` sem fn-ptr
  nativo e casa por nome. Viaja pela **mesma** travessia que `NodeKind`.

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
  (aplicada por `map_text`). As outras formas falham explicitamente sobre `Text`
  (DEBT-19 ENCERRADO).
- `Style(Styles)` — **show-set** (`#show k: set …`, P352). Os styles do `set` são
  **capturados** na declaração (sem mutar `engine.styles` globalmente) e
  **transportados** para o nó casado embrulhando-o num `Content::Styled(elem,
  styles)` — o carregador `StyledElem`-scoped da fatia 1 (P339, `f_fronteira_e1.md`
  §3a.8). **NÃO consome o passe** (espelha `map.apply(transform); continue` do
  vanilla, `typst-realize/src/lib.rs:458-464`): o elemento renderiza sob a chain
  aumentada, não é substituído. Paridade: `styles.rs:504`
  (`content.styled_with_map(styles)`).

### `ShowRule`

Triplo `(id, selector, transform)` armazenado no `EvalContext` durante a avaliação.
`id` é um `RuleId` único por sessão de avaliação.
`transform` é uma `Transformation` (ver acima).

## Invariantes

- `ShowRule` é `Clone` — o motor clona o `Vec<ShowRule>`/`Arc<[ShowRule]>` antes de
  aplicar (snapshot explícito para evitar borrow conflict durante `apply_show_rules`).
- `Selector::NodeKind` identifica o tipo pelo enum, não por string (DEBT-21 MITIGADO);
  `Selector::DynKind` identifica o elemento dinâmico pelo nome de kind interned.
- `Selector::Text` suporta apenas `Transformation::Str`; `Func`/`Content`/`Style`
  sobre `Text` falham explicitamente (DEBT-19 ENCERRADO).
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

## Fora de escopo (registrado)

- **Caso 1 (composição multi-regra sobre o mesmo elemento)** — NÃO materializado.
  A composição fiel ao vanilla (cada regra uma vez, innermost-first) exige um guard
  por-recipe incompatível com o re-apply do modelo α (caso 2, fechado). Decisão de
  desenho do dono (relatório P352 §4). Este L0 não a especifica.

## Layer

L1 — domínio puro. Sem I/O, sem estado global.

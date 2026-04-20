# Prompt L0 — Entidade `ShowRule` (Passo 68-70)
Hash do Código: 3f4024ee

## Propósito

Define as entidades de dados para o motor de show rules do cristalino.
Uma show rule (`#show selector: transform`) interceta nós de conteúdo no
momento da sua criação (eager) e aplica uma transformação declarativa.

## Tipos

### `NodeKind`

Tipo de nó de conteúdo para selectorção por tipo.
Conjunto completo (Passo 69): `Heading`, `Figure`, `Strong`, `Emph`, `Raw`,
`Equation`, `ListItem`.
Outros tipos (`EnumItem`, `Link`, etc.) adicionados em passos futuros.

### `Selector`

Selector de uma show rule. Variantes:
- `Text(String)` — substitui ocorrências literais de um texto
- `NodeKind(NodeKind)` — interceta nós de um tipo específico

### `RuleId`

Tipo alias `pub type RuleId = u64`. Identificador único atribuído a cada
`ShowRule` no momento da sua criação em `EvalContext` (campo `next_rule_id: RuleId`,
incrementado a cada regra registada).

### `ShowRule`

Quádruplo `(id, selector, transform)` armazenado no `EvalContext` durante a avaliação.
`id` é um `RuleId` único por sessão de avaliação.
`transform` é um `Value` — pode ser `Value::Func` (closure) ou `Value::Content`
(substituição estática) ou `Value::Str` (para `Selector::Text`).

## Invariantes

- `ShowRule` é `Clone` — o motor clona o `Vec<ShowRule>` antes de aplicar
  (snapshot explícito para evitar borrow conflict durante `apply_show_rules`).
- `Selector::NodeKind` identifica o tipo pelo enum, não por string (DEBT-21 MITIGADO).
- `Selector::Text` suporta apenas substituição por `Value::Str`; transformação
  por `Value::Func` ou `Value::Content` falha explicitamente (DEBT-19 ENCERRADO).
- Anti-recursão via `active_guards: Vec<RuleId>` no `EvalContext` (DEBT-20 ENCERRADO).
  Uma regra é saltada se o seu `id` já está em `active_guards` — permite composição
  de show rules distintas (a regra A pode chamar B, mas B não re-activa B).
- `apply_show_rules` faz uma única travessia `map_content` para todos os NodeKind
  rules (DEBT-23 ENCERRADO): itera o snapshot de regras dentro da closure.

## Layer

L1 — domínio puro. Sem I/O, sem estado global.

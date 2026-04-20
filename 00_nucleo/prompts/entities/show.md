# Prompt L0 — Entidade `ShowRule` (Passo 68)

## Propósito

Define as entidades de dados para o motor de show rules do cristalino.
Uma show rule (`#show selector: transform`) interceta nós de conteúdo no
momento da sua criação (eager) e aplica uma transformação declarativa.

## Tipos

### `NodeKind`

Tipo de nó de conteúdo para selectorção por tipo.
Subset inicial — apenas `Heading` e `Figure`.
Outros tipos (`Raw`, `ListItem`, `Equation`, etc.) adicionados em passos futuros (DEBT-19).

### `Selector`

Selector de uma show rule. Variantes:
- `Text(String)` — substitui ocorrências literais de um texto
- `NodeKind(NodeKind)` — interceta nós de um tipo específico

### `ShowRule`

Triplo `(selector, transform)` armazenado no `EvalContext` durante a avaliação.
`transform` é um `Value` — pode ser `Value::Func` (closure) ou `Value::Content`
(substituição estática) ou `Value::Str` (para `Selector::Text`).

## Invariantes

- `ShowRule` é `Clone` — o motor clona o `Vec<ShowRule>` antes de aplicar
  (snapshot explícito para evitar borrow conflict durante `apply_show_rules`).
- `Selector::NodeKind` identifica o tipo pelo enum, não por string (tipos fortes).
- `Selector::Text` suporta apenas substituição por `Value::Str`; transformação
  por `Value::Func` ou `Value::Content` falha explicitamente (DEBT-19).

## Layer

L1 — domínio puro. Sem I/O, sem estado global.

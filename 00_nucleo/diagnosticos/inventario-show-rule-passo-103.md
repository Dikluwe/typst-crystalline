# Inventário `#show` — Passo 103.A

Data: 2026-04-23.

---

## Parte 1 — Estado actual

### Machinery

`#show` **já está activo** no cristalino para selectores de texto e
de `NodeKind`:

| Componente | Localização | Passo origem |
|-----------|-------------|--------------|
| `ShowRule { id, selector, transform }` | `entities/show.rs` | 69 |
| `Selector { Text(String), NodeKind(NodeKind) }` | `entities/show.rs` | 69 |
| `NodeKind { Heading, Figure, Strong, Emph, Raw, Equation, ListItem }` | `entities/show.rs` | 69 |
| `apply_show_rules` | `rules/eval/rules.rs:37` | 70 |
| `intercept_content` | `rules/eval/rules.rs:155` | 70 |
| `eval_show_rule` | `rules/eval/rules.rs:307` | 70 |
| Selector match para `Strong`/`Emph` via `Content::Styled + Style::Bold/Italic(true)` | `rules/eval/rules.rs:80-94` | 101 |

### NodeKind matching actual

```rust
// rules/eval/rules.rs:80-94 (após Passo 101)
use crate::entities::style::Style;
let is_bold_styled = matches!(node, Content::Styled(_, ss)
    if ss.iter().any(|s| matches!(s, Style::Bold(true))));
let is_italic_styled = matches!(node, Content::Styled(_, ss)
    if ss.iter().any(|s| matches!(s, Style::Italic(true))));

let is_match = matches!(
    (node, kind),
    (Content::Heading { .. },  NodeKind::Heading)
    | (Content::Figure { .. },   NodeKind::Figure)
    | (Content::Raw { .. },      NodeKind::Raw)
    | (Content::Equation { .. }, NodeKind::Equation)
    | (Content::ListItem(_),     NodeKind::ListItem)
) || (matches!(kind, NodeKind::Strong) && is_bold_styled)
  || (matches!(kind, NodeKind::Emph)   && is_italic_styled);
```

### Selector closure

Em `eval_show_rule`, quando `selector` é `Value::Func`, o
cristalino usa `native_fn_addr()` para resolver a identidade da
função nativa (`native_heading`, `native_figure`, `native_strong`,
`native_emph`, `native_raw`) e mapear para o `NodeKind`
correspondente. Implementado no Passo 84.3 (DEBT-21 encerrado).

### Testes existentes

- `eval_show_rule_text_substitui_ocorrencias`
- `eval_show_rule_funcao_no_heading`
- `eval_show_rule_falha_explicita_tipo_retorno_invalido`
- Mais 4 testes em bloco (alias, scoping, ordem).

Total: ~8 testes `#show` em `eval/tests.rs` — todos passando.

---

## Parte 2 — AST `ShowRule`

```rust
pub struct ShowRule<'a>(SyntaxNode);

impl<'a> ShowRule<'a> {
    pub fn selector(&self) -> Option<Expr<'a>>;  // Some(expr) ou None (catch-all, não suportado)
    pub fn transform(&self) -> Expr<'a>;         // closure ou Content directo
}
```

Confirmado lendo `01_core/src/entities/ast/code.rs:78`.

---

## Parte 3 — Fluxo `Value::Content` em closures

Confirmado lendo `eval_show_rule` + `apply_show_rules`:

- Selector `NodeKind(kind)` detecta content do tipo certo (e, para
  `Strong`/`Emph`, via `Content::Styled` + bold/italic).
- `apply_show_rules` embrulha `Content` em `Value::Content(content.clone())`
  e passa como único argumento posicional via `Args::positional`.
- `apply_func` invoca a closure com o scope capturado + `it: Value::Content`.
- Retorno: aceita `Value::Content(c)` → substitui; `Value::Str(s)` →
  converte para `Content::text(s)`; outros tipos → erro.

Fluxo **funcional** — zero mudança necessária no Passo 103.

---

## Parte 4 — Dívida exposta (spec 103)

A spec do Passo 103 identifica dívida latente: o selector
`NodeKind::Strong` apanha **qualquer** `Content::Styled` com
`Style::Bold(true)`, incluindo os produzidos por `#set text(bold:
true)` e por `#set text(fill: red, bold: true)`.

### Cenário do bug

```typst
#show strong: it => [HIT]
#set text(bold: true)
texto
```

**Vanilla**: "texto" em bold; `#show strong` **não** dispara porque
`#set` não cria um elemento Strong.

**Cristalino pós-Passo 102**: `#set text(bold: true)` empilha
`StyleDelta { bold: Some(true), .. }` em `*styles`. Os subsequentes
`Content::Text(s, style_with_bold)` são produzidos.

Mas há uma subtileza: o `#set text` cria conteúdo bold via
**bake-in** directo no `Content::Text.style`, **não** via
`Content::Styled([Bold(true)], ...)`. Logo, o selector `Strong` que
casa `Content::Styled` **não** dispara para texto afectado por `#set
text`.

### Re-análise: é realmente um bug?

Na arquitectura bake-in actual:
- `*bold*` → `Content::strong(body)` → `Content::Styled(body,
  [Bold(true)])` → selector match.
- `#set text(bold: true); texto` → `StyleDelta` empilhado →
  `Content::Text(texto, { bold: true })` → selector **não** casa
  porque o nó é `Text`, não `Styled`.

**Conclusão**: a dívida latente teórica **não se manifesta** hoje.
Confirmar por teste. Se confirmado, a ADR-0041 pode documentar
isto com mais tranquilidade: a arquitectura bake-in do `#set text`
(Passo 30) acidentalmente mantém o selector `Strong` preciso.

A dívida **real** seria se futuramente:

1. `#set text(bold: true)` passar a produzir `Content::Styled` wrapping
   (refactoring do Passo 102 deferido).
2. OU se qualquer outra directiva começar a emitir `Content::Styled`
   com Bold como efeito colateral.

Nesse momento a ambiguidade torna-se real. Por agora, **documentar**
que a dívida está latente mas não activa.

---

## Parte 5 — Pontos de aplicação

`apply_show_rules` é chamado via `intercept_content` em cada ponto
de produção de `Content` no eval. Verificar em `eval_markup`
onde é invocado:

```rust
// eval/mod.rs
for child in node.children() {
    match child.kind() {
        SyntaxKind::Text => {
            ...
            let text_node = Content::Text(...);
            parts.push(rules::intercept_content(text_node, ctx, ...)?);
        }
        ...
        _ => {
            if let Some(expr) = Expr::from_untyped(child) {
                match eval_expr(expr, ...)? {
                    Value::Content(c) => parts.push(rules::intercept_content(c, ctx, ...)?),
                    ...
                }
            }
        }
    }
}
```

Cada `Content` produzido passa por `intercept_content` → recursivamente
aplica `#show` em todos os nós via `map_content`. Todas as produções
relevantes são interceptadas.

---

## Recomendação

`#show` já está activo. Passo 103 é **validação + documentação**:

1. Acrescentar 4–5 testes de integração que validam `#show heading`,
   `#show strong`, `#show emph` end-to-end através do Layouter
   (não apenas "não dá Err" — verificar que o Content resultante
   reflecte a transformação).
2. Acrescentar **teste que documenta a dívida latente**:
   `#show strong: it => [HIT]; #set text(bold: true); texto` — com
   assertion do comportamento **actual** (provavelmente "texto"
   sem "HIT" porque `#set text` usa bake-in e não Styled).
3. Criar ADR-0041 formalizando o estado + reconhecendo a dívida
   latente.
4. Abrir DEBT-50 "Show selector para Strong/Emph não distingue
   origem quando Styled wrapping for adoptado" — dívida condicional
   que fica adormecida enquanto bake-in for o mecanismo de `#set text`.
5. Actualizar DEBT-1 — `#show` concluído para heading/strong/emph;
   selectores restantes (where, catch-all, literal além de Text,
   raw, list) pendentes.

Critério: ≥ 5 testes novos; dívida documentada; DEBT-50 aberto; ADR
`EM VIGOR`.

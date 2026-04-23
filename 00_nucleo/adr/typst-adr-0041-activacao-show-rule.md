# ADR-0041 — Activação de `#show` — heading, strong, emph

**Status**: EM VIGOR (Passo 103.E) — validado empiricamente com 795
testes L1 a passar (+5 integração `#show` novos), zero violations.
**Data**: 2026-04-23
**Autor**: Humano + IA
**Passo associado**: 103

---

## Contexto

A directiva `#show SELECTOR: it => BODY` do Typst transforma
conteúdo do tipo alvo via closure. No cristalino, `#show` está
activo desde o **Passo 70** (DEBT-23 encerrado) para selectores
de texto (`#show "A": "B"`) e de `NodeKind` (`#show heading: ...`,
`#show strong: ...`, `#show emph: ...`, etc.).

A análise do Passo 103.A confirmou:

1. `ShowRule` / `Selector` / `NodeKind` em `entities/show.rs` desde Passo 69.
2. `apply_show_rules` + `intercept_content` em `rules/eval/rules.rs`
   desde Passo 70.
3. `native_fn_addr` para resolver selectores funcionais
   (`strong`, `emph`, `heading`) desde Passo 84.3 (DEBT-21 encerrado).
4. Selector match actualizado no Passo 101 para usar `Content::Styled`
   com `Style::Bold(true)` / `Style::Italic(true)`.

Este ADR formaliza o estado actual como `EM VIGOR`, documenta
limites, e regista dívida latente.

## Decisão

### Selectores suportados (catálogo)

| Selector | Match |
|----------|-------|
| `#show "literal": ...` | `Selector::Text(s)` — substitui ocorrências via `Content.map_text` |
| `#show heading: it => ...` | `Selector::NodeKind(NodeKind::Heading)` — casa `Content::Heading { .. }` |
| `#show strong: it => ...` | `Selector::NodeKind(NodeKind::Strong)` — casa `Content::Styled(_, ss)` onde `ss` contém `Style::Bold(true)` |
| `#show emph: it => ...` | Análogo: casa `Content::Styled(_, ss)` com `Style::Italic(true)` |
| `#show raw: ...` | Casa `Content::Raw { .. }` |
| `#show figure: ...` | Casa `Content::Figure { .. }` |
| `#show equation: ...` | Casa `Content::Equation { .. }` |
| `#show list: ...` | Casa `Content::ListItem(_)` |

Todos os selectores funcionais (não-string) são resolvidos via
`Func::native_fn_addr` — alias `#let h = heading` funcionam porque
partilham o mesmo `fn` ptr.

### Selectores não cobertos

- `#show heading.where(level: N): ...` — selectores com filtro. AST não expõe `.where`.
- `#show: rest => ...` — catch-all. `eval_show_rule` rejeita selector `None`.
- `#show "texto": it => F(it)` — text selector com closure (apenas
  text-to-text funciona hoje).
- `#show REGEX: ...` — regex selectors.
- `#show LABEL: ...` — selectores por label.

Estas formas ficam adiadas; **abrir DEBT** se passo futuro precisar.

### Semântica de escopo

Consistente com `#set` (Passo 102, ADR-0040). `#show` em markup
afecta o conteúdo seguinte no mesmo bloco; sobreposição por outro
`#show` para o mesmo selector é honrada (último ganha).

Implementado via `show_rules: &mut Arc<[ShowRule]>` parâmetro
propagado às funções `eval_*` (Passo 95, ADR-0036 terceira
aplicação). Cada `eval_show_rule` empilha um `ShowRule` via
`Arc::make_mut` em `*show_rules`.

### Anti-recursão

Um `ShowRule` em execução não pode re-entrar em si própria:
`active_guards: &mut Vec<RuleId>` rastreia as rules em execução.
O `RuleId` é alocador monotónico de `EvalContext.next_rule_id`
(Regra 4 — ADR-0036).

### Dívida latente reconhecida

Quando `#set text(bold: true)` for migrado de bake-in para
`Content::Styled` wrapping (Passo futuro, não decidido neste ADR),
o selector `NodeKind::Strong` começará a apanhar `Styled`
produzidos por `#set text`. Neste momento, o cristalino ficará
divergente do vanilla:

- Vanilla: `#show strong` só dispara para `*bold*` (elementos
  Strong explícitos).
- Cristalino pós-migração: `#show strong` dispararia também para
  `#set text(bold: true)\ntexto`.

**Hoje (Passo 103) o bug não se manifesta** porque `#set text` usa
bake-in no `Content::Text`, não wrapping. O selector Strong só
apanha `Content::Styled` — que é produzido por `*bold*` mas não por
`#set text`.

**DEBT-50 aberto** — registar a dívida condicional. Solução futura:

- **Flag de origem no enum `Style`**: `Style::Bold { value: bool, from_strong: bool }` — intrusivo.
- **Marcador no `Content::Styled`**: `Content::Styled(body, styles, origin: Option<ElementKind>)` — mudança estrutural.
- **Selector rigoroso**: apenas `Content` com styles contendo **exactamente** `[Style::Bold(true)]` (e nada mais) — heurístico frágil.

Nenhuma destas soluções é activada neste passo.

### Não-recursividade aceite

Quando `#show strong: it => heading(1, it.body)` — o retorno é
`Content::Heading`. No vanilla, isto pode disparar `#show heading`
(recursão limitada por `MAX_SHOW_RULE_DEPTH = 64`).

O cristalino tem `route_check_show_depth` (Passo 93, paridade
vanilla), mas a aplicação é numa **única travessia via `map_content`**
— a primeira rule que casa transforma; o retorno não re-entra no
selector.

**Aceitável** para o catálogo actual. DEBT-51 (opcional) pode ser
aberto se casos reais aparecerem.

### O que esta ADR não decide

- **Catch-all `#show: rest => ...`**: exige mudança na AST/parser.
- **Selectores com filtro `.where`**: AST não expõe, adiado.
- **Migração `#set text` bake-in → wrapping**: decisão estrutural
  independente.
- **Resolução da dívida DEBT-50**: quando o cenário se manifestar.

## Alternativas consideradas

1. **Não expor dívida**: usar só `#show heading` neste passo, adiar
   strong/emph. Rejeitado — strong/emph já funcionam hoje (Passo
   101); limitar o ADR seria puro marketing.

2. **Resolver DEBT-50 agora**: adicionar flag de origem a `Style` ou
   marcador a `Content::Styled`. Rejeitado — mudança estrutural
   invasiva sem ganho imediato (a dívida é latente, não activa).

3. **Catch-all e where neste passo**: adicionar parsing + selector
   logic. Rejeitado — excede escopo; ADR próprio quando AST
   expandir.

## Consequências

### Positivas

- Estado funcional de `#show` documentado e validado.
- ADR formaliza limites — próximos passos sabem o que está pronto.
- DEBT-50 registado como trigger: quando `#set text` migrar para
  wrapping, o cenário activa-se e o DEBT torna-se accionável.

### Negativas

- Documentação cresce (mais um ADR).
- DEBT-50 fica como cautela para futuro — pode ficar adormecido
  muito tempo se a migração nunca acontecer.

### Neutras

- Zero mudança funcional. Testes existentes continuam a passar; testes
  novos validam mais casos.

---

## Referências

- `00_nucleo/diagnosticos/inventario-show-rule-passo-103.md`
- `00_nucleo/materialization/typst-passo-103.md`
- ADR-0040 (activação de `#set`)

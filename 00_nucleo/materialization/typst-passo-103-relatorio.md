# Passo 103 — Relatório de encerramento (Activação de `#show`)

**Data**: 2026-04-23
**Precondição**: Passo 102 encerrado; `#set text(...)` funcional;
790 L1 + 174 L3 + 6 ignorados; zero violations.
**ADR criada**: ADR-0041 "Activação de `#show` — heading, strong, emph"
— **PROMOVIDA A EM VIGOR** em 103.E.

---

## Sumário

**Descoberta do 103.A**: `#show` já estava activo desde **Passo 70**
(DEBT-23 encerrado) para selectores `Text` e `NodeKind`. Nos Passos
84.3 (DEBT-21) e 101, foi ampliado para resolver funções nativas
via `native_fn_addr` e para casar `Content::Styled` com
`Style::Bold/Italic(true)`.

Passo 103 foi **validação + formalização**: 5 testes de integração
end-to-end + ADR-0041 + DEBT-50 para dívida latente.

Zero regressão funcional: **790 → 795 L1** (+5), 174 L3 + 6
ignorados inalterados. `crystalline-lint .` → zero violations.

---

## 103.A — Inventário

Inventário em `00_nucleo/diagnosticos/inventario-show-rule-passo-103.md`.

### Machinery já presente

| Componente | Localização | Passo |
|-----------|-------------|-------|
| `ShowRule { id, selector, transform }` | `entities/show.rs` | 69 |
| `Selector { Text, NodeKind }` + `NodeKind { Heading, ..., ListItem }` | `entities/show.rs` | 69 |
| `apply_show_rules` | `rules/eval/rules.rs:37` | 70 |
| `intercept_content` | `rules/eval/rules.rs:155` | 70 |
| `eval_show_rule` | `rules/eval/rules.rs:307` | 70 |
| `native_fn_addr` selector resolution | `entities/func.rs` | 84.3 |
| Match `Content::Styled` com `Style::Bold/Italic` | `rules/eval/rules.rs:80` | 101 |
| `show_rules: &mut Arc<[ShowRule]>` parâmetro | todas `eval_*` | 95 |

### Testes `#show` existentes

- `eval_show_rule_text_substitui_ocorrencias`
- `eval_show_rule_funcao_no_heading`
- `eval_show_rule_funcao_com_alias_dispara`
- `eval_show_rule_falha_explicita_tipo_retorno_invalido`
- +4 testes de scoping/ordem.

---

## 103.B — ADR-0041

Criada em `00_nucleo/adr/typst-adr-0041-activacao-show-rule.md`.
**Promovida a EM VIGOR em 103.E**.

Documenta:

- **Catálogo**: `#show "literal"`, `#show heading/strong/emph/raw/figure/equation/list: ...`
  com closure `it => body` ou valor `Content` directo.
- **Selectores não cobertos**: `.where`, catch-all `: rest => ...`,
  text selector com closure, regex, label selectors. Ficam como
  extensões futuras.
- **Semântica de escopo**: consistente com `#set` (Passo 102). `show_rules`
  propagado como parâmetro desde Passo 95 (ADR-0036 terceira aplicação).
- **Anti-recursão**: `active_guards: &mut Vec<RuleId>`. `RuleId`
  alocado em `EvalContext.next_rule_id` (Regra 4 da ADR-0036).
- **Dívida latente aceite**: selector Strong/Emph apanha qualquer
  `Content::Styled` com Bold/Italic. Inofensivo hoje
  (`#set text` usa bake-in), mas activa-se se `#set text` migrar
  para wrapping. DEBT-50 regista a dívida condicional.

---

## 103.C — Modificações

**Zero modificações de código.** `#show` já funciona. Passo 103 é
puramente validação + documentação.

---

## 103.D — Testes de integração

5 testes novos em `rules/layout/tests.rs::tests_show_rule_integration`:

### 1. `show_heading_transforma_em_uppercase`

```rust
let doc = layout_typst("#show heading: it => upper(it.body)\n\n= Intro");
assert!(plain_text(&doc).contains("INTRO"));
```

End-to-end: parse → `Content::Heading { level: 1, body: "Intro" }` →
`apply_show_rules` casa selector Heading → closure `upper` executa
→ `Content::text("INTRO")` → Layouter → `FrameItem::Text("INTRO", ...)`.

### 2. `show_strong_transforma`

```rust
let doc = layout_typst("#show strong: upper\n*alvo*");
assert!(plain_text(&doc).contains("ALVO"));
```

Selector `strong` (via `native_strong` fn ptr) casa
`Content::Styled([Bold(true)], body)` produzido por `*alvo*`.

### 3. `show_emph_transforma`

```rust
let doc = layout_typst("#show emph: lower\n_TIPO_");
assert!(plain_text(&doc).contains("tipo"));
```

Análogo para italic.

### 4. `regressao_sem_show_mantem_comportamento`

Sem `#show`: `*bold*` produz item com `style.bold=true`; `_italic_`
produz item com `style.italic=true`. Garante zero interferência
quando `show_rules` está vazio.

### 5. `debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in`

Documenta a **dívida latente** do DEBT-50:

```rust
let doc = layout_typst(
    "#show strong: it => [HIT]\n\
     #set text(bold: true)\n\
     texto"
);
assert!(!plain_text(&doc).contains("HIT"));  // expected NO HIT
assert!(plain_text(&doc).contains("texto"));
```

**Racional**: `#set text(bold: true)` usa **bake-in** —
empilha `StyleDelta` em `*styles`; subsequentes `Content::Text(s,
style_with_bold)` são produzidos sem `Content::Styled` wrapping.
Selector `Strong` só casa `Content::Styled`, portanto **não**
dispara. Paridade com vanilla preservada.

Este teste é um **canário**: se falhar num passo futuro que migrar
`#set text` para wrapping, DEBT-50 torna-se accionável.

---

## 103.E — Encerramento

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 795 passed; 0 failed; 0 ignored ...
test result: ok. 174 passed; 0 failed; 6 ignored ...

$ crystalline-lint .
✓ No violations found
```

### DEBTs

- **DEBT-50 aberto** — "Show selector Strong/Emph não distingue
  origem (dívida latente)". Critério de conclusão explícito;
  dependência: activa-se quando `#set text` migrar de bake-in para
  wrapping.
- **DEBT-1 actualizado** — secção "Actualização Passo 103". A
  pendência "activar `#show`" concluída. Selectores remanescentes
  (where, catch-all, regex, label) fora do escopo do DEBT-1
  original.

### ADR

- **ADR-0041** `EM VIGOR`.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 790 | **795** (+5) |
| L3 tests | 174 | 174 |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| Selectores `#show` suportados | `Text` + 7 `NodeKind` | idem (validado) |
| ADRs activas | 40 | **41** (+0041) |
| DEBTs em aberto | 12 | **13** (+DEBT-50) |

---

## Lições

1. **Validação como forma de activação**: Passos 102 e 103
   descobriram que `#set` e `#show` já estavam activos há muito
   (Passos 30 e 70). A "activação" tornou-se formalização +
   testes end-to-end + documentação da dívida latente.

2. **Dívida condicional é legítima**: DEBT-50 é uma "bomba-relógio"
   — dormente hoje, accionável quando um trigger específico
   (migração `#set text` → wrapping) for disparado. O teste-canário
   `debt_50_...` deixa o trigger auto-detectável. Este padrão de
   DEBT merece ser reproduzido.

3. **Consistência de paridade**: a arquitectura bake-in do `#set text`
   (Passo 30) revelou-se felizmente alinhada com a semântica vanilla
   do `#show strong`. Esta coincidência preserva paridade enquanto
   a arquitectura dual (bake-in + wrapping) coexistir.

4. **Zero mudança de código, máximo de valor**: Passo 103 não
   alterou uma única linha de `.rs`. O valor foi entregue via
   testes (que provam o funcionamento) + ADR (que formaliza) +
   DEBT-50 (que documenta dívida condicional). Padrão válido para
   consolidações puras.

---

## Estado pós-Passo 103

### `#set` + `#show` pipeline end-to-end

```
Parse  → Expr::SetRule    → eval_set_rule    → *styles (bake-in ou wrapping)
       → Expr::ShowRule   → eval_show_rule   → *show_rules (Arc<[ShowRule]>)
       → Content produzido normalmente
                                 ↓
              intercept_content(content, show_rules, ...)
                                 ↓
                 apply_show_rules travessia map_content
                                 ↓
        matches Heading/Strong/Emph/Figure/Raw/Equation/List
                                 ↓
              closure executada via apply_func
                                 ↓
                Content transformado → Layouter → FrameItem
```

### DEBT-1 estado final pós-Passo 103

| Pendência original | Estado |
|-------------------|--------|
| Scoping de `#set` por bloco | ✓ Passo 33 + 94 |
| Arquitectura partilhada de `styles` | ✓ Passo 94 |
| Remover wrappers Strong/Emph | ✓ Passo 101 |
| Activação de `#set`/`#show` | ✓ Passos 102 + 103 (dentro do catálogo) |
| Propriedades adicionais (font, lang, leading) | **Pendente** — bloqueado por tipos |

DEBT-1 pode ser revisto para **PARCIALMENTE RESOLVIDO (estrutura
paga em Passos 100 + 101 + 102 + 103; bloqueadores restantes são
materialização de Font/Lang/Par)**.

### Trabalho futuro identificado

1. **Propriedades adicionais em `#set text`**: `font`, `lang`,
   `weight` como string, `par.leading` — bloqueadas por tipos não
   materializados em L1. Passos dedicados quando Font/Lang/Par
   entrarem.
2. **Selectores `#show` avançados**: `where`, catch-all, regex,
   label. Requer extensão da AST/parser.
3. **DEBT-50**: condicional. Activa-se se `#set text` migrar para
   wrapping.
4. **DEBT-49** (propriedades silenciadas): depende de `Sink`.
5. **Materialização de `Engine<'a>`**: agregador dos 9 parâmetros
   das funções `eval_*`. Evidência empírica do Passo 98 continua a
   ganhar força.

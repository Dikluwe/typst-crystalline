# Prompt L0 — `compiler/eval/selector_matching` — matching de selectores de show rule
Hash do Código: 788a489e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/selector_matching.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval.md` (dono de `compiler/eval/rules.rs`)
**ADRs relevantes**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0109 (atomização)
**Técnica**: pattern matching estrutural

---

## Contexto

Este nó contém as operações puras de matching e conversão de selectores usadas pelo hub `rules.rs` durante a aplicação de show rules:

- converter um `entities::selector::Selector` (query) num `entities::show::Selector` (show rule);
- decidir se um selector de show rule casa com um nó de conteúdo;
- decidir se um selector viaja pela travessia de nós (`map_content`) ou é tratado noutro sítio;
- fatiar texto nas ocorrências de um padrão para aplicação de show rules de texto.

Extraído de `compiler/eval/rules.rs` no Passo 1011 conforme ADR-0109 (atomização — forma B, free function no arquivo da unidade).

---

## Instrução

### 1. Contrato público

```rust
pub(crate) fn query_selector_to_show_selector(
    sel: QuerySelector,
    span: Span,
) -> SourceResult<Selector>;

pub(crate) fn selector_matches(work: &Content, selector: &Selector) -> bool;

pub(crate) fn is_node_rule(selector: &Selector) -> bool;

pub(crate) fn splice_text_rule_matches(
    text: &str,
    pattern: &str,
    mut replacement: impl FnMut(&str) -> SourceResult<Content>,
) -> SourceResult<Option<Content>>;
```

- `query_selector_to_show_selector` — converte `QuerySelector` (de `heading.where(level: 1)`, combinadores `And`/`Or`, etc.) para `Selector` de show rule. Rejeita selectors não suportados com mensagem clara.
- `selector_matches` — casa um `Content` contra um `Selector`. Puro: não toca `Engine`/`EvalContext`.
- `is_node_rule` — decide se um selector deve viajar pela travessia de nós (`NodeKind`, `DynKind`, `Where`/`And`/`Or` sobre node-like). `Text`/`Regex`/`Label` retornam `false`.
- `splice_text_rule_matches` — fatia uma string nas ocorrências de `pattern`, substituindo cada match por `replacement(matched)`. Devolve `Ok(None)` se não houver match.

### 2. Comportamento

Manter exatamente o comportamento actual:

- `NodeKind`: casamento por tipo de nó, com regras especiais para `List`/`Enum` (sequence uniforme ou item isolado) e para origens sintáticas de `Strong`/`Emph`/`Subscript`/`Superscript`/`Highlight` via `is_styled_origin`.
- `DynKind`: casa `Content::Dynamic` com o mesmo `dyn_kind`.
- `Text`/`Regex`: nunca casam em `selector_matches` (tratados em loops dedicados).
- `Label`: nunca casa em `selector_matches` (aplicado em `intercept_labelled`).
- `Where`: casa a base e verifica igualdade semântica do campo (`values_eq_semantic`, com coerção Int↔Float).
- `And`/`Or`: curto-circuito; vazios retornam `false`.
- Conversão de query selector: `Kind` → `NodeKind` para elementos nativos suportados; `Where` com base node-like; `And`/`Or` recursivos; resto rejeitado.

### 3. Helper privado

`values_eq_semantic(actual: &Value, expected: &Value) -> bool` move-se com o nó (usado apenas por `selector_matches`). Replica ADR-0025 (coerção Int↔Float) e ADR-0107 (paridade comportamental).

### 4. Gatilhos de reabertura

- Novo tipo de `Selector` ou `QuerySelector`.
- Novo tipo de nó `Content` com regras de matching especiais.
- Mudança de fase (eval ↔ layout) no processamento de show rules.

---

## Critérios de verificação

```
Dado selector NodeKind(Heading) e Content::heading → true
Dado selector Where(Heading, level=1) e Content::heading(1) → true
Dado selector Where(Heading, level=2) e Content::heading(1) → false
Dado combinador And vazio → false
Dado combinador Or vazio → false
Dado is_node_rule(Text) → false
Dado splice_text_rule_matches("aXbXc", "X", …) → sequence ["a", repl, "b", repl, "c"]
```

Aplicação final: `cargo build && crystalline-lint .` — zero violations.

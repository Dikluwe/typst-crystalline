# Prompt L0 — `show-regex` — wiring de `#show regex(...)`
Hash do Código: a3f2c53b

**Camada**: L1
**Ficheiros alvo**: `01_core/src/entities/value.rs`, `01_core/src/entities/show.rs`, `01_core/src/rules/stdlib/text.rs` (ou módulo regex dedicado), `01_core/src/rules/eval/mod.rs`, `01_core/src/rules/eval/rules.rs`
**Origem**: Passo 393 (`typst-passo-393.md`) — dívida genuína acidental (balde D), S, zero I/O.
**ADRs**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0077 (`Selector::Regex` em L1), ADR-0017 (`Regex` já existe em L1; variant `Value::Regex` habilitado).

---

## 1. Contexto

O vanilla expõe `#show regex(pattern): it => body` — aplica uma show-rule a nós de texto cujo conteúdo textual casa com `pattern`. Em cristalino, o tipo `Regex` já existe em L1 (`entities/regex.rs`, ADR-0077) e o tipo `Selector::Regex` já existe no selector de query (`entities/selector.rs`). O que falta é:

1. Tornar `regex(...)` construtível em eval (`Value::Regex`).
2. Permitir que `#show regex(...): ...` produza `Selector::Regex` no `ShowRule`.
3. Aplicar a transformação aos nós de texto que casam.

## 2. Arquitetura

- **Sem tipo Rust novo**: reutiliza `entities::regex::Regex` existente.
- **Variant `Value::Regex`**: adiciona à enum `Value` (tipo já existe em L1; ADR-0017 satisfeito).
- **Variant `Selector::Regex`**: adiciona à enum `entities::show::Selector` (o `Regex` já existe).
- **Sem layout/render**: o wiring é em eval/show-rules, antes do layout.
- **Scope-out**: split interno do nó de texto casado; `.where(field:)` é materializado em P417/P467 (não neste passo).

## 3. Construtor `regex(pattern)`

`regex(pattern)` é função nativa (`native_regex`) que recebe um único argumento posicional `Str` e devolve `Value::Regex(Regex::new(pattern)?)`.

- `pattern` obrigatório, `Str`.
- Regex inválida → erro de eval contextual (mensagem da crate `regex`).
- Rejeita argumentos nomeados.

## 4. Show rule selector

Em `eval_show_rule`, quando o selector avaliado é `Value::Regex(re)`, criar `Selector::Regex(re)`.

A transformação pode ser `Func` (mais comum), `Content` ou `Str`. Sobre `Selector::Regex`, `Transformation::Style` continua inválido (mesma regra de `Selector::Text`).

## 5. Aplicação de show-rules regex

Em `apply_show_rules`, após o loop de NodeKind/DynKind e antes de retornar, percorrer as regras com `Selector::Regex`.

- Para cada nó de texto (`Content::Text`), se `regex.is_match(text)`, aplicar a transformação:
  - `Func(f)`: chamar `f([Value::Content(Content::text(text))])`; o resultado (Content ou Str) substitui o nó de texto.
  - `Content(c)`: substitui o nó de texto por `c`.
  - `Str(s)`: substitui o nó de texto por `Content::text(s)`.
- Nós que não casam permanecem inalterados.
- A paridade é semântica (ADR-0107): todo o nó de texto que casa é transformado. A divisão interna do nó (split por match) é scope-out deste passo.

## 6. Paridade vanilla

| Caso | Resultado esperado |
|------|--------------------|
| `#show regex("\\d+"): it => strong(it)` sobre texto com dígitos | texto fica strong |
| Mesma regra sobre texto sem dígitos | sem alteração |
| Regex inválida | erro de eval |
| Múltiplas show-rules (regex + NodeKind) | ordem de declaração, última declaração vence no mesmo selector |

## 7. Testes

- `regex("\\d+").is_match("abc123")` via `Value::Regex`.
- `#show regex("\\d+"): it => strong(it)` aplica strong a texto com dígitos.
- Regex inválida → erro.
- Regex sem match → sem alteração.
- `regex("[")` → erro.

## 8. Scope-out

- Não criar `Selector::Where`.
- Não dividir nós de texto internamente; transformar o nó inteiro que casa.
- Não tocar em layout/render.
- Não aplicar regex a conteúdo não-texto.

---

> **Estatuto: MATERIALIZADO — P393 + confirmado P473 + P474.**
> `Value::Regex`, `Selector::Regex`, `native_regex`, `eval_show_rule` wiring e `apply_show_rules` implementados e testados (P393). P473 confirma o wiring via L2 tests. P474 (sonda) confirma `#show heading.where(level: N)` E2E completo desde P417. **Trilha 3: 3/3 completo (fechado).**

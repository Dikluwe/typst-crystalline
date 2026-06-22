# Passo 393 — relatório: materialização de `#show regex(...)`

**Tipo:** materialização (L1 — wiring de show-rules; zero tipo Rust novo; zero I/O).
**Data:** 2026-06-22. **HEAD:** pós-`08fed76d3`.
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente
em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Materializou-se `#show regex(pattern): it => body` como wiring de show-rule sobre texto.

- `01_core/src/entities/value.rs` — adiciona `Value::Regex(Regex)` (o tipo `Regex` já existia em L1;
  ADR-0017 satisfeito).
- `01_core/src/entities/show.rs` — adiciona `Selector::Regex(Regex)` à enum de show-rules.
- `01_core/src/rules/stdlib/text.rs` — novo `native_regex(pattern)`; devolve `Value::Regex` ou erro
  contextual para pattern inválida.
- `01_core/src/rules/eval/mod.rs` — regista `regex` em `make_stdlib`.
- `01_core/src/rules/eval/rules.rs` — wiring central:
  - `eval_show_rule` aceita `Value::Regex` como selector;
  - `apply_show_rules` aplica regras `Selector::Regex` a nós de texto que casam
    (`re.is_match(text)`), suportando transformações `Func`, `Content` e `Str`;
  - rejeita show-set sobre regex (paridade com `Selector::Text`).
- `00_nucleo/prompts/rules/show-regex.md` — L0 novo.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` — `#show regex(...)`
  reclassificado de `ausente` para `implementado`; `.where(field:)` permanece `ausente`.

`cargo build --workspace` + `crystalline-lint .` verdes; **9** testes novos passam
(5 em `rules/eval/tests.rs` + 4 em `rules/stdlib/mod.rs`).

## Protocolo de Nucleação cumprido

1. Redigiu-se o L0 (`show-regex.md`) e propagaram-se hashes.
2. Implementou-se o wiring de show-rules regex (TDD).
3. Linhagem `@prompt`/`@prompt-hash` actualizada via `crystalline-lint --fix-hashes`.

## Decisão de engenharia

`#show regex(pattern): it => body` aplica a transformação a **nós de texto inteiros** cujo
conteúdo casa com a pattern. A paridade (ADR-0107) é semântica: texto que bate é transformado;
texto que não bate permanece inalterado. A divisão interna de um nó de texto em múltiplos
segmentos (split por match) é **scope-out** deste passo — a complexidade extra não é necessária
para os casos de teste declarados e manteria o passo dentro do escopo S.

`Value::Regex` foi adicionado sem violar ADR-0017 porque o tipo `Regex` já existia em L1
(`entities/regex.rs`, P209D/ADR-0077). Apenas se criaram os variants de enum necessários para
expô-lo a eval e show-rules.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `#show regex("\\d+"): it => strong(it)` sobre "abc123def" | texto com dígitos fica strong | ✓ |
| Mesma regra sobre "abcdef" | sem alteração | ✓ |
| Regex inválida (`"["`) | erro de eval | ✓ |
| `#show regex("\\d+"): set text(bold: true)` | erro (show-set sobre regex) | ✓ |
| Múltiplas regex — última declaração vence | última transformação aplica-se | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `#show regex("\\d+"): it => strong(it)` aplica strong a texto com dígitos | ✓ |
| 2 | Zero tipo Rust novo; zero I/O | ✓ (`Regex` já existia) |
| 3 | Tests verdes; lint zero; hashes propagados | ✓ 9/9; `✓ No violations` |
| 4 | Inventário 148 actualizado | ✓ A.2 + gaps |
| 5 | L0 salvo e hashado antes do código | ✓ `show-regex.md` |

## Artefactos

- Código: `01_core/src/entities/value.rs`, `entities/show.rs`, `rules/stdlib/text.rs`,
  `rules/eval/mod.rs`, `rules/eval/rules.rs`.
- L0: `00_nucleo/prompts/rules/show-regex.md`.
- Testes: `01_core/src/rules/eval/tests.rs` (5), `01_core/src/rules/stdlib/mod.rs` (4).
- Inventário 148 — `#show regex(...)` implementado; `.where()` ainda ausente.
- este relatório.

## Nota sobre o Tekt

Este passo é o **primeiro S após três XS** (`square`, `lorem`, `panic`). O salto de escopo foi
controlado: zero tipo Rust novo, zero I/O, só wiring de show-rules. O ciclo manteve-se limpo
porque o substrato (`Selector::Regex`, `Regex` L1) já estava pronto — a sonda 389 tinha razão.

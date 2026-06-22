# Passo 402 — relatório: refino de `Regex` como tipo de primeiro-cidadão

**Tipo:** refino de tipo L1 (S puro; zero I/O; não adiciona variant novo ao enum `Value`).  
**Data:** 2026-06-22. **HEAD:** pós-`84614c1a1`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Refinou-se o tipo `Regex` (introduzido em P209D para `Selector::Regex` e exposto em `Value::Regex` em P393) para primeiro-cidadão L1: `EcoString` + `Arc<regex::Regex>` + cast.

- L0:
  - `00_nucleo/prompts/entities/regex.md` — actualizado para P402 (refino, consumers `Value::Regex` + `Selector::Regex`, cast).
- `01_core/src/entities/regex.rs`:
  - `pattern` passou de `String` para `EcoString`.
  - `compiled` passou de `regex::Regex` directo para `Arc<regex::Regex>` — clone O(1), sem recompilação.
  - `Clone` passou a ser derive (cheap clone do Arc).
  - `Default` adicionado (pattern vazia válida).
  - `Hash`/`PartialEq`/`Eq`/`Debug` mantidos via pattern.
  - Testes unitários actualizados + `regex_default_empty_pattern`.
- `01_core/src/entities/value.rs`:
  - `Value::Regex(Regex)` já existia; adicionado `cast_regex()` (identidade + `Str → Regex` fallible).
  - Adicionado `From<Regex> for Value`.
  - Testes unitários: 5 testes (`type_name`, cast identity, cast from str, cast invalid, partial eq).
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:
  - Tabela B.1: adicionada entrada explícita `Regex` como `implementado` (P209D + P402); variants 22 → **23**, total 35 → **36**.
  - Tabela B resumo: implementado 78 → **79**, total arquitectural 110 → **111**.
  - Contagem user-facing total mantida em **141** (`Regex` já estava contabilizado).
  - Nota de rodapé ⁸⁶ para P402.

`cargo test --workspace` verde; `crystalline-lint .` — `✓ No violations found`; hashes propagados via `crystalline-lint --fix-hashes`.

## Protocolo de Nucleação cumprido

1. L0 (`regex.md`) actualizado e hash propagado.
2. TDD: testes escritos antes/paralelamente ao refino.
3. Zero variant novo — refino de tipo existente, mantendo `Value::Regex`.
4. Zero I/O; zero consumer novo — refino puro.

## Decisão de engenharia

A representação interna com `Arc<regex::Regex>` evita recompilar a regex em cada `clone` (padrão `Value::Tiling` P395). `EcoString` para `pattern` alinha com `Value::Str` e reduz alocação. A igualdade continua a ser por pattern string (valor linguagem), não por ponteiro do `Arc` — decisão ADR-0107.

`cast_regex` aceita `Str` e compila fallible, preparando o terreno para operações futuras (`match`, `replace`) e para `text.font` dict (DEBT-52), que precisam de `Value::Regex` como tipo de primeiro-cidadão.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `Regex::new("a.*b").is_match("axxxb")` | `true` | ✓ |
| `Regex::new("[")` | `Err` | ✓ |
| `Regex::new("a").clone()` == `Regex::new("a")` | `true` (pattern) | ✓ |
| `Value::Regex(...).type_name()` | `"regex"` | ✓ |
| `Value::Str("a.*b").cast_regex()` | compila com sucesso | ✓ |
| `Value::Str("[").cast_regex()` | `None` | ✓ |
| `#show regex("a.*b")` (regressão P393) | continua a funcionar | ✓ (teste P393 passou) |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Regex` struct L1 com `pattern` + `compiled` (Arc) | ✓ |
| 2 | `Value::Regex(Regex)` reusa tipo L1 | ✓ |
| 3 | `Selector::Regex(Regex)` reusa tipo L1 | ✓ (sem alteração) |
| 4 | `PartialEq` por `pattern` string | ✓ |
| 5 | `Cast`: `Str → Regex` fallible; `Regex → Regex` identity | ✓ |
| 6 | `native_regex` continua funcionando sem regressão | ✓ |
| 7 | `#show regex(...)` continua funcionando | ✓ |
| 8 | Testes verdes; lint zero; hashes propagados | ✓ 10 unit + regressões |
| 9 | Inventário 148 actualizado | ✓ |
| 10 | L0 salvo e hashado | ✓ `regex.md` |

## Artefactos

- Código:
  - `01_core/src/entities/regex.rs`
  - `01_core/src/entities/value.rs`
- L0:
  - `00_nucleo/prompts/entities/regex.md`
- Inventário 148: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- Plano: `00_nucleo/materialization/typst-passo-402.md`.
- Este relatório.

## Nota sobre o Tekt

P402 é **refino de tipo existente**, não modelagem de novo. O benefício é destravar duas frentes futuras:
1. Operações regex (`match`, `replace`, etc.) — agora têm tipo L1 com `is_match` e `Arc` para cheap clone.
2. `text.font` dict (DEBT-52) — requer `Value::Regex` como tipo de primeiro-cidadão para pattern matching em font names.

O ritmo de refino foi comparável ao de modelagem pura, validando que o wrapper L1 inicial (P209D) já estava bem isolado.

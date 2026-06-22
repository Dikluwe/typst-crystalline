# Passo 397 — relatório: `document(...)` e `asset(...)`

**Tipo:** implementação de stdlib + Content variants (L1; zero tipo novo; zero I/O).  
**Data:** 2026-06-22. **HEAD:** pós-`ffaad1df8`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Materializaram-se os elementos user-facing `document(...)` (metadata pura) e `asset(...)` (placeholder de resource, extensão cristalina), fechando a fila limpa do balde D da sonda 389.

- L0:
  - `00_nucleo/prompts/rules/model/document.md` — metadata wrapper `document(...)`.
  - `00_nucleo/prompts/rules/model/asset.md` — resource placeholder `asset(...)`.
- `01_core/src/entities/content.rs`:
  - Novos variants `Content::Document { title, author, date, keywords }` e `Content::Asset { path, kind }`.
  - Construtores `Content::document(...)` e `Content::asset(...)`.
  - Arms de `is_empty`, `plain_text`, `PartialEq`, `map_content` e `map_text` actualizados.
- `01_core/src/rules/stdlib/structural.rs`:
  - `native_document(...)` — aceita `title` (content), `author`/`keywords` (`Str | Array[Str]`), `date` (`Datetime`); rejeita args desconhecidos.
  - `native_asset(...)` — aceita `path` posicional ou named; `kind` explícito ou inferido por extensão.
  - Helper `extract_string_list` e `infer_asset_kind`.
  - 16 testes unitários (document: 9; asset: 5; erros: 2).
- `01_core/src/rules/eval/mod.rs`:
  - Registo em `make_stdlib`: `document` e `asset` como `Func::native(...)`.
- `01_core/src/rules/layout/mod.rs`:
  - `Content::Document { .. }` e `Content::Asset { .. }` são no-op em `layout_content`; medem `(0,0)`.
- `01_core/src/rules/introspect.rs` + `introspect/locatable.rs`:
  - Document/Asset marcados como não-locatables; walk trata-os como terminais.
- `01_core/src/rules/stdlib/mod.rs`:
  - Referências `@prompt` a `rules/model/document.md` para linhagem.
- `03_infra/src/integration_tests.rs`:
  - E2E `document_metadata_nao_emite_frames_no_pdf`.
  - E2E `asset_placeholder_nao_emite_frames_no_pdf`.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:
  - Tabela A.6: `document(...)` e `asset(...)` reclassificados de `ausente` para `implementado`; `title(...)` mantém-se `ausente` (não é função standalone no vanilla).
  - Tabela B.2: adicionados `Document` e `Asset`; count 62 → **64**.
  - Tabela C: entradas `document(...)`/`asset(...)` marcadas como resolvidas.
  - Resumo de contagens actualizado: Model 7/4/7/4/0=22 → 9/4/7/2/0=22; total user-facing 71/27/24/17/2=141 → 73/27/24/15/2=141.
  - Nota de rodapé ⁸⁰ para P397.

`cargo test --workspace` verde; `crystalline-lint .` — `✓ No violations found`.

## Protocolo de Nucleação cumprido

1. L0 (`document.md` + `asset.md`) escritos e hashes propagados via `crystalline-lint --fix-hashes`.
2. TDD: testes unitários + E2E escritos antes/paralelamente à implementação.
3. Nenhum novo `Value` variant — reuso intencional do enum `Content`.
4. Nenhum I/O adicionado; PDF Info dict e resource registry reais continuam scope-out ADR-0054.

## Decisão de engenharia

`document(...)` é um **Content variant** (não `Value` variant), paralelo a `Heading`/`Figure`/`Quote`: pode aparecer no markup, ser manipulado por show rules no futuro, e atravessa o pipeline de layout sem emitir frames. `asset(...)` é uma **extensão cristalina intencional** (ADR-0033): não existe no vanilla como elemento standalone, mas modela explicitamente resources externos para futura registry.

Ambos são **metadata/resources puras**: o Layouter trata-os como no-op, e o exportador PDF actual não precisa de os consumir. O scope-out de PDF Info dict real e de resource registry real mantém-se dentro da disciplina ADR-0054 graded.

`title()` como função stdlib standalone continua **fora de scope**: no vanilla `title` é campo de `document`/`heading`/`figure`, nunca função isolada. Esta decisão está documentada no inventário e no relatório.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `document(title: [T])` | `Content::Document` com `title` Some | ✓ |
| `document(author: "Ana")` | author `vec!["Ana"]` | ✓ |
| `document(author: ("Ana", "Bob"))` | author `vec!["Ana","Bob"]` | ✓ |
| `document(date: datetime(...))` | date Some | ✓ |
| `document(keywords: ("a","b"))` | keywords `vec!["a","b"]` | ✓ |
| `document(foo: "x")` | erro (arg desconhecido) | ✓ |
| `asset("logo.png")` | `Content::Asset` kind `image` | ✓ |
| `asset("font.ttf")` | kind `font` | ✓ |
| `asset("x", kind: "custom")` | kind override `custom` | ✓ |
| `asset(123)` | erro de tipo | ✓ |
| `#document(title: [T])` no PDF | zero frames emitidos | ✓ E2E |
| `#asset("x.png")` no PDF | zero frames emitidos | ✓ E2E |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Content::Document` e `Content::Asset` variants implementados | ✓ |
| 2 | `native_document` e `native_asset` registados em `make_stdlib` | ✓ |
| 3 | `document`/`asset` não emitem frames (layout no-op) | ✓ |
| 4 | Introspection trata Document/Asset como não-locatables | ✓ |
| 5 | Testes unitários + E2E passam | ✓ 16 unit + 2 E2E |
| 6 | `cargo test --workspace` verde; `crystalline-lint .` zero | ✓ |
| 7 | Inventário 148 actualizado | ✓ |
| 8 | L0 salvos e hashados | ✓ `document.md` + `asset.md` |
| 9 | Fila limpa original (balde D, sonda 389) **100% fechada** | ✓ |

## Artefactos

- Código:
  - `01_core/src/entities/content.rs`
  - `01_core/src/rules/stdlib/structural.rs`
  - `01_core/src/rules/stdlib/mod.rs`
  - `01_core/src/rules/eval/mod.rs`
  - `01_core/src/rules/layout/mod.rs`
  - `01_core/src/rules/introspect.rs`
  - `01_core/src/rules/introspect/locatable.rs`
  - `03_infra/src/integration_tests.rs`
- L0:
  - `00_nucleo/prompts/rules/model/document.md`
  - `00_nucleo/prompts/rules/model/asset.md`
- Inventário 148: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- Planos: `00_nucleo/materialization/typst-passo-397.md`.
- este relatório.

## Nota sobre o Tekt

Este passo fecha a **fila limpa original** do balde D da sonda 389: `square` → `lorem` → `panic` → `#show regex` → `eval` → `tiling` → `document`/`asset`. Cada item foi executado de ponta a ponta, testado, lintado e documentado, sem derivação para trabalho não-planeado. Após P397, o projecto transita para a próxima fase: tipos S pendentes (`Bytes`, `Decimal`, `Duration`, `Version`) e features com dependências reais (bibliography CSL, shaping rustybuzz, etc.).

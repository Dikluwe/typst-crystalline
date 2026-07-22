# Relatório — typst-passo-827: `typst_syntax::package` — mensagem de "pacote não encontrado" diverge para não-preview (achado #15 de P810)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-827.md`; caso exacto conferido em `00_nucleo/diagnosticos/typst-passo-810-relatorio.md` §15).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; **working tree não commitado**. Estado na medição "antes" (`git diff HEAD --stat`):

```text
00_nucleo/README-indice-813-827.md         |  27 ---   (removido, pré-existente)
00_nucleo/prompts/engine/stdlib/loading.md |  23 ++-
01_core/src/engine/stdlib/loading.rs       | 285 +++++++++++++++++++++++++++--   (P823/P824)
crystalline.toml                           |   2 +-
```

Ou seja: a working tree já continha as alterações de P823/P824 (`loading.rs`) feitas hoje — nenhuma toca a resolução de pacotes. Estado na medição "depois": os 4 ficheiros acima + `03_infra/src/world.rs` (alteração deste passo). Binário cristalino rebuildado (`cargo build --release`, ~00:45 UTC) antes da medição "depois".
**Binários:** `./target/release/typst` (cristalino — CLI sem subcomando `compile`: `typst <INPUT> [OUTPUT]`), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Sonda (medição ANTES)

Fixtures (conforme o caso exacto de P810 §15):

- `temp/p827/local.typ` = `#import "@local/inexistente:1.0.0"`
- `temp/p827/preview.typ` = `#import "@preview/inexistente:1.0.0"` (controlo de paridade)

**Caso não-preview (`@local`) — ANTES:**

Cristalino (`./target/release/typst temp/p827/local.typ temp/p827/local-crys.pdf`, exit 1):

```text
/home/dikluwe/Documentos/Antigravity/typst-crystalline/temp/p827/local.typ:1:8: error: pacote '@local/inexistente:1.0.0' não encontrado na cache local; download ainda não implementado (ver P-γ de P678)
```

Vanilla (`lab/typst-original/target/release/typst compile temp/p827/local.typ temp/p827/local-van.pdf`, exit 1):

```text
error: package not found (searched for @local/inexistente:1.0.0)
  ┌─ temp/p827/local.typ:1:8
  │
1 │ #import "@local/inexistente:1.0.0"
  │         ^^^^^^^^^^^^^^^^^^^^^^^^^^
```

**Controlo `@preview` — ANTES:** cristalino `... preview.typ:1:8: error: package not found (searched for @preview/inexistente:1.0.0)` (exit 1) — mensagem já em paridade com o vanilla (`error: package not found (searched for @preview/inexistente:1.0.0)` + bloco de span). Confirma o achado de P810: só o caminho não-preview divergia.

**Divergência confirmada:** a mensagem não-preview estava em português e com texto desactualizado ("download ainda não implementado (ver P-γ de P678)" — P763 implementou o downloader), enquanto o vanilla emite `package not found (searched for {spec})` para qualquer namespace.

**Código identificado:**

- **Vanilla:** `lab/typst-original/crates/typst-library/src/diag.rs:719-721` — `PackageError::NotFound(PackageSpec)` formata `write!(f, "package not found (searched for {spec})")`. A mensagem **não varia por namespace**: `NotFound` é o erro de cache-miss devolvido por `PackageStorage` para qualquer namespace (o download só é tentado para `preview`, mas o texto final é o mesmo). A rotina de parsing da spec (`typst-syntax/src/package.rs`) já estava em paridade verbatim (5 casos, P810 §15) e não foi tocada.
- **Cristalino:** `03_infra/src/world.rs:521-524` — ramo final de `SystemWorld::resolve_package` (contrato `01_core/src/contracts/world.rs:76`) emitia a mensagem PT desactualizada. O caminho `@preview` já produzia a mensagem vanilla via `PackageDownloadError::NotFound` em `01_core/src/contracts/package_downloader.rs:62-64`.

## Passo 2 — Implementação

Alteração única em `03_infra/src/world.rs` (ramo final de `resolve_package`):

```diff
-        Err(format!(
-            "pacote '{}' não encontrado na cache local; download ainda não implementado (ver P-γ de P678)",
-            spec
-        ))
+        // P827 — mensagem em paridade com o vanilla (`PackageError::NotFound`,
+        // `typst-library/src/diag.rs`): inglês, sem sufixo PT desactualizado.
+        Err(format!("package not found (searched for {spec})"))
```

A correcção fica em L3 (`03_infra`, crate `typst-infra`), onde a mensagem é gerada — o contrato L1 (`Result<Source, String>`) não muda. Nenhuma impureza introduzida em L1 (a alteração nem toca L1).

**Teste novo** (`#[cfg(test)]` em `03_infra/src/world.rs`, junto dos testes de `SystemWorld`):

- `system_world_resolve_package_nao_preview_mensagem_vanilla` — `SystemWorld` em dir temporário, `PackageSpec::from_str("@local/inexistente:1.0.0")`, aserta `resolve_package(&spec).unwrap_err() == "package not found (searched for @local/inexistente:1.0.0)"`.

## Passo 3 — Validação (medição DEPOIS)

`./target/release/typst temp/p827/local.typ temp/p827/local-crys.pdf` (exit 1):

```text
/home/dikluwe/Documentos/Antigravity/typst-crystalline/temp/p827/local.typ:1:8: error: package not found (searched for @local/inexistente:1.0.0)
```

Texto da mensagem **idêntico ao vanilla** (`package not found (searched for @local/inexistente:1.0.0)`). A moldura de apresentação continua a divergir (cristalino: uma linha `path:1:8: error:`; vanilla: bloco com `┌─` e sublinhado) — divergência de apresentação já registada em P810, fora do âmbito deste passo.

**Controlo `@preview` — DEPOIS:** `... preview.typ:1:8: error: package not found (searched for @preview/inexistente:1.0.0)` (exit 1) — **sem regressão**, paridade mantida.

**Teste novo:** `cargo test -p typst-infra --lib world::tests::system_world_resolve_package_nao_preview_mensagem_vanilla` → `1 passed; 0 failed`.

**Suítes completas (comando + contagem):**

| Suíte | ANTES | DEPOIS |
|---|---|---|
| `cargo test -p typst-core` | `4362 passed; 0 failed; 2 ignored` (+ alvo doc: `0 passed; 3 ignored`) | `4362 passed; 0 failed; 2 ignored` (+ `0 passed; 3 ignored`) — **inalterado** (a correcção é em `typst-infra`, não toca `typst-core`) |
| `cargo test -p typst-infra` | `658 passed; 1 failed; 5 ignored` | `659 passed; 1 failed; 5 ignored` (+1 = teste novo deste passo) |

O `1 failed` em `typst-infra` é **pré-existente e alheio a este passo**: `integration_tests::integration::read_binario_pipeline` falha com `failed to convert to string (file is not valid UTF-8 in logo.png:1:1)` — comportamento de `read()` introduzido pelas alterações P823/P824 já presentes na working tree antes deste passo (medição "antes" acima feita sobre essa mesma working tree; nenhum teste existente asertava a mensagem antiga de pacote). Fica registado para o dono decidir se o pipeline `read_binario` deve passar a consumir bytes (`read(..., encoding: none)`) ou se o fixture muda.

**Lint:** `crystalline-lint .` (binário `~/.cargo/bin/crystalline-lint`; não existe em `./target/release/`) → exit 0, **zero violations**; restam apenas 6 warnings V7 de prompts órfãos pré-existentes (`field-access.md`, `enum_item.md`, `document.md`, `layout.md` (stdlib), `structural.md`, `package_version_resolution.md`), não relacionados com este passo.

## Conclusão

Achado #15 de P810 fechado: a mensagem de "pacote não encontrado" para namespaces não-`@preview` está em paridade textual com o vanilla (`package not found (searched for {spec})`), o controlo `@preview` não regride, e o caso está coberto por teste novo em `03_infra/src/world.rs`.

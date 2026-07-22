# Relatório — typst-passo-823: `loading::cbor_` — mensagem de erro CBOR usa `Debug` interno da crate (achado #10 de P810)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-823.md`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree não commitado. Estado no momento das medições "antes" (`git diff HEAD --stat`): apenas `D 00_nucleo/README-indice-813-827.md` (eliminação pré-existente, externa a este passo). Estado nas medições "depois": os dois ficheiros acima mais `01_core/src/engine/stdlib/loading.rs` (+78/-29 neste passo). Binário cristalino rebuildado às ~00:20 UTC após a alteração.
**Binários:** `./target/release/typst` (cristalino), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Sonda (medição ANTES)

Fixture: `temp/p823/invalid.cbor` = 1 byte `0xFF` (break byte CBOR); `doc.typ` = `#cbor("invalid.cbor")`.

Comando vanilla: `lab/typst-original/target/release/typst compile doc.typ`

```text
error: failed to parse CBOR (invalid type: break, expected non-break in invalid.cbor)
  ┌─ doc.typ:1:6
  │
1 │ #cbor("invalid.cbor")
  │       ^^^^^^^^^^^^^^
exit=1
```

Comando cristalino: `./target/release/typst doc.typ` (o CLI cristalino não tem subcomando `compile`)

```text
/home/dikluwe/.../temp/p823/doc.typ:<detached>: error: cbor inválido: Semantic(None, "invalid type: break, expected non-break")
exit=1
```

Fonte `bytes` (sem ficheiro), `#cbor(bytes((0xff,)))` — só vanilla (o cristalino não tem construtor `bytes()`, colateral já registado em P810):

```text
error: failed to parse CBOR (invalid type: break, expected non-break)
```

Controlo de regressão: CBOR válido (`{"a": 1}`) compila nos dois binários, ANTES e DEPOIS.

**Código identificado:**

- Vanilla: `lab/typst-original/crates/typst-library/src/loading/cbor.rs:88-98` (`format_cbor_error`) — o `Display` do `ciborium::de::Error` delega em `Debug`, por isso o vanilla extrai a razão por variante (`Io` → `IO error: {e}`, `Syntax` → `syntax error`, `Semantic` → a string, `RecursionLimitExceeded` → `recursion limit exceeded`) e embrulha em `LoadError::binary("failed to parse CBOR", razão)`; o sufixo ` in {ficheiro}` é acrescentado por `load_err_in_binary` (`diag.rs:889-925`) quando a fonte é path (fonte bytes fica sem sufixo).
- Cristalino: `01_core/src/engine/stdlib/loading.rs:170` — `format!("cbor inválido: {e}")`, expondo o `Debug` do ciborium (`Semantic(None, "...")`), sem ficheiro, em português.

Nota: o relatório de P810 transcreveu a mensagem vanilla como "failed to parse **parse** CBOR" — a medição literal acima mostra `failed to parse CBOR` (typo de transcrição em P810).

## Passo 2 — Implementação

Em `01_core/src/engine/stdlib/loading.rs` (único ficheiro alterado):

1. **`cbor_error_reason` (nova fn)** — replica `format_cbor_error` do vanilla: extrai a razão por variante do `ciborium::de::Error` em vez de usar o `Display`/`Debug`.
2. **`decode_cbor`** passa a delegar em **`decode_cbor_with_source(bytes, path: Option<&str>)`** (nova fn privada): mensagem `failed to parse CBOR ({razão})`, com sufixo ` in {path}` quando a fonte é um caminho.
3. **`native_cbor` sai da macro `native_loader!`** e passa a função manual: extrai o caminho quando o 1.º posicional é `Str` e passa-o ao decode. Os outros 4 loaders (`json`/`yaml`/`toml`/`xml`) ficam na macro, intocados.

O span continua `<detached>` — débito transversal já registado em P810 ("spans `<detached>` generalizados nas mensagens de erro L1"), fora do âmbito deste passo (que visa o texto da mensagem).

## Passo 3 — Validação (medição DEPOIS)

Comando: `./target/release/typst doc.typ` (mesma fixture da sonda):

```text
/home/dikluwe/.../temp/p823/doc.typ:<detached>: error: failed to parse CBOR (invalid type: break, expected non-break in invalid.cbor)
exit=1
```

Texto da mensagem **byte-idêntico ao vanilla** (`failed to parse CBOR (invalid type: break, expected non-break in invalid.cbor)`); a localização continua `<detached>` (débito transversal registado). CBOR válido continua a compilar (`ok-c`).

**Testes novos** (2, em `loading.rs` `#[cfg(test)]`; confirmados a falhar antes da implementação — `0 passed; 2 failed`):

- `p823_cbor_malformado_mensagem_vanilla` — `decode_cbor(&[0xff])` → `failed to parse CBOR (invalid type: break, expected non-break)`.
- `p823_native_cbor_path_acrescenta_ficheiro` — `native_cbor` com path → mesma mensagem + ` in invalid.cbor`.

**Suíte `typst-core`** (`cargo test -p typst-core`):

| | ANTES | DEPOIS |
|---|---|---|
| lib | 4356 passed; 0 failed; 2 ignored | **4358 passed; 0 failed; 2 ignored** |
| doc-tests | 0 passed; 3 ignored | 0 passed; 3 ignored |

4358 = 4356 + 2 testes novos ✓.

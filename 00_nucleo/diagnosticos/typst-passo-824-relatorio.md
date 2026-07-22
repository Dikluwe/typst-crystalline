# Relatório — typst-passo-824: `loading::read_` — `encoding:` rejeitado, não-UTF8 devolve bytes em silêncio (achado #11 de P810)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-824.md`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree não commitado. Estado nas medições "antes" (`git diff HEAD --stat`): `D 00_nucleo/README-indice-813-827.md` (pré-existente, externo) + as alterações de P823 em `01_core/src/engine/stdlib/loading.rs` (não tocam `native_read`). Estado nas medições "depois": os 4 ficheiros acima (`loading.rs` +285/-50 acumulado P823+P824, `loading.md` L0, `crystalline.toml`). Binário cristalino rebuildado às ~00:33 UTC após a alteração de P824.
**Binários:** `./target/release/typst` (cristalino), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Sonda (medição ANTES)

Fixtures em `temp/p824/`: `texto.txt` (UTF-8 válido), `latin1.txt` = `ol\xE1 mundo\n` (não-UTF8), `multi.txt` = `ab\ncd\xE1` (não-UTF8 após newline).

**Vanilla** (`lab/typst-original/target/release/typst compile <doc>.typ`):

| Documento | Saída literal |
|---|---|
| `#read("texto.txt", encoding: "utf8")` | exit 0 (compila) |
| `#read("texto.txt", encoding: none)` | exit 0 (compila) |
| `#read("texto.txt", encoding: "latin1")` | `error: expected "utf8" or none` (exit 1) |
| `#read("texto.txt", encoding: 5)` | `error: expected "utf8" or none, found integer` (exit 1) |
| `#read("texto.txt", encoding: true)` | `error: expected "utf8" or none, found boolean` (exit 1) |
| `#read("latin1.txt")` | `error: failed to convert to string (file is not valid UTF-8 in latin1.txt:1:1)` (exit 1) |
| `#read("multi.txt")` | `error: failed to convert to string (file is not valid UTF-8 in multi.txt:2:3)` (exit 1) |

**Lista de encodings aceites pelo vanilla (medida): apenas `"utf8"` ou `none`** — o enum `Encoding` do vanilla (`read.rs:42-47`) só tem a variante `Utf8`; `none` devolve bytes crus.

**Cristalino** (`./target/release/typst <doc>.typ`), ANTES:

```text
enc-utf8.typ:  <detached>: error: argumento nomeado inesperado em read(): 'encoding'   (exit 1)
enc-none.typ:  idem                                                                          (exit 1)
enc-latin1.typ: idem                                                                         (exit 1)
enc-int.typ:   idem                                                                          (exit 1)
noutf8.typ:    (sem qualquer saída — compila em silêncio, exit 0)
```

**Refutação reconfirmada:** o comentário no código cristalino (`loading.rs:430-431`, "Heurística vanilla: tenta UTF-8; se falhar, retorna bytes opacos") **não corresponde ao vanilla medido**: o vanilla **erra** (`file is not valid UTF-8`) em ficheiro não-UTF8 sem `encoding:`, enquanto o cristalino devolvia bytes em silêncio (exit 0 acima). A "heurística" não existe no vanilla.

**Código identificado:**

- Vanilla: `lab/typst-original/crates/typst-library/src/loading/read.rs:24-47` (named `encoding: Option<Encoding>`, default `Some(Utf8)`; `None` → `Readable::Bytes`, `Some(Utf8)` → `to_str()`), `diag.rs:797-807` (`Utf8Error` → `LoadError::text(range, "failed to convert to string", "file is not valid UTF-8")`) e `diag.rs:889-925,991-1039` (sufixo ` in {ficheiro}:{linha}:{col}` via `LineCol::try_from_byte_pos` — quirk confirmado por medição: sem `'\n'` antes do byte, a coluna reportada é 1).
- Cristalino: `01_core/src/engine/stdlib/loading.rs:438` (`reject_named` como 1.ª instrução) e `:441-444` (fallback silencioso para `Bytes`), com o comentário refutado em `:430-431`. O L0 `loading.md` §4 documentava a mesma "heurística" — também corrigido.

## Passo 2 — Implementação

Em `01_core/src/engine/stdlib/loading.rs`:

1. **`native_read` reescrito**: aceita apenas o named `encoding:` (outros named → rejeição como antes); `None`/ausente ou `Str("utf8")` → `read_utf8`; `Value::None` → `Bytes`; outra string → `expected "utf8" or none`; outro tipo → `expected "utf8" or none, found {tipo}` (via `vanilla_type_name`).
2. **`read_utf8` (nova fn)**: UTF-8 → `Str`; inválido → `failed to convert to string (file is not valid UTF-8 in {path}:{linha}:{col})`, com posição calculada por **`vanilla_line_col` (nova fn)** — réplica exacta de `LineCol::try_from_byte_pos` + `numbers()` (linha = nº de `'\n'` + 1; coluna = chars desde o último `'\n'` + 1, com o quirk do `unwrap_or(bytes.len())` verificado contra o vanilla nos casos `1:1` e `2:3`).
3. **Comentário refutado removido** (substituído pela referência à medição) — no código e no L0 `loading.md` §4.
4. **`vanilla_type_name`**: acrescentado `bool`→`boolean` (medido: `encoding: true` → `found boolean`).

Colateral: `crystalline.toml` — `[l1_allowed_external.ciborium]` ganhou `de::Error` (V14; formatação do erro CBOR de P823). L0 `loading.md` §4/tabela/§testes/histórico actualizados; hash do prompt regenerado (`crystalline-lint --fix-hashes`, novo hash `9bcf12e4`). `crystalline-lint .` → **zero violations** (restam apenas warnings V7 de prompts órfãos pré-existentes, não relacionados).

## Passo 3 — Validação (medição DEPOIS)

`./target/release/typst <doc>.typ` (mesmas fixtures):

```text
enc-utf8.typ:   exit 0 ✓ (vanilla: exit 0)
enc-none.typ:   exit 0 ✓ (vanilla: exit 0)
enc-latin1.typ: <detached>: error: expected "utf8" or none ✓
enc-int.typ:    <detached>: error: expected "utf8" or none, found integer ✓
enc-bool.typ:   <detached>: error: expected "utf8" or none, found boolean ✓
noutf8.typ:     <detached>: error: failed to convert to string (file is not valid UTF-8 in latin1.txt:1:1) ✓
multi.typ:      <detached>: error: failed to convert to string (file is not valid UTF-8 in multi.txt:2:3) ✓
```

Todos os textos **byte-idênticos ao vanilla**; a localização continua `<detached>` (débito transversal registado em P810, fora do âmbito). Regressão: `#read("texto.txt")` simples (UTF-8, sem `encoding:`) compila nos dois binários.

**Testes** (`#[cfg(test)]` em `loading.rs`; confirmados a falhar antes da implementação — `28 passed; 5 failed`):

- `p824_read_encoding_utf8_aceite` (novo) — `encoding: "utf8"` aceite, equivale ao default.
- `p824_read_encoding_none_devolve_bytes` (novo) — `encoding: none` → `Bytes`.
- `p824_read_encoding_fora_do_dominio_erro_vanilla` (novo) — `"latin1"` → `expected "utf8" or none`; `5` → `..., found integer`; `true` → `..., found boolean`.
- `p824_read_nao_utf8_linha_col_vanilla` (novo) — erro UTF-8 com posição `1:1` e `2:3`, e o mesmo erro com `encoding: "utf8"` explícito.
- `read_binario_nao_utf8` (**modificado**) — codificava o comportamento refutado (fallback silencioso para `Bytes`); passa a verificar o erro `failed to convert to string (... logo.png:1:1)`.

**Suíte `typst-core`** (`cargo test -p typst-core`):

| | ANTES (pós-P823) | DEPOIS |
|---|---|---|
| lib | 4358 passed; 0 failed; 2 ignored | **4362 passed; 0 failed; 2 ignored** |
| doc-tests | 0 passed; 3 ignored | 0 passed; 3 ignored |

4362 = 4358 + 4 testes novos ✓ (`read_binario_nao_utf8` modificado não altera a contagem).

---

## Adenda (2026-07-22, follow-up) — teste de integração em `typst-infra`

A mudança de comportamento de `read()` deixou a falhar o teste de integração `integration_tests::read_binario_pipeline` (`03_infra/src/integration_tests.rs:1616`), que codificava o comportamento antigo refutado (bytes em silêncio). Actualizado à semelhança de `read_binario_nao_utf8` (typst-core): passa a verificar o erro `failed to convert to string (file is not valid UTF-8 in logo.png:1:1)`; acrescentado `read_binario_encoding_none_pipeline` (novo) a cobrir `read("logo.png", encoding: none)` → `Bytes` no pipeline completo (world real com ficheiro em disco temporário).

**Suíte `typst-infra`** (`cargo test -p typst-infra`): ANTES `659 passed; 1 failed; 5 ignored` → DEPOIS **`661 passed; 0 failed; 5 ignored`** (659 + 1 modificado + 1 novo ✓). **`typst-core` mantém-se verde**: `4362 passed; 0 failed; 2 ignored` (inalterado — a alteração é só em `03_infra`). Proveniência: mesmo HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`, working tree não commitado.

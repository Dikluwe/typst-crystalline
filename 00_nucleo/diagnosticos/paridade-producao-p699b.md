# Paridade Produção — P699b — Confirmação com documento `.typ` real

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-699b.md`
**Hash do commit (implementação):** ver §4b (preenchido no commit seguinte, prática já usada em P695–P699).
**Estado:** FECHADO — bug encontrado, corrigido, validado ponta a ponta; lint 0, workspace verde.

---

## 1. Proveniência das medições

- **HEAD base:** `93d727586` (fim de P699, detached HEAD).
- **Hora da medição:** sessão de 2026-07-11 (ver histórico da conversa; `date -u`
  não corrido por não ser necessário distinguir sub-momentos nesta verificação).
- Rebuild necessário: `target/release/typst` estava desatualizado (10/jul, anterior
  ao commit de P699 de 11/jul) — recompilado com `cargo build --release --workspace`
  antes de qualquer teste.
- `hello.wasm` reconstruído com o encoder Python do próprio passo (192 bytes).

---

## 2. Resultado — documento real expõe bug que os testes unitários não apanharam

### 2.1 Reprodução

```
#let p = plugin("hello.wasm")
#str(p.hello())
```

- **Vanilla** (`lab/typst-original/target/release/typst compile`): exit 0,
  `pdftotext` → `hello`.
- **Cristalino** (`target/release/typst`): exit 1 —
  `/tmp/p699b-plugin.typ:<detached>: error: str() não suporta bytes`.

### 2.2 Isolamento da causa

Testado sem `str()`, só com `type(...)`, para separar "despacho de plugin via
sintaxe real" de "conversão str()":

```
#let p = plugin("hello.wasm")
#type(p.hello())
```
→ exit 0, output `bytes`.

```
#import plugin("hello.wasm"): hello
#type(hello())
```
→ exit 0, output `bytes`.

**Conclusão:** o despacho `plugin()` → `Module` → `PluginFunc` → chamada (P699,
níveis 4–5 de P696) está **correto** via sintaxe real — parser, field access e
`FuncRepr::Plugin` funcionam. O único bug é que `native_str`
(`01_core/src/rules/stdlib/foundations.rs:319-356`) não tem braço para
`Value::Bytes`, caindo no braço genérico `other => err("str() não suporta
{type}")`.

### 2.3 Confirmação de paridade vanilla

`lab/typst-original/crates/typst-library/src/foundations/str.rs:871`:
```rust
v: Bytes => Self::Str(v.to_str().map_err(|_| "bytes are not valid UTF-8")?),
```
Doc do construtor (`str.rs:139`): "Bytes are decoded as UTF-8." Confirmado que
o vanilla suporta `str(bytes)` como decodificação UTF-8 — não é feature nova,
é paridade que faltava.

---

## 3. Causa raiz e porque os testes unitários de P699 não apanharam isto

Os testes de P699 (`plugin_devolve_module_com_funcao_por_export`,
`call_plugin_devolve_bytes`, etc.) constroem e inspecionam o `Value::Bytes`
directamente em Rust, ou comparam com `Value::Bytes(...)` — nunca passam o
resultado por `native_str`. O documento `.typ` real da validação de P699 no
passo original (`str(p.hello())`) nunca foi de facto corrido (P699 §9
scope-out só menciona o teste E2E de PDF em `04_wiring/tests/`, não esta
verificação leve). Precedente idêntico: P679 (gap só visível compilando
documento real).

---

## 4. Ação tomada — L0 confirmado pelo humano, código corrigido

- **L0 actualizado**: `00_nucleo/prompts/rules/stdlib/foundations.md`,
  secção `native_str` — adicionado braço `Bytes` (decodifica UTF-8, erro
  `"bytes are not valid UTF-8"` verbatim do vanilla) e dois testes canónicos.
  Scope-out clarificado: `Bytes` não é tipo complexo, é suportado. **Confirmado
  pelo humano** antes do passo seguinte (Protocolo de Nucleação, CLAUDE.md).
- **Código**: `01_core/src/rules/stdlib/foundations.rs::native_str` — novo
  braço `Value::Bytes(b) => match std::str::from_utf8(b.as_slice()) { Ok(s) =>
  s.to_string(), Err(_) => return err("bytes are not valid UTF-8") }`,
  inserido antes do braço `Color`/genérico.
- **Testes** (`foundations.rs`, `mod tests_p699b_str_bytes`, novo):
  `str_de_bytes_valido_utf8_decodifica` (`Bytes(b"hello")` → `Str("hello")`),
  `str_de_bytes_invalido_utf8_erro_verbatim` (`Bytes([0xFF, 0xFE])` → erro
  contendo `"bytes are not valid UTF-8"`).
- `crystalline-lint --fix-hashes .` — realinhou `@prompt-hash` de
  `foundations.rs` (`1771dfcd` → `32063f1f`, hash do L0 pós-edição).

## 4b. Validação final (proveniência — regra P569/ADR-0108)

- **Re-executada a reprodução do §2.1** após o fix, com `target/release/typst`
  recompilado (`cargo build --release --workspace`):
  - `#let p = plugin("hello.wasm"); #str(p.hello())` → exit 0, `pdftotext` →
    `hello` (igual ao vanilla).
  - `#import plugin("hello.wasm"): hello; #str(hello())` → exit 0, `pdftotext`
    → `hello`.
- `cargo test --workspace`: **3728 passed**, 0 failed (3726 de P699 + 2 novos
  de P699b); `typst-infra` 626 passed / 5 ignored — inalterado.
- `crystalline-lint .` → `✓ No violations found`.
- **Hash do commit:** a preencher no commit seguinte (`git rev-parse --short
  HEAD`), seguindo a prática já usada em P695–P699.

---

## 5. Critério de fecho do passo — cumprido

- [x] Documento real testado — problema encontrado, **corrigido e revalidado**.
- [x] `#import plugin(...): item` testado com sintaxe real — despacho
      confirmado correto (isolado do bug de `str()`); revalidado com o fix.
- [x] Correção aplicada e validada — L0 confirmado pelo humano antes do código
      (Trava Arquitetural cumprida).
- [x] Relatório com resultado exacto (este ficheiro).
- [x] `cargo test --workspace` sem regressão; `crystalline-lint .` zero
      violations.

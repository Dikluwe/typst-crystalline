# Paridade de Produção — P772d: Verificação do `<detached>` de P772c

**Data:** 2026-07-16T12:47:58-03:00  
**Commit base:** `9ceed124d12380d25fb1745dd935d531dd5c7b67`  
**Estado da working tree:** existem modificações não commitadas além deste passo (ver `git diff HEAD --stat` no final).

---

## 1. Objectivo

P772c encontrou `<detached>` no path de erro para:

```typst
#import "preview/nome:1.0.0"
```

(sem `@`, tratado como path relativo; erro de I/O "file not found").  
P772b corrigiu `<detached>` para spans cross-file válidos. P772d verifica se são a mesma causa ou causas distintas.

---

## 2. Reprodução (pós-P772b)

```bash
cat > /tmp/p772d-test.typ <<'EOF'
#import "preview/nome:1.0.0"
EOF
lab/typst-original/target/release/typst compile /tmp/p772d-test.typ 2>&1 | head -5
./target/release/typst /tmp/p772d-test.typ 2>&1 | head -5
```

### Antes da correcção de P772d

**Vanilla:**

```text
error: file not found (searched at /tmp/preview/nome:1.0.0)
  ┌─ ../../../../../tmp/p772d-test.typ:1:8
  │
1 │ #import "preview/nome:1.0.0"
  │         ^^^^^^^^^^^^^^^^^^^^
```

**Cristalino:**

```text
/tmp/p772d-test.typ:<detached>: error: include: ficheiro não encontrado: /tmp/preview/nome:1.0.0
```

---

## 3. Causa

O problema era **distinto** do de P772b:

- P772b: span válido apontando para outro `FileId`, mas o formatter recebia a source principal.
- P772d: o erro de I/O era criado em `01_core/src/rules/eval/modules.rs` com `Span::detached()` em vez de usar o span da string do caminho no documento principal.

Locais afectados:

- `eval_module_import` (linha ~141): `resolve_package` error usava `Span::detached()`.
- `eval_module_import` (linha ~146): `include_source` error usava `Span::detached()`.
- `eval_module_include` (linha ~249): `include_source` error usava `Span::detached()`.

---

## 4. Correcção

Alterámos os três locais para usarem o span da expressão do caminho:

- `eval_module_import`: `source_span` (já existia).
- `eval_module_include`: novo `path_span` obtido de `include.source().span()`.

Também aproveitámos para usar `path_span` no erro de tipo do caminho em `eval_module_include`.

### Depois da correcção

**Cristalino:**

```text
/tmp/p772d-test.typ:1:9: error: include: ficheiro não encontrado: /tmp/preview/nome:1.0.0
```

Já não mostra `<detached>` e aponta para a linha/coluna do path no documento principal.

---

## 5. Teste

Adicionado em `04_wiring/tests/cli.rs`:

```rust
#[test]
fn p772d_io_import_path_inexistente_nao_detached() { ... }
```

Verifica que:
- exit code é 1;
- stderr contém `error:` e `ficheiro não encontrado`;
- stderr **não** contém `<detached>`;
- stderr contém `:1:` (linha do path).

Resultado: **passou**.

---

## 6. Validação

```bash
cargo test --workspace
crystalline-lint .
```

- `cargo test --workspace`: todos os testes passaram.
- `crystalline-lint .`: zero violações (apenas warning pré-existente V7 sobre `package_version_resolution.md` órfão).

---

## 7. Estado do `git diff HEAD --stat`

```text
 00_nucleo/0.15.0.typ                     | 463 -------------------------------
 00_nucleo/testing/fontes-padrao-teste.md |  74 -----
 01_core/src/rules/eval/modules.rs        |  12 +-
 04_wiring/tests/cli.rs                   |  38 +++
 4 files changed, 45 insertions(+), 542 deletions(-)
```

Nota: os ficheiros `00_nucleo/0.15.0.typ` e `00_nucleo/testing/fontes-padrao-teste.md` já estavam modificados na working tree antes deste passo; não foram alterados por P772d.

---

## 8. Critério de fecho

- [x] Caso reproduzido com o código actual (pós-P772b).
- [x] Confirmado que era causa distinta (span detached em erro de I/O, não resolução cross-file).
- [x] Correcção pequena e isolada aplicada em `modules.rs`.
- [x] Teste de integração adicionado.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772d.md`.

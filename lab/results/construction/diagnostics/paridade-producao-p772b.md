# Paridade de Produção — P772b: `typst_syntax::span`

**Data:** 2026-07-16T12:35:57-03:00  
**Commit base:** `b45f745d75f0203406bde1c5c91f459382ac49b2`  
**Estado da working tree:** existem modificações não commitadas além deste passo (ver `git diff HEAD --stat` no final).  
**Prompt L0 afetado:**
- `00_nucleo/prompts/shell/diagnostic.md` → hash `51b5d920`
- `00_nucleo/prompts/infra/system-world.md` → hash `abf5d4a5`
- `00_nucleo/prompts/wiring.md` → hash `9faaacf1`

---

## 1. Lista de itens classificados

Fonte da lacuna:

```bash
awk -F'\t' '$1=="lacuna-inventario" && $5 ~ /typst_syntax::span/' \
  00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt | cut -f5 | sort
```

| # | Item | Classificação ADR-0107 | Justificação |
|---|------|------------------------|--------------|
| 1 | `typst_syntax::span::DiagSpan` | **mecânica / diverge** | Tipo auxiliar do vanilla para agrupar spans de diagnósticos com labels/secundários. O cristalino usa `SourceDiagnostic` com um único `Span`. Não altera a semântica da linguagem. |
| 2 | `typst_syntax::span::DiagSpanKind` | **mecânica / diverge** | Enum auxiliar de tipos de span secundário (primary/secondary). Mecânica de apresentação de diagnósticos. |
| 3 | `typst_syntax::span::Mapping` | **mecânica / diverge** | Helper interno para mapear spans quando conteúdo é expandido (ex. `#include`). Mecânica de implementação. |
| 4 | `typst_syntax::span::RangeMapper` | **mecânica / diverge** | Estrutura interna do vanilla para converter entre offsets expandidos e originais. Não é observável na língua. |
| 5 | `typst_syntax::span::saturate` | **mecânica / diverge** | Função interna de saturação de offsets. O cristalino já satura em `Span::from_range`. |
| 6 | `typst_syntax::span::SpanDetached` | **mecânica / diverge** | Marker type do vanilla. O cristalino usa `Span::detached()` como valor único. |
| 7 | `typst_syntax::span::SpanKind` | **mecânica / diverge** | Distingue numbered vs raw range no vanilla. O cristalino codifica isso nos bits do `Span`. |
| 8 | `typst_syntax::span::SpanNumber` | **mecânica / diverge** | Newtype interno do vanilla. O cristalino usa `u64` directo. |
| 9 | `typst_syntax::span::SubRange` | **mecânica / diverge** | Helper interno para sub-intervalos de spans. |
| 10 | `typst_syntax::span::to_u32_saturated` | **mecânica / diverge** | Conversão interna de offsets para `u32`. |

**Conclusão da classificação:** nenhum dos 10 itens é semântica/sintaxe/morfologia da linguagem Typst. São todos mecanismos internos do vanilla para diagnósticos ricos e mapeamento de spans. O `Span` do cristalino já cobre o essencial: identificador de ficheiro (16 bits) + número de 48 bits ou raw range de 23+23 bits.

---

## 2. Bug real encontrado: spans cross-file reportados como `<detached>`

### 2.1 Reprodução

Criámos um documento com um ficheiro importado contendo um erro:

```bash
mkdir -p /tmp/p772b-doc/subdir
cat > /tmp/p772b-doc/subdir/lib.typ <<'EOF'
#let broken(x) = x + y
EOF
cat > /tmp/p772b-doc/main.typ <<'EOF'
#import "subdir/lib.typ": broken
#broken(1)
EOF
```

**Vanilla:**

```text
error: unknown variable: y
  ┌─ ../../../../../tmp/p772b-doc/subdir/lib.typ:1:21
  │
1 │ #let broken(x) = x + y
  │                      ^

  while calling `broken` at ../../../../../tmp/p772b-doc/main.typ:2:1
    broken(1)
```

**Cristalino (antes da correção):**

```text
/tmp/p772b-doc/main.typ:<detached>: error: unknown variable: y
```

### 2.2 Causa

O formatter em `02_shell/src/diagnostic.rs` recebia sempre a `Source` do documento principal e chamava `source.span_to_line_col(diag.span)`. Quando o span apontava para outro `FileId` (o ficheiro importado), `span_to_line_col` devolvia `None` e o formatter caía em `<detached>`.

A responsabilidade de resolver o `Source` correcto por `FileId` estava no caller (L4), mas `04_wiring/src/main.rs::drain_to_stderr` usava a source principal para todos os diagnostics.

### 2.3 Correção

1. `03_infra/src/world.rs`: `path_of` tornou-se `pub` para L4 poder obter o path do ficheiro alvo.
2. `04_wiring/src/main.rs`: `drain_to_stderr` passou a receber `&SystemWorld` e resolve, para cada `diag.span`, o `Source` e o path correctos via `world.source(id)` e `world.path_of(id)`.
3. `02_shell/src/diagnostic.rs`: manteve a API inalterada; o L0 foi actualizado para deixar claro que a resolução cross-file é responsabilidade do caller.

**Cristalino (depois da correção):**

```text
/tmp/p772b-doc/subdir/lib.typ:1:21: error: unknown variable: y
```

Isto já não mostra `<detached>` e aponta para o ficheiro/linha/coluna correctos. A stack trace (`while calling...`) continua não implementada — isso é um scope-out separado.

### 2.4 Teste

Adicionado em `04_wiring/tests/cli.rs`:

```rust
#[test]
fn p772b_span_cross_file_aponta_para_ficheiro_importado() { ... }
```

Verifica que:
- exit code é 1;
- stderr contém `error:` e `unknown variable: y`;
- stderr contém `lib.typ`;
- stderr **não** contém `<detached>`;
- stderr contém `:1:` (linha 1).

Resultado: **passou**.

---

## 3. Validação

```bash
cargo test --workspace
crystalline-lint .
```

- `cargo test --workspace`: todos os testes passaram (incluindo o novo teste P772b).
- `crystalline-lint .`: zero violações (apenas warning pré-existente V7 sobre `package_version_resolution.md` órfão).

---

## 4. Estado do `git diff HEAD --stat`

```textn 00_nucleo/0.15.0.typ                     | 463 -------------------------------
 00_nucleo/prompts/infra/system-world.md  |   6 +-
 00_nucleo/prompts/shell/diagnostic.md    |   5 +-
 00_nucleo/prompts/wiring.md              |  14 +-
 00_nucleo/testing/fontes-padrao-teste.md |  74 -----
 02_shell/src/diagnostic.rs               |   2 +-
 03_infra/src/world.rs                    |   8 +-
 04_wiring/src/main.rs                    |  29 +-
 04_wiring/tests/cli.rs                   |  57 +++-
 04_wiring/tests/crystalline_lint.rs      |   2 +-
 10 files changed, 99 insertions(+), 561 deletions(-)
```

Nota: os ficheiros `00_nucleo/0.15.0.typ` e `00_nucleo/testing/fontes-padrao-teste.md` já estavam modificados na working tree antes deste passo; não foram alterados por P772b.

---

## 5. Critério de fecho

- [x] Os 10 itens de `typst_syntax::span` classificados item a item.
- [x] Verificado especificamente se mensagens de erro dentro de pacotes/ficheiros importados apontam para o local correcto.
- [x] Bug real corrigido com teste de mensagem exacta.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772b.md`.

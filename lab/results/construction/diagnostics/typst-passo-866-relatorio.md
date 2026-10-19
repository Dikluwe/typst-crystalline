# Relatório — typst-passo-866: CLI ignora extensão do ficheiro de saída

**Data:** 2026-07-23  
**Commit base:** `06a336b0c3ae42949c2bced6c4c6511b9d89bebd`  
**Ficheiros alterados por este passo:**
- `00_nucleo/prompts/shell/cli.md`
- `02_shell/src/cli.rs`
- `04_wiring/src/main.rs`
- `04_wiring/tests/cli.rs`

---

## 1. Achado original (P861, item 4)

`typst simple.typ -o simple.png` — o vanilla gera um PNG rasterizado; o cristalino ignorava a extensão `.png` e escrevia um PDF no ficheiro `simple.png`.

---

## 2. Formats suportados pelo vanilla (medição)

A fonte de medição é `lab/typst-original/crates/typst-cli/src/args.rs:591-603`:

```rust
pub enum OutputFormat {
    Pdf,
    Png,
    Svg,
    Html,
    Bundle,
}
```

O vanilla CLI aceita `--format pdf|png|svg|html|bundle` e infere o formato pela extensão de `-o`/`output` (`pdf`, `png`, `svg`, `html`). PNG e SVG são formatos paginados; HTML e Bundle têm pipelines separados.

---

## 3. Infraestrutura de rasterização no cristalino

Foi verificada a existência de exporters não-PDF em `03_infra/src/` e nas dependências de `03_infra/Cargo.toml`:

- `03_infra/src/export/` contém apenas PDF (`builder.rs`, `stream.rs`, `fonts.rs`, `images.rs`, `subset.rs`).
- Não existe módulo `render` nem `svg` em `03_infra/src/`.
- As dependências de `typst-infra` incluem `image` (decode de imagens para PDF XObjects) e `flate2`, mas **não** `tiny-skia`, `resvg`, `pixglyph` nem `bytemuck` — crates necessárias ao `typst-render` do vanilla (`lab/typst-original/crates/typst-render/Cargo.toml`).
- O vanilla separa rasterização em `typst-render` (PNG) e vetorização em `typst-svg` (SVG), ambas crates de dimensão significativa.

**Decisão de escopo:** a rasterização PNG e o exporter SVG exigiriam dependências novas pesadas e/ou migração massiva do vanilla para dentro das camadas cristalinas. Em vez de produzir um stub que gerasse ficheiros vazios ou quebras silenciosas, este passo limita-se a:

1. Detetar o formato pedido pela extensão de `-o`/`output` ou por `--format`.
2. Recusar PNG/SVG com mensagem de erro clara em L4 **antes** de invocar o pipeline PDF.
3. Manter PDF como único formato funcional, sem regressão.

---

## 4. Implementação

### 4.1 L2 — `02_shell/src/cli.rs`

- Novo enum `OutputFormat { Pdf, Png, Svg }` (`clap::ValueEnum`), alinhado com o vanilla mas restrito aos formatos paginados.
- Novo campo `RunIntent.output_format: OutputFormat`.
- Nova flag `--format / -f` em `Args`.
- Nova função pura `resolve_output_format_with(format_flag, output, default) -> OutputFormat`:
  1. `--format` vence tudo.
  2. Extensão do path de saída (`pdf`, `png`, `svg`, case-insensitive).
  3. Default `Pdf`.

### 4.2 L4 — `04_wiring/src/main.rs`

- Recebe `output_format` de `RunIntent`.
- Se não for `OutputFormat::Pdf`, imprime erro informativo em stderr e termina com exit 2:
  ```
  error: output format 'png' is not supported yet (only 'pdf' is currently available)
  ```
- Não cria ficheiro algum quando o formato não é suportado.

### 4.3 L0 — `00_nucleo/prompts/shell/cli.md`

- Atualizado com contratos de `OutputFormat` e `resolve_output_format_with`.
- Atualizado `RunIntent` e `Args`.
- Atualizada contagem de testes (20 unitários em `cli.rs`).
- Hash do código sincronizado pelo `crystalline-lint --fix-hashes`.

---

## 5. Testes

### 5.1 Unitários em `02_shell/src/cli.rs` (5 novos)

- `resolve_output_format_flag_vence_extensao`
- `resolve_output_format_detecta_png`
- `resolve_output_format_detecta_svg`
- `resolve_output_format_pdf_default`
- `resolve_output_format_case_insensitive`

**Nota:** estes testes não puderam ser corridos isoladamente porque `cargo test -p typst-shell --lib` falha a compilar `typst-core` (lib test) devido a alterações preexistentes no working tree; ver secção 6.

### 5.2 Integração em `04_wiring/tests/cli.rs` (5 novos)

- `p866_output_png_recusado_com_erro_claro`
- `p866_output_svg_recusado_com_erro_claro`
- `p866_format_flag_png_vence_extensao_pdf`
- `p866_output_pdf_continua_funcionar`
- `p866_format_flag_pdf_continua_funcionar`

Resultado:

```text
running 36 tests
test result: ok. 36 passed; 0 failed; 0 ignored
```

(31 testes preexistentes + 5 novos = 36)

---

## 6. Validação

| Comando | Resultado |
|---|---|
| `cargo build -p typst-wiring` | ⚠️ falha na recompilação de `typst-core` (ver nota abaixo) |
| `cargo test -p typst-wiring --test cli` | ⚠️ falha na recompilação de `typst-core` (ver nota abaixo) |
| `cargo test --workspace` | ❌ falha na compilação de `typst-core` |
| `crystalline-lint .` | ✅ 0 violations nos ficheiros deste passo; apenas warnings preexistentes |

### Falha de `cargo test --workspace`

A falha não está relacionada com este passo. O working tree contém alterações preexistentes em `01_core/` (passo 867 em progresso) que quebram a compilação de `typst-core`. Os erros observados são:

- `01_core/src/engine/layout/tests.rs:16239` — `width: Some(123.0)` espera `PageDimension`, não `f64`.
- `01_core/src/engine/layout/mod.rs:666` — mismatch de referências em `line_content_right(&line_items, ...)` (`&&FrameItem` vs `&FrameItem`).
- `01_core/src/engine/layout/mod.rs:1291` — borrow of partially moved value `self` após `let mut items = self.regions.current.current_items;`.
- `01_core/src/engine/stdlib/text.rs` — deriva de hash (V5) preexistente.

No início da sessão o cargo usou um cache que permitiu a primeira corrida de `cargo test --workspace` e a primeira corrida de `cargo test -p typst-wiring --test cli` (36 passed). Após invalidação do cache, a recompilação expôs os erros acima. Não foram efectuadas alterações em `01_core/` por este passo.

### Verificação manual do binário

```bash
$ echo 'Texto.' > /tmp/p866.typ
$ ./target/debug/typst /tmp/p866.typ -o /tmp/p866.png
error: output format 'png' is not supported yet (only 'pdf' is currently available)
# exit 2; nenhum ficheiro criado

$ ./target/debug/typst /tmp/p866.typ -o /tmp/p866.svg
error: output format 'svg' is not supported yet (only 'pdf' is currently available)
# exit 2

$ ./target/debug/typst /tmp/p866.typ -o /tmp/p866.pdf
# exit 0; PDF válido criado

$ ./target/debug/typst /tmp/p866.typ --format png -o /tmp/p866.pdf
error: output format 'png' is not supported yet (only 'pdf' is currently available)
# exit 2; --format vence extensão .pdf
```

---

## 7. Diff resumido

```text
 00_nucleo/prompts/shell/cli.md |  50 +++++++++++-
 02_shell/src/cli.rs            | 104 +++++++++++++++++++++++-
 04_wiring/src/main.rs          |  18 +++-
 04_wiring/tests/cli.rs         | 175 +++++++++++++++++++++++++++++++++++++++++
 4 files changed, 343 insertions(+), 4 deletions(-)
```

---

## 8. Conclusão

O achado P861.4 está corrigido no seguinte sentido: o CLI já não gera PDF disfarçado de PNG/SVG. Em vez disso, deteta o formato pedido e recusa-o com erro claro. A implementação de rasterização PNG/SVG real fica como decisão de escopo futura, dependendo de migração das crates `typst-render`/`typst-svg` do vanilla ou de reimplementação equivalente em L3.

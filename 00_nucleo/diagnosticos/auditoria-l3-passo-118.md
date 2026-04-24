# Passo 118.A — Auditoria de L3

**Data**: 2026-04-23
**Objectivo**: classificar cada `pub fn|pub struct` em `03_infra/src/`
como L3 correcto, Candidato L2, Candidato L4, ou Fronteiriço.

---

## Estrutura

`03_infra/src/` contém 8 módulos:
- `diagnostic_format.rs`
- `export.rs`
- `font_metrics.rs`
- `fonts.rs`
- `image_sizer.rs`
- `layout.rs`
- `pipeline.rs`
- `world.rs`

Plus `lib.rs` (re-exports) e `integration_tests.rs` (`#[cfg(test)]`).

---

## Classificação por função pública

| Ficheiro | Função / Tipo | Classificação | Razão |
|----------|---------------|---------------|-------|
| **`diagnostic_format.rs`** | `format_diagnostic(diag, source, path, colored)` | **Candidato L2** | Produz string user-facing com palavras literais ("warning:", "error:", "hint:"), escapes ANSI, indentação. Decisão puramente de apresentação. Já identificado como candidato óbvio no Passo 117 (correcção parcial). |
| `diagnostic_format.rs` | `drain_diagnostics_to_stderr(...)` | **Fronteiriço** | 3 linhas de `for ... eprint!(...)`. O `eprint!` é I/O trivial (std); a string vem de função que é candidata L2. Se `format_diagnostic` migrar para L2, este par natural separa-se. Opções: migrar ambos para L2 (mas L2 idealmente não faz I/O) ou inline em L4 após migração do format. |
| **`pipeline.rs`** | `eval_to_module_with_sink(world, source)` | **Fronteiriço — tendência L3/L4** | Orquestra `eval()` + boilerplate comemo (Routines/Traced/Sink/Route/.track). Não faz I/O. É composição. Pode viver em L3 como "pipeline braçal" ou migrar para L4. Argumento para L3: esconde `comemo` do L4 — L4 fica mesmo thin. Argumento para L4: é composição, não I/O. |
| `pipeline.rs` | `compile_to_pdf_bytes(world, source)` | **Fronteiriço — tendência L3/L4** | Igual ao anterior; chama `eval → introspect → layout → export_pdf`. Não faz I/O além do `export_pdf` (que retorna bytes — o `write` fica no caller). |
| **`export.rs`** | `export_pdf(doc)` | **L3 correcto** | Produz bytes PDF (serialização). I/O de formato. |
| `export.rs` | `export_pdf_with_font(doc, font_data)` | **L3 correcto** | Idem + usa font bytes. |
| `export.rs` | `process_png_for_pdf(raw_data)` | **L3 correcto** | Processa PNG → payload PDF. |
| `export.rs` | `PdfImagePayload` | **L3 correcto** | Tipo de dados de I/O. |
| **`fonts.rs`** | `discover_fonts(paths)` | **L3 correcto** | Lê filesystem. |
| `fonts.rs` | `font_info_from_bytes(data, index)` | **L3 correcto** | Parsing de bytes OTF. |
| `fonts.rs` | `build_font_book(slots)` | **L3 correcto** | Constrói catálogo. |
| `fonts.rs` | `FontSlot` | **L3 correcto** | Tipo com lazy path→bytes. |
| **`font_metrics.rs`** | `FontBookMetrics<'a>` | **L3 correcto** | Métricas de font real (ttf-parser). |
| **`image_sizer.rs`** | `ImageSizeImageSizer` | **L3 correcto** | Lê cabeçalho de imagem. |
| **`layout.rs`** | `layout_with_font(content, font_data, size)` | **L3 correcto** | Usa bytes de font + L1 Layouter. I/O (bytes) presente. |
| **`world.rs`** | `SystemWorld` | **L3 correcto** | World real, filesystem, FileId registry. |
| `world.rs` | `SystemWorldError` | **L3 correcto** | Erro de I/O. |

---

## Resumo por classificação

| Classificação | Contagem | Itens |
|---------------|---------:|-------|
| **L3 correcto** | 14 | `export_pdf`, `export_pdf_with_font`, `process_png_for_pdf`, `PdfImagePayload`, `discover_fonts`, `font_info_from_bytes`, `build_font_book`, `FontSlot`, `FontBookMetrics`, `ImageSizeImageSizer`, `layout_with_font`, `SystemWorld`, `SystemWorldError`, `SourceSlot` (private). |
| **Candidato L2** | **1** | `format_diagnostic`. |
| **Fronteiriço** | **3** | `drain_diagnostics_to_stderr`, `eval_to_module_with_sink`, `compile_to_pdf_bytes`. |

**Zero Candidato L4** claros — os 2 de pipeline são fronteiriços, não puros composição.

---

## Análise dos fronteiriços

### `drain_diagnostics_to_stderr`

Corpo:

```rust
for diag in diagnostics {
    eprint!("{}", format_diagnostic(diag, source, source_path, colored));
}
```

Três linhas. `eprint!` é I/O trivial. A string vem de
`format_diagnostic` (candidato L2).

**Opções**:

- **(a)** Migrar o par `format_diagnostic` + `drain_*` para L2.
  **Problema**: L2 idealmente não faz I/O directo (`eprint!`);
  L2 devolve strings, L4 escreve. Mas o pattern "L2 pede a L3
  para escrever" é ceremonia sem ganho.
- **(b)** Migrar `format_diagnostic` para L2, manter `drain_*` em L3.
  **Problema**: cria fronteira estranha — `format` em L2 chamado
  via `drain` em L3 que re-exporta `format`. Inverso do normal.
- **(c)** Migrar `format_diagnostic` para L2, **remover** `drain_*`
  de L3 e inline em L4 (3 linhas de `for ... eprint!(...)`).
  **Vantagem**: L4 ganha 3 linhas; L3 perde 1 função; L2 fica com
  só a parte de apresentação. L4 faz I/O trivial — consistente
  com "L4 compõe L2+L3". Recomendado.
- **(d)** Manter ambos em L3. **Vantagem**: zero mudança. **Contra**:
  V12 pendente (L2 é o lugar para apresentação).

**Recomendação**: **(c)** — `format_diagnostic` em L2; `drain_*`
removido de L3, inline em L4 (`for ... eprint!(format_diagnostic(...))`).

### `eval_to_module_with_sink` + `compile_to_pdf_bytes`

Estas funções orquestram eval + layout + export. Não fazem I/O
directo (export_pdf retorna bytes; escrita é do caller em L4).

**Opções**:

- **(a)** Mover para L4. L4 ganha ~30 linhas de boilerplate
  comemo + composição. **Problema**: L4 deixa de ser thin.
  Passo 117 estabeleceu "~75 linhas em L4"; migração daria
  ~105. Limite não rígido mas contraria espírito.
- **(b)** Mover para L2. **Problema**: L2 não tem deps de comemo,
  eval, layout. Importar L1 + `comemo` em L2 só para isto cria
  camada gorda onde L2 devia ser fino (só apresentação).
- **(c)** Manter em L3 como **"pipeline braçal"**. Aceitar que L3
  tem dois papéis: I/O bruto + composição que esconde infra-estrutura
  (comemo, orquestração). **Vantagem**: L4 fica thin, L2 fica
  fino. **Contra**: L3 ganha função de composição, violando "L3 é
  só I/O".

A arquitectura Cristalina (do `typst-migracao-estado.md`) diz
"L3: I/O". Mas o padrão de facto em outras ferramentas (rustc,
clang) é que a "camada que conhece infra-estrutura" também esconde
orquestração — é mais prático.

**Recomendação**: **(c)** — manter em L3 com aceitação explícita
de que "pipeline L3" é papel secundário. Documentar em
ADR futura se/quando for contestado. **Alternativa**: mover para
L4 se L4 puder crescer para ~120 linhas sem degradação.

### Nota sobre "fronteiriço definitivo"

Nenhum dos 3 fronteiriços é **definitivo para L2** — todos têm
argumentos para ficar em L3. Só `format_diagnostic` é **definitivo
para L2**. A auditoria confirma o pre-anticipated.

---

## Conclusões 118.A

1. **1 candidato óbvio L2**: `format_diagnostic`.
2. **1 candidato dependente**: `drain_diagnostics_to_stderr` (liga-se
   ao primeiro — tratar como par).
3. **2 fronteiriços pipeline** — recomendação: **manter em L3**
   (não migrar neste round).
4. **14/18 funções** correctamente classificadas como L3.
5. **Nenhum candidato grande** — o trabalho é localizado em
   `diagnostic_format.rs` (~120 linhas).

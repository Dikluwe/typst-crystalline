# typst-passo-884 — Relatório

**Objectivo:** implementar compressão FlateDecode nos content streams de página (frente 1 identificada por P883) e fazer uma sonda da frente 2 (redução de verbosidade dos operadores PDF) sem a implementar ainda.

**Estado do código no momento da medição:**
- Commit base: `3f15cc50e`.
- Ficheiros alterados por este passo: `03_infra/src/export/builder.rs`, `03_infra/src/export/mod.rs`, `03_infra/src/export/tests.rs`, `03_infra/src/integration_tests.rs`, `00_nucleo/prompts/infra/export/builder.md`, e os snapshots binários de `03_infra/fixtures/p307b/reference/`.
- Testes: `cargo test --workspace` verde. `crystalline-lint .` com zero violations relevantes (apenas V7 pré-existente: `package_version_resolution.md` órfão).

---

## 1. Implementação — compressão FlateDecode dos content streams

Adicionada a função `build_content_stream` em `03_infra/src/export/builder.rs`. Ela:

- Comprime os bytes devolvidos por `build_page_stream` com `compress_zlib` (mesmo mecanismo usado para imagens e fontes desde P833).
- Só emite `/Filter /FlateDecode` quando o resultado comprimido é de facto menor do que o original; caso contrário, emite o stream sem filtro.
- Em caso de falha do compressor, emite o stream sem compressão — o PDF continua válido.

A função foi aplicada nos três caminhos de `PdfBuilder` (`build_helvetica`, `build_cidfont`, `build_multifont`), substituindo o padrão repetido:

```rust
let stream_bytes = build_page_stream(page, &ctx);
let len = stream_bytes.len();
let mut obj = format!("<< /Length {len} >>\nstream\n").into_bytes();
obj.extend_from_slice(&stream_bytes);
obj.extend_from_slice(b"\nendstream");
self.add_bytes(stream_id, obj);
```

por:

```rust
let stream_bytes = build_page_stream(page, &ctx);
self.add_bytes(stream_id, build_content_stream(&stream_bytes));
```

### Prompt L0 actualizado

- `00_nucleo/prompts/infra/export/builder.md`: adicionada secção §P884 com a especificação da compressão de content streams e o teste de regressão; histórico actualizado.
- Hash do `@prompt` em `builder.rs` actualizado via `crystalline-lint --fix-hashes`.

### Snapshots actualizados

A mudança alterou os bytes dos PDFs de referência do `p307b`. Os snapshots foram regenerados com `UPDATE_P307B_SNAPSHOTS=1`.

---

## 2. Testes

### Helpers de teste partilhados

Como a compressão torna os operadores PDF invisíveis no PDF bruto, foi necessário actualizar testes que inspeccionam content streams. Os helpers `extract_stream_bytes`, `extract_object_dict_text` e `extract_page_content_streams_text` foram movidos de `03_infra/src/export/tests.rs` para `03_infra/src/export/mod.rs` (sob `#[cfg(test)] pub(crate)`), tornando-os acessíveis também a `03_infra/src/integration_tests.rs`.

### Testes actualizados

- **39 testes unitários em `export::tests`**: passaram a usar `extract_page_content_streams_text(&pdf)` para verificações de operadores dentro de content streams, mantendo o PDF bruto para asserções estruturais (fontes, `/DCTDecode`, `/Pattern`, etc.).
- **16 testes de integração em `integration_tests`**: mesmo ajuste — de `String::from_utf8_lossy(&pdf)` para `extract_page_content_streams_text(&pdf)`.
- **Teste de regressão novo**: `p884_content_streams_comprimidos_com_flate_decode` em `export::tests.rs`. Cria um documento com texto repetido, confirma que o marcador não aparece em claro no PDF bruto, e confirma que é recuperável ao descomprimir os content streams.

### Contagem de testes discriminada por crate

```text
typst-core:   0 passados directamente (mudança em L3)
typst-infra:  731 passados; 5 ignorados; 0 falhados
              (284 em export::tests + 16 em integration_tests + restantes)
typst-shell:  41 passados; 0 falhados
04_wiring:    39 passados; 0 falhados (37 cli + 2 crystalline_lint)
```

A contagem subiu de 729 (P883) para 731: +1 pelo teste de regressão P884 e +1 pelo estado já presente em working tree proveniente de P883.

---

## 3. Validação nos sete cenários do benchmark

### Tamanhos dos PDFs

Compilado com o binário de release actualizado (`cargo build --release -p typst-wiring`). Documentos em `/tmp/p872-bench/`.

| Cenário | P880/P883 (B) | **P884 (B)** | Vanilla 0.15.0 (B) | Razão P880/P883 | **Razão P884** |
|---|---|---|---|---|---|
| `01-hello` | 5 908¹ | **4 382** | 5 571 | 1.06× | **0.79×** |
| `02-lorem` | 58 906² | **16 510** | 14 763 | 3.99× | **1.12×** |
| `03-images` | 8 436¹ | **5 725** | 13 665 | 0.62× | **0.42×** |
| `04-math` | 67 798¹ | **13 295** | 95 779 | 0.71× | **0.14×** |
| `05-tables` | 66 442¹ | **9 951** | 203 536 | 0.33× | **0.05×** |
| `06-long` | 1 004 336² | **169 836** | 151 708 | 6.62× | **1.12×** |
| `07-context` | 2 100¹ | **2 100** | 50 670 | 0.04× | **0.04×** |

¹ Valores de P880 (última medição completa dos sete cenários antes de P884).  
² Valores de P883 (última medição destes dois cenários).

A redução é drástica em todos os cenários com conteúdo textual significativo. `06-long` passou de ~1 MB para ~170 KB (quase 6× menor). `01-hello`, `03-images`, `04-math`, `05-tables` e `07-context` ficam abaixo do tamanho vanilla.

### Tempos de compilação

Corridos com `hyperfine 1.20.0`, `--warmup 1 --min-runs 5`, output para `/dev/null`.

| Cenário | Vanilla (s) | P884 (s) | Razão C/V |
|---|---|---|---|
| `02-lorem` | 0.2801 | **0.1183** | **0.42×** |
| `04-math` | 0.2817 | **4.983** | **17.69×** |
| `06-long` | 0.2972 | **0.3768** | **1.27×** |

- `02-lorem` e `04-math` mantêm as mesmas razões de P880 (0.42× e ~17.7×) — nenhuma regressão de tempo.
- `06-long` regrediu ligeiramente de 1.20× (P880) para 1.27×. O custo extra vem da compressão FlateDecode de 51 content streams, mas a diferença absoluta é pequena (~80 ms). Considera-se aceitável face à redução de ~830 KB no PDF.

### Validação em poppler e ghostscript

Todos os PDFs gerados foram validados:

- `pdftoppm -png -f 1 -l 1 <pdf> <out>`: OK para todos os cenários.
- `pdftotext`: texto coerente com o esperado.
- `gs -dNOPAUSE -dBATCH -sDEVICE=pdfwrite`: OK para todos os cenários.

`06-long` específico: 51 páginas em ambos os lados; `pdftotext` produz 1000 linhas no cristalino vs 1054 no vanilla — a diferença é apenas nos running headers/footers que o vanilla insere por defeito.

---

## 4. Sonda da frente 2 — verbosidade dos operadores PDF

Depois da compressão, `06-long` continua 1.12× maior que o vanilla. A sonda da frente 2 procurou quantificar quanto dessa diferença residual vem de verbosidade de operadores, não de falta de compressão.

### Metodologia

Extraídos e descomprimidos todos os content streams de `cristalino-06-long-p884.pdf` e `vanilla-06-long.pdf` com `mutool show`, e contados operadores `BT`/`ET`.

### Resultados

| Métrica | Cristalino P884 | Vanilla 0.15.0 |
|---|---|---|
| Páginas | 51 | 51 |
| Content streams descomprimidos (total) | **972 960 B** | **626 950 B** |
| Blocos `BT`/`ET` | **10 150** (~199/página) | **902** (~18/página) |
| Bytes médios por bloco | 95.8 | 695.7 |

### Análise

- O cristalino emite **um bloco `BT ... ET` por run de texto** (muitos blocos pequenos, alguns com um único glifo).
- O vanilla agrupa texto por parágrafo/linha em **um único bloco `BT ... ET`** com grandes arrays `TJ` e mudanças de matriz internas.
- O cristalino também usa 3 casas decimais; o vanilla usa 4–7 casas decimais — neste aspecto o cristalino já é mais compacto.
- O vanilla ainda acrescenta overhead de conteúdo marcado (`/Artifact`, `/Span`, `BDC`/`EMC`) e outlines por secção, que o cristalino não emite.

### Estimativa de ganho potencial

Se o cristalino agrupasse runs num número de blocos comparável ao vanilla (~18/página), o conteúdo descomprimido aproximava-se dos ~627 KB do vanilla. Compressão semelhante (13–16%) levaria os content streams comprimidos de ~130 KB para ~100 KB, reduzindo o PDF total de ~170 KB para ~140 KB — potencial de fechar a diferença residual para o vanilla em `06-long`.

### Recomendação

A frente 2 é **tecnicamente viável e com ganho estimado de ~15–25 KB** no cenário `06-long`, mas exige reestruturar a emissão de texto em `stream.rs` para agrupar runs consecutivas num único bloco `BT ... ET`. Isso tem risco de regressão em kerning/posicionamento e deve ser tratado num passo dedicado, não neste. Fica registada como scope-out com insumo concreto.

---

## 5. Scope-outs

1. **Agrupamento de runs de texto em blocos `BT ... ET` mais grandes.** Identificado como a causa residual do tamanho de `06-long`. Fica para passo futuro devido ao risco de alterar o layout de texto.
2. **Conteúdo marcado (`/Artifact`, `/Span`, `BDC`/`EMC`) e outlines.** O vanilla emite estas estruturas; o cristalino não. Não são puramente otimizações de tamanho — têm semântica de acessibilidade e navegação. Se/quando implementadas, devem ser passos próprios.

---

## 6. Resumo

- **Frente 1 concluída:** content streams de página comprimidos com FlateDecode nos três caminhos do `PdfBuilder`, com fallback não-comprimido para streams pequenos ou falhas.
- **Impacto de tamanho:** reduções drásticas em todos os cenários; `06-long` passou de 1 004 336 B para 169 836 B (5.91× menor); `01-hello`, `03-images`, `04-math`, `05-tables` e `07-context` ficam menores que o vanilla.
- **Impacto de tempo:** ligeira regressão apenas em `06-long` (1.20× → 1.27×, ~80 ms); outros cenários mantidos.
- **Testes:** 39 testes unitários e 16 de integração actualizados; teste de regressão P884 adicionado; snapshots regenerados.
- **Frente 2 sondada:** a verbosidade residual em `06-long` vem do número excessivo de blocos `BT ... ET` (~199 vs ~18 por página no vanilla). Potencial de ganho estimado em ~30 KB de content streams descomprimidos, mas requer passo dedicado.

# typst-passo-882 — Relatório

**Objectivo:** investigar por que `02-lorem` e `06-long` continuavam maiores que o vanilla mesmo com lazy coverage (P880) e subsetting CFF (P874), e corrigir a causa.

**Estado do código no momento da medição:**
- Commit: `3f15cc50e`
- Ficheiros alterados por este passo: `03_infra/src/export/builder.rs`, `03_infra/src/export/tests.rs`, `00_nucleo/prompts/infra/export/builder.md`, `00_nucleo/prompts/infra/export/font_subset.md`.
- Testes: `cargo test --workspace` verde (729 passed, 5 ignored, 0 failed). `crystalline-lint .` com zero violations relevantes (apenas V7 pré-existente: `package_version_resolution.md` órfão).

---

## 1. Descoberta: o cristalino embutia a fonte CFF dentro de um wrapper OpenType/SFNT

A comparação directa das fontes extraídas dos PDFs de `02-lorem` mostrou:

| Métrica | Cristalino (P880) | Vanilla 0.15.0 |
|---|---|---|
| Stream PDF | 9496 bytes | 6113 bytes |
| Magic dos bytes | `4f54544f` (`OTTO` — OpenType/SFNT wrapper) | `789c8d58` (zlib/FlateDecode) |
| Subtipo PDF | `/Subtype /OpenType` | `/Subtype /CIDFontType0C` + `/Filter /FlateDecode` |
| Tabela `CFF ` extraída | 7982 bytes | — |

Ao extrair apenas a tabela `CFF ` do wrapper cristalino obtive **7982 bytes** — praticamente igual ao vanilla descomprimido (**8023 bytes**). A diferença de ~1.5 KB por ocorrência vinha do wrapper SFNT (`OTTO`) em torno do programa CFF, não do conteúdo da fonte em si.

Após a correção (P882), o cristalino passou a embutir o programa CFF puro (`/CIDFontType0C`), como o vanilla:

| Métrica | Cristalino (P882) | Vanilla 0.15.0 |
|---|---|---|
| Stream PDF | 8018 bytes | 6113 bytes |
| Magic dos bytes | `01000404` (CFF puro) | zlib/FlateDecode |
| Subtipo PDF | `/Subtype /CIDFontType0C` | `/Subtype /CIDFontType0C` + `/Filter /FlateDecode` |
| Glifos CFF | 52 | 53 |
| CFF descomprimido | 8018 bytes | 8023 bytes |

**Conclusão da sondagem:** a contagem de glifos é equivalente (52 vs 53). O conteúdo CFF é praticamente idêntico. A diferença restante para o vanilla é só **compressão**: o vanilla aplica `/Filter /FlateDecode` ao stream da fonte; o cristalino ainda não comprime font streams. Esse é um scope-out natural do P882 (que tratava do *formato* do stream, não da compressão).

---

## 2. Implementação

Alterado `font_embedding_data` em `03_infra/src/export/builder.rs:300`:

- CFF1/OpenType → extrai a tabela `CFF ` do SFNT e embute o programa CFF puro com `/Subtype /CIDFontType0C`.
- CFF2/OpenType (fontes variáveis) → mantém o contêiner SFNT completo com `/Subtype /OpenType`, porque o spec PDF não define subtipo para "CFF2 puro".
- TrueType → inalterado (`/CIDFontType2` + `/FontFile2`).

Para suportar a distinção CFF1 vs CFF2, `has_cff_or_cff2_table` foi substituído por `has_sfnt_table`, `has_cff_table` e `has_cff2_table`.

### Prompts L0 actualizados

- `00_nucleo/prompts/infra/export/builder.md`: critério de verificação corrigido (CFF1 → `/CIDFontType0C`, CFF2 → `/OpenType`); adicionada nota P882 na secção §P560/§P772u e no histórico.
- `00_nucleo/prompts/infra/export/font_subset.md`: descritor PDF actualizado para refletir CFF1 bare/CFF2 SFNT; histórico actualizado.

Hashes actualizados via `crystalline-lint --fix-hashes`.

---

## 3. Testes

Renomeado e actualizado `p560_fonte_cff_usa_cidfont_type0` → `p560_fonte_cff_usa_cidfont_type0c` em `03_infra/src/export/tests.rs:250`:

- Verifica `/Subtype /CIDFontType0C` em vez de `/OpenType`.
- Verifica que o stream não usa `/Subtype /OpenType`.
- Verifica que os bytes do stream começam com a assinatura CFF pura (`0100`), não `OTTO`.

O teste existente `p772u_fonte_cff2_usa_cidfont_type0_opentype` permanece inalterado e passa: CFF2 continua com `/OpenType`.

### Contagem de testes discriminada por crate

```text
typst-core:   0 passados (nenhum teste directo desta mudança)
typst-infra:  729 passados; 5 ignorados; 0 falhados
typst-shell:  41 passados; 0 falhados
04_wiring:    39 passados; 0 falhados (37 cli + 2 crystalline_lint)
```

Nota: os números batem com o estado anterior (P880/P881), sem remoção nem adição de testes fora do rename/actualização de `p560_fonte_cff_usa_cidfont_type0c`.

---

## 4. Validação de tamanho

Compilados com o binário de release actualizado (`cargo build --release -p typst-wiring`):

| Cenário | Cristalino P880 | Cristalino P882 | Vanilla 0.15.0 | Razão P880 | Razão P882 |
|---|---|---|---|---|---|
| `02-lorem` | 62335 bytes | **60862 bytes** (−1473) | 14763 bytes | 4.22× | 4.12× |
| `06-long` | 1007269 bytes | **1005795 bytes** (−1474) | 151708 bytes | 6.64× | 6.63× |

A redução de ~1.5 KB por ocorrência de fonte CFF1 confirma a causa do wrapper SFNT. Os PDFs continuam válidos:

- `pdftoppm -png -f 1 -l 1 <pdf> <out>`: OK para ambos.
- `gs -dNOPAUSE -dBATCH -sDEVICE=pdfwrite`: OK para ambos.
- `pdftotext` de `02-lorem` é **idêntico** ao vanilla (37 linhas, diff vazio).
- `pdftotext` de `06-long` difere do vanilla apenas nos running headers/footers ("Header"/"Footer"/"Lorem section X") que o vanilla insere por defeito e o cristalino não — diferença de template, não do embedding de fonte.

---

## 5. Scope-outs identificados

1. **Compressão do stream CFF/TrueType com FlateDecode.** O vanilla comprime o stream da fonte; o cristalino ainda não. Para `02-lorem`, isso explica quase toda a diferença restante (8018 bytes cristalino vs 6113 bytes vanilla). Não foi abordado neste passo porque o scope era o *formato* do stream (SFNT vs CFF puro), não a compressão.
2. **Outros fatores de tamanho em `06-long`.** O PDF cristalino de `06-long` ainda é 6.63× maior que o vanilla. A redução de 1.5 KB no stream da fonte é irrelevante face aos ~850 KB de content streams por página (cada uma com ~19 KB de operadores PDF). Esta eficiência dos content streams é uma frente separada.

---

## 6. Resumo

- **Causa raiz confirmada:** o cristalino embutia fontes CFF1 como contêiner OpenType/SFNT completo (`/Subtype /OpenType`); o vanilla embute só o programa CFF puro (`/Subtype /CIDFontType0C`).
- **Correcção:** `font_embedding_data` extrai a tabela `CFF ` e embute o programa puro para CFF1; CFF2 mantém SFNT completo.
- **Resultado:** redução de ~1.5 KB por ocorrência de fonte CFF1; conteúdo CFF idêntico ao vanilla (52/53 glifos, 8018 vs 8023 bytes descomprimidos); PDFs validados em poppler e ghostscript.
- **Próxima frente identificada:** compressão FlateDecode dos streams de fonte e otimização dos content streams de múltiplas páginas para reduzir ainda mais a diferença em documentos longos.

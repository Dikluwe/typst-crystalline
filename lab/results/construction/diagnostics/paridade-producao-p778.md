# Relatório de Paridade — P778

**Data:** 2026-07-16
**Passo:** P778
**Objetivo:** Verificar se o ICC teve efeito real no resíduo de AE 195/138/0 reportado em P776/P777 e identificar a causa real do padrão.

---

## Estado base da medição

- **Commit base:** `0e355978c07c8892d34a0e168470edc8beac4ecc` (P777)
- **Estado:** working tree sem alterações de código (P778 é sonda de verificação).
- **Ferramentas:** `mutool 1.23.7`, `pdftoppm 24.02.0`, `Ghostscript 10.02.1`, `ImageMagick compare 6.9.12`.
- **Dados de comparação gerados em:** `/tmp/p778-work/`
  - P776: binário compilado a partir de `6ba0a1042e6787da0d4be5f8f14850b7f58a3132` via `git worktree`.
  - P777: binário do working tree atual (`target/release/typst`).
  - Vanilla: `lab/typst-original/target/release/typst` (0.15.0, commit `969087ec`).
  - Fixture: imagem JPEG 200×100 px com EXIF orientation 1–8.

---

## Passo 0 — O ICC teve efeito real?

Comparação direta dos renders P776 vs P777 para a mesma orientação (mesmo fixture, mesma resolução, mesmo rasterizador).

| Orientação | AE P776 vs P777 (`mutool draw -r 300`) |
|-----------|----------------------------------------|
| 1         | 0                                      |
| 2         | 0                                      |
| 3         | 0                                      |
| 4         | 0                                      |
| 5         | 0                                      |
| 6         | 0                                      |
| 7         | 0                                      |
| 8         | 0                                      |

**Conclusão:** a mudança de `/DeviceRGB` para `/ICCBased` **não alterou nenhum pixel** na imagem renderizada. O ICC foi uma correção estrutural/metadados correta, mas **não é a causa de nenhum resíduo**, presente ou passado. A atribuição do resíduo ao color space em P776/P777 estava incorrecta.

---

## Passo 1 — Há reamostragem de pixels no caminho?

Leitura do código do vanilla (`lab/typst-original/crates/typst-pdf/src/image.rs`) e do cristalino (`03_infra/src/export/stream.rs`, `03_infra/src/export/images.rs`):

- Ambos os caminhos aplicam a orientação EXIF exclusivamente via **matriz de transformação `cm`** no content stream.
- Nenhum dos dois recodifica pixels; o stream JPEG é passado raw (`/DCTDecode`).
- Não há operação de `rotate90`, `rotate270`, `interpolate`, `resample`, etc., no pipeline de imagem.

**Conclusão:** não existe reamostragem de pixels. A hipótese de "interpolação de rotação" no código cristalino cai.

---

## Passo 2 — Matrizes `cm` comparadas com precisão total

Extraídas das streams de conteúdo dos PDFs (cristalino P777 vs vanilla).

| Orient | Transformação | Cristalino P777 | Vanilla |
|--------|---------------|-----------------|---------|
| 1 | normal | `200 0 0 100 70.86667 671.02333` | `200 0 -0 100 70.86614 70.86615` |
| 2 | flip H | `-200 0 0 100 270.86667 671.02333` | `-200 -0 -0 100 270.86616 70.86615` |
| 3 | rotate 180 | `-200 0 0 -100 270.86667 771.02333` | `-200 -0 -0 -100 270.86616 170.86615` |
| 4 | flip V | `200 0 0 -100 70.86667 771.02333` | `200 0 -0 -100 70.86614 170.86615` |
| 5 | rot 90 + flip H | `0 -200 -100 0 170.86667 771.02333` | `0.000000000000012246468 -200 -100 -0.000000000000006123234 170.86615 771.0236` |
| 6 | rot 90 | `0 -200 100 0 70.86667 771.02333` | `0.000000000000012246468 -200 100 0.000000000000006123234 70.86615 771.0236` |
| 7 | rot 270 + flip H | `0 200 100 0 70.86667 571.02333` | `-0.000000000000012246468 200 100 0.000000000000006123234 70.86615 571.0236` |
| 8 | rot 270 | `0 200 -100 0 170.86667 571.02333` | `-0.000000000000012246468 200 -100 -0.000000000000006123234 170.86615 571.0236` |

*Nota: as coordenadas y aparentemente diferentes entre cristalino e vanilla no formato `mutool trace` resultam da inversão do eixo y do device space; as streams mostram a translação em user space.*

**Diferenças observadas:**
1. **Translação:** cristalino usa margem `min(w,h) * 2.5 / 21 = 70.86667 pt`; vanilla usa margem `2.5 cm = 70.86614 pt`. Diferença de ~0.0005 pt nas coordenadas de translação.
2. **MediaBox:** cristalino `595.28 841.89`; vanilla `595.2756 841.8898`.
3. **Valores quase-zero:** vanilla representa rotações 90°/270° com valores do tipo `1.2246468e-14` (precisão de float64 de cos/sin); cristalino usa `0` exacto.

---

## Passo 3 — Causa real do resíduo

### Testes de isolamento (PDF do cristalino P777 editado com `sed`)

| Alteração no PDF cristalino | Orient 2 AE | Orient 5 AE |
|-----------------------------|-------------|-------------|
| Nenhuma (P777 original)     | 195         | 138         |
| Translação ajustada para vanilla | 0           | 0           |
| Só MediaBox ajustado para vanilla | —           | 0           |
| Só valores quase-zero da rotação (sem ajustar translação) | —           | 138         |

### Comparação com diferentes rasterizadores

| Rasterizador | Resolução | AE orient 2 | AE orient 5 | AE orient 7 |
|--------------|-----------|-------------|-------------|-------------|
| `pdftoppm`   | 150 dpi   | 195         | 138         | 195         |
| `pdftoppm`   | 300 dpi   | 0           | 0           | 0           |
| `mutool draw`| 300 dpi   | 0           | 0           | 0           |
| `gs`         | 300 dpi   | 0           | 0           | 0           |

### Causa identificada

O resíduo de AE 195/138/0 é um **artefacto de medição do `pdftoppm -r 150`**, provocado por uma diferença de **~0.0005 pt na translação da matriz `cm`** entre cristalino e vanilla. Essa diferença origina-se em:

- **Margem:** cristalino calcula `min(w,h) * 2.5 / 21`; vanilla usa `2.5 cm = 70.86614 pt` exacto.
- **MediaBox:** cristalino `595.28 × 841.89 pt`; vanilla `595.2756 × 841.8898 pt`.

A magnitude (~0.0005 pt ≈ 0.001 px a 150 dpi) é suficiente para fazer com que o rasterizador `pdftoppm` arredonde a posição de alguns pixels para o lado oposto em certas orientações, gerando o padrão observado. O padrão separa por tipo de transformação porque a posição final da imagem (e a sua relação com a grelha de pixels) depende da orientação.

**Valores quase-zero da matriz de rotação** (cos/sin em float64) no vanilla **não são a causa**: testes isolados mantiveram AE=138 quando só esses valores foram alterados.

---

## Decisões

1. **A atribuição do resíduo ao color space em P776/P777 estava errada.** O ICC não altera pixels renderizados; a mudança é estrutural/metadados e correcta.
2. **Não há reamostragem de pixels no caminho de imagem.** Ambos os compiladores usam matriz `cm` pura.
3. **O resíduo é um artefacto do rasterizador de comparação (`pdftoppm -r 150`)**, não uma diferença de paridade semântica entre cristalino e vanilla. Outros rasterizadores e resoluções mais altas dão AE=0.
4. **Não se abre P779 para corrigir este resíduo.** A diferença sub-pixel nas unidades de página/margem é real, mas o seu efeito só se manifesta numa configuração específica de medição. Corrigir exigiria alterar `PageConfig::default()` em L1 para replicar exactamente as dimensões A4 e margem 2.5 cm do vanilla, com impacto alargado em snapshots existentes — decisão que ultrapassa o scope desta sonda.
5. **A linha de imagem P769–P778 fecha com o resíduo documentado como artefacto de medição.**

---

## Resíduos e próximos passos

- **Resíduo técnico:** diferença de ~0.0005 pt na translação de imagens entre cristalino e vanilla, proveniente das unidades de página/margem.
- **Impacto observável:** só aparece com `pdftoppm -r 150`; inexistente com `pdftoppm -r 300`, `mutool draw -r 300` e `gs -r 300`.
- **Próximo passo recomendado (fora do scope de P778):** passo dedicado ao alinhamento de `PageConfig::default()` com as dimensões A4 exactas do vanilla (`595.2756 × 841.8898 pt`) e margem 2.5 cm exato, se se decidir que a paridade mecânica de posicionamento sub-pixel é necessária.

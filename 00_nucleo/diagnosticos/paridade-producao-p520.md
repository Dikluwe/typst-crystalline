# Relatório de Paridade de Produção — Passo 520

## Resumo

O Passo 520 corrigiu dois defeitos independentes no export PDF cristalino:

1. **Kerning** — o operador `TJ` não aplicava os ajustes de posicionamento
   (`x_offset`/`x_advance`) produzidos pelo shaper. O texto era renderizado
   com avanços nominais, perdendo o kerning da fonte.

2. **Ligatures no subsetting** — quando o subsetting TrueType estava activo,
   glifos de ligature (`fi`, `fl`, `ffi`) eram emitidos como `.notdef`
   porque o mapeamento `old_gid → new_gid` só era reconstruído para glifos
   com codepoint Unicode próprio.

Ambos os problemas foram corrigidos, testados e commitados de forma
**causalmente independente**, conforme ADR-0109.

## Sondas (pré-decisão)

| # | Pergunta | Medição | Conclusão |
|---|----------|---------|-----------|
| 1 | O PDF usa `/W` com larguras nominais ou `TJ` com ajustes? | `builder.rs:250`, `stream.rs:215-242` | Modelo `/W` + `TJ` com deltas (positivo aproxima, negativo afasta). |
| 2 | Onde calcular o delta do `TJ`? | `ShapedGlyph` tem `x_advance` e `x_offset`. | `advance_tu = (x_advance - nominal) / upm * 1000`. |
| 3 | Como obter largura nominal? | `face.glyph_hor_advance(gid)` na fonte original. | Construir `glyph_to_nominal` antes do subset. |
| 4 | O subsetting sabe quais glifos shaped são usados? | `collect_glyph_ids(doc)` em `fonts.rs:79`. | Sim, retorna todos os `glyph_id` reais. |
| 5 | Qual codepoint usar para ligatures? | `ShapedGlyph.cluster`/`char_code` = primeiro caractere do cluster. | Mapear para o primeiro caractere; ToUnicode fica parcial. |

## Decisões técnicas

### Kerning (delta model)

- O `/W` array mantém as larguras nominais dos glifos no subset.
- O operador `TJ` aplica a diferença entre `x_advance` real do shaper e a
  largura nominal:
  ```text
  <new_gid> (x_advance - nominal) / upm * 1000
  ```
- Positivo → aproxima o glifo seguinte; negativo → afasta. Isto reproduz o
  efeito do kerning sem precisar de tabelas `GPOS`/`kern` no subset.

### Ligatures (PUA mapping)

- `subset_font_with_mapping` recebe `additional_gids`: glifos reais do shaper
  sem codepoint Unicode próprio (ligatures).
- Cada `additional_gid` recebe um codepoint na Área de Uso Privado
  (`0xF0000+`) que é incluído no `cp_to_old_gid` passado ao
  `oxifont_subset`.
- Após o subset, o `new_gid` é recuperado via `face.glyph_index(pua_cp)`.
- Estes codepoints privados **não** são expostos no ToUnicode CMap do PDF.

### ToUnicode e `/W` a partir de glifos reais

- `builder.rs` constrói `to_unicode_mappings` a partir de
  `collect_shaped_glyph_mappings(doc)` re-mapeado para `new_gid`.
- Garante que o ToUnicode CMap e o array `/W` cobrem todos os glifos
  efectivamente usados no stream, não apenas os caracteres Unicode únicos.

## Ficheiros alterados

- `03_infra/src/export/stream.rs` — delta model no operador `TJ`.
- `03_infra/src/export/subset.rs` — PUA mapping para `additional_gids`.
- `03_infra/src/export/builder.rs` — `to_unicode_mappings` e `/W` a partir de
  shaped glyphs.
- `03_infra/src/export/fonts.rs` — `collect_shaped_glyph_mappings`.
- `00_nucleo/prompts/infra/export/font_subset.md` — actualização do L0.
- `00_nucleo/diagnosticos/debt/DEBT.md` — abertura de DEBT-64.
- `lab/parity/corpus/p520/` — corpus de regressão.
- `lab/parity/tools/p520_sentinela.sh` — sentinela de regressão.
- `03_infra/fixtures/p307b/reference/09-cidfont.pdf` — snapshot regenerado.

## Commits

1. `7e8d1d230` — `P520: corrigir kerning no export PDF (delta model no TJ)`
2. `c966ff2ef` — `P520: WIP ligatures subsetting + L0 font_subset.md`
3. `0900cebb0` — `P520: corrigir ligatures no subsetting PDF (PUA mapping + ToUnicode/widths)`

## Validação

```bash
# Unit tests
cargo test -p typst-infra --lib export::subset

# Suite completa
cargo test --workspace

# Linter
crystalline-lint .

# Sentinelas de regressão
bash lab/parity/tools/p520_sentinela.sh
```

Resultados:

- `cargo test --workspace`: **3550 passed; 0 failed** (entre outras suites).
- `cargo test -p typst-infra --lib`: **558 passed; 0 failed; 6 ignored**.
- `crystalline-lint .`: **No violations found**.
- `p520_sentinela.sh`: todas as verificações passaram.

## Observações de paridade

- **Visual**: `fi`, `fl`, `ffi` renderizam como ligatures; kerning é visível
  em pares como "AV".
- **ToUnicode**: ligatures extraem como `"f"` cada uma (ToUnicode parcial).
  Documentado em DEBT-64.
- **Mecânica**: a implementação diverge propositadamente da mecânica do
  Typst original (ADR-0107) — paridade é ao nível da linguagem (semântica,
  sintaxe, morfologia), não byte-a-byte com o PDF vanilla.

## Débito técnico

- **DEBT-64 — ToUnicode parcial para ligatures**: as ligatures mapeiam no
  ToUnicode CMap apenas para o primeiro caractere do cluster. Fechar exige
  suporte a strings multi-caractere no CMap ou transportar o texto original
  do cluster no `ShapedGlyph`.

## Reprodução manual

```bash
cargo run --bin typst -- lab/parity/corpus/p520/test-ligatures.typ /tmp/p520-ligatures.pdf
cargo run --bin typst -- lab/parity/corpus/p520/test-kerning.typ /tmp/p520-kerning.pdf

pdftotext /tmp/p520-ligatures.pdf -
mutool show /tmp/p520-kerning.pdf 4
```

# Relatório de Paridade de Produção — Passo 523

| Campo | Valor |
|-------|-------|
| Passo | P523 |
| Foco | Polimento de CFF subsetting e correção de narrativa arquitetural |
| Data | 2026-07-01 |
| Autor | IA (Kimi Code CLI) sob direção do utilizador |
| Status | Concluído |

## Resumo executivo

O Passo 523 executou o polimento deixado pela sonda P522: verificou-se que `oxifont-subset` já suporta CFF/CFF2, pelo que a narrativa de "CFF scope-out XL" estava desactualizada. Foram corrigidos o Prompt L0, a docstring do código e o handoff de contexto; adicionou-se uma fonte CFF (Nimbus Sans) ao corpus de paridade e um teste unitário em `03_infra`; e fez-se uma sonda rápida do descritor PDF gerado (`CID TrueType` vs `CID Type 0C`). A paridade de **linguagem** Typst mantém-se; a paridade de **produção** melhorou porque CFF deixa de cair em fallback de fonte completa.

## Alterações realizadas

### 1. Correção da narrativa CFF scope-out

- `00_nucleo/prompts/infra/export/font_subset.md` — removido CFF dos scope-outs; adicionada secção "Notas P523".
- `03_infra/src/export/subset.rs` — docstring de `subset_font_with_mapping` actualizada para refletir suporte interno a CFF/CFF2.
- `00_nucleo/adr/typst-adr-0055-font-consumer-cidfont.md` — anotação factual P523 na tabela de alternativas.
- `00_nucleo/diagnosticos/cristalino-contexto-handoff.md` — linha "CFF subsetting" passou a "Fechado em P523", com nota sobre polimento de descritor PDF.

### 2. Fixture e teste CFF no corpus

- Adicionada `03_infra/fixtures/fonts/NimbusSans-Regular.otf` (fonte CFF da família URW Base 35, licença AGPL-3 com Font exception).
- Adicionado `03_infra/fixtures/fonts/LICENSE-NimbusSans.txt`.
- Criado `lab/parity/corpus/p523/test-cff-nimbus.typ` para compilação manual.
- Adicionado teste unitário `p523_subset_cff_nimbus_sans_preserves_cff_table` em `03_infra/src/export/subset.rs`:
  - Verifica que a fonte subsetada continua parseável.
  - Verifica que a tabela `CFF ` é preservada.
  - Verifica que o número de glifos é `notdef + A + B = 3`.

### 3. Sonda do descritor PDF

Comparação entre saída cristalina e vanilla Typst para o mesmo documento CFF:

| Propriedade | Cristalino | Vanilla Typst |
|-------------|-----------|---------------|
| Tipo reportado por `pdffonts` | `CID TrueType` | `CID Type 0C` |
| Fonte embutida extraída | Contém tabela `CFF ` | Contém tabela `CFF ` |
| Tamanho da fonte no PDF | ~21 KB (apenas A, B, fi, fl, ffi, notdef) | ~10 KB |
| Texto extraível | Sim | Sim |
| Kerning `AV` | Aplicado (TJ com valor negativo) | — |

Observação: `verapdf` não está disponível no ambiente, pelo que não foi possível validar formalmente a conformidade do descritor com a spec PDF.

## Resultados de medição

### Subsetting CFF

```text
Fonte original:  82 264 bytes
Fonte subsetada: ~21 000 bytes
Redução:         ~74%
Glifos esperados: 3 (notdef, A, B)
Tabela CFF:      preservada
```

Teste unitário:

```bash
cargo test -p typst-crystalline-infra p523_subset_cff_nimbus_sans_preserves_cff_table -- --nocapture
```

Resultado: passou.

### Compilação do corpus CFF

```bash
cargo run --bin typst -- lab/parity/corpus/p523/test-cff-nimbus.typ /tmp/p523-cff.pdf
```

Resultado: PDF gerado com sucesso; texto seleccionável; ligatures `fi`/`fl`/`ffi` extraíveis.

## Pendências e scope-outs

| Item | Estado | Notas |
|------|--------|-------|
| CFF subsetting funcional | ✅ Fechado em P523 | `oxifont-subset` trata CFF/CFF2 internamente |
| Descritor PDF `CID Type 0C` | ⏸️ Scope-out mecânico | `pdffonts` reporta `CID TrueType`; a fonte embutida é CFF. Sob ADR-0107, isto é observável mecânico, não paridade de linguagem. |
| Variation fonts (VF) | ⏸️ Scope-out XL | Não abordado |
| Kerning no subset | ⏸️ Scope-out | GPOS/GSUB removidas pelo subsetter; posicionamento aplicado previamente pelo shaper |

## Recomendações

1. **Não reabrir "CFF subsetting" como XL.** A funcionalidade está operacional; o único trabalho restante é polimento do descritor PDF, que pode ser tratado num passo pequeno (P524) se o utilizador considerar relevante.
2. **Se P524 for iniciado**, o escopo deve ser: gerar `/Subtype /CIDFontType0C` (ou equivalente) e reduzir o overhead do container CIDFont, sem alterar a lógica de subsetting.
3. **Manter o teste unitário CFF** como regressão para futuras actualizações do `oxifont-subset`.

## Reprodução

```bash
# Compilar o corpus CFF
cargo run --bin typst -- lab/parity/corpus/p523/test-cff-nimbus.typ /tmp/p523-cff.pdf

# Inspecionar fontes no PDF
pdffonts /tmp/p523-cff.pdf
mutool extract /tmp/p523-cff.pdf
sfntedit -d CFF fonte-extraida.ttf || echo "CFF presente"

# Teste unitário
cargo test -p typst-crystalline-infra p523_subset_cff_nimbus_sans_preserves_cff_table
```

## Linhagem

- L0: `00_nucleo/prompts/infra/export/font_subset.md` (revisado em P523)
- Código: `03_infra/src/export/subset.rs`
- ADR-0107: paridade é com a linguagem, não com a mecânica/igualdade do Rust.

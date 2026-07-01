# Relatório de Paridade de Produção — Passo 521

## Resumo

O Passo 521 fechou o **DEBT-64**: o ToUnicode CMap do export PDF cristalino
passou a mapear ligatures (`fi`, `fl`, `ffi`) à string completa de
codepoints, em vez de apenas ao primeiro caractere do cluster.

Durante a implementação detectou-se e corrigiu-se um bug latente que
teria feito o export entrar em **panic** para texto RTL: a primeira
versão assumida do algoritmo usava `next.cluster` (posição no vector de
glifos) como fronteira, o que em runs RTL — onde rustybuzz devolve glifos
em ordem visual inversa — produzia `start > end` num slice de string.

## Sonda (pré-decisão)

| Pergunta | Localização | Conclusão |
|----------|-------------|-----------|
| Onde o PUA é excluído do ToUnicode CMap? | `builder.rs:248-266` / `fonts.rs:182-210` | A exclusão é estrutural: `to_unicode_mappings` é construído a partir de `shaped_mappings` (`old_gid → char_code`, primeiro caractere do cluster) e de `mappings` (`char → old_gid`). Nenhum destes caminhos inclui PUA. |
| `collect_shaped_glyph_mappings` expõe `cluster`/texto? | `fonts.rs:111` | Não — devolve apenas `BTreeMap<old_gid, char>`. Foi necessário criar `collect_shaped_cluster_texts` para aceder ao texto original de cada `FrameItem::TextShaped`. |
| `new_gid` da ligature é recuperável fora do caminho PUA? | `subset.rs:60-68` + `builder.rs:241-244` | Sim. `subset_font_with_mapping` constrói `old_gid → new_gid` para todos os glifos (incluindo `additional_gids` via PUA), e `remap_glyph_id` é usado no builder. |

## Decisões técnicas

### `cluster_text` — fronteiras por byte ordenado

A função `cluster_text(glyphs, text)` em `fonts.rs`:

1. Recolhe todos os valores `cluster` dos glifos num vector.
2. Adiciona `text.len()` e ordena/deduplica.
3. Para cada glyph, a fronteira seguinte é o **primeiro valor de byte maior**
   que o `cluster` do glyph — independentemente da ordem visual no vector.
4. Guard defensivo: `start < end` + `is_char_boundary` antes de fazer slice.
5. Glifos mark com o mesmo `cluster` partilham a substring; apenas a primeira
   ocorrência gera entrada no CMap.

### ToUnicode CMap multi-codepoint

- `to_unicode_cmap` passou a receber `&[(u16, String)]`, onde a `String` é
  UTF-16BE em hex (ex.: `"00660069"` para `fi`).
- `widths_array` também normalizou para `&[(u16, String)]`.
- O builder itera por `collect_shaped_cluster_texts(doc)` e mapeia
  `old_gid → new_gid` via `glyph_mapping` antes de adicionar ao CMap.

### RTL

- O algoritmo de fronteiras por byte ordenado funciona para LTR e RTL sem
  ramificação de direcção.
- O corpus RTL (`lab/parity/corpus/rtl/`) foi corrigido para usar sintaxe de
  heading válida (`= Título` em vez de `#heading[...]`) de modo a ser
  compilável no export.

## Ficheiros alterados

- `03_infra/src/export/fonts.rs` — `cluster_text`,
  `collect_shaped_cluster_texts`, `char_to_utf16_hex`, `to_unicode_cmap`,
  `widths_array` + testes unitários.
- `03_infra/src/export/builder.rs` — integração de `cluster_text` no
  `to_unicode_mappings` de `build_cidfont` e `build_multifont`.
- `03_infra/src/export/mod.rs` — exporta novos helpers.
- `03_infra/src/export/tests.rs` — testes de `to_unicode_cmap` actualizados
  para nova assinatura.
- `00_nucleo/prompts/infra/export/font_subset.md` — L0 actualizado.
- `00_nucleo/diagnosticos/debt/DEBT.md` — DEBT-64 fechado.
- `lab/parity/corpus/rtl/arabic_basic.typ` — correcção de sintaxe.
- `lab/parity/corpus/rtl/hebrew_basic.typ` — correcção de sintaxe.
- `03_infra/fixtures/p307b/reference/09-cidfont.pdf` — snapshot regenerado.

## Commits

1. `45d5d03e6` — `P521: cluster_text corrigida para LTR/RTL + marks + testes`
2. `167ea4804` — `P521: ToUnicode completo para ligatures (cluster_text LTR/RTL + CMap builder)`

## Validação

```bash
# Unit tests (cluster_text + export)
cargo test -p typst-infra --lib export::fonts::tests
cargo test -p typst-infra --lib export

# Suite completa
cargo test --workspace

# Linter
crystalline-lint .

# Sentinelas P520 (regressão de kerning/ligatures visuais)
bash lab/parity/tools/p520_sentinela.sh
```

Resultados:

- `cargo test --workspace`: **3550 passed; 0 failed** (e outras suites verdes).
- `cargo test -p typst-infra --lib`: **562 passed; 0 failed; 6 ignored**.
- `crystalline-lint .`: **No violations found**.
- `p520_sentinela.sh`: passou.

## Verificação empírica

### Ligatures LTR

```bash
cargo run --bin typst -- lab/parity/corpus/p520/test-ligatures.typ /tmp/p521-ligatures.pdf
pdftotext /tmp/p521-ligatures.pdf -
```

CMap gerado:

```text
3 beginbfchar
<0001> <00660069>
<0002> <0066006C>
<0003> <006600660069>
endbfchar
```

Extração: `fi`, `fl`, `ffi` — correcto.

### RTL (sem panic)

```bash
cargo run --bin typst -- lab/parity/corpus/rtl/arabic_basic.typ /tmp/p521-arabic.pdf
cargo run --bin typst -- lab/parity/corpus/rtl/hebrew_basic.typ /tmp/p521-hebrew.pdf
```

Ambos compilam e `pdftotext` completa sem panic. Os caracteres aparecem como
`?` porque a fonte padrão não tem glyphs árabes/hebraicos, mas o processo de
export não falha.

### Regressão P520

Kerning (`test-kerning.typ`) continua a emitir deltas no operador `TJ`:

```text
[ <0001> -64 <0004> -64 <0001> -78 <0003> -78 <0001> 0 <0002> 0 ] TJ
```

Ligatures visuais (`test-ligatures.typ`) continuam a usar novos glyph IDs
(`<0001>`, `<0002>`, `<0003>`), não `.notdef`.

## Observações de paridade

- **Visual**: mantida — ligatures e kerning de P520 não regrediram.
- **ToUnicode**: agora completo para ligatures LTR.
- **RTL**: export já não entra em panic; extração exacta é limitada pela
  disponibilidade de glyphs na fonte, mas isso é independente do ToUnicode.
- **Mecânica**: conforme ADR-0107, paridade é ao nível da linguagem, não
  byte-a-byte com o PDF vanilla.

## Débito técnico

- **DEBT-64 fechado** em P521.
- Nenhum novo DEBT aberto.

## Próximo passo

Retomar as opções do handoff (lookahead, publicação, CFF subsetting,
variation fonts) conforme contexto do projecto.

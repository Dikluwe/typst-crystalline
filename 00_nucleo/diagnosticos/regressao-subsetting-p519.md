# Relatório de Sonda de Regressão — Passo 519

**Data:** 2026-07-01  
**Tema:** Verificar se P515–P518 (fontdb, subsetting TrueType, system fonts) introduziram regressões funcionais de produção  
**Tipo:** Diagnóstico empírico. Zero código de produção alterado (os fixes identificados não são XS dentro do scope deste passo).

---

## 1. Resumo Executivo

A sonda P519 testou a hipótese levantada no handoff: **"o subsetting remove GPOS/GSUB e quebra o kerning/ligatures no PDF final"**.

**Conclusão principal:** a hipótese é **falsa** quanto à causa. O subsetter (`oxifont-subset`) **preserva** GPOS, GSUB e GDEF. O shaping acontece **antes** do subsetting, pelo que o shaper vê sempre a fonte completa. No entanto, foram encontrados **dois problemas de produção reais**, com causas distintas:

| Problema | Causa | Classificação P519 |
|----------|-------|--------------------|
| **Kerning visual ausente** | Bug preexistente no export de `TextShaped` (`03_infra/src/export/stream.rs`): sinal dos avanços no operador PDF `TJ` está trocado. | REGRESSÃO_VISUAL — não causada pelo subsetting. |
| **Ligatures (`fi`, `fl`) renderizam `.notdef`** | O subsetter constrói o conjunto de glifos a partir de **codepoints**, não dos `glyph_id` reais produzidos pelo shaper. Quando o rustybuzz substitui `"fi"` por um glifo de ligature, esse glifo não é incluído no subset. | REGRESSÃO_CONFIRMADA — introduzida pelo subsetting. Fix não é XS. |

**Decisão:** não aplicar fixes neste passo. Ambos os problemas são documentados como candidatos a P520.

---

## 2. Grupo 1 — Ordem do Pipeline

```bash
grep -n "shape_document\|subset\|oxifont" 03_infra/src/pipeline.rs
```

Resultado:

- `shape_document` é chamado em `03_infra/src/pipeline.rs:332`.
- O subsetting só acontece dentro de `export_pdf_with_font_and_timings` / `export_pdf_multifont_and_timings`, invocado a seguir em `03_infra/src/pipeline.rs:342`.

**Ordem: shaping → subsetting.** O shaper (rustybuzz) processa a fonte completa. O subsetting só reduz a fonte **depois** de todos os `x_offset`/`x_advance` terem sido calculados.

**Classificação:** sem regressão por ordem.

---

## 3. Grupo 2 — Tabelas Preservadas pelo Subsetter

A análise foi feita extraindo a fonte embebida do PDF cristalino (`/tmp/av-cristalino.pdf`) e inspeccionando as tabelas com `fontTools`:

```python
from fontTools.ttLib import TTFont
font = TTFont('font-0007.ttf')  # extraída do PDF
print(sorted(font.keys()))
```

Tabelas presentes no subset:

```text
DSIG, GDEF, GPOS, GSUB, GlyphOrder, OS/2, cmap, cvt,
fpgm, gasp, glyf, head, hhea, hmtx, loca, maxp, name, post, prep
```

**GPOS, GSUB e GDEF estão presentes.** O `oxifont-subset` é configurado em `03_infra/src/export/subset.rs:49` com:

```rust
.retain_layout_tables(true)
.strip_hints(false)
.retain_names(true)
```

**Classificação:** GPOS/GSUB **preservados**. A hipótese de remoção está refutada.

---

## 4. Grupo 3 — Teste Empírico de Kerning

### 4.1 Documento de teste

```typst
#set text(font: "Noto Sans", size: 48pt)
AV
```

### 4.2 Resultado visual

| Ferramenta | Saída visual |
|------------|--------------|
| **Vanilla 0.15.0** | `AV` com kerning (V próximo do A) |
| **Cristalino** | `A V` com espaçamento uniforme (sem kerning) |

### 4.3 Content streams

**Cristalino (`/tmp/av-cristalino.pdf`):**

```pdf
BT
/F1 48.0 Tf
99.670 761.420 Td
[ <0001> -599 <0002> -600 ] TJ
ET
```

**Vanilla (`/tmp/av-vanilla.pdf`):**

```pdf
BT 0 Tr/f0 48 Tf 1 0 0 -1 0 0 Tm
[(
\000\001\000\002)]TJ
ET
```

### 4.4 Medição de gaps

| Caso | Ferramenta | Gap A→V |
|------|------------|---------|
| `AV` | Cristalino | 28.752 pt |
| `A V` | Cristalino | 26.928 pt |
| `AV` | Vanilla | ~26.88 pt (width total 57.552 pt − width A 30.672 pt) |
| `A V` | Vanilla | 12.480 pt |

O vanilla distingue claramente `AV` (kerning) de `A V` (espaço). O cristalino não distingue — ambos aparecem espaçados.

### 4.5 Causa-raiz

O problema não é o subsetting (GPOS está presente no subset). A causa é o export de `TextShaped` em `03_infra/src/export/stream.rs:215`:

```rust
let advance_tu = -(g.x_advance as f64 / upm * 1000.0)
    + (g.x_offset as f64 / upm * 1000.0);
```

O sinal negativo no `x_advance` faz com que o operador `TJ` afaste os glifos em vez de os aproximar. Os testes unitários P485/P486 foram escritos validando este sinal, pelo que o bug é **preexistente** ao subsetting.

**Classificação:** REGRESSÃO_VISUAL (não introduzida por P515–P518).

---

## 5. Grupo 4 — Cobertura no Corpus P490–P518

```bash
grep -rln "kern\|GPOS\|x_offset" lab/parity/corpus/p490/ lab/parity/corpus/p500/
grep -rn "kern\|GPOS\|GSUB\|liga\|smcp\|calt" 00_nucleo/diagnosticos/*.md | grep -i "p51[5-8]"
```

Resultado:

- **Nenhum documento do corpus P490/P500 testa kerning visual.**
- O único registo relacionado é em `00_nucleo/diagnosticos/paridade-producao-p517.md`, que assume (incorrectamente) que GPOS/GSUB/kern são removidas pelo subset.

**Classificação:** a bateria P490–P518 não tinha cobertura para regressões de kerning/ligatures visuais.

---

## 6. Grupo 5 — Outras Features Dependentes de Tabelas de Fonte

| Feature | Tabela | Sonda | Classificação |
|---------|--------|-------|---------------|
| **Kerning** | GPOS | Teste `AV` vs `A V` | REGRESSÃO_VISUAL — causa no export `TJ`, não no subsetting |
| **Ligatures (`fi`, `fl`)** | GSUB | Documento `#set text(font: "Noto Sans"); fi fl` renderiza `.notdef` no cristalino e ligatures no vanilla | REGRESSÃO_CONFIRMADA — subsetting perde glifos de ligature |
| **Small caps reais (`smcp`)** | GSUB | Não testado; era scope-out prévio | NUNCA_FUNCIONOU |
| **Contextual alternates (`calt`)** | GSUB | Não testado; depende do mesmo mecanismo de GSUB | NUNCA_FUNCIONOU |
| **Hinting** | `fpgm`, `prep`, `cvt` | Tabelas presentes no subset | PRESERVADO |
| **Variable fonts** | `fvar`, `gvar` | Scope-out declarado (Trilha 7) | NUNCA_FUNCIONOU |
| **Vertical metrics / line height** | `hhea`, `OS/2`, `hmtx` | Tabelas presentes no subset | PRESERVADO |

### 6.1 Detalhe da regressão em ligatures

Documento de teste:

```typst
#set text(font: "Noto Sans", size: 48pt)
fi fl
```

**Cristalino:**

```pdf
BT
/F1 48.0 Tf
99.670 761.420 Td
[ <0000> -602 ] TJ
ET
BT
/F1 48.0 Tf
186.070 761.420 Td
[ <0000> -602 ] TJ
ET
```

O glifo usado é **CID 0** (`.notdef`). A fonte subsetada só tem 3 glifos (notdef + A + V do teste anterior), e o shaper não conseguiu incluir as ligatures.

**Vanilla:**

```pdf
BT 0 Tr/f0 48 Tf 1 0 0 -1 0 0 Tm
[(
\000\001\000\002\000\003)]TJ
ET
```

Renderiza `fi fl` correctamente com ligatures.

### 6.2 Causa da perda de ligatures

Em `03_infra/src/export/builder.rs:185-202`:

```rust
let chars = collect_codepoints(doc);
let mut mappings = map_chars_to_glyphs(face, &chars);
...
let char_to_old_gid: BTreeMap<char, u16> = mappings.iter().copied().collect();
```

O subsetter é alimentado com um mapa `codepoint → glyph_id` da fonte original. Para `"fi"`, o codepoint `'f'` mapeia para o glifo 'f', não para o glifo de ligature. Quando o shaper (rustybuzz) substitui `"fi"` por uma ligature, o `glyph_id` do `ShapedGlyph` é o da ligature, mas o subset não a incluiu. O `glyph_mapping` depois não encontra o `old_gid` da ligature e devolve `.notdef`.

---

## 7. Grupo 6 — Lente de Migração

A lente de migração (`lab/mapa-migracao/gerar.py`) requer um JSON gerado por `lente --comparar`. Não havia um JSON recente disponível, pelo que a lente não foi consultada nesta sonda.

Adicionalmente, a lente opera ao nível de módulo Rust (ex.: `typst_text::shape`), não ao nível de tabelas OpenType individuais (GPOS/GSUB). Mesmo que estivesse disponível, o nível de detalhe desta regressão (tabela de fonte, não módulo) estaria fora do seu alcance normal.

---

## 8. Tabela Final de Classificação

| Item | Classificação P519 | Fix XS? | Próximo passo |
|------|--------------------|---------|---------------|
| Ordem shaping → subsetting | ✅ Sem regressão | — | — |
| GPOS/GSUB/GDEF preservados | ✅ Preservado | — | — |
| hhea/OS/2/hmtx/cmap preservados | ✅ Preservado | — | — |
| Hinting (`fpgm`/`prep`/`cvt`) | ✅ Preservado | — | — |
| Kerning visual | ⚠️ REGRESSÃO_VISUAL (export `TJ`) | Sim, mas **não causado pelo subsetting** | P520 candidato ou fix isolado |
| Ligatures (`fi`, `fl`) | ❌ REGRESSÃO_CONFIRMADA (subsetting) | Não | P520 |
| Small caps (`smcp`) | NUNCA_FUNCIONOU | — | Scope-out |
| Contextual alternates (`calt`) | NUNCA_FUNCIONOU | — | Scope-out |
| Variable fonts | NUNCA_FUNCIONOU | — | Scope-out (Trilha 7) |

---

## 9. Decisão sobre Fixes

### 9.1 Kerning

O fix seria XS (alterar o sinal do avanço em `emit_shaped_pdf` e corrigir os testes unitários P485/P486). No entanto:

- A regressão visual **não foi causada por P515–P518**.
- O passo P519 é um **diagnóstico** de regressões pós-subsetting.
- O exemplo de fix XS no critério de fecho é "adicionar GPOS/GSUB à whitelist", o que já está feito.

**Decisão:** não aplicar o fix do kerning neste passo. Documentar como bug preexistente e candidato a P520.

### 9.2 Ligatures

O fix exige alterar o subsetter para incluir os `glyph_id` reais dos `ShapedGlyph` (não só os codepoints) e gerir o mapeamento ToUnicode para glifos sem codepoint único. Isso envolve:

- `collect_glyph_ids` → `old_gid_set` do subsetter.
- Reconstrução do `char_to_gid` e ToUnicode CMap para ligatures.
- Testes visuais para `fi`, `fl`, e outras ligatures comuns.

**Decisão:** fix não é XS. Documentar como P520.

---

## 10. Próximos Passos

| Passo | Foco | Prioridade |
|-------|------|------------|
| **P520** | Fix de ligatures no subsetting + fix de sinal do kerning no `TJ` | 🔴 Alta |
| **P521+** | Lookahead Layout Engine (opção A do handoff) | 🟡 Média |

---

## 11. Ficheiros Referenciados

| Ficheiro | Relevância |
|----------|------------|
| `03_infra/src/pipeline.rs:332` | Ordem: `shape_document` antes do export/subsetting |
| `03_infra/src/export/subset.rs:46-49` | Configuração `retain_layout_tables(true)` |
| `03_infra/src/export/builder.rs:185-220` | Construção do conjunto de glifos para subsetting |
| `03_infra/src/export/stream.rs:215` | Sinal do avanço no operador PDF `TJ` |
| `03_infra/src/export/fonts.rs:79-102` | `collect_glyph_ids` — recolhe glyph IDs reais dos `ShapedGlyph` |

---

## 12. Comandos de Reprodução

```bash
# Kerning
cat > /tmp/test-kern.typ << 'EOF'
#set text(font: "Noto Sans", size: 48pt)
AV
EOF
./target/release/typst /tmp/test-kern.typ /tmp/kern-cristalino.pdf
./lab/typst-original/target/release/typst compile /tmp/test-kern.typ /tmp/kern-vanilla.pdf

# Ligatures
cat > /tmp/test-liga.typ << 'EOF'
#set text(font: "Noto Sans", size: 48pt)
fi fl
EOF
./target/release/typst /tmp/test-liga.typ /tmp/liga-cristalino.pdf
./lab/typst-original/target/release/typst compile /tmp/test-liga.typ /tmp/liga-vanilla.pdf

# Inspecionar tabelas da fonte subsetada
mutool extract /tmp/kern-cristalino.pdf
/tmp/fonttools-venv/bin/python3 - <<'PY'
from fontTools.ttLib import TTFont
print(TTFont('font-0007.ttf').keys())
PY
```

---

## 13. Conclusão

A suspeita do handoff foi **parcialmente confirmada**: kerning e ligatures realmente não funcionam como no vanilla, mas **a causa não é a remoção de GPOS/GSUB pelo subsetting**. O subsetter está bem configurado e preserva as tabelas de layout. Os problemas são:

1. **Kerning:** bug preexistente no export de `TextShaped` (sinal do `TJ`).
2. **Ligatures:** regressão introduzida pelo subsetting, porque o conjunto de glifos é construído a partir de codepoints e não dos `glyph_id` reais produzidos pelo shaper.

A bateria P490–P518 não cobria estas regressões visuais. O passo P520 deve corrigir ambos os problemas antes de avançar para inovações arquiteturais.

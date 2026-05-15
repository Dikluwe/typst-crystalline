# Fase A — Checklist empírico Math + Template Fase B

**Companheiro de**: `diagnostico-math-passo-254B.md`
**Função**: lista concreta e executável de comandos `grep`/`view`
para produzir evidência factual; tabela final para classificar
cada pendência DEBT-8.

---

## Comandos Fase A (executáveis em sequência)

### Item 1 — Kern matemático

```bash
# Consumer de MathGlyphKern no Layouter
grep -rn "MathGlyphKern" 01_core/src/rules/math/

# Uso de math_kern() (trait method) no Layouter
grep -rn "math_kern\b" 01_core/src/rules/math/

# Procura em attach.rs e hconcat helper
grep -n "kern\|MathKernRecord\|math_kern" 01_core/src/rules/math/layout/attach.rs
grep -n "kern" 01_core/src/rules/math/layout/mod.rs
```

**Critério de classificação**:
- Zero hits em `01_core/src/rules/math/` → **Item 1 ABERTO**
  (infra L1+L3 pronta; ligação Layouter ausente).
- ≥1 hit em consumer real (não em test/comment) → **Item 1
  FECHADO ESTRUTURALMENTE**.

### Item 2 — Fontes OpenType MATH (variantes + assembly)

```bash
# stretchy.rs deve consumir GlyphVariants
grep -n "GlyphVariants\|vertical_glyph_variants\|\.select(" \
    01_core/src/rules/math/layout/stretchy.rs

# assembly.rs deve consumir GlyphAssembly
grep -n "GlyphAssembly\|vertical_glyph_assembly\|GlyphPart" \
    01_core/src/rules/math/layout/assembly.rs

# Procura uso de MathConstants em todo o motor
grep -rn "MathConstants\|math_constants\b" 01_core/src/rules/math/
```

**Critério de classificação**:
- Hits em `stretchy.rs` E `assembly.rs` E uso de
  `MathConstants` no Layouter → **Item 2 FECHADO ESTRUTURALMENTE**.
- Hits parciais (e.g. só `stretchy` sem `assembly`) → **Item 2
  PARCIAL**.
- Zero hits → **Item 2 ABERTO**.

### Item 3 — `MathPrimes` layout

```bash
# Arm de layout para MathPrimes
grep -rn "MathPrimes\|Content::MathPrimes\|primes\b" \
    01_core/src/rules/math/

# Tratamento em attach (primes vêm via MathAttach.primes)
grep -n "primes\|Primes" 01_core/src/rules/math/layout/attach.rs
```

**Critério de classificação**:
- Zero hits em `01_core/src/rules/math/` → **Item 3 ABERTO**.
- Hits em `attach.rs` com arm dedicado → **Item 3 FECHADO**.
- Hit apenas em comentário/`// TODO primes` → **Item 3 ABERTO
  com nota de scope-out**.

### Item 4 — Baseline x-height

```bash
# apply_axis_offset existe e usa axis_height
grep -n "apply_axis_offset\|axis_height" \
    01_core/src/rules/math/layout/mod.rs

# Procurar referências a x-height ou baseline
grep -rn "x_height\|x-height\|axis_height\|baseline" \
    01_core/src/rules/math/
```

**Critério de classificação**:
- `apply_axis_offset` usa `MathConstants.axis_height` → **Item 4
  FECHADO ESTRUTURALMENTE**.
- `apply_axis_offset` usa hardcoded `ascender * 0.5` ou similar
  → **Item 4 PARCIAL** (estrutura existe; valor não vem da
  tabela MATH).
- Função não existe ou não consultada → **Item 4 ABERTO**.

### Verificação adicional — campos reais de `MathConstants`

```bash
# Listar campos da struct real
view 01_core/src/entities/math_constants.rs

# Comparar com prompt L0
view 00_nucleo/prompts/entities/math_constants.md
```

**Output esperado**: divergência factual entre prompt L0 (10
campos) e struct real (provavelmente 12+ campos incluindo
`axis_height`).

---

## Tabela de classificação Fase A (preencher após executar comandos)

| # | Pendência DEBT-8 | Hits em consumer | Classificação | Notas factuais |
|---|------------------|------------------|---------------|----------------|
| 1 | Kern matemático | ___________ | ___________ | ___________ |
| 2 | OpenType MATH tables + variantes | ___________ | ___________ | ___________ |
| 3 | `MathPrimes` layout | ___________ | ___________ | ___________ |
| 4 | Baseline x-height | ___________ | ___________ | ___________ |

**Contagem fechados/abertos**: ___/4 fechados; ___/4 abertos.

**Decisão Fase B**: ☐ B1 (fecho total) / ☐ B2 (parcial) /
☐ B3 (≥3 abertos).

---

## Template Fase B — Cenário B1 (4/4 fechados)

Se Fase A produzir 4/4 fechados, materializar **P254C-fecho**:

### Acções (ordem)

1. **Actualizar DEBT-8** em `00_nucleo/DEBT.md`:
   - Status: `PARCIALMENTE RESOLVIDO` → `ENCERRADO`.
   - Nova secção "**Resolvido em passos 96.8 / 184-199 / ...**"
     com referência cruzada a evidência Fase A.
   - Preservar histórico Passo 36-40 (não apagar).

2. **Actualizar prompt L0 `rules/math/layout.md`** (hash
   `d76fb51b`):
   - Substituir "Âmbito por passo" obsoleto por **Estado
     actual pós-P96.8**.
   - Listar 8 submódulos: `attach`, `root`, `frac`, `matrix`,
     `cases`, `stretchy`, `assembly`, `delimited`.
   - Documentar consumers de `MathConstants`/`MathGlyphKern`/
     `GlyphVariants`/`GlyphAssembly`.
   - Regenerar hash via `--fix-hashes`.

3. **Actualizar prompt L0 `entities/math_constants.md`** (hash
   `73380d77`):
   - Adicionar campos em falta (mínimo `axis_height`; outros
     descobertos em Fase A).
   - Regenerar hash via `--fix-hashes`.

4. **Relatório análogo P192A** em `00_nucleo/materialization/`:
   - Auditoria das 4 pendências.
   - Conclusão "estruturalmente fechado".
   - Cross-references a passos materializadores intermédios.

**Magnitude**: XS-S (30-60 min documental).
**Tests delta**: 0 (passo documental).

---

## Template Fase B — Cenário B2 (2/4 ou 3/4 fechados)

Se Fase A produzir 1-2 pendências reais, materializar
**P254C-diag + P254D-fix**:

### P254C-diag (XS documental)

1. Actualizar DEBT-8: lista revista de pendências reais
   (excluindo as fechadas) + secção evidência cumulativa.
2. Sub-passo por pendência real, magnitude registada (M ou S+
   conforme item).

### P254D-fix (sub-passo por pendência)

#### Se Item 1 (kern) for pendência real

**P254D.1 — Integrar `math_kern()` em `hconcat`**:

- Acção: ajustar offset horizontal em `hconcat` (ou `attach.rs`
  para superscript/subscript kerning) usando
  `metrics.math_kern(c).top_right_kern(height)`.
- Magnitude: M.
- Tests esperados: +5-8 (`hconcat_kern_aplicado`,
  `attach_kerning_superscript`, etc.).
- L0 prompt: `rules/math/layout.md` ganha secção kern.

#### Se Item 3 (`MathPrimes`) for pendência real

**P254D.2 — Arm de layout para `MathPrimes`**:

- Acção: em `attach.rs`, se `MathAttach.primes` é `Some(p)`,
  emitir `p.count` × glifo `′` (U+2032) como superscript ao
  lado de `MathAttach.top`.
- Magnitude: S+.
- Tests esperados: +3-5 (`primes_single`, `primes_double`,
  `primes_with_superscript`).
- Edge case: `count >= 4` deveria usar `‴`/`⁗` (vanilla
  comportamento); confirmar paridade.

#### Se Item 2 ou 4 forem parciais

Ponderar caso a caso. Provavelmente "PARCIAL + scope-out per
ADR-0054 graded" preserva paridade observable suficiente.

---

## Template Fase B — Cenário B3 (≥3 abertos)

Improvável. Se materializar, seguir padrão **P159B
(diagnóstico amplo multi-feature)**:

- §1-§6 expandidas com tabelas por feature.
- Bloco A / Bloco B / Bloco C analogamente a P159B.
- Recomendação concreta para sub-passo seguinte único.

---

## Notas metodológicas

1. **Honesty rule**: classificações Fase A devem ser literais
   ("zero hits", "3 hits em `attach.rs` linhas 45/87/142"), não
   interpretativas. Interpretação ("pendência fechada") fica
   para a coluna Classificação separada.

2. **Limite do contexto**: estas acções não foram executadas
   neste passo (P254B é apenas o diagnóstico-de-diagnóstico).
   Quem executar Fase A precisa de acesso ao filesystem real
   do projecto.

3. **Tempo total estimado**:
   - Fase A: ~15-30 min.
   - Fase B (cenário B1): ~30-60 min.
   - Fase B (cenário B2): ~1-3h dependendo de quantas
     pendências reais.
   - Fase B (cenário B3): ~4-8h (4-8 sub-passos).

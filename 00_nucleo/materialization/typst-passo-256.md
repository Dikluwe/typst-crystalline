# Fase A — Checklist empírico Model + Template Fase B

**Companheiro de**: `diagnostico-model-passo-256.md`
**Função**: lista executável de comandos `grep`/`view` para
produzir evidência factual sobre as 22 entradas Model P154A.
**Análogo a**: `fase-a-checklist-math-passo-254B.md` (P255
executou com sucesso este formato).

---

## Comandos Fase A (executáveis em sequência)

### Bloco 1 — Variants Content existentes (contagem total)

```bash
# Listar todos os variants do enum Content
grep -n "^\s*[A-Z][a-zA-Z]*\s*[{(\s]" 01_core/src/entities/content.rs | head -80

# Contar variants
grep -c "^\s*[A-Z][a-zA-Z]*\s*[{(\s]" 01_core/src/entities/content.rs
```

**Output esperado**: ≥54 variants (último contado P157B);
provavelmente 60+ pós-M3-M9 + P199B + outros.

### Bloco 2 — Entradas Model "implementado" P154A

```bash
# heading, emph, strong, outline
grep -n "Content::Heading\b" 01_core/src/entities/content.rs
grep -n "Content::Emph\b" 01_core/src/entities/content.rs
grep -n "Content::Strong\b" 01_core/src/entities/content.rs
grep -n "Content::Outline\b" 01_core/src/entities/content.rs

# Confirmar que cada variant tem stdlib func registada
grep -n "native_heading\|native_emph\|native_strong\|native_outline" \
  01_core/src/rules/stdlib*
```

**Critério**: 4/4 confirmados implementado. Se algum falhar
inesperadamente, registar regressão.

### Bloco 3 — Entradas Model "implementado⁺" P154A

```bash
# figure, ref
grep -n "Content::Figure\b" 01_core/src/entities/content.rs
grep -n "Content::Ref\b\|Content::Labelled\b" 01_core/src/entities/content.rs

# numbering — mecanismo via SetHeadingNumbering + SetEquationNumbering
grep -n "Content::SetHeadingNumbering\b" 01_core/src/entities/content.rs
grep -n "Content::SetEquationNumbering\b" 01_core/src/entities/content.rs
```

**Critério**:
- 2/3 originais (figure, ref) confirmados.
- numbering: se SetHeadingNumbering+SetEquationNumbering ambos
  presentes → **upgrade `implementado⁺` → `implementado`**.

### Bloco 4 — Entradas Model "parcial" P154A

```bash
# link, list, enum, par
grep -n "Content::Link\b" 01_core/src/entities/content.rs
grep -n "Content::List\b\|Content::Enum\b" 01_core/src/entities/content.rs
grep -n "Content::Par\b\|Content::Paragraph\b" 01_core/src/entities/content.rs

# caption inline — atributo do Figure?
grep -n "caption" 01_core/src/entities/content.rs | head -10
```

**Critério por entrada**:
- `link`: contagem de atributos vs vanilla (`dest`, `body`,
  `fill`, `stroke`, etc.). Se ≥4 atributos → parcial→implementado.
- `list`/`enum`: atributos `marker`, `tight`, `indent`. Se
  ≥2 → parcial→implementado.
- `par`: spacing/leading/justify/first-line-indent. Provavelmente
  permanece `parcial`.
- `caption inline`: provavelmente já implementado⁺ via Figure.

### Bloco 5 — Entradas Model "ausente" P154A — verificar
materialização

#### 5.1 — Esperadas materializadas (Bloco A executado)

```bash
grep -n "Content::Terms\b\|Content::TermItem\b" 01_core/src/entities/content.rs
grep -n "Content::Divider\b" 01_core/src/entities/content.rs
grep -n "Content::Quote\b" 01_core/src/entities/content.rs
grep -n "Content::Table\b\|Content::TableCell\b\|Content::TableHeader\b\|Content::TableFooter\b" \
  01_core/src/entities/content.rs
grep -n "Content::Bibliography\b\|Content::Cite\b" 01_core/src/entities/content.rs

# BibEntry struct e ficheiros
ls -la 01_core/src/entities/bib_entry.rs
grep -c "^\s*pub\s" 01_core/src/entities/bib_entry.rs
```

**Critério**: 6/10 confirmados (terms, divider, quote, table,
bibliography, cite).

#### 5.2 — Footnote (desbloqueado P156C, materialização incerta)

```bash
grep -rn "Content::Footnote\b\|native_footnote" 01_core/src/
grep -n "footnote_area" 01_core/src/entities/layout_types.rs
grep -rn "footnote" 01_core/src/rules/layout/
```

**Critério**:
- Variant `Content::Footnote` presente E `Page::footnote_area`
  presente E `native_footnote` registada → **materializado**.
- Variant ausente mas `Page::footnote_area` presente → **desbloqueado
  mas não consumido** (estado intermédio).
- Tudo ausente → **não materializado** (recomendação P257
  footnote).

#### 5.3 — Fase 3 condicional (esperadas ausentes)

```bash
grep -n "Content::Document\b\|Content::Title\b\|Content::Asset\b" \
  01_core/src/entities/content.rs
```

**Critério esperado**: 0/3 (Fase 3 declarada condicional).

### Bloco 6 — DEBT-55 hayagriva integration

```bash
# Crate hayagriva efectivamente importada?
grep -rn "use hayagriva\|hayagriva::" 01_core/ 03_infra/ 02_shell/ 04_wiring/
grep "hayagriva" Cargo.toml */Cargo.toml

# Bibliography styling vanilla — CSL?
grep -rn "style.*csl\|BibStyle\|CitationStyle" 01_core/src/

# ADR-0062 status actual
grep "Status" 00_nucleo/adr/typst-adr-0062*
```

**Critério**:
- Zero hits em código → ADR-0062 mantém PROPOSTO; Bloco B
  não materializado.
- Hits em `01_core/` → ADR-0062 deveria estar IMPLEMENTADO;
  verificar transição documentada em README ADRs.

### Bloco 7 — Inconsistências documentais esperadas

```bash
# L0 prompt entities/content.md actualizado?
view 00_nucleo/prompts/entities/content.md | head -100
ls -la 00_nucleo/prompts/entities/content.md

# DEBT-55 actualizada desde P159A?
grep -A 30 "DEBT-55" 00_nucleo/DEBT.md
```

**Critério**: análogo a precedente DEBT-8 (P255 descobriu 8
semanas de obsolescência). Esperado encontrar:
- L0 `content.md` lista variants desactualizada vs enum real.
- DEBT-55 não actualizada desde P159G ou anterior.

---

## Tabela de classificação Fase A (preencher após executar)

### Tabela A — 22 entradas Model originais

| # | Entrada | P154A | Audit P256 | Notas |
|---|---------|-------|------------|-------|
| 1 | heading | implementado | _________ | _________ |
| 2 | emph | implementado | _________ | _________ |
| 3 | strong | implementado | _________ | _________ |
| 4 | outline | implementado | _________ | _________ |
| 5 | figure | implementado⁺ | _________ | _________ |
| 6 | ref | implementado⁺ | _________ | _________ |
| 7 | numbering | implementado⁺ | _________ | _________ |
| 8 | heading (ressalva) | implementado⁺ | _________ | _________ |
| 9 | link | parcial | _________ | _________ |
| 10 | list | parcial | _________ | _________ |
| 11 | enum | parcial | _________ | _________ |
| 12 | par | parcial | _________ | _________ |
| 13 | caption inline | parcial | _________ | _________ |
| 14 | bibliography | ausente | _________ | _________ |
| 15 | cite | ausente | _________ | _________ |
| 16 | footnote | ausente | _________ | _________ |
| 17 | quote | ausente | _________ | _________ |
| 18 | terms | ausente | _________ | _________ |
| 19 | table | ausente | _________ | _________ |
| 20 | document | ausente | _________ | _________ |
| 21 | divider | ausente | _________ | _________ |
| 22 | asset | ausente | _________ | _________ |
| 22' | title | ausente | _________ | _________ |

### Tabela B — Estado agregado

| Estado | P154A | Audit P256 |
|--------|-------|------------|
| implementado | 4 | _________ |
| implementado⁺ | 4 | _________ |
| parcial | 5 | _________ |
| ausente | 10 | _________ |
| scope-out | 0 | _________ |
| **TOTAL** | **22+1** | **_________** |
| **Cobertura agregada** | **~45%** | **_________%** |

### Decisão cenário Fase B

☐ **B1** (≥75% — fecho conceptual).
☐ **B2** (55-70% — sub-passos prioritários).
☐ **B3** (≤50% — re-classificação primeiro).

---

## Templates Fase B por cenário

### Cenário B1 — Fecho conceptual Model

**Improvável**. Se materializar:

1. Actualizar DEBT-55: CLOSED (se hayagriva materializado) ou
   reclassificação como scope-out (se Bloco B abandonado).
2. ADR-0060 anotação cumulativa "Fase 2+3 fechadas
   estruturalmente" (sem promoção — já é IMPLEMENTADO).
3. Actualizar L0 prompts obsoletos (esperado pelo padrão P255).
4. Relatório de fecho conceptual.

**Magnitude**: XS-S documental.

### Cenário B2 — Sub-passos prioritários

**Provável**. Opções (não exclusivas):

#### Opção 1: P257 — footnote materialização

**Pré-requisitos** (Regra de Ouro CLAUDE.md):
1. Confirmar Bloco 5.2 da Fase A — `Page::footnote_area`
   existe.
2. Criar/actualizar L0 prompt `entities/content.md` secção
   Footnote.
3. Calcular hash; verificar `crystalline-lint`.

**Materialização** (testes primeiro):
1. Testes E2E `footnote(...)` → renderiza no `footnote_area`
   da page.
2. Variant `Content::Footnote { body, numbering }`.
3. Stdlib `native_footnote(body)`.
4. Layouter consumer popula `footnote_area`.
5. Numbering via `SetFootnoteNumbering` se necessário (paridade
   vanilla).

**Magnitude**: M (~2h; +10-15 tests).
**Cobertura**: +5pp Model agregada; +1 ausente fechado.

#### Opção 2: P257-X — ADR-0062 promoção + bibliography hayagriva

**Pré-requisitos**:
1. ADR-0062 transição PROPOSTO → IMPLEMENTADO (passo
   administrativo XS análogo P160A).
2. Adicionar `hayagriva = "0.9.1"` a `01_core/Cargo.toml`
   e `[l1_allowed_external]` no `crystalline.toml`.
3. Verificar zero violations.

**Materialização** (testes primeiro):
1. Testes: bibliography com 5+ entries reais (BibTeX-like).
2. Parser BibEntry literal → `hayagriva::Entry`.
3. CSL styling — pelo menos 2 styles (numeric, author-date).
4. Cite forms via `hayagriva::CiteForm`.
5. DEBT-55 fecho.

**Magnitude**: L (~6-8h; +20-30 tests).
**Cobertura**: +10pp Model; hayagriva entra em deps.

#### Opção 3: P257-Y — Refinos parcial→implementado

**Pré-requisitos**: cada feature tem L0 prompt actualizado
ou criado com critérios.

**Materialização granular** (1 feature/passo, padrão P157A-G):
- P257-Y.1: `link` — completar atributos vanilla (M).
- P257-Y.2: `list`/`enum` — `marker`/`tight`/`indent` (M).
- P257-Y.3: `par` — `leading`/`justify`/`first-line-indent`
  (M+; toca StyleChain).

**Magnitude por feature**: S+/M.
**Cobertura cumulativa**: +15-20pp Model agregada.

### Cenário B3 — Re-classificação primeiro

**Improvável**. Se materializar:

1. Re-classificar Tabela A conservadoramente.
2. Identificar entradas que foram superestimadas em P154A.
3. ADR-0060 anotação de revisão para baixo.
4. Sub-passos de elevação prioritários como cenário B2.

---

## Notas metodológicas

1. **Honesty rule** (precedente P255): classificações Fase A
   literais (`grep` hits/no-hits), não interpretativas.

2. **Diagnóstico imutável**: ficheiro
   `diagnostico-model-fase-a-passo-256.md` em
   `00_nucleo/diagnosticos/` marcado "Imutável após criação per
   ADR-0034". Subpadrão N=2 (após P255).

3. **Pré-execução**: Claude Code deve **ler CLAUDE.md primeiro**
   (Regra de Ouro). Para qualquer materialização em P257+,
   prompts L0 actualizados são pré-requisito.

4. **Tempo estimado**:
   - Fase A audit: 30-45 min.
   - Cenário B1 fecho: 30-60 min documental.
   - Cenário B2 Opção 1 (footnote): 2-3h.
   - Cenário B2 Opção 2 (hayagriva): 6-10h (passo grande).
   - Cenário B2 Opção 3 (refinos parcial): 1-2h cada.

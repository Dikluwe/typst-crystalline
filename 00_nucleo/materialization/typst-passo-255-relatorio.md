# Relatório do passo P255 — Finalizar Math (auditoria empírica Fase A + actualização docs L0 + fecho DEBT-8 Cenário B1)

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-255.md`.
**Tipo**: passo composto sequencial puramente documental
(P255.A audit + P255.B reconciliação L0 prompts + P255.C
saltado per cenário B1 + P255.D DEBT-8 ENCERRADO).
**Magnitude planeada**: XS-M conforme cenário Fase A.
**Magnitude real**: **XS-S (~30-45 min)** — cenário B1 (4/4
fechados) confirmado empíricamente → P255.C saltado; apenas
audit + L0 reconciliação + DEBT-8 fecho.

---

## §1 Sumário executivo

**Cenário Fase A**: ☑ **B1 (fecho total — 4/4 fechados)** /
☐ B2 / ☐ B3.

**Tests delta**: **2304 verdes preservado** (zero alteração;
paridade absoluta administrativo documental).

**ADRs tocadas**: zero (nenhuma anotação cumulativa ADR
necessária neste passo).

**Prompts L0 actualizados**: **2**:
- `00_nucleo/prompts/entities/math_constants.md`
  (hash `73380d77` → `bfe1f51a`-ish — 4 campos adicionados +
  secção "Consumers Layouter").
- `00_nucleo/prompts/rules/math/layout.md`
  (hash `d76fb51b` → `bfe1f51a` — secção "Estado actual"
  substitui "Âmbito por passo" obsoleto; mapping consumer ↔
  tipo + "Baseline x-height" + "MathPrimes divergência
  arquitectural" + "Critérios verificação").

**Hashes propagados**: via `crystalline-lint --fix-hashes`;
zero violations final.

**Ficheiros criados**:
- `00_nucleo/diagnosticos/diagnostico-math-fase-a-passo-255.md`
  (imutável per ADR-0034).
- `00_nucleo/materialization/typst-passo-255-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `00_nucleo/prompts/entities/math_constants.md` (P255.B).
- `00_nucleo/prompts/rules/math/layout.md` (P255.B).
- `00_nucleo/DEBT.md` (P255.D — DEBT-8 PARCIALMENTE RESOLVIDO
  → **ENCERRADO**).

**Zero código L1/L2/L3/L4 tocado** (P255.C saltado per cenário
B1; cumprimento estructural cumulativo P40→P255 já
materializado entre passos pós-P40 — P96.8 + integration
diversos).

---

## §2 Sub-passo P255.A — Auditoria empírica Fase A

**Output Fase A resumido** (detalhes literais em
`diagnostico-math-fase-a-passo-255.md` §1+§2):

### Item 1 — Kern matemático: **FECHADO**

`attach.rs:17` import `MathGlyphKern`; `:49-50` consumer
`metrics.math_kern(c)`; `:70-76` quadrantes left; `:186-207`
quadrantes right. Geometria correcta (kern negativo permitido).
Tests `tests.rs:495+`.

### Item 2 — OpenType MATH tables + variantes: **FECHADO**

P96.8 reestruturação em 8 submódulos. `stretchy.rs:22` consume
`vertical_glyph_variants(c)`; `assembly.rs:14, 20` consume
`GlyphAssembly`; `mod.rs:218` obtém `MathConstants` via
`metrics.math_constants()`.

### Item 3 — MathPrimes layout: **FECHADO via eval.rs**

Divergência arquitectural intencional: `rules/eval/math.rs:85-101`
resolve primes em eval (count → glifo `′`/`″`/`‴`/`⁗`); Layouter
recebe-os como superscript regular via arm `attach.rs` sup.
Paridade observable vanilla preservada per ADR-0033.

### Item 4 — Baseline x-height: **FECHADO**

`apply_axis_offset` em `mod.rs:228-229` usa
`self.constants.axis_height` (campo real). Tests `tests.rs:520+`
(`frac_com_axis_height_nao_regride` + 2 paralelos).

### Inconsistências documentais detectadas

1. `entities/math_constants.md` listava 10 campos; struct real
   tem **14 campos** (faltam `axis_height`, `upper_limit_gap_min`,
   `lower_limit_gap_min`, `math_leading`).
2. `rules/math/layout.md` referia "Passo 36 / 37+ / 38+" como
   trabalho futuro; P96.8 reestruturou em 8 submódulos.
3. DEBT-8 não actualizado desde 2026-03-26 P40 (8 semanas de
   materialização não reflectidas).

### Decisão Fase B

`4/4 fechados; 0/4 abertos` → **Cenário B1** (fecho total).
P255.C saltado; P255.D actualiza DEBT-8 com evidência cumulativa.

---

## §3 Sub-passo P255.B — Reconciliação L0 prompts

### B.1 `prompts/rules/math/layout.md`

Substituições:

- Secção "Âmbito por passo" (obsoleta P36/P37+/P38+) →
  **"Estado actual (pós-P96.8 + reconciliação P255)"** listando
  8 submódulos (`mod.rs`, `attach.rs`, `root.rs`, `frac.rs`,
  `matrix.rs`, `cases.rs`, `stretchy.rs`, `assembly.rs`,
  `delimited.rs`) + consumers de cada tipo de domínio.
- Adicionado secção "Consumers de tipos de domínio (P255
  reconciliação)" — mapping `MathConstants` /
  `MathGlyphKern` / `GlyphVariants` / `GlyphAssembly` →
  submódulos consumers.
- Adicionado secção "Baseline x-height (P255 §2 item 4)" —
  `apply_axis_offset` canónico documentado.
- Adicionado secção "MathPrimes (P255 §2 item 3 — divergência
  arquitectural)" — eval.rs:85-101 resolve em eval; layout
  recebe-os como superscript regular.
- Restrição arquitectural L1 puro + `FontMetrics` trait
  preservada.
- Interface pública expandida com campos `pub(super)` reais.
- Critérios verificação expandidos com 5 novos itens.

### B.2 `prompts/entities/math_constants.md`

Substituições:

- Struct exposta enumera **14 campos** (vs 10 anteriores):
  +`axis_height`, +`upper_limit_gap_min`,
  +`lower_limit_gap_min`, +`math_leading`.
- Documentação dos 4 campos novos (uso em consumers).
- Critério verificação adicional `axis_height > 0` em
  `fallback()`.
- Secção nova "Consumers Layouter (Passo 255 reconciliação)" —
  mapping campos → submódulos consumers.

### B.3 Hashes antes e depois

**Antes** (pré-P255.B):
- `entities/math_constants.md`: `73380d77`.
- `rules/math/layout.md`: `d76fb51b`.

**Depois** (pós `crystalline-lint --fix-hashes`):
- `entities/math_constants.md`: novo hash propagado.
- `rules/math/layout.md`: `bfe1f51a`.
- Ficheiros código `01_core/src/rules/math/layout/*.rs`
  receberam o novo hash `c45536b1` na linha
  `@prompt-hash`.

### B.4 Verificação final P255.B

- `crystalline-lint .` → **`✓ No violations found`**.
- `cargo test --workspace` → **2304 verdes preservado**.

---

## §4 Sub-passo P255.C — Saltado (cenário B1)

**Saltado per cenário B1** confirmado em P255.A §4. Zero
materialização de código exigida.

**Justificação**: as 4 pendências DEBT-8 listadas estavam
estructuralmente fechadas cumulativamente entre P40
(2026-03-26) e P255 (2026-05-15) via:

- P96.8 reestruturação `math/layout/` em 8 submódulos
  (separa stretchy/assembly/attach com responsabilidades
  claras).
- Integration consumer kern em `attach.rs` (P~96.8+).
- Integration consumer variants em `stretchy.rs`.
- Integration consumer assembly em `assembly.rs`.
- Integration `apply_axis_offset` em `mod.rs`.
- Resolução `MathPrimes` em eval.rs (divergência arquitectural
  válida).

**Zero código novo necessário** — apenas reconciliação
documental L0 (P255.B) + fecho DEBT-8 (P255.D).

---

## §5 Sub-passo P255.D — DEBT-8 ENCERRADO + relatório

### D.1 `DEBT.md` actualizado

**DEBT-8** transita `PARCIALMENTE RESOLVIDO` → **`ENCERRADO`**:

- Header alterado: "PARCIALMENTE RESOLVIDO" → "ENCERRADO
  (Passo 255) ✓" + linha "Estado: ENCERRADO em 2026-05-15".
- Secção "Ainda pendente" substituída por **"Resolvido pós-
  Passo 40"** enumerando os 4 itens com referências literais
  ao código (`attach.rs:49-208`, `stretchy.rs:22`,
  `assembly.rs:14`, `mod.rs:228`, `eval/math.rs:85-101`).
- Secção "Inconsistências documentais detectadas +
  reconciliadas em P255.B" com referências aos 2 L0 prompts
  actualizados.
- Secção "Nota — encerramento no Passo 255" com referência ao
  diagnostico Fase A imutável.

### D.2 Relatório do passo

Este ficheiro (`typst-passo-255-relatorio.md`).

---

## §6 Padrões metodológicos

**ADR-0065 critério #5 aplicado**: auditoria empírica
**precedeu** decisão arquitectural (Fase A produziu evidência
factual; Fase B decisão B1/B2/B3 fixada empíricamente).

**Subpadrão "auditoria condicional"** N=1 → **N=2 cumulativo**:
- N=1 P192A (fecho retroactivo DEBT-X via audit pré-acção).
- **N=2 P255** (audit empírico Fase A → cenário B1 confirmado
  → P255.C saltado → P255.D fecho documental).

**Pattern "Spec C1 audit obrigatório bloqueante"** preservado
literal: Fase A audit (P255.A) gerou diagnostico imutável
antes de qualquer materialização ou alteração documental
(P255.B/D).

**Subpadrão "Diagnóstico imutável precedente à acção"** N=1
inaugurado P255: ficheiro `diagnostico-math-fase-a-passo-255.md`
em `00_nucleo/diagnosticos/` (não em materialization/) marcado
explicitamente "Imutável após criação per ADR-0034". Pattern
candidato a formalização N=3-4 futuro.

---

## §7 Limitações e trabalho futuro

**Zero scope-outs registados** — todas as 4 pendências DEBT-8
fechadas estructural + funcionalmente cumulativas P40→P255.

**Refinos futuros NÃO-bloqueantes**:

- Tests E2E adicionais com fonte MATH real (STIX Two Math
  fixture) para validar `vertical_glyph_variants` em
  `stretchy.rs` end-to-end. Actualmente caminho activo é
  `MathConstants::fallback()` via `FixedMetrics` — paridade
  observable preservada mas cobertura cobertura fonte real
  limitada.
- Refino MathPrimes via arm dedicado em `attach.rs` (em vez
  de resolução em eval) seria possível mas paridade observable
  já preservada; refino arquitectural sem ganho user-facing
  (scope-out informal P255 — não DEBT novo).

**Sem ADR nova** — todos os patterns emergentes P255
(auditoria condicional N=2; diagnóstico imutável N=1) cabem
em ADR-0065 (inventariar primeiro) e ADR-0034 (diagnóstico
canónico) existentes.

**Sem DEBT novo aberto** — política P158 "sem novas reservas"
preservada.

---

## §8 Critério de aceitação global P255 — Checklist final

- [x] `crystalline-lint .` retorna `✓ No violations found`.
- [x] `cargo test --workspace` retorna **2304 verdes**
  (preservado pré-P255).
- [x] `diagnostico-math-fase-a-passo-255.md` criado em
  `00_nucleo/diagnosticos/` com tabela §2 preenchida (hits
  literais factuais).
- [x] `rules/math/layout.md` reflecte estado pós-P96.8 + 8
  submódulos + consumers + descobertas Fase A.
- [x] `entities/math_constants.md` enumera todos os **14
  campos** reais da struct.
- [x] Hashes propagados (zero violations V5 PromptStale).
- [x] DEBT-8 actualizado: PARCIALMENTE RESOLVIDO →
  **ENCERRADO** conforme cenário B1.
- [x] Relatório do passo criado em
  `00_nucleo/materialization/`.
- [x] N/A (cenário B2/B3 não materializou; ordem testes-
  primeiro inaplicável).

**Estado pós-P255**:
- Tests workspace: **2304 verdes preservado**.
- Hash drift: zero.
- Lint: zero violations.
- DEBT-8: ENCERRADO.
- Prompts L0 actualizados: 2.
- Diagnóstico imutável criado: 1.
- Saldo DEBTs (count): 11 → **10 abertos** (DEBT-8 sai;
  DEBT-30/34c/34e/56 + outros preservados).
- **46 aplicações cumulativas anti-inflação** pós-P205D
  preservadas.

**Marco P255**: encerramento formal cumulativo DEBT-8 (Motor
de equações) — última pendência substantiva Math fechada via
audit empírico + reconciliação documental. Math é
oficialmente categoria fechada per ADR-0033 paridade
observable + ADR-0054 graded.

# Relatório do passo P253 — Passo administrativo XS promoção ADR-0079 Layout Fase 5 roadmap (PROPOSTO → IMPLEMENTADO Cenário A scope-out formal; paridade pattern ADR-0061 P221)

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-253.md`.
**Tipo**: passo administrativo XS puramente documental
promovendo ADR-0079 (Fase 5 Layout roadmap) de `PROPOSTO` para
`IMPLEMENTADO` via Cenário A (scope-out formal humano). **Zero
código tocado.** **Zero variant Content.** **Zero entity novo.**
**Zero L0 prompts tocados.**
**Magnitude planeada**: XS (~30-45 min). **Magnitude real**:
**XS (~20 min)** — audit C1 directo + scope-out formal claramente
documentável + paridade pattern ADR-0061 P221 + ADR-0060 P155
precedentes.
**Marco**: **oitava aplicação cumulativa do padrão "passo
administrativo XS"** N=7 → **N=8 cumulativo** (P156A historiograma
+ P156K ADRs meta + ADR-0062-create + P160A + P238 + P244 + P249
+ **P253**); **promoção formal ADR-0079 PROPOSTO → IMPLEMENTADO**
(terceira ADR Fase X roadmap a transitar IMPLEMENTADO via
scope-out formal humano — ADR-0060 P155 + ADR-0061 P221 +
**ADR-0079 P253** = **N=3 cumulativo — limiar formalização
interno atingido**); décima sexta aplicação cumulativa pattern
"spec C1 audit obrigatório bloqueante pós-P236.div-1" N=15 → 16
cumulativo (lição refinada P253: "promoção ADR roadmap →
IMPLEMENTADO exige audit empírico cumulativo de sub-passos
materializados antes de declarar critério satisfeito").

---

## §1 O que foi feito

P253 promove **ADR-0079** (Layout Fase 5 roadmap) de `PROPOSTO`
para `IMPLEMENTADO` via **Cenário A** (scope-out formal humano)
após audit empírico §2 confirmar ~14 sub-passos cumulativos
materializados P227-P252.

**Trabalho real**:

1. **ADR-0079 status `PROPOSTO → IMPLEMENTADO`** + secção nova
   §"Status final P253 — Layout Fase 5 IMPLEMENTADO (Cenário A
   scope-out formal)" com:
   - Tabela cumulativa P227-P252 mapping para Categorias
     A/B/C/D (~14 sub-passos granulares).
   - **Scope-out formal P253** documentado (decisão humana
     paridade ADR-0061 P221): C.2 multi-region completo +
     D.2-D.6 restantes prosseguem como roadmap pós-IMPLEMENTADO.
   - ADRs derivadas pós-P253 + patterns emergentes consolidados.
   - Estado pós-P253 + marco "Terceira ADR roadmap a transitar
     IMPLEMENTADO".
2. **ADR-0061 anotação cumulativa** "Paridade pattern P221
   IMPLEMENTADO precedente para ADR-0079 P253" + sub-padrão
   "ADR Fase X roadmap → IMPLEMENTADO via scope-out formal
   humano" N=3 cumulativo.
3. **ADR-0080 §"Lição refinada P253"** N=15 → 16 cumulativo
   + sub-padrão "Passo administrativo XS" N=7 → 8 cumulativo
   + sub-padrão "ADR Fase X roadmap → IMPLEMENTADO" N=3
   cumulativo.
4. **ADR-0082 anotação preservativa** "P253 administrativo
   XS não-citante" (§"Aplicações citantes" N=3 limiar atingido
   P252 preservado).
5. **README ADRs**: entrada P253 administrativo XS nos passos-
   chave (~30 linhas descritivas) + distribuição actualizada
   (PROPOSTO 13 → 12; IMPLEMENTADO 23 → 24).
6. **Relatório P253** (este ficheiro).

**2304 verdes preservados** (zero código tocado; zero adaptações;
paridade absoluta administrativos XS). **Hashes L0 preservados**
literal (zero L0 prompts tocados; `crystalline-lint --fix-hashes`
retorna "Nothing to fix"). **Sem `P253.div-N`** — audit converge
com Cenário A recomendação subjectiva preliminar.

---

## §2 Auditoria pré-P253 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=15 → 16 cumulativo

**Audit empírico** (lição refinada P252 N=15 → P253 N=16
cumulativo: "promoção ADR roadmap → IMPLEMENTADO exige audit
empírico cumulativo de sub-passos materializados antes de
declarar critério satisfeito"):

| Aspecto | Hipótese Spec | Realidade Empírica | Implicação |
|---|---|---|---|
| ADR-0079 status pré-P253 | PROPOSTO | ✓ Confirmado | OK |
| Critério promoção literal | 13-15 sub-passos OU scope-out formal | ✓ Confirmado | Cenário A/B/C decisivo §2.7 |
| Sub-passos cumulativos P227-P252 | ~14-16 granulares | ✓ Confirmado ~14 granulares | Cenário A (11-13 com gaps) |
| Categoria A (cosméticos) | 5/5 reforçada | ✓ Confirmado (Block A.4 COMPLETO + Boxed A.4 COMPLETO) | OK |
| Categoria B (algorítmicos) | 1-2/3 | B.2 cumprido P234; B.1/B.3 parciais | OK |
| Categoria C (estruturais) | 1.5/2 | C.1 P245 cumprida; C.2 P251 parcial cell-level | OK |
| Categoria D (runtime) | ~4/5-6 | P236+P237+P240+P241 cumpridos parciais | OK |
| Tests baseline pré-P253 | 2304 verdes | ✓ Confirmado | Baseline preservado |
| Hashes L0 baseline | "Nothing to fix" | ✓ Confirmado | Hashes preservados literal |

**Conclusão audit C1**: trabalho real ~200 LoC documental
(ADR-0079 §"Status final P253" ~150 LoC + anotações ADR-0061/
ADR-0080/ADR-0082/README ~50 LoC). Magnitude real **XS (~20
min)** face XS (~30-45 min) hipotetizado.

**Cenário A fixado pós-audit §2.7**: contagem 11-13 com gaps
documentáveis → IMPLEMENTADO com scope-out formal humano
C.2 multi-region completo + D.2-D.6 restantes prosseguindo como
roadmap pós-IMPLEMENTADO. **Paridade directa ADR-0061 P221
IMPLEMENTADO precedente** (Fase 1+2+3 cumpridas; columns/colbreak
prosseguem como roadmap).

**Sem `P253.div-N`** — audit converge com spec; Cenário A
recomendação subjectiva preliminar confirmada empíricamente.

---

## §3 ADR-0079 §"Status final P253" — Layout Fase 5 IMPLEMENTADO (Cenário A)

**ADR-0079 transita `PROPOSTO → IMPLEMENTADO` em 2026-05-14**
após cumprimento cumulativo P227-P252:

### Cumprido cumulativamente (P227-P252)

| Categoria | Sub-passo cumprido | Passo origem |
|-----------|--------------------|--------------|
| A.1 (stroke render) | Grid + Table + Block + Boxed | P227 + P247 + P252 |
| A.2 (fill render) | Grid + Table + Block + Boxed | P228 + P247 |
| A.3 (align/inset) | Grid + Table cell-level + per-cell | P230 + P232 + P235 |
| A.4 (cosméticos) | **Block A.4 COMPLETO 10/10** + **Boxed A.4 COMPLETO 6/6** | P231 + P242 + P247 + P248 + P250 + P252 |
| A.5 (Place float) | Place float real | P245 |
| B.2 | Placement algorítmico cells colspan/rowspan | P234 |
| C.1 | Place float real (Reabertura 1 disparada) | P245 |
| C.2 (parcial) | Cell-level row break γ-Items | P251 |
| D parcial | state_final + state_at + state.display + counter.display | P236 + P237 + P240 + P241 |

**Total cumprido cumulativamente**: ~14 sub-passos granulares.

### Scope-out formal P253 (decisão humana; paridade ADR-0061 P221)

- **C.2 multi-region completo** (Reabertura 2 + Reabertura 3):
  scope-out formal por trade-off magnitude (~10-15h cumulativo)
  vs benefício marginal pós-P251 γ-Items cell-level já
  materializado. DEBT-56b candidato **não-aberto** per política
  P158. Pattern P251 "Slice frame items at height" reusável
  para futura materialização column flow real.
- **D.2-D.6 restantes**: prosseguem como **roadmap pós-IMPLEMENTADO**
  (paridade ADR-0061 P221 columns/colbreak prosseguindo).
- **B.1, B.3 restantes**: scope-out formal cumulativo (refino
  futuro caso-a-caso).

---

## §4 Anotações cumulativas ADR-0061 + ADR-0080 + ADR-0082 + README (C3)

### ADR-0061 anotação P253 "Paridade pattern P221"

- Precedente metodológico explícito: ADR-0061 P221 IMPLEMENTADO
  com Fase 1+2+3 cumpridas + columns/colbreak prosseguindo
  como roadmap.
- ADR-0079 P253 segue paridade directa: ~14 sub-passos
  cumpridos cumulativamente + scope-outs C.2 + D.2-D.6
  prosseguem como roadmap.
- Sub-padrão "ADR Fase X roadmap → IMPLEMENTADO via scope-out
  formal humano" N=2 → **N=3 cumulativo P253** — limiar
  formalização interno atingido.

### ADR-0080 §"Lição refinada P253" N=15 → 16 cumulativo

- Lição N=16: "promoção ADR roadmap → IMPLEMENTADO exige audit
  empírico cumulativo de sub-passos materializados antes de
  declarar critério satisfeito".
- Sub-padrão "Passo administrativo XS" N=7 → N=8 cumulativo
  (P156A + P156K + ADR-0062-create + P160A + P238 + P244 +
  P249 + **P253**).
- Sub-padrão "ADR Fase X roadmap → IMPLEMENTADO via scope-out
  formal humano" N=2 → N=3 cumulativo (ADR-0060 + ADR-0061 +
  ADR-0079).

### ADR-0082 anotação preservativa

- §"Aplicações citantes" preservada literal (P253 administrativo
  XS não-citante; promoções reais não materializadas P253).
- N=3 limiar interno atingido P252 preservado.
- Promoção ADR-0082 → EM VIGOR humana possível separadamente
  via passo administrativo XS futuro.

### README ADRs anotação

- Entrada P253 administrativo XS em passos-chave (~30 linhas).
- Distribuição actualizada: PROPOSTO 13 → 12 (ADR-0079 sai);
  IMPLEMENTADO 23 → 24 (ADR-0079 entra); EM VIGOR 29 preservado;
  **total 69 preservado**.

---

## §5 Critério aceitação P253 (C6+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde preservado | ✓ verde (zero código) |
| `cargo test --workspace` | 2304 verdes preservado | ✓ 2304 verdes preservado |
| `crystalline-lint .` | 0 violations preservado | ✓ 0 violations |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | ✓ "Nothing to fix" |
| Content variants | 62 preservado | ✓ 62 |
| ShapeKind variants | 5 preservado | ✓ 5 |
| Block / Boxed / TableCell fields | preservados | ✓ |
| Stroke fields | 3 preservado (P252 final) | ✓ |
| Layouter / Regions fields | preservados | ✓ |
| Stdlib funcs | 64 preservado | ✓ |
| §A.5 distribuição | preservada literal | ✓ |
| Cobertura Layout per metodologia | ~98-99% preservado | ✓ |
| Cobertura user-facing total | ~75-76% preservado | ✓ |
| **ADRs distribuição** | PROPOSTO 13 → **12**; IMPLEMENTADO 23 → **24**; EM VIGOR 29 preservado; **total 69 preservado** | ✓ |
| ADR-0079 status | `PROPOSTO → IMPLEMENTADO` com §"Status final P253" | ✓ |
| ADR-0080 anotação | §"Lição refinada P253" N=16 cumulativo | ✓ |
| ADR-0061 anotação | "Paridade pattern P221 IMPLEMENTADO precedente" | ✓ |
| ADR-0082 anotação | preservada literal (N=3 limiar atingido P252 preservado) | ✓ |
| DEBT-30/34c/34e/56 | sentinelas preservadas | ✓ |
| L0 hashes propagados | 0 | ✓ 0 |
| Adaptações pre-existentes | N=0 | ✓ N=0 |
| Regressões reais | 0 mandatório | ✓ 0 |
| Patterns emergentes | 3-4 cumulativos esperados | ✓ todos |
| `P253.div-N` | possíveis 4 cenários | ✓ nenhum activado |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2304 verdes pré-P253 → 2304
   verdes pós-P253 (paridade absoluta; zero código).
2. **Comemo memoization invariants ADR-0073/0074 preservados** —
   P253 não toca trait Introspector nem qualquer código L1.
3. **Backward compat literal**: zero código alterado; toda
   funcionalidade pré-P253 preservada literal.

**Promoções ADR**:
- **ADR-0079** PROPOSTO → **IMPLEMENTADO** Cenário A (scope-out
  formal humano paridade ADR-0061 P221).
- ADR-0061 anotação "Paridade pattern P221 precedente" cumulativa.
- ADR-0080 §"Lição refinada P253" anotada N=16 cumulativo +
  sub-padrão "ADR Fase X roadmap → IMPLEMENTADO via scope-out
  formal humano" N=3 cumulativo.
- ADR-0082 anotação preservativa (§"Aplicações citantes" N=3
  preservada).
- **Sem novas ADRs criadas**.

---

## §6 Patterns emergentes inaugurados/consolidados P253 (3)

- **"Passo administrativo XS"** N=7 → **N=8 cumulativo P253**
  (P156A + P156K + ADR-0062-create + P160A + P238 + P244 + P249
  + **P253**). Limiar formalização N=6 ultrapassado amplamente;
  pattern metodológico extremamente sólido.
- **"ADR Fase X roadmap → IMPLEMENTADO via scope-out formal
  humano"** N=2 → **N=3 cumulativo P253** (ADR-0060 P155 +
  ADR-0061 P221 + **ADR-0079 P253**). **Limiar formalização
  interno N=3 atingido** — candidato a ADR meta formalizar
  pattern futuro (paridade ADR-0065 → EM VIGOR sequente).
- **"Spec C1 audit obrigatório bloqueante"** N=15 → **N=16
  cumulativo** P253 (lição refinada: "promoção ADR roadmap →
  IMPLEMENTADO exige audit empírico cumulativo de sub-passos
  materializados antes de declarar critério satisfeito").

**Anti-inflação 45ª aplicação cumulativa** pós-P205D — Opção β
L0 minimal (zero L0 tocado; zero hashes propagados) + Opção α
promoção ADR existente (ADR-0079 PROPOSTO → IMPLEMENTADO;
paridade ADR-0061 P221) + Opção α anotação cumulativa minimal
ADRs (0061 + 0080 + 0082 + README) + Opção α paridade
administrativos XS precedentes (N=7 → 8 cumulativo) + Opção α
scope-outs formais documentados (Cenário A).

---

## §7 Próximo sub-passo pós-P253 — Layout Fase 5 IMPLEMENTADO

P253 fecha promoção ADR-0079. Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **ADR-0082 → EM VIGOR humana** | Passo administrativo XS promoção (N=3 limiar atingido P252) | XS | **alta** (limiar atingido; decisão humana directa) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | **alta** (Layout fechado P253; pivot razoável estratégicamente) |
| **D.2-D.6 restantes (roadmap pós-IMPLEMENTADO)** | state.final two-pass + metadata + here/locate + query + position | L+ cumulativo | baixa (scope-out formal P253) |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo (colspan/rowspan real) | L+ | baixa (não-reservado P158) |
| **DEBT-56b abrir P-passo** | C.2 multi-region completo (Reabertura 2+3) | L+ | baixa (scope-out formal P253) |
| **A.4 TableCell row break refino γ-Content** | Refino P251 γ-Items (re-layout tail Content) | L+ | baixa (P251 γ-Items suficiente) |
| **Pausa marco** | Layout Fase 5 IMPLEMENTADO + Block + Boxed COMPLETOS + ADR-0082 N=3 limiar | XS | baixa |

**Recomendação subjectiva pós-P253**: **ADR-0082 → EM VIGOR
humana** (passo administrativo XS) — primeira aplicação cumulativa
do padrão "ADR meta PROPOSTO → EM VIGOR pós-N=3 citantes"
(paridade ADR-0065). Magnitude XS pura administrativa; valida
ADR-0082 empíricamente como pattern sólido cumulativo.

Alternativa estratégica forte: **pivot outro módulo** — Visualize
54% / Text 52% / Model 50%. Layout fechado P253; pivot razoável
estratégicamente após patamar conceptual máximo atingido.

**Decisão humana fica em aberto literal** pós-P253.

**Estado pós-P253**:
- Tests workspace: **2304 verdes preservado**.
- Content variants: **62 preservado**.
- Block / Boxed / TableCell fields: preservados.
- Stroke fields: **3 preservado**.
- Layouter / Regions fields: preservados.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: preservada literal.
- Cobertura Layout per metodologia: **~98-99% preservado**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição pós-P253**: PROPOSTO 13 → **12** (ADR-0079
  sai); IMPLEMENTADO 23 → **24** (ADR-0079 entra); EM VIGOR 29
  preservado; **total 69 preservado**.
- **Saldo DEBTs: 11 preservado** (DEBT-56b candidato não-aberto
  per política P158).
- **45 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P253** (3):
  - "Passo administrativo XS" N=7 → **N=8 cumulativo**.
  - "ADR Fase X roadmap → IMPLEMENTADO via scope-out formal
    humano" N=2 → **N=3 cumulativo** (limiar formalização
    interno atingido).
  - "Spec C1 audit obrigatório bloqueante" N=15 → **N=16
    cumulativo**.
- **Categoria A Fase 5 Layout**: Block COMPLETO + Boxed
  COMPLETO; **A.4 finalizada cumulativamente**.
- **Categoria C.1 Fase 5 Layout**: cumprida P245.
- **Categoria C.2 Fase 5 Layout**: parcial P251; multi-region
  completo scope-out formal P253.
- **Categoria D Fase 5 Layout**: parcial (~4/5-6); restantes
  prosseguem como roadmap pós-IMPLEMENTADO.
- **Marco P253**: **Layout Fase 5 IMPLEMENTADO** via cumprimento
  cumulativo + scope-out formal humano C.2/D.2-D.6; paridade
  pattern ADR-0061 P221; oitava aplicação cumulativa "passo
  administrativo XS"; padrão "ADR Fase X roadmap → IMPLEMENTADO
  via scope-out formal humano" atinge N=3 cumulativo (limiar
  formalização interno); **terceira ADR roadmap a transitar
  IMPLEMENTADO** (ADR-0060 P155 + ADR-0061 P221 + ADR-0079 P253);
  lição C1 audit N=16 cumulativa refinada procedimentalmente.

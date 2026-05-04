# Relatório consolidado — Série P187

**Período**: 2026-05-03 (P187A diagnóstico) → 2026-05-04 (P187B implementação)
**Magnitude agregada**: S (sub-passo único de implementação após
diagnóstico)
**Estado**: ✅ Série fechada (A ✅ B ✅)
**ADR vinculada**: nenhuma (replicação P184D)
**DEBT**: cobertura M4-residual reduzida — agora apenas C2

---

## §1 Resumo executivo

Migração C1 (heading prefix) materializada em
`mod.rs:Content::Heading` — primeiro consumer onde
Introspector é **caminho funcional real** para counter
hierárquico. Replica padrão P184D Figure com primitiva
location-aware fornecida por P185:

```rust
let num_str = self.current_location
    .and_then(|loc| self.introspector
        .formatted_counter_at("heading", loc))
    .or_else(|| self.counter.format_hierarchical("heading"));
```

**P183B aprendizado retroactivamente validado**: a
estratégia substitution-with-fallback estava certa; a
primitiva (`formatted_counter` snapshot-final P170) era
inadequada para sequências re-update. P185 (4 sub-passos)
forneceu `formatted_counter_at(key, location)` (P177) +
`current_location: Option<Location>` (P185C) — primitiva
correcta. P187B fecha o que P183B tentou.

Δ tests cumulativo: **+4** (1801 → 1805) com **zero
regressões**.

C1 fechado. **DEBT M4-residual reduzido** para cobrir apenas
C2 (cenário B per P187A §8 — sem DEBT formal aberto; nota
preventiva).

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | L0s tocados |
|-------|---------------------|-----------------|---------|-------------|
| **P187A** | S (diagnóstico) | S | 0 | nenhum (cria diagnóstico) |
| **P187B** | S (agregado) | S | **+4** | `rules/layout.md` |
| **Total** | — | — | **+4** | 1 L0 produção |

P187B agregou em sub-passo único:
- `.B` migração consumer C1 em `mod.rs:Content::Heading`.
- `.C` actualização L0 `rules/layout.md` (nova secção C1).
- `.D` 4 tests E2E em submódulo `p187b_c1_heading_prefix`.
- `.E` verificação estrutural (12/12).
- `.F` actualização nota DEBT M4-residual (in-line neste
  relatório).
- `.G` relatório consolidado P187 (este ficheiro).

---

## §3 Decisões arquiteturais

### 6 cláusulas P187A fechadas

| # | Cláusula | Decisão | Sub-passo |
|---|----------|---------|-----------|
| 1 | Forma da expressão | **Combinação Opção B + Opção A**: `current_location.and_then(...).or_else(legacy)` | P187B `.B` |
| 2 | `None` do Introspector | **Opção A**: `or_else` para legacy `format_hierarchical` (replica P184D) | P187B `.B` |
| 3 | `None` do `current_location` | **Opção B**: `and_then` defensivo (sem panic) | P187B `.B` |
| 4 | P183B aprendizado | **Não-aplicável** após P185 — primitiva location-aware corrige | P187B `.D.4` empiricamente valida |
| 5 | Forma migração | substitution-with-fallback per P184D padrão | P187B `.B` |
| 6 | Critério fecho | **Opção 3**: consumer migrado + tests E2E + DEBT actualizado | P187B `.G` |

### Sem ADR — replicação de padrão

ADR avaliada e dispensada. Substitution-with-fallback é
padrão P184D. Decisões registadas no diagnóstico P187A §2.

---

## §4 Achados não-triviais durante execução

### P187A §11.1 — Site real `mod.rs:345`, não 310

Spec referenciava "mod.rs:310" — referência herdada de P183B
e P184F. Após P185C e P186 introduzirem código antes do
arm Heading, site real é `mod.rs:345`. Diferença irrelevante
para implementação; corrigida na materialização P187B.

### P187A §11.3 — P183B retroactivamente validado

P183B falhou por escolha errada de primitiva
(`formatted_counter` em vez de `formatted_counter_at`).
Resto da estrutura (substitution-with-fallback) era
correcta. P187 valida P183B retroactivamente: a estratégia
era certa; a primitiva precisava de adaptação.

P185 (4 sub-passos: A/B/C/D/E) foi necessário para construir
`formatted_counter_at` location-aware E `current_location`
no Layouter. Sem P185, P183B aprendizado seria
inaplicável.

### P187A §11.4 — Heading-arm consolidado

Após P187B, heading-arm em `mod.rs:Content::Heading` faz
**duas** consultas ao Introspector:
1. `is_numbering_active("numbering_active:heading")` (P182D
   — gating se numbering activo).
2. `formatted_counter_at("heading", current_location)`
   (P187B — counter hierárquico location-aware).

Heading-arm é o consumer com **maior consolidação**
Introspector na fase M4-residual. Padrão a replicar em
outros consumers em fases futuras.

### P187B `.D.4` — Test re-update empiricamente valida resolução P183B

Test `c1_heading_prefix_re_update_correctness` exercita
exactamente o cenário onde P183B falhou (sequência H1, H2,
H1 → output esperado "1.", "1.1", "2."). Validação
intermédia mostra que `formatted_counter_at` retorna
snapshot por Location:
- loc(0) → `"1"`
- loc(1) → `"1.1"`
- loc(2) → `"2"`

Output observable confirma sequência correcta no documento.
Garantia explícita anti-P183B: assert que o segundo H1
("Conclusao") **não** ganha prefixo "1." (que indicaria
snapshot-final preempt fallback).

Empiricamente valida que P185 desbloqueio resolve P183B
aprendizado.

### P187B `.D.2` — Test fallback legacy requer pre-população state

Test `c1_heading_prefix_via_fallback_legacy` força
Introspector vazio (`TagIntrospector::empty()`) para testar
o fallback. Mas `numbering_on` gate (P182D) consulta
ambos paths via `||`, e o legacy `is_numbering_active("heading")`
exige `state.numbering_active["heading"] == true`.

Walk legacy de `Content::SetHeadingNumbering` popula este
campo (`introspect.rs:455-457`). O test re-corre `introspect()`
para extrair `walk_state.numbering_active` e copiá-lo para
`state.numbering_active` antes de chamar
`layout_with_introspector` — caminho idêntico ao que
`layout_with_introspector` (mod.rs:1431) já faz internamente
no short-circuit path.

Pequeno achado documental: o caminho de fallback legacy
para C1 funciona porque walk legacy popula state
independentemente do Introspector. Janela compat M6 fechará
quando este caminho for redundante (Introspector é caminho
funcional unificado).

---

## §5 Estado final M9 e M5/M4

### M9 (counter-feature) — inalterado: 11/11

P187 não introduz feature M9 nova. Migração de consumer
C1 — não slot novo.

### M5/M4 (read-site migration) — **7/12** (era 6/12 antes de P187)

C1 fechado em P187B. Read-sites migrados:
- C3 figure-arm (P184D) ✓
- C1 heading-prefix (P187B) ✓ — NEW
- 5 outros via consultas Introspector estabelecidas em
  P181G/P182D/P184D/P186 (parciais).

C2 ainda bloqueado (depende P186 + P188 — ambos os passos
trabalham juntos: P186 estrutural pronto; P188 migração
consumer).

### Trait `Introspector` — 18 métodos (inalterado)

Sem método novo. P185B (`is_numbering_active_at` +
`flat_counter_at`) + P177 (`formatted_counter_at`) já
cobrem tudo necessário.

### Layouter — sem mudança em fields

`current_location: Option<Location>` (P185C) reaproveitado.
`self.introspector` (P168) reaproveitado.

### `mod.rs:Content::Heading` — consumidor consolidado

Antes P187: 1 consulta Introspector (`is_numbering_active`,
P182D) + 1 leitura legacy (`format_hierarchical`).

Após P187: 2 consultas Introspector (`is_numbering_active`
+ `formatted_counter_at`) + 1 fallback legacy
(`format_hierarchical` via `or_else`).

---

## §6 Estado final lacunas

Inalterado em P187. As lacunas catalogadas até P186
permanecem na mesma situação. P187 não foi sobre lacunas —
foi sobre fechar C1.

---

## §7 Pendências cumulativas + DEBT M4-residual

### Activas

- **C2 (P183C)** — eixo 1 (P185) + eixo 2 (P186)
  resolvidos estruturalmente. Apenas P188 falta (migração
  consumer com Introspector path dormente em produção).
- **`Content::SetEquationNumbering` ausente** — passo
  fora de série; activará caminho funcional Introspector
  para C2 quando materializar.
- **4 sites M4-fora-de-escopo** (TOC, fixpoint side-channels,
  resolved labels) — fora de escopo M4-residual.

### Resolvidas estruturalmente em P187B

- **C1 (P183B)** — retroactivamente fechado via P185+P187.

### DEBT M4-residual — **actualização**

**Antes de P187**: notas preventivas em P184F/P185-consolidado/
P186-consolidado mencionavam DEBT cobrindo C1 + C2.

**Após P187B**: DEBT M4-residual cobre apenas **C2**. C1
fechado em P187B com Introspector como caminho funcional.

Quando P188 fechar C2 (Introspector dormente + fallback
legacy permanente), DEBT M4-residual torna-se **vazio em
prática**:
- C1: fechado, Introspector funcional.
- C2: fechado, Introspector dormente; fallback legacy
  serve produção.

P183F formal pode arquivar sem cobrir nada (ou ser
dispensado). Decisão fica para P188 ou passo subsequente.

---

## §8 Próximos passos sugeridos

### Independente — pode prosseguir

1. **P188 — Migrar C2 (equation counter)**: substitui
   `state.get_flat("equation")` em `equation.rs:97` por
   `self.introspector.flat_counter_at("equation",
   self.current_location.unwrap_or(...)).or_else(legacy)`.
   Substitution-with-fallback. **Documentar honestamente**
   que migração resulta em Introspector path dormente em
   produção até equation set rule materializar; fallback
   legacy é caminho funcional permanente. Magnitude S.

### Sequência fechamento M4-residual

2. **Após P188**: M4-residual fechado funcionalmente; DEBT
   M4-residual fecha (vazio em prática); segue M5 (P189)
   com novos read-sites ou M9 slot 11.

### Trabalho identificado fora de escopo

3. **`Content::SetEquationNumbering` materialização** —
   passo dedicado quando equation set rule for prioridade.
   Activa Introspector path em P188; permite janela compat
   M6 abrir para Equation.

4. **F1 retomar (M6 janela compat)** — quando os caminhos
   Introspector forem unificadamente funcionais para todos
   os C1-C12, eliminar `CounterStateLegacy.numbering_active`,
   `hierarchical`, etc., e os fallbacks `or_else(legacy)`.

---

## §9 Conclusão

P187 fechou em 2 sub-passos (A diagnóstico + B
implementação agregada) com magnitude correctamente
estimada (S em ambos). Replicação literal de padrão P184D
Figure, mas com primitiva location-aware (`formatted_counter_at`)
que P185 forneceu.

Achados centrais:
- **C1 fechado** com Introspector como caminho funcional
  real — segundo caso da série M4-residual (após P184D
  Figure).
- **P183B aprendizado validado retroactivamente** — a
  estratégia substitution-with-fallback estava certa;
  apenas a primitiva precisava de adaptação.
- **Test re-update empírico** confirma que P185 desbloqueio
  resolve completamente o gate substancial que P183B
  encontrou.
- **Heading-arm consolidado** — após P187, é o consumer
  com mais consultas Introspector (numbering active +
  formatted counter), padrão a replicar em outros sites
  em fases futuras.

Diferença chave face a P186 (Equation): em P187, Introspector
é **funcional** porque `Content::SetHeadingNumbering` existe
e popula state em produção (P182C). Em P186, Introspector
fica **dormente** porque `Content::SetEquationNumbering` não
existe ainda.

A série P187 termina como **penúltima peça funcional de
M4-residual**. P188 (C2 migration) é a última — fecha
M4-residual; DEBT M4-residual fecha a vazio; segue M5
(P189).

**55 passos executados** após P187B (P186F = 54 + P187A =
55 + P187B = 56). Recontagem com P187: **56 passos**.

Padrão diagnóstico-primeiro mantido — 12/12 acertaram a
magnitude planeada ±1 nível
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A).

Próximo passo sugerido: **P188A** (diagnóstico migração
C2 equation counter — agora desbloqueado por P185 + P186).

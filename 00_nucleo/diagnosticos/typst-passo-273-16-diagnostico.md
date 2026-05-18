# Diagnóstico Fase A P273.16.A — Bbox.y topo-exacto inline (Fase A; spec premise factualmente actualizada)

**Data**: 2026-05-18.
**Passo**: typst-passo-273.16.A.
**Magnitude**: S documental (~30 min — descoberta empírica revisa premissas da spec).
**Cluster**: Visualize / Gradient (sétimo e último sub-passo na sequência terminar cluster — escopo máximo).
**Tipo**: Fase A empírica com decisão **binária** go/no-go per ADR-0034 + ADR-0085.
**Vigésimo sétimo consumo directo de fonte**.

---

## §A.0 — Descoberta empírica que actualiza a premissa da spec

A spec P273.16 §0 declara:

> "DEBT-56 registado em Passo 156B (2026-04-25) durante diagnóstico
> Layout (Fase X). [...] Está **EM ABERTO** desde 2026-04-25."

**Verificação literal em `00_nucleo/DEBT.md:535`**:

> "## DEBT-56 — Column flow Fase 3 Layout (L+; refactor multi-region
> do Layouter) — **ENCERRADO (Passo 221) ✓**"
>
> **Fechado em**: Passo 221 (2026-05-12) — **CLOSED via materialização**
> (paridade pattern P206E DEBT-53).

**Conclusão empírica**: DEBT-56 está **encerrado** desde 2026-05-12,
6 dias antes da spec P273.16 ser escrita. A premissa "EM ABERTO" da
spec está empíricamente desactualizada.

### Reanálise do bloqueador real

Sem DEBT-56 como bloqueador específico, qual é o bloqueador real?

Inventário literal:

1. **P273.7 §A.3 Decisão 1**: "3γ.2.γ-inline-baseline-y aceitável;
   refino topo-exacto fica registado como `P273.X-bis2` per
   ADR-0054 graded — promoção apenas se houver demanda empírica".
2. **P156H limitação consciente** em `00_nucleo/prompts/entities/content.md:817-829`:
   - "`width`/`height` armazenados mas não impõem limite real
     (refino multi-region per DEBT-56)" — **referência a DEBT-56
     também desactualizada na L0**.
   - "`baseline` armazenado mas semantic real adiada (cursor.rs
     actual sem mecânica de offset mid-linha)".
   - "`inset.top`/`inset.bottom` armazenados mas não aplicados em
     layout inline (alterariam line_height)".
3. **ADR-0078 §"Decisão" sub-fase (b)** (per DEBT-56 fechamento):
   "Refino multi-region flow real fica como **Fase 4 candidata
   NÃO-reservada** per política P158 — Opção A multi-region
   completa documentada como scope-out."

**Bloqueador real identificado**: **não há DEBT específico**; é
**P156H limitação consciente** sobre inline line_height per
ADR-0054 graded. Refactor inline line_height para bbox.y topo-exacto
requer trabalho não-reservado actualmente, mas **não está
formalmente bloqueado** pelo encerramento de DEBT-56.

---

## §A.1 — Inventário literal do estado actual

### DEBT-56 (bloqueador previsto pela spec — verificação)

`00_nucleo/DEBT.md:535-565`:
- Status: **ENCERRADO (Passo 221)** desde 2026-05-12.
- Resolvido por: Sub-fases (a) refactor Region/Regions (P216A+B)
  + (b) consumer real graded (P217-P220) materializadas em série
  Layout Fase 3.
- Critério §"Plano" 5/5 cumprido (ADR-0078 IMPLEMENTADO).
- Refino multi-region flow real → Fase 4 candidata NÃO-reservada
  (Opção A scope-out documentada).

### P156H limitação consciente (bloqueador real)

`00_nucleo/prompts/entities/content.md:817-829`:
- Limitação "**`inset.top`/`inset.bottom` armazenados mas não
  aplicados em layout inline (alterariam line_height)**" — directa
  ao bbox.y topo-exacto.
- Refino futuro per ADR-0054 graded.

### Demanda empírica

Análogo a P273.15 §A.1:
- 9 sub-passos consecutivos (P273.7-P273.15) sem caso registado onde
  3γ.2.γ-inline-baseline-y produziu output visualmente insuficiente.
- Zero tests cristalino exercitam expectativas de bbox.y topo-exacto.
- Decisão P273.7 §A.3 — aproximação aceitável per ADR-0054 graded
  — preservada.

**Conclusão §A.1**: zero demanda empírica registada para refino
topo-exacto. P156H limitação consciente aceita pela arquitectura.

---

## §A.2 — Inventário dos 3 caminhos

### Caminho 1 — Refactor inline line_height (escopo L+)

**Mecanismo**: refactor do `Layouter` inline para suportar line_height
ajustável por arm Boxed (parte de uma hipotética "Fase 4 multi-region
completa" ADR-0078).

**Magnitude**: L+ (escopo análogo a sub-fases DEBT-56 P216A+B+P217-P220).

**Pré-requisito**: ADR dedicada Fase 4 multi-region completa.

**Fora do escopo P273.16** (escopo S-M cluster Gradient).

### Caminho 2 — `font_metrics.ascender` no arm Boxed (ad-hoc)

**Mecanismo**:
```rust
// L1 arm Content::Boxed em mod.rs P273.7:
let saved_parent_bbox_p273_9 = self.parent_bbox;
if let (Some(w), Some(h)) = (width, height) {
    let w_pt = w.resolve_pt(font);
    let h_pt = h.resolve_pt(font);
    let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
    self.parent_bbox = Some(Rect {
        x: self.regions.current.cursor_x,
        // P273.16 — topo-exacto via ascender (P273.7 era cursor.y baseline-relative).
        y: self.regions.current.cursor_y - ascender,
        w: Pt(w_pt),
        h: Pt(h_pt),
    });
}
```

**Magnitude**: S (~15-25 LOC L1).

**Risco**: cria mecânica ad-hoc que pode divergir de:
1. Quando hipotética "Fase 4 multi-region" for materializada.
2. Outros sítios análogos (Block inline-context, futuros).

Cria dívida invisível: code-comment "P273.16 — topo-exacto via
ascender ad-hoc; remover quando refactor inline line_height
materializado".

**Demanda**: zero (§A.1 confirmado).

### Caminho 3 — Scope-out preserved

**Mecanismo**: 3γ.2.γ-inline-baseline-y P273.7 preserved literal.

**Magnitude**: zero código.

**Trade-off**: bbox.y baseline-relative continua a ser aproximação
aceitável per ADR-0054 graded + coerente com P156H limitação
consciente. Aceito por 9 sub-passos consecutivos.

---

## §A.3 — Decisão go/no-go primária — **NO-GO**

**Fixada**: **NO-GO via §A.5 critério #1 + #2 + #3 + #4 combinados**.

Razão concreta (quádrupla, similar a P273.15 mas com fundamentos
distintos):

1. **§A.1 confirma zero demanda empírica** — 9 sub-passos
   consecutivos (P273.7-P273.15) sem caso registado onde
   3γ.2.γ-inline-baseline-y produziu output visualmente insuficiente.
2. **Caminho 1 (refactor inline line_height) fora do escopo
   P273.16** — magnitude L+ vs escopo S-M cluster Gradient.
3. **Caminho 2 (font_metrics ad-hoc) cria dívida invisível** sem
   demanda registada — over-engineering per ADR-0054 graded;
   código adicionado pode precisar reverter quando Fase 4
   multi-region for materializada (se for).
4. **3γ.2.γ-inline-baseline-y P273.7 aceito por ADR-0054 graded** +
   coerente com P156H limitação consciente — "menor mudança
   suficiente" preserved.

### Premissa da spec actualizada por verificação empírica

Spec §0 afirma "DEBT-56 EM ABERTO" — empíricamente desactualizado.
**Conclusão NO-GO permanece correcta** mas via fundamentos
factuais diferentes:
- Spec previa: NO-GO obrigatório por construção (DEBT-56 bloqueia).
- Empírica: NO-GO por **zero demanda + dívida ad-hoc + ADR-0054
  graded aceitação** (DEBT-56 fechado mas refactor inline
  line_height permanece deferido).

Esta actualização **não invalida o passo** — é cumprimento honesto
do critério "verificar empíricamente" registado em todos os
relatórios anteriores P273.7-P273.15. Fase A factual prevalece
sobre premissa da spec.

---

## §A.4 — Critério para GO — não cumprido

Per spec §A.4:

| Critério GO | Estado |
|---|---|
| Caminho 2 escolhido E demanda empírica concreta | ❌ Zero demanda registada em 9 sub-passos |
| Decisão deliberada de que caminho ad-hoc não cria dívida invisível | ❌ Caminho 2 explicitamente cria dívida invisível (code-comment "remover quando refactor" sinaliza) |

Critério #1 (demanda empírica) não cumprido → **NO-GO automaticamente**
per spec §A.4.

---

## §A.5 — Critério para NO-GO — cumprido absoluto

Per spec §A.5 (adaptado pela verificação empírica DEBT-56):

| Critério NO-GO | Cumprido | Como |
|---|---|---|
| §A.1 confirma bloqueador estrutural relevante | ✅ | P156H limitação consciente sobre inline line_height — não DEBT-56 (fechado) mas equivalente arquitectural per ADR-0054 graded |
| Caminho 1 fora de escopo P273.16 | ✅ | L+ vs escopo S-M cluster Gradient; Fase 4 multi-region scope-out per ADR-0078 |
| Caminho 2 cria dívida ad-hoc | ✅ | Sem demanda + risco divergência futuro refactor |
| 3γ.2.γ-inline-baseline-y P273.7 aceito ADR-0054 graded | ✅ | 9 sub-passos sem contraproba |

**4 critérios NO-GO cumpridos absolutos** — NO-GO honesto.

---

## §A.6 — Análise de risco

| Risco | Estado |
|---|---|
| Dívida invisível por caminho ad-hoc | ✅ Mitigado — §A.4 critério 2 explícito bloqueou GO |
| Refino sem demanda vira over-engineering | ✅ Mitigado — §A.4 critério 1 obrigatório |
| Bloqueador interpretado como falha | ✅ Mitigado — §A.5 explicita; sub-padrão "Scope-out reconfirmado por Fase A" N=3 cumulativo |
| Premissa spec desactualizada (DEBT-56 fechado) | ✅ Mitigado — Fase A factual prevalece; conclusão NO-GO permanece correcta via fundamentos actualizados |
| Refactor inline line_height permanece deferido sem ETA | ✅ Aceito — ADR-0054 graded; Fase 4 multi-region scope-out per ADR-0078 |

---

## §A.7 — Decisões fixadas Fase A

1. **Decisão 1 (caminho)**: **3 — scope-out preserved**.
2. **Decisão 2 (apenas se GO)**: **N/A** — NO-GO.
3. **Decisão 3 (sempre)**: documento
   `00_nucleo/diagnosticos/typst-passo-273-16-trabalho-previo-externo.md`
   produzido — referenciando bloqueador **REAL** (P156H + refactor
   inline line_height per ADR-0054 graded; NÃO DEBT-56 fechado).

### Actualização recomendada (não obrigatória este passo)

L0 `00_nucleo/prompts/entities/content.md:824` actualmente declara:
> "`width`/`height` armazenados mas não impõem limite real (refino
> multi-region per DEBT-56)"

Referência a DEBT-56 **factualmente desactualizada** após fechamento
P221. Candidato XS futuro (similar a P273.8 cleanup pattern):
**P273.X-bis-content-md-debt56-update** — atualizar L0 para
referenciar "Fase 4 multi-region scope-out per ADR-0078"
em vez de "DEBT-56". NÃO reservado.

---

## §A.8 — Critério de aceitação Fase A

- ✓ §A.1 cita estado factual literal (DEBT-56 ENCERRADO P221 +
  P156H limitação consciente preservada).
- ✓ §A.2 inventário dos 3 caminhos com magnitude e risco concretos
  + actualização empírica (Caminho 1 não é mais "bloqueado por
  DEBT-56 aberto" mas "fora de escopo + Fase 4 scope-out").
- ✓ §A.3 decisão **NO-GO** fixada com fundamento quádruplo + nota
  honesta sobre actualização da premissa da spec.
- ✓ §A.5 risco mitigado por 5 critérios explícitos.
- ✓ §A.7 documento de trabalho prévio externo produzido referenciando
  bloqueador real (não DEBT-56 fechado).

**Fase A produzida — critério §A.8 cumprido absoluto. Decisão
NO-GO confirmada empíricamente; premissa da spec actualizada por
verificação factual.**

---

## §A.9 — Plano de implementação (Fase C — REDUZIDA por NO-GO)

Por NO-GO, Fase C reduzida a:

1. ADR-0091 anotação cumulativa décima sexta (template NO-GO).
2. L0 `entities/gradient.md` anotação P273.16 (NO-GO outcome).
3. Documento `typst-passo-273-16-trabalho-previo-externo.md`
   referenciando bloqueador real.
4. Relatório P273.16 com status **SCOPE-OUT-RECONFIRMED**.

**Zero alterações código L1/L3**. Tests workspace preserved bit-exact.

### Sub-padrão "Scope-out reconfirmado por Fase A" N=2 → N=3 cumulativo crossing limiar formalização N=3-4

P273.16 é a **terceira aplicação** do sub-padrão — atinge limiar
formalização ADR meta N=3-4:

- **N=1 (P273.14)**: CMYK-ICC scope-out via NO-GO; razão constraints
  externas (profile licensing + crate externa + invariante L0).
- **N=2 (P273.15)**: Bbox medido pós-layout via NO-GO; razão
  constraints internas (custo perf + ausência demanda).
- **N=3 (P273.16)**: Bbox.y topo-exacto inline via NO-GO; razão
  bloqueador estrutural aceito + actualização empírica da premissa
  da spec.

**Limiar formalização N=3-4 atingido com folga consolidada** —
candidato meta-ADR formalização NÃO reservado.

3 razões NO-GO distintas e legítimas estabelecem padrão consolidado.

---

*Diagnóstico imutável produzido em 2026-05-18. Vigésimo sétimo
consumo. Decisão **NO-GO** confirmada empíricamente; premissa da
spec actualizada (DEBT-56 fechado P221, não EM ABERTO como spec
afirmava); sub-padrão "Scope-out reconfirmado por Fase A" N=2 → N=3
cumulativo crossing limiar formalização N=3-4 — terceira aplicação
consolidando padrão.*

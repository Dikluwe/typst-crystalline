# Passo 254A — Relatório

**Data**: 2026-05-15
**Tipo**: passo arquitectural de diagnóstico
(**não materializa código**)
**Análogo estrutural**: P160 (diagnóstico Introspection original);
P159B (diagnóstico amplo expansão com tecto realista).
**Motivação**: correcção factual do resumo cumulativo pós-P254
que cita "Introspection ~17%". Número desactualizado por ~50
sub-passos de materialização (M2-M9 + P204A-H).

---

## Sumário executivo

1. **Resumo cumulativo pós-P254 contém erro factual**: cita
   Introspection ~17% (de P160, 2026-04-25). Estado real
   2026-05-15: ~85-92% observable.

2. **Introspection deixou de ser módulo mais fraco**. Passou a
   estar entre os mais cobertos junto com Layout (98-99%).

3. **Único marco arquitectural não fechado**: M8 (`comemo::Track`
   adopção). Ganho principal pós-M8 é arquitectural
   (performance, paridade interna vanilla), não observable.

4. **Recomendação primária**: pivot para Visualize (~54%
   pré-pivot) — Introspection saturado pragmaticamente sem M8.

5. **Recomendação secundária**: ADR-create M8 administrativo + 5-8
   sub-passos comemo. Padrão `ADR-0062-create`/`P160A`.

---

## Artefactos produzidos

| Ficheiro | Localização canónica | Conteúdo |
|----------|----------------------|----------|
| `diagnostico-introspection-actualizado-passo-254A.md` | `00_nucleo/diagnosticos/` | Diagnóstico principal 6 secções (padrão P160) |
| `inventario-introspection-features-passo-254A.md` | `00_nucleo/diagnosticos/` | Anexo factual 5 tabelas (features × estado × passo) |
| `typst-passo-254A-relatorio.md` | `00_nucleo/materialization/` | Este ficheiro |

---

## Padrões metodológicos aplicados

### ADR-0065 critério #5 — scope determinado por inventário

Décima Xª aplicação concreta (contagem exacta depende do estado
pós-P254 que não foi totalmente expandido no contexto). Aplicação
particular: inventário **corrige número desactualizado** num
resumo cumulativo, não determina scope de materialização nova.
Subpadrão emergente: "diagnóstico correctivo" — N=1 (este passo)
ou N=2 se P192A (que auditou M7 estruturalmente fechado sem ADR
explícita) for contado como precedente.

### ADR-0034 — diagnóstico canónico antes de decisão

Aplicado: secções §1-§6 padrão; persistência em
`00_nucleo/diagnosticos/`; referenciado pelo ADR / passo via
linha "**Diagnóstico prévio**: ver ...".

### Política "sem novas reservas"

Preservada — diagnóstico actualiza estado factual, não cria
reservas. Recomendação primária (pivot Visualize) e secundária
(ADR-create M8) são para validação humana, não compromissos.

---

## Estado cumulativo pós-P254A

### Cobertura por módulo (estimativa actualizada)

| Módulo | Estimativa P254 (resumo antigo) | Estimativa P254A (corrigida) |
|--------|----------------------------------|-------------------------------|
| Layout | 98-99% | 98-99% (inalterado) |
| Model | 50% | 50% (não auditado este passo) |
| **Introspection** | **~17%** ← desactualizado | **~85-92%** (corrigido) |
| Visualize | ~54% | ~54% (não auditado este passo) |
| Text | ~52% | ~52% (não auditado este passo) |
| Math | a confirmar | a confirmar |

### Ranking de prioridade pivot (corrigido)

1. **Visualize ~54%** — maior potencial de ganho cumulativo;
   reusa `Stroke` P252.
2. **Text ~52%** — StyleChain refino + DEBT-53 rustybuzz real.
3. **Math** — confirmar estado factualmente.
4. **Model 50%** — refinos Fase 2/3 restantes; menor magnitude.
5. ~~Introspection ~17%~~ → **~85-92%**: saturado pragmaticamente;
   não-prioritário sem trabalho M8 comemo.

---

## Decisões registadas

1. **Número "~17%" deve ser substituído por "~85-92%"** em
   sumários futuros que citem cobertura Introspection.

2. **Introspection sai da lista de "módulos mais fracos"** —
   passa a estar entre os mais cobertos.

3. **M8 comemo é o único trabalho arquitectural significativo
   restante** em Introspection puro.

4. **Cross-document refs ficam fora de Introspection puro** —
   domínio cross-módulo multi-document pipeline.

---

## Padrões cumulativos (não-impactados por este passo)

- Granularidade: N inalterada (passo diagnóstico, não
  materialização).
- Inventariar primeiro: contagem +1 (critério #5 com aplicação
  particular "diagnóstico correctivo").
- §análise de risco: passo diagnóstico baixo risco; +1 contagem.
- Hashes L0 (`region.md`, `content.md`, `geometry.md`):
  preservados (passo documental).
- Tests: inalterados em 2304 verdes (passo documental).
- ADRs: distribuição inalterada — sem nova ADR, sem promoção.

---

## Próximos passos sugeridos

### Sequência A — pivot Visualize (recomendação primária)

1. **P254B** — diagnóstico Visualize (análogo P160 / P157 /
   P158); inventário shape primitives, paths, curves, Stroke
   reuso P252.
2. **P254C** — primeiro sub-passo materialização Visualize Fase
   1 minimal.

### Sequência B — M8 comemo (recomendação secundária)

1. **ADR-create M8** (XS administrativo) — autorização
   `comemo::Track` adopção.
2. **P254B'** — `#[comemo::track]` no trait `Introspector` (M).
3. **P254C'-G'** — migração queries location-aware,
   sub-stores, fixpoint replacement (5 sub-passos M).

### Sequência C — refinos qualitativos (recomendação terciária)

1. **R1** — refino `measure()` edge cases.
2. **R2** — refino fixpoint convergence paridade vanilla.
3. **R3** — refino `query()` predicates avançados.

Cada R em sub-passo S+. Valor incremental baixo; útil para
preservar momentum granular se pivot Visualize for adiado.

---

## Referências

- Diagnóstico principal: `diagnostico-introspection-actualizado-passo-254A.md`.
- Inventário anexo: `inventario-introspection-features-passo-254A.md`.
- Resumo cumulativo pós-P254 (fonte do número desactualizado).
- ADRs: 0029, 0033, 0034, 0054, 0065, 0066, 0072, 0073.
- Passos precedentes: P160 (original), P164-P204H (série de
  materialização).

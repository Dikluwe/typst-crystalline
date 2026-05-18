# Relatório P276 — DEBT-35b fecho OBSOLETED (cache available_width nunca materializado)

**Data**: 2026-05-18.
**Status**: **IMPLEMENTADO** (admin XS — fecho honesto via pattern P206E).
**Magnitude real**: zero LOC código; DEBT-35b movido para Secção 2 do `DEBT.md` com etiqueta OBSOLETED + nota arquitectural preservada; cabeçalho `DEBT.md` actualizado (8 → 7 abertos).
**Cluster**: Metodologia / DEBTs / Fecho honesto.
**Tipo**: passo administrativo P276 — fecho de DEBT por pattern P206E (OBSOLETED).
**Spec**: `00_nucleo/materialization/typst-passo-276.md`.

---

## §1 — Validação contra spec P276

Tabela de critérios §7 da spec:

| Critério | Status | Evidência |
|---|---|---|
| Fase A produzida; §A.1-A.5 preenchidos empíricamente | ✓ | `00_nucleo/diagnosticos/diagnostico-debt-35b-passo-276.md` |
| §A.1 confirmou cache ausente | ✓ | 5/5 verificações positivas (método tempo real; zero campos cache; 13 callsites método; arm SetPage sem invalidação; histórico git zero matches) |
| DEBT.md actualizado: DEBT-35b Secção 2 + cabeçalho com linha P276 | ✓ | Entrada movida; cabeçalho com 2 linhas novas (P275 reconciliação + P276 fecho) |
| Nota arquitectural preservada | ✓ | Em DEBT.md Secção 2 entrada DEBT-35b; substituto documental do DEBT preventivo |
| Tests workspace 2644 preserved | ✓ | 2179 core + 418 infra + 24 + 21 + 2 + 0 + 0 = 2644 |
| Lint zero violations | ✓ | "✓ No violations found" |
| Cap documental hard respeitado | ✓ | Diagnóstico ~280 linhas (cap soft 250 estourado 12%; hard 400 folga 30%); relatório ~250 linhas (cap soft 400 folga 38%) |
| Relatório consolidado §1-§7 completos | ✓ | Este documento |

**P276 NÃO fecha se** (gates):
- §A.1 revelar cache já adicionado. **Não disparou** (cache ausente confirmado).
- Regressão tests não-documentada. **Não disparou** (2644 preserved).
- Lint não-zero. **Não disparou** (zero violations).
- Algum dos 4 caminhos P206E inaplicáveis. **Não disparou** (OBSOLETED aplica absoluto).

**8/8 critérios cumpridos** — P276 fecha **IMPLEMENTADO**.

---

## §2 — Resumo factual fecho

### §2.1 — Verificação empírica §A.1

5 sub-verificações cobertas no diagnóstico Fase A confirmam **cache
ausente**:

| Verificação | Resultado |
|---|---|
| Método `available_width` existe sem cache | ✓ `mod.rs:372` calcula em tempo real |
| Campo cache no struct Layouter | ✓ Zero matches (`cached_width`, `width_cache`, `cached_available`) |
| Consumidores chamam método (não campo) | ✓ 13 callsites (mod.rs 10 + placement.rs 2 + grid.rs 1) |
| `Content::SetPage` arm dedicado sem invalidação cache | ✓ `mod.rs:1009` configura `page_config` sem cache invalidation |
| Histórico git registou cache adicionado | ✓ Zero commits (working tree limpo; histórico sem matches) |

### §2.2 — Veredicto P206E

| Caminho | Aplicabilidade | Veredicto |
|---|---|---|
| **CLOSED** | NÃO — não há código a materializar | Não aplica |
| **REPLACED-BY** | NÃO — não há substituto material | Não aplica |
| **OBSOLETED** | **SIM** — hipótese preventiva inicial não materializada em ~195 passos | **APLICA** |

**Veredicto absoluto**: **OBSOLETED**.

### §2.3 — Total abertos: 8 → 7

Pré-P276: 8 DEBTs em aberto/parciais (per auditoria P275).
Pós-P276: 7 DEBTs em aberto/parciais (DEBT-35b → Secção 2
encerrados).

---

## §3 — Operações realizadas

### §3.1 — Edições em `00_nucleo/DEBT.md`

**Operação 1 — Remoção entrada DEBT-35b da Secção 1**:
Linhas originais 391-396 (entrada "EM ABERTO Passo 81") removidas.

**Operação 2 — Inserção entrada DEBT-35b na Secção 2**:
Nova entrada com etiqueta "ENCERRADO (Passo 276) ✓" + justificação
literal + paralelo DEBT-54 + nota arquitectural preservada +
evidência empírica P276 Fase A + cross-reference diagnóstico +
**histórico pré-fecho preservado** per pattern P201/P202.

**Operação 3 — Acréscimo cabeçalho DEBT.md**:
2 linhas novas no cabeçalho histórico cumulativo:

```markdown
> **Passo 275 (2026-05-18)**: auditoria empírica pós-cluster
> Gradient. Contagem actual reconciliada: **8 abertos** ... [P156B
> herdada desactualizada]. Pós-P156B fechados: DEBT-53/54 P206E,
> DEBT-56 P221, DEBT-34d P233, DEBT-34e P224, DEBT-8 P255.

> **Passo 276 (2026-05-18)**: fecho de **DEBT-35b** como OBSOLETED
> ... Total abertos: **8 → 7**.
```

**Nota**: a linha P275 (reconciliação histórica) foi adicionada
neste passo também — porque a auditoria P275 já tinha identificado
a desactualização do cabeçalho mas reservou a actualização como
"acção §4.1 não executada" (per disciplina anti-scope-creep P275).
P276 fecha simultaneamente o cabeçalho P275-pendente + P276-fecho
para coerência.

### §3.2 — Nota arquitectural preservada (substituto documental)

Texto literal incorporado em DEBT.md Secção 2 entrada DEBT-35b:

> Se passo futuro adicionar cache de `available_width` como campo
> do `Layouter` (motivado por perf benchmark concreto), o arm
> `Content::SetPage` (`mod.rs:1009+`) deve invalidar o cache. Esta
> nota fica preservada aqui em vez de manter DEBT aberto
> especulativamente.

A nota é **prevenção textual** sobre risco arquitectural hipotético,
não DEBT formal. Disciplina: DEBTs são para dívidas concretas; notas
arquitecturais cobrem riscos preventivos. Anti-padrão "DEBT como
wishlist arquitectural" evitado.

### §3.3 — Crystalline-lint propagação

`crystalline-lint .` → "✓ No violations found".

Zero hashes corrigidos (DEBT.md não tem hash L0; L0 prompts não
foram tocados). Confirmação literal.

### §3.4 — Histórico preservado

Per pattern P201/P202 ("histórico textual preservado"), a entrada
original do DEBT-35b é preserved em formato `### (Histórico)
Estado pré-fecho — DEBT-35b — EM ABERTO (Passo 81)` dentro da
nova entrada na Secção 2. **Zero perda de informação**.

---

## §4 — Sub-padrões emergentes detectados

### §4.1 — "Fecho OBSOLETED de DEBT preventivo" N=2 cumulativo

- **N=1 (P206E)** — DEBT-54 ("Setup vanilla typst workspace"):
  hipótese arquitectural inicial revelou-se factualmente
  desnecessária.
- **N=2 (P276)** — DEBT-35b ("Cache available_width invalidation"):
  hipótese preventiva sobre cache futuro hipotético não materializou
  em ~195 passos.

**Limiar formalização N≥3-4 NÃO atingido**. **NÃO formalizar ADR**
per anti-padrão over-formalização P273.17 §0. Aguardar terceira
aplicação cross-cluster para considerar formalização.

### §4.2 — "Passo administrativo dedicado a fecho de DEBT" N=1 inaugural

P206E fechou múltiplos DEBTs como parte de auditoria ampla (passo
multi-DEBT). P276 é o primeiro **passo dedicado a fecho de um
único DEBT** (escopo XS).

**Limiar formalização NÃO atingido** (N=1 inaugural). Aguardar
reaplicação.

### §4.3 — "Diagnóstico imutável" N=34 → N=35 cumulativo

P276 é o **30º consumo** directo de fonte do pattern diagnóstico-
primeiro (continuação P275 N=34; 29º consumo).

### §4.4 — Pattern P206E (3 caminhos fecho) — 4ª aplicação cumulativa

- **N=1-3 (P206E)** — DEBT-53/54 OBSOLETED.
- **N=4 (P276)** — DEBT-35b OBSOLETED.

Pattern P206E continua a ser **paradigma estável** para fecho honesto
de DEBTs. Formalização pode ser candidato futuro se N≥5-6.

---

## §5 — Próximos passos da sequência

Per spec P275 §7 + spec P276 §3 §C.2 §5:

### Cenário A (cleanup XS combinado — preferido)

1. ✓ **P276** — DEBT-35b OBSOLETED (este passo; fechado).
2. **P277** — DEBT-43 fecho OBSOLETED? Análise empírica necessária
   (Linter whitelist crate-level vs type-level — verificar se há
   demanda concreta). Magnitude esperada XS administrativa se
   também OBSOLETED.
3. **P278** — DEBT-33 CLOSED via Bézier bbox analítica (S+M com
   código; primeiro passo com materialização real pós-cluster
   Gradient).
4. **P279** — Cleanup XS combinado: `P273.X-bis-content-md-debt56-update`
   (~5 LOC L0) + `P273.X-bis-helper-group-bbox` (~10-15 LOC L3 net
   negativo) + `P273.X-bis-draw-item-local-text-image` (S; fora
   cluster).

### Cenário B (alternativo — atacar DEBT directo)

Pular para P277 = DEBT-33 directo se houver demanda concreta para
Bézier bbox. P276 fecha estado; cleanup XS pode acontecer quando
conveniente.

### Recomendação

**Cenário A** preferido — continuar com fechos honestos OBSOLETED
até esgotar candidatos preventivos, depois materializar DEBTs reais
e cleanups XS. Disciplina: fechar dívidas "fáceis" antes de
comprometer com magnitudes maiores.

Decisão final: **humana**.

---

## §6 — Métricas finais

| Métrica | Pré-P276 | Pós-P276 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2644 | **2644** | 0 (zero alterações código) |
| **DEBTs em aberto/parciais** | **8** | **7** | **-1 (DEBT-35b OBSOLETED)** |
| ADRs vigentes | 84 | 84 | 0 |
| Lint violations | 0 | 0 | 0 |
| Build warnings (novos) | 0 | 0 | 0 |
| Hashes L0 propagados | — | 0 | 0 (L0 prompts não tocados) |
| Documentos novos | — | 2 | Diagnóstico Fase A + Relatório |
| Edições `DEBT.md` | — | 3 | Remoção Secção 1 + Inserção Secção 2 + 2 linhas cabeçalho |

### §política condições verificadas

- ✓ Cap LOC L1/L3/stdlib hard 0 — real 0; literal.
- ✓ Cap documental Fase A hard 400 — real ~280; folga 30%.
- ⚠ Cap documental Fase A soft 250 — real ~280; **estouro soft
  12%** registado per ADR-0094 Pattern 1.
- ✓ Cap documental relatório hard 600 — real ~250; folga 58%.
- ✓ Cap documental relatório soft 400 — real ~250; folga 38%.
- ✓ Tests workspace 2644 preserved bit-exact.
- ✓ Lint zero preserved.
- ✓ ADR-0029 pureza física L1 preserved (absoluto — zero código).
- ✓ Histórico DEBT-35b preserved per pattern P201/P202.
- ✓ Nota arquitectural preservada (substituto documental).

**10 condições §política verificadas — 9 satisfeitas absolutas + 1
estouro soft documental** registado.

---

## §7 — Referências cross-passos

- **Spec P276** — `00_nucleo/materialization/typst-passo-276.md`.
- **Diagnóstico Fase A** —
  `00_nucleo/diagnosticos/diagnostico-debt-35b-passo-276.md`.
- **DEBT.md** — `00_nucleo/DEBT.md` (Secção 2 entrada DEBT-35b
  ENCERRADO; cabeçalho cumulativo P275 + P276).
- **P81** — origem DEBT-35b preventivo (`Content::SetPage`
  materialização inicial; comentário inline em layout.rs linha 288).
- **P206E** — pattern fecho 3-caminhos (CLOSED / REPLACED-BY /
  OBSOLETED); precedente directo DEBT-54 OBSOLETED.
- **P125** — auditoria DEBTs original; classificou DEBT-35b como
  "manter" (sem evidência irrelevância na altura — 44 passos
  pós-abertura).
- **P275** — auditoria empírica pós-cluster Gradient; reconciliação
  14 → 8 abertos; identificou DEBT-35b como "accionável directo S".
- **P201/P202** — pattern "histórico textual preservado".
- **ADR-0085** — Diagnóstico imutável (30º consumo).
- **ADR-0029** — Pureza física L1 (preserved absoluto).
- **ADR-0094** — Meta-operacional specs (Pattern 1 cap LOC; estouro
  soft Fase A 12% registado).

---

## §8 — Marco final P276

**DEBT-35b fechado OBSOLETED** via pattern P206E:

- Fase A factual: 5/5 verificações empíricas positivas (cache
  ausente; método tempo real; SetPage arm sem invalidação;
  histórico git zero matches; menções DEBT-35b todas
  administrativas).
- Veredicto OBSOLETED fundamentado per ADR-0054 graded — "menor
  mudança suficiente" preserved; DEBT preventivo sobre risco
  hipotético não materializado em ~195 passos.
- DEBT.md actualizado: DEBT-35b movido Secção 1 → Secção 2 com
  histórico preserved; cabeçalho com 2 linhas novas (P275 + P276).
- Nota arquitectural preservada como substituto documental
  (anti-padrão "DEBT como wishlist arquitectural" evitado).
- Tests workspace 2644 preserved bit-exact.
- Lint zero violations.
- Zero alterações código L1/L3/stdlib (ADR-0029 preserved absoluto).

Sub-padrão **"Fecho OBSOLETED de DEBT preventivo" N=2 cumulativo**
emergente (DEBT-54 + DEBT-35b). Limiar formalização N≥3-4 NÃO
atingido — preserved sem ADR.

Sub-padrão **"Passo administrativo dedicado a fecho de DEBT" N=1
inaugural** emergente. P206E fechou DEBTs em auditoria ampla; P276
é primeiro passo dedicado XS a um único DEBT.

Sub-padrão **"Diagnóstico imutável" N=34 → N=35 cumulativo** (30º
consumo).

Pattern **P206E (3 caminhos fecho)** atinge **N=4 cumulativo**
(DEBT-53 + DEBT-54 + DEBT-35b). Paradigma estável; candidato
formalização futura se N≥5-6.

**Total DEBTs abertos: 8 → 7**. Pendência cluster Gradient
candidatos XS (3) + scope-outs reconfirmados (3) + pendências fora
cluster (5) preserved per P275.

**Próximo passo natural**: decisão humana entre Cenário A (P277
DEBT-43 fecho OBSOLETED se aplicável) ou Cenário B (DEBT directo).

---

*Relatório imutável produzido em 2026-05-18. DEBT-35b fechado
OBSOLETED via pattern P206E (irrelevância empírica) — DEBT preventivo
aberto P81 sobre cache hipotético; cache nunca materializado em ~195
passos; risco previsto não activou. Sub-padrão "Fecho OBSOLETED de
DEBT preventivo" N=2 cumulativo preserved (DEBT-54 + DEBT-35b).
Pattern P206E 4ª aplicação cumulativa. Total DEBTs abertos: 8 → 7.
Zero alterações código L1/L3/stdlib; 2644 tests preserved bit-exact;
lint zero. Nota arquitectural preservada como substituto documental
do DEBT preventivo per disciplina anti-"DEBT como wishlist".*

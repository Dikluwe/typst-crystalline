# P443 — Decisão: `get_unchecked` no scanner — eliminar ou autorizar?

> **Passo:** 443  
> **Data:** 2026-06-24  
> **Foco:** Correr benchmark comparativo (baseline P441 vs candidate P442), aplicar critério ADR-0032, e fechar DEBT-42 definitivamente.  
> **Pré-requisitos:** P441 (infra) + P442 (refactor experimental em branch).  
> **ADR-0032:** regressão < 5% → eliminar; 5–20% → decisão humana; > 20% → excepção permanente com ADR de número concreto.  

---

## Contexto

**P441** criou a infra de benchmark (5 inputs, Criterion, ns/byte).  
**P442** executou o refactor experimental em branch: 7 ocorrências de `get_unchecked` substituídas por `&self.string[start..end]`.

Este passo corre o benchmark na branch experimental, compara com o baseline do P441, e **toma a decisão final** sobre DEBT-42.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| Benchmark baseline disponível? | Sim — P441 produziu medianas para B1–B5 | ✅ |
| Branch experimental pronta? | Sim — P442 substituiu 7 `get_unchecked` | ✅ |
| Critério de decisão definido? | Sim — ADR-0032 com 3 thresholds | ✅ |
| `cargo test` passa na branch? | Sim — P442 verificou zero regressões | ✅ |
| Bloqueadores? | Nenhum | ✅ |

**Reclassificação:** S (~15 min; 1 run de benchmark + análise + decisão + documentação).

---

## Procedimento de medição

### Passo 1 — Baseline (branch `main`)
```bash
git checkout main
cargo bench --bench scanner_bench
```
Capturar medianas para B1–B5.

### Passo 2 — Candidate (branch `p442-get-unchecked-removal`)
```bash
git checkout p442-get-unchecked-removal
cargo bench --bench scanner_bench
```
Capturar medianas para B1–B5.

### Passo 3 — Cálculo de delta
Para cada input:
```
delta% = (candidate - baseline) / baseline * 100
```

### Passo 4 — Agregação
- **Média ponderada** por tamanho de input (bytes) para evitar que B1 micro domine.
- **Máximo delta** por input como indicador de pior caso.

---

## Decisão arquitetural (pré-condicionada aos números)

| Cenário | Delta agregado | Decisão | Ação |
|---------|---------------|---------|------|
| A | < 5% | **Eliminar `unsafe`** | Mergear P442 para `main`; escrever ADR-0116 (autorização implícita por eliminação); DEBT-42 **FECHADO** |
| B | 5% – 20% | **Decisão humana** | Apresentar números ao dono; aguardar indicação; DEBT-42 mantém aberto com nota "aguarda decisão humana" |
| C | > 20% | **Autorizar excepção permanente** | Rejeitar merge do P442; escrever **ADR-0116** com número concreto de regressão; DEBT-42 **FECHADO** como excepção permanente |

---

## Scope-out explícito

- **Não** reabre DEBT-42 em qualquer cenário — a decisão definitiva é tomada neste passo.
- **Não** adiciona novos inputs de benchmark — os 5 do P441 são suficientes para decisão.
- **Não** investiga otimizações alternativas ao slicing (ex: `get_unchecked` em release-only, `#[cfg]`). A decisão é binária: elimina ou autoriza.

---

## Critério de fecho

- [ ] Benchmark candidate executado; medianas capturadas para B1–B5.
- [ ] Delta percentual calculado por input e agregado.
- [ ] Decisão aplicada segundo tabela de cenários A/B/C.
- [ ] Se Cenário A: P442 mergeado para `main`; ADR-0116 escrito (eliminação).
- [ ] Se Cenário C: ADR-0116 escrito (excepção permanente com número concreto).
- [ ] `DEBT.md` atualizado: DEBT-42 **FECHADO (P443)** com nota do cenário aplicado.
- [ ] `cargo test --workspace` verde (Cenário A) ou verde em `main` (Cenário C).
- [ ] `crystalline-lint` zero novas violações.

---

**Próximo passo:** Com DEBT-42 fechado, o inventário de débitos técnicos estará **limpo**. O próximo trabalho será **novas features** ou **refinamentos de paridade** — não mais fecho de débito. Indique se quer ajustar o escopo do P443 ou pivotar para outra frente.

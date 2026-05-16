# ⚖️ ADR-0084: Auditoria condicional — audit empírico antes de decisão B1/B2/B3

**Status**: `EM VIGOR`
**Data**: 2026-05-15
**Autor**: Humano + IA
**Validado**: Passo 260 — formaliza padrão N=5 das aplicações
P192A/P255/P257/P258/P259.
**Diagnóstico prévio**: inline em P260.A (paridade P156K
auto-aplicação ADR-0065 critério #5).

---

## Contexto

Cinco passos consecutivos aplicaram o mesmo padrão metodológico
para decisões de scope sobre módulos com cobertura ambígua:

1. **Fase A empírica** — comandos `grep`/`view` produzem
   inventário literal do estado real (vs estado declarado em
   relatório/DEBT desactualizado).
2. **Decisão B1/B2/B3** — baseada em evidência factual:
   - **B1**: ≥75% cobertura → fecho conceptual (relatório
     documental).
   - **B2**: 55-70% → sub-passos prioritários materializáveis.
   - **B3**: ≤50% → re-classificação primeiro.
3. **Acções P260.B+** — actualização documental (L0 prompts
   obsoletos), eventual materialização condicional, fecho
   cumulativo.

**N=5 cumulativo** com zero reformulações:

| Passo | Módulo/alvo | Estado declarado pré | Estado factual pós | Cenário |
|-------|-------------|----------------------|---------------------|---------|
| P192A | M7 fixpoint runtime | ambíguo "intermédio" | estruturalmente fechado | B1 retroactivo |
| P255 | DEBT-8 Math (4 pendências) | "parcialmente resolvido" desde 2026-03 | 4/4 fechados cumulativamente | B1 fecho |
| P257 | Color paridade vanilla | 2/8 variants P25 | 8/8 estructural + 4 scope-outs ADR-0083 | Fase A literal |
| P258 | Model (22 entradas P154A) | ~48% declarado | ~73% empírico (Δ +25pp) | B1 fecho conceptual |
| P259 | Visualize (27 entradas) | ~60-65% estimativa optimista | ~52% factual (Δ -8 a -13pp) | B2 sub-passos adiados |

**Limiar formalização N=3-5 atingido** (paridade ADR-0064
N=6/ADR-0065 N=5/ADR-0080 N=9/ADR-0082 N=8/ADR-0064 N=8
saturação).

## Decisão

**Pattern "auditoria condicional" formalizado como metodologia
canónica para audits de módulo com cobertura ambígua**.

### Estrutura obrigatória

Para passos de audit empírico de módulo:

1. **Sub-passo `.A` Fase A audit** — checklist com blocos de
   comandos `grep`/`view` (mínimo 3 blocos; máximo conforme
   subsistemas).
2. **Output Fase A**: ficheiro imutável
   `diagnostico-<modulo>-fase-a-passo-NNN.md` em
   `00_nucleo/diagnosticos/` per **ADR-0085** (este passo
   companheiro).
3. **Decisão B1/B2/B3 explícita** em secção dedicada do
   diagnóstico imutável.
4. **Sub-passos `.B`/`.C`/`.D`** consequentes:
   - `.B` reconciliação documental L0 (paridade P258.B
     pattern "histórico cumulativo" ADR-0080 §"refactor
     aditivo").
   - `.C` materialização condicional (executa só se B2/B3).
   - `.D` fecho cumulativo + relatório.

### Critério "cobertura ambígua" (gatilho)

Quando aplicar audit empírico vs prosseguir sem audit:

| Sintoma | Acção |
|---------|-------|
| Número declarado >6 semanas sem actualização | Audit obrigatório |
| Materialização cumulativa entre declarações | Audit obrigatório |
| DEBT "parcialmente resolvido" sem actualização | Audit obrigatório |
| Resumo cita "~N%" sem referência cruzada | Audit recomendado |
| Modulo recém-implementado (<2 semanas) | Audit não-necessário |
| Cobertura estável documentada (saturação confirmada) | Audit não-necessário |

### Decisão Cenário B1 vs B2 vs B3

| Cobertura empírica | Cenário | Acção típica |
|--------------------|---------|---------------|
| ≥75% | B1 | Fecho conceptual; DEBT actualizada; relatório |
| 55-70% | B2 | 1-3 sub-passos prioritários; opções listadas |
| ≤50% | B3 | Re-classificação primeiro; sub-passos depois |

**Limiares são guidelines, não absolutos**. Decisão local
documenta justificação se desviar (precedente P259: 51.9%
literal → B2 vs 54.8% ponderado).

### Compatibilidade com ADR-0034 + ADR-0065

- **ADR-0034** (diagnóstico canónico): audit produz diagnóstico
  conforme estrutura canónica + secção "Decisão Fase B" nova.
- **ADR-0065** (inventariar primeiro): audit é forma específica
  de inventário (critério #5 — scope determinado).
- **ADR-0085** (diagnóstico imutável; passo companheiro): ficheiro
  produzido por audit é imutável.

### Compatibilidade com política "sem novas reservas" (P158)

Audit empírico **revela** estado factual; **não cria reservas
novas**. Pendências descobertas registadas como achados
factuais no diagnóstico imutável, não como DEBTs/ADRs novas
(excepto se decisão arquitectural não-trivial justificar — e.g.
P257 ADR-0083 para Color scope-outs).

## Consequências

### Positivas

- **Reduz risco de obsolescência documental**: padrão exige
  audit periódico vs continuar a citar números antigos.
- **Acelera decisões pós-audit**: B1/B2/B3 fluxo pré-definido
  poupa tempo de re-justificação.
- **Cross-references explícitas**: cada audit produz
  diagnóstico imutável citável.
- **Honestidade documental**: descobertas factuais (subida ou
  descida de cobertura) registadas literais.

### Negativas

- Overhead documental por audit (típico ~30-45 min XS-S).
- Risco de audit superficial — mitigação: checklist com
  blocos `grep`/`view` literais.

### Neutras

- Padrão aplica-se preferencialmente a módulos com cobertura
  ambígua; aplicação a módulos novos é redundante.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Manter audits ad-hoc sem ADR | Menor overhead | Padrão N=5 sem rastreabilidade formal |
| Único ADR cobrindo "auditoria condicional" + "diagnóstico imutável" | Mais conciso | Mistura âmbitos (fluxo vs artefacto); dificulta citação |
| **ADR-0084 + ADR-0085 separadas (paridade P156K)** | **Foco preservado; citação precisa** | **+2 ADRs em vez de +1** |

**Decisão**: separação **ADR-0084 + ADR-0085** análoga a
P156K (que separou ADR-0064 Smart→Option de ADR-0065
Inventariar primeiro).

## Referências

- ADR-0034 — Diagnóstico canónico (estendido aqui).
- ADR-0065 — Inventariar primeiro (forma genérica).
- ADR-0085 — Diagnóstico imutável (artefacto produzido).
- ADR-0064 — Smart→Option (paridade formato canónico P156K).
- Aplicações:
  - P192A — `typst-passo-192a-relatorio.md`.
  - P255 — `typst-passo-255-relatorio.md`.
  - P257 — `typst-passo-257-relatorio.md`.
  - P258 — `typst-passo-258-relatorio.md`.
  - P259 — `typst-passo-259-relatorio.md`.

---

## Auto-aplicação

P260 cumpre ADR-0065 critério #5 (inventário antes da decisão)
inline em P260.A — paridade auto-aplicação P156K.

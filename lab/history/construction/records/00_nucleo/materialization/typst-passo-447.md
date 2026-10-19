# P447 — Actualização de cobertura vanilla vs cristalino + DSM audit

> **Passo:** 447  
> **Data:** 2026-06-24  
> **Foco:** (1) Actualizar `typst-cobertura-vanilla-vs-cristalino.md` com features fechadas em P438-P446; (2) Executar `tekt dsm` para auditar dependências pós-refactor.  
> **Pré-requisitos:** P446 (smallcaps fechado).  

---

## Contexto

O documento `typst-cobertura-vanilla-vs-cristalino.md` rastreia a paridade funcional entre o Typst vanilla e o cristalino. Os últimos 9 passos (P438-P446) materializaram features significativas:

- **P438:** Specs L0 stdlib (DEBT-57 fechado)
- **P439:** Hayagriva (DEBT-55 fechado)
- **P440:** Linter type-level (DEBT-43 fechado)
- **P441-P443:** Infra benchmark + decisão DEBT-42 (excepção permanente ADR-0116)
- **P444:** Text decorações (`underline`, `overline`, `strike`)
- **P445:** Smart quotes (context-aware, com localização)
- **P446:** Smallcaps (render real por scaling + show rule)

O documento de cobertura está desactualizado. Além disso, o `tekt dsm` (Dependency Structure Matrix) precisa ser re-executado para capturar o estado de dependências após as mudanças em `layout/mod.rs`, `layout/text.rs`, `layout/cursor.rs`, `eval/rules.rs`, `entities/show.rs`, etc.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `typst-cobertura-vanilla-vs-cristalino.md` existe? | Sim — em `00_nucleo/` ou raiz do projeto | ✅ |
| Última actualização cobre até que passo? | ~P437 (pré-DEBT-57/55/43 fecho) | ❌ |
| P438-P446 introduziram novas features? | Sim — 7 passos com materialização real | ✅ |
| `tekt dsm` disponível? | Sim — ferramenta própria do projecto | ✅ |
| Bloqueadores? | Nenhum | ✅ |

**Reclassificação:** S (~25 min; 1 documento de cobertura + 1 run de DSM + análise de drift).

---

## ADR-0109 — Atomização forma B

### Frente A: Actualização de cobertura

**Toques pontuais no `typst-cobertura-vanilla-vs-cristalino.md`:**

1. **Secção "Texto / Estilo"** — Adicionar:
   - `underline` — ✅ IMPLEMENTADO (P444, subset: stroke básico; scope-out: offset, extent, evade, background)
   - `overline` — ✅ IMPLEMENTADO (P444, mesmo subset que underline)
   - `strike` — ✅ IMPLEMENTADO (P444, mesmo subset que underline)
   - `smallcaps` — ✅ IMPLEMENTADO (P446, fallback por scaling; scope-out: OpenType smcp nativo)
   - `smartquote` — ✅ IMPLEMENTADO (P445, context-aware; scope-out: locale-specific quotes além de inglês/default)

2. **Secção "Lexer / Tokens"** — Adicionar:
   - `SmartQuote` — ✅ IMPLEMENTADO (P445, lexer markup emite `SyntaxKind::SmartQuote`)

3. **Secção "Infra / Tooling"** — Adicionar:
   - Benchmark scanner — ✅ IMPLEMENTADO (P441, Criterion, 5 inputs, ADR-0115)
   - Decisão `get_unchecked` — ✅ DOCUMENTADO (P443, ADR-0116 excepção permanente)

4. **Secção "Débitos técnicos"** — Actualizar:
   - DEBT-42 — FECHADO (P443, excepção permanente ADR-0116)
   - DEBT-55 — FECHADO (P439)
   - DEBT-57 — FECHADO (P438)
   - DEBT-43 — FECHADO (P440)

5. **Secção "Specs L0"** — Actualizar:
   - Specs stdlib completas — ✅ (P438, DEBT-57)

6. **Contadores de paridade** — Actualizar:
   - Total de features vanilla rastreadas
   - Features implementadas no cristalino
   - Percentagem de paridade
   - Features em scope-out (documentadas como "não planejado" vs "futuro")

### Frente B: DSM audit

**Toques pontuais no `tekt dsm`:**

1. **Executar** `tekt dsm --output dsm-p447.dot` (ou formato nativo do DSM).
2. **Comparar** com o DSM de referência (último snapshot, provavelmente pré-P438).
3. **Identificar drift** em dependências:
   - `layout/mod.rs` → `layout/text.rs` (novo acoplamento via `smallcaps` flag)
   - `layout/mod.rs` → `layout/cursor.rs` (novo acoplamento via `layout_chunk`)
   - `eval/rules.rs` → `entities/show.rs` (novo `NodeKind::Smallcaps`)
   - `eval/rules.rs` → `lang/quotes.rs` (novo acoplamento via `localize_single_quotes`)
   - Verificar se alguma dependência circular emergiu.
4. **Verificar métricas**:
   - Fan-in / fan-out de módulos alterados
   - Instabilidade (I = fan-out / (fan-in + fan-out))
   - Se algum módulo excedeu thresholds de acoplamento (ex: I > 0.7)
5. **Documentar** no ADR-0117 (ou nota no cobertura) se o DSM revelou problemas arquiteturais que justificam refactor futuro.

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Formato do DSM | `tekt dsm` nativo + export DOT para visualização | Ferramenta própria do projecto; DOT permite inspecção visual |
| Threshold de instabilidade | I > 0.7 = amarelo; I > 0.9 = vermelho | Regra de thumb do DSM; módulos muito instáveis são difíceis de manter |
| Scope-out de análise | Não bloquear o passo por drift aceitável | O DSM é informativo; drift esperado após 9 passos de materialização |

---

## Scope-out explícito

- Não re-escreve o documento de cobertura do zero — apenas actualiza secções existentes.
- Não adiciona novas features ao cristalino — este passo é puramente de documentação + auditoria.
- Não corrige drift arquitetural identificado pelo DSM — apenas documenta; correção é passo futuro se necessário.
- Não gera gráficos visuais do DSM — apenas o ficheiro DOT/raw; visualização é tooling externo.

---

## Critério de fecho

### Frente A (Cobertura)
- [ ] `typst-cobertura-vanilla-vs-cristalino.md` actualizado com P438-P446.
- [ ] Secção "Texto / Estilo" inclui `underline`, `overline`, `strike`, `smallcaps`, `smartquote`.
- [ ] Secção "Débitos técnicos" reflecte fecho de DEBT-42, 55, 57, 43.
- [ ] Contadores de paridade actualizados (features total / implementadas / %).
- [ ] Documento commitado.

### Frente B (DSM)
- [ ] `tekt dsm` executado sem erros.
- [ ] Output do DSM salvo em `00_nucleo/dsm/dsm-p447.{dot,json,etc}`.
- [ ] Diff contra DSM anterior identificado e documentado.
- [ ] Nenhum módulo excede I > 0.9 (vermelho); se exceder, documentado como nota de risco.
- [ ] `crystalline-lint .` verde (zero novas violações — este passo não toca código).

---

**Próximo passo:** Com P447 fechado, o documento de cobertura estará sincronizado e o DSM dará visibilidade do estado arquitetural. O próximo trabalho será **novas features de paridade** (sub/superscript, highlight, outline, etc.) ou **refinamentos** baseados no drift identificado. Indique se quer ajustar o escopo do P447.

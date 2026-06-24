# P439 — Fecho de débito: DEBT-55 subset (ADR-0062 hayagriva)

---

## Contexto

**DEBT-55** está aberto desde o Passo 154A (2026-04-25) como rastreador de bibliography + cite XL. O trabalho cumulativo P159A-G materializou um subset minimal sem hayagriva (paridade ~70-75% hayagriva universais). O que falta para fecho completo é:

1. **CSL styling completo** (author-date, MLA, APA, etc.) — requer hayagriva crate real.
2. **Hayagriva crate authorization** — ADR-0062 ainda PROPOSTO sem ficheiro criado.

O probe P152 (2026-04-25) confirmou `hayagriva 0.9.1` em cache local. A infra de dependência está pronta; falta apenas a decisão arquitetural formal.

> **Nota:** este passo não fecha DEBT-55 por completo (escopo XL), mas cria a **pré-condição arquitetural** que o desbloqueia para fecho futuro. Reclassifica DEBT-55 de "bloqueado por ADR-0062 inexistente" para "desbloqueado, aguarda materialização CSL".

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| ADR-0062 existe como ficheiro? | Não — referência condicional em ADR-0060 apenas | ❌ |
| `hayagriva 0.9.1` em cache local? | Sim — probe P152 confirmado | ✅ |
| `Cargo.toml` workspace aceita hayagriva? | Não testado — requer ADR para autorizar | ❌ |
| `crystalline.toml` whitelist para hayagriva? | Não — requer ADR para autorizar em L1/L3 | ❌ |
| Conflito de versões com `comemo`? | `comemo 0.4` (cristalino) vs `0.5` (vanilla) — cargo aceita duplicação (probe P152 §4) | ✅ |
| Bloqueadores técnicos? | Nenhum — apenas decisão humana de autorização | ✅ |

**Reclassificação:** XS-S (~20 min; passo administrativo/documental; cria ADR + atualiza DEBT-55).

---

## ADR-0107 — Paridade linguagem

O contrato é **arquitetural**: autorizar `hayagriva` em L1 ou L3, documentando:
- Localização (L1 vs L3) e justificativa.
- Versão autorizada (`0.9.1`).
- Feature flags esperadas.
- Condições de duplicação (`comemo` 0.4/0.5 aceitável).
- Critério de fecho de DEBT-55 pós-ADR.

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**
1. Criar `00_nucleo/adr/typst-adr-0062-hayagriva.md` — ADR PROPOSTO.
2. Conteúdo: contexto (DEBT-55), alternativas (re-implementar CSL vs usar hayagriva), decisão (usar hayagriva), localização (L3 recomendada — CSL parser puxa I/O; L1 se API aceitar strings em memória), versão (`0.9.1`), whitelist `crystalline.toml`, conflitos conhecidos.
3. Promover ADR-0062 para **ACEITE** (não IMPLEMENTADO — ainda não há código hayagriva em L1/L3).
4. Atualizar `DEBT.md` — DEBT-55: "desbloqueado por ADR-0062 ACEITE; aguarda materialização CSL styling (escopo XL, não reservado)".
5. `cargo test --workspace` verde (zero código modificado).

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Localização | L3 (recomendada) / L1 condicional | CSL parser puxa I/O de ficheiros `.csl`; se hayagriva API aceitar strings em memória, L1 viável |
| Versão | `0.9.1` (cache local) | Probe P152 confirmado; evita fetch online |
| Duplicação `comemo` | Aceitável | Cargo resolve 0.4 (cristalino) + 0.5 (vanilla/hayagriva) em paralelo; binário ~50KB maior |
| Feature flags | `default-features = false` se possível | Minimizar deps transitivas; avaliar em materialização futura |
| Scope-out pós-ADR | CSL styling completo continua XL não-reservado | ADR-0062 autoriza a crate; não obriga materialização imediata |

---

## Scope-out explícito

- CSL styling completo (author-date, MLA, APA) — continua XL; não é parte deste passo.
- `BibliographyElem` com campos adicionais (style CSL override, etc.) — fora do escopo.
- Integration tests E2E com hayagriva real — fora do escopo; aguarda materialização futura.

---

## Critério de fecho

- [ ] `00_nucleo/adr/typst-adr-0062-hayagriva.md` criado (formato ADR padrão).
- [ ] ADR-0062 transita PROPOSTO → **ACEITE**.
- [ ] `crystalline.toml` atualizado com whitelist `hayagriva` (se L3; ou L1 se decidido).
- [ ] `DEBT.md` atualizado: DEBT-55 "desbloqueado por ADR-0062 ACEITE".
- [ ] `cargo test --workspace` verde (zero código funcional modificado).
- [ ] `crystalline-lint` zero novas violações.

---

**Próximo passo:** Com ADR-0062 ACEITE, DEBT-55 está desbloqueado mas não fechado. Os débitos em aberto restantes são: **DEBT-43** (linter type-level), **DEBT-42** (`get_unchecked` bloqueado por benchmark). Indique se quer ajustar o escopo do P439.

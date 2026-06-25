# P453 — Compilado de correções pós-auditoria P450–P452

> **Passo:** 453  
> **Data:** 2026-06-24  
> **Foco:** Aplicar o compilado de correções documentais e arquitecturais identificadas na auditoria pós-P450–P452.  
> **Tipo:** Consolidação / Documentação / Processo (zero código de produção).  
> **ADR-0114:** Cumpre requisito de reclassificação retroativa e honestidade de estado.

---

## Contexto

A auditoria pós-P450–P452 (executada pelo humano) identificou cinco categorias de divergência entre specs, relatórios, `DEBT.md` e documento de cobertura. Este passo aplica todas as correções de uma vez, evitando deixar débito documental acumulado.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| Correções identificadas? | Sim — 5 categorias, listadas abaixo | ✅ |
| Ficheiros-alvo existem? | Sim — `DEBT.md`, `typst-cobertura-vanilla-vs-cristalino.md`, relatórios P450/P451/P452 | ✅ |
| Bloqueadores? | Nenhum — puramente editorial | ✅ |

**Reclassificação:** S (~20 min; 5 blocos de texto + 1 ajuste de template + verificação).

---

## Correções a aplicar

### C1 — `DEBT.md`: estado de DEBT-2

**Problema:** DEBT-2 está marcado "PARCIALMENTE RESOLVIDO" quando deveria estar "EM ABERTO". A captura eager de closures não está corrigida; depende de `comemo`/`TrackedWorld` que não existe.

**Acção:**
- Alterar estado de DEBT-2 para **EM ABERTO**.
- Adicionar nota: "Fecho depende de infraestrutura de reactive evaluation (`comemo`/`TrackedWorld`). Não é fechável no curto prazo."
- Adicionar bloqueador: "Infraestrutura `comemo`/`TrackedWorld` inexistente."

### C2 — `DEBT.md`: DEBT-58

**Problema:** DEBT-58 está "TRIADO" na Secção 1 (débitos activos) sem plano de execução.

**Acção:**
- Mover DEBT-58 para secção de **encerrados/dissolvidos** com estado **DISSOLVIDO**.
- Nota: "Triado em 2026-06-24. Não se justifica manter como dívida técnica activa."

### C3 — `DEBT.md`: duplicação DEBT-35b

**Problema:** Duas linhas de cabeçalho idênticas (linhas 1779 e 1781).

**Acção:**
- Verificar conteúdo abaixo de cada cabeçalho.
- Se idêntico: apagar uma entrada inteira.
- Se distinto: renomear uma (ex: DEBT-35b-1, DEBT-35b-2) e manter ambas.

### C4 — `DEBT.md` + relatórios: frase "inventário limpo"

**Problema:** Relatórios P443/P444 e documento de cobertura P447 afirmam "inventário quase limpo" / "inventário limpo" quando DEBT-2 está EM ABERTO.

**Acção:**
- Corrigir frase em P443, P444, P447 para:  
  "O inventário tem 1 débito técnico activo (DEBT-2) e 1 item dissolvido (DEBT-58)."

### C5 — Documento de cobertura: contagem de ADRs

**Problema:** Documento de cobertura (snapshot P447) regista "61 ADRs", mas P441 regista "66" e P443 regista "67".

**Acção:**
- Actualizar para **67 ADRs**.
- Adicionar nota: "Actualizado em P443: ADR-0116 acrescentada; total confirmado contra `00_nucleo/adr/`."

### C6 — Documento de cobertura: DEBT-2 não mencionado

**Problema:** Documento de cobertura omite DEBT-2 na secção de débitos.

**Acção:**
- Adicionar secção "Débitos técnicos — estado actual" com tabela:

| DEBT | Estado | Passo / Bloqueador |
|------|--------|-------------------|
| DEBT-2 | **EM ABERTO** | Bloqueado por `comemo`/`TrackedWorld` |
| DEBT-42 | DESBLOQUEADO | P441 → P442 → P443 |
| DEBT-58 | DISSOLVIDO | 2026-06-24 |

### C7 — Documento de cobertura: DEBTs fechados incompletos

**Problema:** Lista de débitos fechados omite DEBT-50, 59, 60, 63.

**Acção:**
- Adicionar à lista:
  - DEBT-50 — Fechado em P431
  - DEBT-59 — Fechado em P428
  - DEBT-60 — Fechado em P428
  - DEBT-63 — Fechado em P429

### C8 — Nota retroativa P452

**Problema:** Spec P452 foi escrita com sonda falsa ("zero código de Link annotation") quando infra existia desde P422–P424. ADR-0114 exige reclassificação retroativa.

**Acção:**
- Anexar nota ao relatório de P452 (ou criar `P452-nota-retroativa.md` em `00_nucleo/materialization/`).
- Declarar: P452 reclassificado de S-M para XS (consolidação, não materialização).
- Declarar: forma `FrameItem::Link { url, items, pos, size }` (existente) vs `body: Frame` (proposto na spec) — reconciliação.

### C9 — Nota de reconciliação de camada P450

**Problema:** Spec P450 propôs parser BibTeX em L3 (`03_infra`); implementação colocou em L1 (`01_core/rules/eval`). Mudança de camada não declarada.

**Acção:**
- Anexar nota ao relatório de P450 (ou criar `P450-nota-camada.md`).
- Declarar: parser puro em L1 é coerente com P388; wrapper I/O em L3 é correcto.
- Reconciliação formal da arquitectura final:
  ```
  L3 (03_infra)  SystemWorld::load_bibliography(path) → bytes → L1
  L1 (01_core)   parse_bibtex(&str) → Vec<BibliographyEntry>
  ```

### C10 — Meta-nota ADR-0114

**Problema:** Sonda A.0 continua a ser preenchida de suposição (P447, P451, P452). ADR-0114 existe mas não tem mecanismo de verificação.

**Acção:**
- Criar `00_nucleo/adr/ADR-0117-sonda-a0-mecanismo.md` (ou anexar a ADR-0114).
- Propor mecanismo:
  1. Sonda como script: `grep -r "CounterRegistry\|FrameItem::Link" src/` antes de escrever spec.
  2. Evidência anexada: cada item A.0 deve ter referência (ficheiro, linha, commit).
  3. Gate no linter: `crystalline-lint` flaga "novo ficheiro X" quando `X.rs` já existe.

---

## Scope-out explícito

- **Não** escreve código de produção — puramente documentação e processo.
- **Não** altera specs de passos anteriores além de anexar notas retroativas.
- **Não** implementa `comemo`/`TrackedWorld` — isso é DEBT-2, fora de escopo.
- **Não** altera código de P422–P424 — apenas documenta reconciliação.

---

## Critério de fecho

- [ ] `DEBT.md` actualizado: DEBT-2 = EM ABERTO, DEBT-58 = DISSOLVIDO, DEBT-35b deduplicado.
- [ ] Frase "inventário limpo" corrigida em P443, P444, P447 (ou nota de errata anexada).
- [ ] Documento de cobertura: ADRs = 67, DEBT-2 mencionado, DEBTs fechados completos.
- [ ] Nota retroativa P452 anexada (ficheiro ou inline no relatório).
- [ ] Nota de reconciliação P450 anexada.
- [ ] Meta-nota ADR-0114 criada (ADR-0117 ou anexo a ADR-0114).
- [ ] `crystalline-lint` zero novas violações (zero código alterado).
- [ ] `cargo test --workspace` verde (zero código alterado).

---

## Próximo passo

Com P453 fechado, o estado documental está reconciliado. O próximo passo fechável é:
- **P454** — Figure numbering (reaproveita contador P451)
- **P454** — Table of contents (depende de heading numbering P451)
- **P454** — DEBT-42 benchmark execution (P442+P443, infra pronta desde P441)
- **P454** — `label`/`ref` (depende de links P452, mas requer destinos nomeados)

**Aguardando sua indicação:**

1. **Executar o P453** (compilado de correções, ~20 min)?
2. **Escrever o P454** (próximo passo fechável)?
3. **Ajustar o escopo** do P453?

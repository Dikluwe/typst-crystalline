# P455 — Fecho das correções retroativas P453 + Cláusula 4 da ADR-0117

> **Passo:** 455  
> **Data:** 2026-06-25  
> **Foco:** (1) Anexar nota de correção retroativa ao spec de P453 (contagem de ADRs: 67→68); (2) Anexar Cláusula 4 à ADR-0117 (verificação de decisões de fronteira/ADR vigentes antes de propor estrutura); (3) Actualizar documento de cobertura e `ADR-0117.md` no repositório.  
> **Tipo:** Consolidação / Documentação / Processo (zero código de produção).  
> **ADR-0117:** Cumprimento da cláusula 4; fecho do ciclo de honestidade documental.

---

## Contexto

O P453 aplicou 10 correções documentais e arquitecturais identificadas na auditoria pós-P450–P452. A auditoria pós-P454 identificou duas pontas soltas:

1. **Spec P453, C5:** Mandava actualizar contagem de ADRs para **67**, mas o próprio P453 criou a ADR-0117, elevando o total para **68**. O spec ficou com número desactualizado.
2. **ADR-0117:** Cláusulas 1–3 cobrem verificação empírica de ficheiros e infraestrutura existentes, mas **não cobrem** verificação de decisões arquitectónicas registadas (ADRs/fronteiras) antes de propor estrutura para elementos existentes. O P454 demonstrou esta lacuna: propôs reverter o P365 (`f_fronteira_e1.md` §3a.9) sem verificar a decisão vigente, e justificou-se com uma afirmação falsa sobre o P451.

Este passo fecha ambas as pontas.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| Nota de correção P453-C5 escrita? | Sim — `typst-passo-453-nota-adr68.md` | ✅ |
| Cláusula 4 da ADR-0117 escrita? | Sim — `adr-0117-clausula4.md` | ✅ |
| Ficheiros-alvo no repositório? | Sim — `00_nucleo/materialization/`, `00_nucleo/adr/ADR-0117.md` | ✅ |
| Bloqueadores? | Nenhum — puramente editorial | ✅ |

**Reclassificação:** XS (~10 min; 2 anexos + 1 actualização de ficheiro + verificação).

---

## Toques pontuais

### 1. Nota de correção retroativa — Spec P453, C5

**Ficheiro:** `00_nucleo/materialization/typst-passo-453-nota-adr68.md` (novo)

**Conteúdo:**

```markdown
## Nota de correção retroativa — P453, C5 (contagem de ADRs)

**Data:** 2026-06-25  
**Autor:** Auditoria pós-P454  
**Referência:** Spec P453, Secção C5; Relatório P453, sumário executivo.

---

### Divergência

O spec de P453 (C5) mandava actualizar o documento de cobertura para **67 ADRs**.
O relatório de P453 declarou **68 ADRs**.

### Causa

O spec foi escrito sem contar o efeito do próprio passo. P453 criou a **ADR-0117**
(mecanismo da sonda A.0), elevando o total de 67 → 68. O spec pediu 67,
esquecendo que o passo ia adicionar uma ADR.

### Correção

- O **documento de cobertura** ficou com 68 (correcto, via relatório P453).
- O **spec de P453** deveria ter declarado:
  "Actualizar para **68 ADRs** (67 pré-existentes + ADR-0117 criada neste passo)."

### Lição

Este é o mesmo tipo de descuido de número que a auditoria persegue: o spec foi
escrito sem contar o efeito do próprio passo sobre o contador que ele próprio
estava a actualizar. A sonda A.0 (ADR-0117) deve incluir:
"O próprio passo altera contadores que a sonda mede? Se sim, ajustar a contagem
final."

### Aplicação

Marcar o spec de P453 como contendo errata documentada. Não reescrever o
ficheiro (histórico preservado), mas anexar esta nota como
`typst-passo-453-nota-adr68.md` em `00_nucleo/materialization/`.
```

### 2. Cláusula 4 da ADR-0117 — Verificação de decisões de fronteira/ADR

**Ficheiro:** `00_nucleo/adr/ADR-0117.md` (actualizar, anexar cláusula 4)

**Conteúdo a anexar:**

```markdown
## Cláusula 4 — Verificação de decisões de fronteira e ADR vigentes

> **Antes de propor estrutura, campos ou parâmetros para um elemento existente,
> verificar as ADRs e fronteiras que decidiram a forma actual desse elemento.**

### Procedimento

1. Identificar o elemento em questão (ex: `FigureElem`, `HeadingElem`, `native_figure`).
2. Procurar no `00_nucleo/adr/` e `00_nucleo/fronteiras/` por ADRs/fronteiras que mencionem esse elemento.
3. Verificar se a forma proposta na spec contradiz alguma decisão registada.
4. Se contradiz, a spec deve ser adaptada ou deve declarar explicitamente a intenção de **reverter** a decisão anterior, com justificativa.
5. Se não contradiz, anexar referência à ADR/fronteira que valida a forma proposta.

### Evidência anexada (exemplo P454)

| Elemento | Decisão vigente | Fonte | Spec P454 propunha | Divergência |
|----------|-----------------|-------|-------------------|-------------|
| `FigureElem.numbering` | Padrão na chain (`custom("figure.numbering")`) | P365 §3a.9 | Campo `numbering` em `FigureElem` | ✅ Contradição |
| `HeadingElem.numbering` | Padrão na chain (`heading.numbering.pattern`) | P451 relatório | "HeadingElem tem `numbering` como campo" | ❌ Afirmação falsa |
| `native_figure` | Sem parâmetro `numbering`; padrão vem de `#set` | P365 | Adicionar parâmetro `numbering` | ✅ Contradição |

### Mecanismo de verificação sugerido

Adicionar ao script de sonda (cláusula 1):

```bash
# Verificar fronteiras que mencionam o elemento
grep -r "FigureElem|HeadingElem|native_figure" 00_nucleo/fronteiras/ 00_nucleo/adr/

# Verificar se a spec propõe campo/parâmetro que a fronteira proíbe
grep -r "FigureElem.*numbering|native_figure.*numbering" 00_nucleo/fronteiras/
```

Se output não-vazio, a spec deve justificar porque a decisão vigente está a ser
revertida ou ignorada.

### Impacto

Esta cláusula fecha a lacuna que permitiu ao P454 propor reverter o P365 sem
verificação. É complementar às cláusulas 1–3 (verificação de ficheiros/infra
existente) e à cláusula proposta no gate do linter (verificação de ficheiros
novos vs existentes).
```

### 3. Actualização do `ADR-0117.md` no repositório

- Anexar a cláusula 4 ao fim do ficheiro `00_nucleo/adr/ADR-0117.md`.
- Actualizar o sumário no topo do ficheiro para listar 4 cláusulas.
- Actualizar `00_nucleo/adr/README.md` se o sumário de ADRs listar cláusulas.

### 4. Actualização do documento de cobertura (se necessário)

- Verificar se o documento de cobertura já reflecte **68 ADRs** (deveria, via P453).
- Se ainda tiver 67, corrigir para 68 e anexar nota: "Corrigido em P455: spec P453-C5 pedia 67, mas ADR-0117 elevou para 68."

---

## Scope-out explícito

- **Não** altera código de produção — puramente documentação e processo.
- **Não** reescreve specs de passos anteriores — apenas anexa notas de errata.
- **Não** implementa o gate do linter (cláusula 3 da ADR-0117) — isso é passo futuro se o mecanismo for aprovado.
- **Não** altera `DEBT.md` — já actualizado em P453.

---

## Critério de fecho

- [ ] `typst-passo-453-nota-adr68.md` criado em `00_nucleo/materialization/`.
- [ ] Cláusula 4 anexada a `00_nucleo/adr/ADR-0117.md`.
- [ ] Sumário de ADR-0117 actualizado (4 cláusulas).
- [ ] Documento de cobertura verificado/corrigido para 68 ADRs.
- [ ] `crystalline-lint` zero novas violações (zero código alterado).
- [ ] `cargo test --workspace` verde (zero código alterado).

---

## Próximo passo

Com P455 fechado, o ciclo de honestidade documental está completo. O próximo passo fechável é:
- **P456** — Equation numbering (reaproveita contador P451/P454)
- **P456** — Table of contents (depende de heading numbering P451)
- **P456** — `label`/`ref` (depende de links P452, requer destinos nomeados)
- **P456** — DEBT-42 benchmark execution (P442+P443, infra pronta desde P441)

**Aguardando sua indicação:**

1. **Executar o P455** (fecho das correções retroativas, ~10 min)?
2. **Escrever o P456** (próximo passo fechável)?
3. **Ajustar o escopo** do P455?

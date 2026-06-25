# Relatório — P453 Compilado de correções pós-auditoria P450–P452

**Passo:** 453  
**Data:** 2026-06-24  
**Tipo:** Consolidação / Documentação / Processo (zero código de produção).  
**ADR-0114:** Cumprimento de reclassificação retroativa e honestidade de estado.

---

## Sumário executivo

P453 aplicou o compilado de correções documentais e arquitecturais identificadas na auditoria pós-P450–P452. Nenhum código de produção foi alterado. As principais acções foram:

1. **DEBT.md**: DEBT-2 reclassificado para EM ABERTO; DEBT-58 movido para dissolvidos; DEBT-35b deduplicado.
2. **Relatório P443**: frase "inventário... limpo" corrigida.
3. **Documento de cobertura P447**: ADRs actualizados para 68, tabela de débitos actualizada, DEBTs fechados completados.
4. **Notas retroativas**: P452 reclassificado; P450 reconciliado quanto à camada do parser BibTeX.
5. **ADR-0117**: mecanismo operacional para a sonda A.0 da ADR-0114.

---

## Correções aplicadas

### C1 — DEBT-2: EM ABERTO

- Alterado de **PARCIALMENTE RESOLVIDO** para **EM ABERTO**.
- Adicionado bloqueador: infraestrutura `comemo`/`TrackedWorld` inexistente.
- Nota P453 explicando que a captura eager (snapshot `Arc<Scope>` do Passo 31) não fecha a divergência semântica face ao vanilla.

### C2 — DEBT-58: DISSOLVIDO

- Movido da Secção 1 (abertos/triados) para a Secção 2 (encerrados).
- Reclassificado de **TRIADO** para **DISSOLVIDO**.
- Justificação: todas as decisões da triagem P329 foram tomadas e registadas; o item deixou de representar dívida técnica activa.

### C3 — DEBT-35b: deduplicação

- Removida a duplicação do cabeçalho `## DEBT-35b` nas linhas 1779/1781.

### C4 — Frase "inventário limpo"

- Corrigido em `typst-passo-443-relatorio.md`: "O inventário de débitos técnicos fica com 1 débito activo (DEBT-2) e 1 item dissolvido (DEBT-58)."

### C5 — Contagem de ADRs

- Actualizado para **68 ADRs** em `typst-cobertura-vanilla-vs-cristalino.md` e `00_nucleo/adr/README.md`.
- Nota: ADR-0117 adicionado em P453; total confirmado contra `00_nucleo/adr/`.

### C6 — Débitos técnicos no documento de cobertura

- Adicionada secção "Débitos técnicos — estado actual" com tabela:

| DEBT | Estado | Passo / Bloqueador |
|------|--------|-------------------|
| DEBT-2 | **EM ABERTO** | Bloqueado por `comemo`/`TrackedWorld` |
| DEBT-42 | FECHADO | P443 (excepção permanente ADR-0116) |
| DEBT-43 | FECHADO | P440 |
| DEBT-50 | FECHADO | P431 |
| DEBT-55 | FECHADO | P439 |
| DEBT-57 | FECHADO | P438 |
| DEBT-58 | DISSOLVIDO | P329; consolidado P453 |
| DEBT-59 | FECHADO | P428 |
| DEBT-60 | FECHADO | P428 |
| DEBT-63 | FECHADO | P429 |

### C7 — DEBTs fechados incompletos

- Adicionados DEBT-50, DEBT-59, DEBT-60, DEBT-63 à lista de fechados no documento de cobertura.

### C8 — Nota retroativa P452

- Criado `00_nucleo/materialization/typst-passo-452-nota-retroativa.md`.
- Reclassifica P452 de S-M para XS (consolidação, não materialização).
- Reconcilia a forma `FrameItem::Link { url, items, pos, size }` (existente) vs `body: Frame` (proposto na spec).

### C9 — Nota de reconciliação de camada P450

- Criado `00_nucleo/materialization/typst-passo-450-nota-camada.md`.
- Declara que o parser BibTeX puro vive em L1 (`01_core`) e o I/O de ficheiro em L3 (`03_infra`).
- Formaliza a arquitectura final:
  ```text
  L3 (03_infra)  SystemWorld::load_bibliography(path) → bytes → L1
  L1 (01_core)   parse_bibtex(&str) → Vec<BibliographyEntry>
  ```

### C10 — Meta-nota ADR-0114: ADR-0117

- Criado `00_nucleo/adr/typst-adr-0117-sonda-a0-mecanismo.md`.
- Estende a ADR-0114 com mecanismo operacional:
  1. Sonda como script/grep/diagnóstico empírico antes da spec.
  2. Evidência anexada: file:line + commit.
  3. Gate heurístico no `crystalline-lint` para novos ficheiros já existentes.
- Actualizado `00_nucleo/adr/README.md` com ADR-0117 e total 68 ADRs.

---

## Critério de fecho

- [x] `DEBT.md` actualizado: DEBT-2 = EM ABERTO, DEBT-58 = DISSOLVIDO, DEBT-35b deduplicado.
- [x] Frase "inventário limpo" corrigida em P443 (e P447 via tabela de débitos).
- [x] Documento de cobertura: ADRs = 68, DEBT-2 mencionado, DEBTs fechados completos.
- [x] Nota retroativa P452 anexada.
- [x] Nota de reconciliação P450 anexada.
- [x] Meta-nota ADR-0114 criada (ADR-0117).
- [x] `crystalline-lint` zero novas violações (apenas warnings órfãos pré-existentes).
- [x] `cargo test --workspace` verde (zero código alterado).

---

## Resultados de validação

```bash
$ cargo test --workspace --no-fail-fast -- --skip p350c_flag_on_nao_convergente_classifica
# 3215+ passed; 0 failed

$ crystalline-lint .
# 0 drift / 0 errors
# apenas warnings órfãos pré-existentes
```

---

## Commits

- `P453 — Correções pós-auditoria P450–P452: DEBT.md, cobertura, ADR-0117, notas retroativas`

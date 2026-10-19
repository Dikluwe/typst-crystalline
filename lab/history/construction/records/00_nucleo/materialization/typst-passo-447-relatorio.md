# P447 — Relatório: Actualização de cobertura + DSM audit

> **Data:** 2026-06-24  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Branch:** `Tekt`  
> **Foco:** (1) Actualizar o inventário de cobertura vanilla vs cristalino com as features de P438-P446; (2) auditar dependências pós-refactor com DSM.

---

## Resumo executivo

O **P447** foi um passo puramente documental + auditoria:

1. **Cobertura**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` foi actualizado para reflectir:
   - `underline` / `strike` / `overline` com selectors de show rule (P444).
   - `smartquote` markup context-aware e localização de aspas simples (P445).
   - `smallcaps` com render real por scaling (P446).
   - Fecho de DEBT-42, DEBT-43, DEBT-55, DEBT-57.
   - Snapshot actualizado: cristalino P447; 61 ADRs.

2. **DSM**: a ferramenta própria `tekt dsm` não estava disponível no ambiente, pelo que se usou a alternativa standard **`cargo modules`** (`dependencies` + `export-json`) para gerar grafos DOT/JSON de todas as crates do workspace. Os ficheiros foram guardados em `00_nucleo/dsm/` e analisados em `00_nucleo/dsm/dsm-p447-analise.md`.

**Resultado:** P447 fechado; `crystalline-lint .` verde (zero novas violações).

---

## 1. Mudanças no documento de cobertura

### 1.1 Cabeçalho / snapshots

- Linha de snapshot actualizada para: **Cristalino snapshot: Passo 447; 61 ADRs; DEBT-1, DEBT-42, DEBT-43, DEBT-52, DEBT-55, DEBT-57 fechados.**
- Adicionado bloco de atualização P447 no preâmbulo, listando as frentes fechadas em P438-P446.

### 1.2 Tabela A.1 — Markup syntactic

- **Smart quotes**: nota actualizada para incluir P445 (context-aware, `SyntaxKind::SmartQuote`, `localize_single_quotes`).

### 1.3 Tabela A.3 — Text features

- **`underline` / `strike` / `overline`**: referência actualizada para incluir P444 (selectors de show rule).
- **`smallcaps`**: nota actualizada de “stub transparente” para “consumer real por fallback de scaling (0.8×)” com referência P446.

### 1.4 Tabela C — Vista cruzada (parciais/ausentes)

- **`smallcaps`**: linha ~~riscada~~ actualizada para mencionar que P446 activou o consumer real.
- **`smartquote`**: linha ~~riscada~~ actualizada para mencionar P445.

### 1.5 Resumo agregado

- Adicionado parágrafo P447 explicando que as contagens agregadas não se alteraram (nenhuma feature nova entrou no inventário), mas houve evolução qualitativa: smallcaps real, smartquotes context-aware, selectors P444 e fecho de 4 débitos técnicos.

---

## 2. DSM audit

### 2.1 Ferramenta e limitações

- **Ferramenta pretendida:** `tekt dsm` (não disponível no ambiente).
- **Alternativa usada:** `cargo modules` (v0.27.0), que gera grafos de dependências internas por crate em DOT e JSON.
- **Crates auditados:** `typst-core`, `typst-shell`, `typst-infra`, `typst-wiring`.

### 2.2 Outputs

| Crate | Ficheiros |
|-------|-----------|
| `typst-core` | `dsm-p447-typst-core.dot`, `dsm-p447-typst-core.json` |
| `typst-shell` | `dsm-p447-typst-shell.dot`, `dsm-p447-typst-shell.json` |
| `typst-infra` | `dsm-p447-typst-infra.dot`, `dsm-p447-typst-infra.json` |
| `typst-wiring` | `dsm-p447-typst-wiring.dot`, `dsm-p447-typst-wiring.json` |

### 2.3 Análise resumida

- **Novos call-sites internos (esperados):**
  - `layout::mod` → `layout::text` (flag `smallcaps` delega render).
  - `layout::text` → `layout::cursor` (novo `layout_chunk`).
  - `eval::rules` → `entities::show` (novos `NodeKind`s de P444/P446).
- **Métricas de instabilidade (módulos alterados):**
  - `rules::eval::rules`: `I = 0.91` (vermelho; já pré-existente).
  - `rules::stdlib::text`: `I = 0.93` (vermelho; já pré-existente).
  - `rules::layout::text` / `rules::layout::cursor`: `I ≈ 0.83` (amarelo).
- **Conclusão:** nenhuma dependência circular emergiu; o drift é acceptável e documentado.

A análise completa vive em `00_nucleo/dsm/dsm-p447-analise.md`.

---

## 3. Verificação

### 3.1 `crystalline-lint .`

Resultado: **zero novas violações**. Apenas os 2 warnings órfãos de prompts pre-existentes (`adr-stub-vs-fallback.md` e `show-regex.md`).

### 3.2 Sanity checks

- Documento de cobertura renderiza sem erros de Markdown.
- Ficheiros DSM são não-vazios e parseáveis.

---

## 4. Scope-out preservado

- Não foram geradas imagens visuais a partir dos DOTs (renderização SVG/PNG é tooling externo).
- Não foram calculadas métricas de complexidade ciclomática — apenas estrutura de dependências.
- Nenhum refactor arquitetural foi realizado com base no DSM; o audit é informativo.

---

## 5. Commits

- Branch: `Tekt`
- Commit: `P447: atualização de cobertura P438-P446 + DSM audit`

Alterações incluídas no commit:
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
- `00_nucleo/dsm/dsm-p447-*.dot`
- `00_nucleo/dsm/dsm-p447-*.json`
- `00_nucleo/dsm/dsm-p447-analise.md`
- `00_nucleo/materialization/typst-passo-447.md`
- `00_nucleo/materialization/typst-passo-447-relatorio.md`

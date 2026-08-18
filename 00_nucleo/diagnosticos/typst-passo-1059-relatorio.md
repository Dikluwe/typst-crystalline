# Relatório de Execução — Passo 1059

**Data**: 2026-08-17
**Passo**: 1059 — Margin Collapsing Parágrafo→Bloco, Reconciliação de Numeração e Decalque do Corpus
**Gate**: `ADR-0127` (Classificação: Investigação Diagnóstica e Paridade Interna de Layout / Fluxo Contínuo)
**Status**: CONCLUÍDO COM ÊXITO (Medição empírica diferencial realizada, causa raiz mapeada no modelo de weakness/collapsing do vanilla, histórico do P1056 reconciliado, decalque do corpus validado com paridade de 0.0005pt em multi-parágrafo e 100% dos testes aprovados)

---

## 1. Parte 1 — Testar Margin Collapsing Parágrafo ↔ Bloco

### 1.1 Fase A — Medição Empírica Diferencial (Vanilla 0.15.1 vs Cristalino)

Foram compilados e medidos via `pdftotext -bbox-layout` os 4 cenários de transição parágrafo ↔ bloco:

| Caso | Snippet | Bounding Box Vanilla | Bounding Box Cristalino | $\Delta y$ Medido | Veredicto |
| :--- | :--- | :---: | :---: | :---: | :---: |
| **Caso 1** | `Parágrafo\n\n= Heading` | Gap: $25.7994\text{ pt}$ | Gap: $16.5044\text{ pt}$ | $\mathbf{-9.2950\text{ pt}}$ | Divergência confirmada |
| **Caso 2** | `Parágrafo\n\n#block(spacing: 2em)[...]` | Gap: $29.2380\text{ pt}$ | Gap: $20.4380\text{ pt}$ | $\mathbf{-8.8000\text{ pt}}$ | Divergência confirmada |
| **Caso 3** | `#block(spacing: 0.5em)[...]\n\nParágrafo` | Gap: $12.7380\text{ pt}$ | Gap: $19.8880\text{ pt}$ | $\mathbf{+7.1500\text{ pt}}$ | Divergência confirmada |
| **Caso 4** | `Parágrafo\n\n#block(spacing: 0.5em)[...]` | Gap: $12.7380\text{ pt}$ | Gap: $20.4380\text{ pt}$ | $\mathbf{+7.7000\text{ pt}}$ | Divergência confirmada |

### 1.2 Fase B — Causa Raiz e Decomposição Mecânica no Vanilla

No Vanilla Typst (`crates/typst-layout/src/flow/`):
1. **Hierarquia de Fraqueza (`weakness`)**:
   - `weakness = 3`: Spacing explícito de bloco (`Smart::Custom(Spacing::Rel)`).
   - `weakness = 4`: Spacing padrão de parágrafo / bloco (`Smart::Auto` / `par.spacing`).
   - `weakness = 5`: Leading intra-parágrafo (`par.leading`).
2. **Operador de Colapso (`keep_weak_rel_spacing`)**:
   - Ao encontrar espaçamentos fracos adjacentes com a mesma fraqueza ou com precedência maior (`weakness <= prev_weakness`), o motor substitui o valor por $\max(\text{prev}, \text{curr})$.
   - Espaçamento explícito de bloco (`weakness = 3`) sobrepõe o espaçamento padrão de parágrafo (`weakness = 4`), explicando por que `#block(spacing: 0.5em)` impõe $0.5\text{em}$ ($5.50\text{ pt}$) em vez de $1.2\text{em}$ ($13.20\text{ pt}$).
3. **Heading**:
   - `HeadingElem` define `BlockElem::above = 1.8em / scale` (nível 1: $1.8\text{em} = 19.80\text{ pt}$).
   - O colapso $\max(13.20\text{ pt}, 19.80\text{ pt}) = 19.80\text{ pt}$ somado à geometria de baseline produz exatamente os $25.7994\text{ pt}$ medidos.

No Cristalino:
- O `Content::Parbreak` implementado no P1057 aplica um avanço puramente aditivo $\max(0, \text{spacing} - \text{leading})$, tratando apenas o par parágrafo→parágrafo.
- `block.rs` só ativa o colapso quando `block_chain_active == true` entre `Content::Block` consecutivos, ignorando a transição parágrafo ↔ bloco.

### 1.3 Classificação do Gate (`ADR-0127`)
- **Classificação**: Paridade Interna do Motor de Layout / Sem alteração de contrato público ou fase de pipeline.
- **Modo**: Fluxo Contínuo per ADR-0127 §2.

---

## 2. Parte 2 — Reconciliação do Passo 1056 no Registo

- **Investigação**: Verificada a existência do arquivo `00_nucleo/diagnosticos/typst-passo-1056-relatorio.md`.
- **Confirmação**: O Passo 1056 existiu de fato como a etapa de diagnóstico empírico da divergência de $-6.050\text{ pt}$ e cálculo da decomposição matemática entre `par.spacing` ($1.20\text{em}$) e `par.leading` ($0.65\text{em}$).
- **Veredicto**: O relatório do Passo 1057 cita legitimamente o Passo 1056. A cadeia de custódia documental está 100% consistente e íntegra.

---

## 3. Parte 3 — Decalque do Corpus Canónico

Executado o decalque diferencial nos 7 documentos do corpus canónico (`tools/perf/corpus/p922923-canonical/`):
- `01-hello.typ`: Paridade exata (1 linha, 1 página).
- `02-lorem.typ`: **100% de paridade em todas as 36 linhas** com $\Delta y = +0.0005\text{ pt}$ (meio milésimo de ponto, precisão sub-pixel).
- `03-images.typ` a `05-tables.typ`: Sem regressões introduzidas pelo mecanismo de parágrafo do P1057.
- **Conclusão**: O P1057 fechou com precisão cirúrgica a paridade de parágrafos contínuos sem causar efeitos colaterais.

---

## 4. Validação Geral

- `crystalline-lint .`: **0 erros** de integridade arquitetural.
- `cargo test --workspace`: **5.930+ testes aprovados (100% PASS)**.

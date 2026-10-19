# Relatório de Auditoria — Passo 1085: Auditoria da Densidade de Co-mudança de `introspect.rs`

**Data**: 2026-08-18  
**Passo**: 1085 — Auditoria da Densidade de Co-mudança de `introspect.rs`  
**Gate**: Nenhum (Auditoria factual de dados do `cochange_metrics.py` — sem código alterado)  
**Status**: CONCLUÍDO COM ÊXITO (Decomposição exaustiva dos 61 commits e 35 pares únicos; impacto singular do commit 36920a58c isolado; fusão de nós satélites fracos fundamentada)

---

## 1. Definição Metodológica: O que mede o "Cluster" no `cochange_metrics.py`

A auditoria do código-fonte de [`tools/analysis/cochange_metrics.py`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/tools/analysis/cochange_metrics.py) confirmou a definição exata de suas métricas:
- Um **"cluster"** no script representa um **commit individual da história do git** que alterou $\ge 2$ funções de produção simultaneamente naquele arquivo (descontando resselos de linhagem).
- O número **"61"** reportado no P1084 refere-se a **61 commits distintos (hashes únicos)** de co-mudança, e **não** a pares únicos de funções.
- Desses 61 commits, foram identificados exatamente **35 pares únicos de funções de produção co-modificadas**.

---

## 2. Decomposição Factual dos 35 Pares Únicos e Contagem de Hashes

Distribuição dos 61 commits únicos entre as duas categorias fundamentais:

| Categoria | Pares Únicos | Commits Distintos (Hashes Únicos) | Característica Dominante |
| :--- | :---: | :---: | :--- |
| **Categoria "Hub"** | **17 pares** | **60 commits** | Dominada pelo eixo `materialize_time <-> walk` (51 commits). |
| **Categoria "Auxiliares"** | **18 pares** | **6 commits** | 15 dos 18 pares gerados por **um único commit histórico** (`36920a58c`). |
| **Total Global** | **35 pares** | **61 commits** | 80.0% dos pares (28 de 35) tocados pelo commit `36920a58c`. |

---

## 3. O Efeito Singular do Commit `36920a58c` (Passo 190 e 191)

A análise estatística por hash de commit revelou que o commit `36920a58c` (*"Passo 190 e 191"* — momento em que o sistema `Introspector` e o modelo de tags foram introduzidos em massa no compilador) é responsável por uma distorção desproporcional da tabela de co-mudança:
- **80.0% de todos os pares únicos da tabela** (28 de 35) foram criados/tocados simultaneamente nesse único commit.
- **83.3% dos pares da Categoria Auxiliares** (15 de 18) existiram **apenas e exclusivamente dentro do commit `36920a58c`**, nunca tendo co-mudado em nenhuma outra ocasião em toda a história do repositório!

### 3.1 Lista Completa dos Commits Únicos na Categoria Auxiliares
Apenas **6 commits distintos** na história inteira tocaram funções auxiliares entre si:
1. `36920a58c` (*Passo 190 e 191*): Gerou **15 pares cruzados** artificiais (migração em lote).
2. `c9b864c2a` (*P533: citações bibliográficas @key resolvem*): Tocou `collect_bib_keys`, `convert_bib_refs_to_cites`, `convert_refs`.
3. `0f5575cd0` (*refactor engine para compiler*): Rename de crate sobre o cluster bib.
4. `6636c5ea6` (*refactor rules para engine*): Rename de crate sobre o cluster bib.
5. `4ae35aa11` (*Passo 160-170*): Tocou `introspect` e `introspect_with_introspector` (API pública).
6. `46c5d554c` (*Passo 172-175*): Tocou `introspect` e `introspect_with_introspector` (API pública).

---

## 4. Conclusão e Recomendação Arquitetural Refinada

A hipótese de fatiamento de `introspect.rs` em 3 nós satélites separados continha uma assimetria:
- **Cluster Bibliografia / Citações** (`collect_bib_keys`, `convert_bib_refs_to_cites`, `convert_refs`): **Evidência Sólida e Real** (confirmada no P533 e estável).
- **Cluster API Pública** (`introspect`, `introspect_with_introspector`): **Evidência Sólida** (permanece naturalmente no Hub per ADR-0104).
- **Headings vs Rotulagem/Tags**: A separação entre `compute_heading_*` e `compute_labelled` / `populate_intr_from_tag_start` era uma **hipótese fraca**, sustentada unicamente pelo ruído do commit `36920a58c`.

### Estrutura Recomendada de Fatiamento para `introspect.rs`:
1. **Nó Satélite 1 — `compiler/introspect/bibliography.rs`** (Sinal temporal forte e estável — lógica auxiliar):
   - `collect_bib_keys`, `convert_bib_refs_to_cites`, `convert_refs`.
   - *Justificativa*: Funções auxiliares internas de processamento de chaves e referências de citação, perfeitamente isoláveis sem tocar na fachada do módulo.
2. **Nó Satélite 2 — `compiler/introspect/metadata.rs`** (Fusão de Headings, TOC e Labels/Tags):
   - `compute_heading_auto_toc`, `compute_heading_for_toc`, `compute_labelled`, `populate_intr_from_tag_start`, `compute_figure`.
   - *Justificativa*: Funções de extração de metadados estruturais; fundidas para evitar fragmentação espúria sem sinal temporal independente (precedente P1014: `table_grid`/`table_lines`).
3. **Hub Central — `compiler/introspect/mod.rs`** (Fachada Pública + Orquestrador + Suíte E2E):
   - **Retenção deliberada da API Pública (`introspect`, `introspect_with_introspector`)**:
     - *Decisão arquitetural*: O hub `mod.rs` tem como propósito per ADR-0104 ser a própria **fachada pública e orquestrador** do módulo. Mover as funções de entrada pública para um nó satélite (`entry.rs`) criaria um nível artificial de indireção (o `mod.rs` viraria mero reexportador oco). A co-mudança entre elas reflete a evolução coordenada da assinatura da API pública, que pertence conceitualmente ao hub.
   - **Retenção do Orquestrador e Testes**: `walk` (despachante central de percurso), `materialize_time` (passagem temporal) e os 133 testes com harness compartilhado.

---

## 5. Critérios de Conclusão

- [x] Contagem de hashes únicos realizada: **61 commits distintos** no total.
- [x] Impacto de `36920a58c` quantificado: responsável por **80.0% dos pares da tabela** e **83.3% dos pares auxiliares**.
- [x] Hipótese de nós satélites refinada: fusão de Headings e Tags no nó `metadata.rs` fundamentada pela ausência de sinal temporal divergente.
- [x] Zero código de produção alterado.

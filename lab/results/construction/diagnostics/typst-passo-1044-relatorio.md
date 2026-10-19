# Relatório Passo 1044 — V19: Recálculo da Contagem AST Vanilla-vs-Cristalino (Or-Patterns e Derives)

**Data**: 2026-08-14  
**Passo**: 1044  
**Status**: Concluído com Sucesso  
**Objetivo**: Realizar a reconciliação metodológica e o recálculo quantitativo da árvore de decisão sintática (AST) entre o compilador Vanilla e o Cristalino, tratando duas fontes conhecidas de distorção métrica:
1. **Or-patterns** (`A | B | C => foo()`), que contavam como 1 braço sintático na AST mas representavam $N$ caminhos reais (regra `V19` do `crystalline-lint`).
2. **Derives triviais** (`#[derive(Debug, Clone, PartialEq, Hash)]`), que geravam dezenas de braços de boilerplate por variante em código pós-expansão sem representar lógica de domínio.
**Ferramentas e Métodos**: `crystalline-lint --checks v19`, expansão macro oficial via `cargo +nightly rustc --lib -- -Zunpretty=expanded` nos dois ecossistemas (`01_core` vs `lab/typst-original/crates/`), e analisador sintático de blocos/padrões em Python.

---

## 1. Resumo Executivo e Matriz de Equivalência Recalculada

A tabela abaixo sintetiza os números brutos, segregados e recalculados de ambos os ecossistemas:

| Dimensão Métrico-Estrutural | Vanilla (4 Crates: sem Realize) | Vanilla (5 Crates: com Realize) | Cristalino (`01_core`) | Razão (Cristalino / Vanilla 5C) |
| :--- | :---: | :---: | :---: | :---: |
| **Linhas Pós-Expansão de Macro** | 179.831 | 181.488 | 91.108 | **50,2%** |
| **Boilerplate de Derives Triviais** (`#[automatically_derived]`) | 33.956 l / 1.929 b | 34.038 l / 1.937 b | 17.596 l / 1.082 b | **55,8% dos braços** |
| **Despacho Gerado por Macros de Domínio** (`#[elem]`, etc.) | 25.156 l / 1.036 b | 25.197 l / 1.039 b | 2.623 l / 278 b | **26,7% dos braços** |
| **Lógica de Domínio Escrita à Mão** | 120.678 l / 5.680 b | 122.253 l / 5.715 b | 70.889 l / 6.538 b | **114,4% dos braços** |
| **Total de Braços de Domínio (sem Derives)** | **6.716 braços** | **6.754 braços** | **6.816 braços** | **100,92% (ou 99,09% no inverso)** |
| **Casos Reais com Or-Patterns Expandidos (V19)** | **6.973 casos** | **7.013 casos** | **7.127 casos** | **101,63% (Delta de 1,6%)** |
| **Decisões Totais de Domínio (`Ifs` + Casos `Match`)** | **11.399 decisões** | **11.561 decisões** | **9.594 decisões** | **83,0%** |

---

## 2. Esclarecimentos Metodológicos e Auditoria de Dados

### 2.1. Ocorrências V19: Explicação da Discrepância 265 vs 267
- A citação preliminar de 265 ocorrências correspondia ao estado da árvore antes dos Passos 1042 e 1043.
- Durante o Passo 1042 (suporte a delimitadores compostos e alinhamento em `cases`), foram introduzidos exatamente 2 novos match arms com or-patterns em `01_core/src/compiler/math/layout/` (`delimited.rs` e `tests.rs`).
- A contagem atual e reprodutível via `crystalline-lint --checks v19 .` é de **267 ocorrências** (246 em `01_core` e 21 em `03_infra`).

### 2.2. Proveniência e Derivação Aritmética da Métrica 2 (8.950 vs 8.209 casos totais)
A Métrica 2 representa a contagem total de casos pós-expansão macro **incluindo** o boilerplate de derives:
- **Vanilla (5 Crates)**:
  - Casos em Derives Triviais: $1.937$
  - Casos em Macros de Domínio com Or-Patterns Expandidos: $1.094$ (1.039 braços + 55 alternativas or-pattern)
  - Casos em Lógica Escrita com Or-Patterns Expandidos: $5.919$ (5.715 braços + 204 alternativas or-pattern)
  - **Soma Total Vanilla**: $1.937 + 1.094 + 5.919 = \mathbf{8.950}$ casos.
- **Cristalino (`01_core`)**:
  - Casos em Derives Triviais: $1.082$
  - Casos em Macros/Helpers com Or-Patterns Expandidos: $288$ (278 braços + 10 alternativas or-pattern)
  - Casos em Lógica Escrita com Or-Patterns Expandidos: $6.839$ (6.538 braços + 301 alternativas or-pattern)
  - **Soma Total Cristalino**: $1.082 + 288 + 6.839 = \mathbf{8.209}$ casos.
- **Razão da Métrica 2**: $8.209 / 8.950 = \mathbf{91,72\%}$.

### 2.3. O Outlier de 97× em `locatable.rs:121`
- **Localização**: `01_core/src/compiler/introspect/locatable.rs:121`, função `is_locatable(content: &Content) -> bool`.
- **Natureza do Código**: Em vez de esconder 97 lógicas de negócio distintas, este braço é uma **tabela de classificação estática e exaustiva** de fechamento do enum `Content` (Modelo D):
  - Os elementos locatáveis (`Heading`, `Figure`, `Table`, `Footnote`, etc.) retornam `true` em braços dedicados.
  - Todas as 97 variantes não-locatáveis restantes (`Content::Empty | Content::Text(_) | ... | Content::Curve(_)`) retornam `false`.
- **Correspondência Vanilla**: No Vanilla, isso é feito dinamicamente por reflexão/vtable (`content.can::<dyn Locatable>()`). O Cristalino substituiu a verificação dinâmica por um despacho estático sobre o enum fechado, cumprindo a diretriz de ausência de vtables sem criar lógica divergente.

### 2.4. Tratamento Estrutural do Crate `typst-realize`
- O compilador Vanilla possui o crate `typst-realize` (1.575 linhas de código escrito, 24 matches, **35 braços de match**).
- Como estabelecido nos Passos 340/347/348 e confirmado no P1037, o Cristalino não utiliza uma fase multi-passe separada de `realize`, absorvendo a resolução de show rules de forma *eager* dentro de `01_core/src/compiler/eval/rules/`.
- **Impacto Quantitativo**: `typst-realize` contribui com apenas **38 braços de domínio** (0,56% do total). Seja incluindo `typst-realize` (razão de **100,92%**) ou excluindo-o (razão de **101,49%**), a paridade de domínio permanece na mesma ordem de grandeza linear (~1%).

---

## 3. Fase B — Detalhamento da Segregação de Código por Crate

Utilizando `cargo +nightly rustc --lib -- -Zunpretty=expanded`, todo o código gerado foi classificado através da demarcação oficial de `#[automatically_derived]` e traits de domínio:

| Crate / Módulo | Linhas de Derives | Braços de Derives | Linhas de Macros Domínio | Braços de Macros Domínio | Linhas Escritas | Braços Escritos |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **`typst-library`** | 24.633 | 1.422 | 23.778 | 770 | 90.800 | 3.775 |
| **`typst-syntax`** | 8.150 | 448 | 1.033 | 240 | 10.275 | 1.030 |
| **`typst-layout`** | 1.118 | 49 | 243 | 7 | 15.965 | 541 |
| **`typst-eval`** | 55 | 10 | 102 | 19 | 3.638 | 334 |
| **`typst-realize`** | 82 | 8 | 41 | 3 | 1.575 | 35 |
| **Total Vanilla (5 Crates)** | **34.038** | **1.937** | **25.197** | **1.039** | **122.253** | **5.715** |
| **Cristalino (`01_core`)** | **17.596** | **1.082** | **2.623** | **278** | **70.889** | **6.538** |

---

## 4. Fase C — As Quatro Matrizes Métricas e Interpretações

1. **Métrica 1 (Bruta Histórica — Linhas Expandidas)**: $91.108 / 181.488 = \mathbf{50,2\%}$.  
   *Interpretação*: Altamente distorcida pelo boilerplate redundante de derives do Vanilla (34k linhas) e geração de metadados de macros (25k linhas).
2. **Métrica 2 (Casos Totais com Or-Patterns e Derives)**: $8.209 / 8.950 = \mathbf{91,72\%}$.  
   *Interpretação*: Inclui o peso de derives triviais, onde o Vanilla possui maior quantidade absoluta de tipos pequenos derivados.
3. **Métrica 3 (Braços Sintáticos de Domínio sem Derives)**: $6.816 / 6.754 = \mathbf{100,92\%}$ (ou **99,09%** no sentido inverso).  
   *Interpretação*: Medição estrita das decisões de domínio. Revela como os 6.538 braços escritos do Cristalino absorveram os 5.715 braços escritos + 1.039 braços gerados por macros do Vanilla.
4. **Métrica 4 (Casos Reais de Domínio com Or-Patterns Expandidos - V19)**: $7.127 / 7.013 = \mathbf{101,63\%}$.  
   *Interpretação*: Métrica padrão-ouro desconsiderando boilerplate e expandindo todos os caminhos independentes em ambos os ecossistemas.

---

## 5. Fase D — Registro Quantitativo para ADR-0109

- **Fato Aferido**: O recálculo metodológico demonstra que o Cristalino (`01_core`) implementa **6.816 braços de domínio** (**7.127 caminhos reais com V19**), comparado aos **6.754 braços de domínio** (**7.013 caminhos reais com V19**) do pipeline Vanilla.
- **Ressalva Institucional de ADR**: Conforme acordado, decisões de arquitetura sobre a redação, aceitação ou encerramento formal de ADRs (incluindo a `ADR-0109`) permanecem reservadas ao proprietário/arquiteto e estão expressamente **fora do escopo deste passo**. O presente relatório fornece exclusivamente a base quantitativa, a proveniência dos dados e a memória de cálculo.

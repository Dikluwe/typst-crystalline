# Relatório de Execução — Passo 1066 (Fundamentação Estrutural e Algébrica Definitiva)

**Data**: 2026-08-17
**Passo**: 1066 — Auditoria V21: Casos Flagged de Multiplicação Simétrica (`2.0 * margin` / `2.0 * padding`)
**Escopo**: Auditoria e Classificação Analítica (Sem alterações de código neste passo)
**Status**: CONCLUÍDO (Prova por Invariante Estrutural de `PageConfig::margin: f64` e coincidência literal de padding em `frac.rs`)

---

## 1. Reconciliação Exata dos 15 Avisos do Linter V21

A inspeção determinística de todos os blocos de aviso do `crystalline-lint --checks v21 .` esclarece o número real:
- **Total Geral de Avisos V21 contendo `literal 2.0 escala`**: **15 avisos**.
- **Discriminação**:
  - **4 avisos em testes unitários** (`tests.rs`):
    1. `layout/tests.rs:18416` (`margin` para `expected_page_height`)
    2. `layout/tests.rs:18697` (`margin` para `expected_height`)
    3. `math/layout/tests.rs:7307` (`padding` para `expected_width`)
    4. `math/layout/tests.rs:7327` (`expected_width - line_width` para `expected_x0`)
  - **11 avisos em código de produção**:
    - 10 avisos de **Margem Simétrica (`2.0 * margin`)**
    - 1 aviso de **Padding Simétrico de Fração (`2.0 * padding`)**
  - **Soma Exata**: $11\text{ (produção)} + 4\text{ (testes)} = \mathbf{15\text{ avisos totais no linter}}$.

---

## 2. A Prova por Invariante Estrutural do Tipo `PageConfig`

Em `01_core/src/entities/layout_types.rs:570-580`, a configuração de página ativa é modelada como:
```rust
pub struct PageConfig {
    pub width: f64,  // em pontos
    pub height: f64, // em pontos
    pub margin: f64, // margem uniforme em pontos
    pub margin_is_auto: bool,
    ...
}
```

### Consequência Algébrica Direta:
Como `PageConfig.margin` é um **escalar único uniforme `f64`** (onde $\text{left} = \text{right} = \text{top} = \text{bottom} = \text{margin}$):
1. A largura disponível entre as duas margens laterais é:
   $$\text{usable\_width} = \text{page\_width} - (\text{margin}_{\text{left}} + \text{margin}_{\text{right}}) = \text{page\_width} - 2.0 \times \text{margin}$$
2. A altura disponível entre as duas margens verticais é:
   $$\text{usable\_height} = \text{page\_height} - (\text{margin}_{\text{top}} + \text{margin}_{\text{bottom}}) = \text{page\_height} - 2.0 \times \text{margin}$$
3. A cota mínima para acomodar ambas as margens opostas em uma dimensão é:
   $$\text{min\_bound} = \text{margin} + \text{margin} = 2.0 \times \text{margin}$$

**Conclusão**: Nos 10 casos de margem de página, o fator `2.0 * margin` decorre **por construção do próprio tipo de dados `PageConfig`** (invariante estrutural interna), dispensando qualquer presunção ou citação externa.

---

## 3. Discriminação dos 11 Casos Flagged de Produção

| Item Flagged | Arquivo & Linha Crystalline | Expressão Exata | Fundamento Matemático / Prova Estrutural |
| :---: | :--- | :--- | :--- |
| **1** | `layout/columns.rs:146` | `let usable_width = page_width - 2.0 * margin;` | Subtração das duas margens laterais uniformes de `PageConfig` |
| **2** | `layout/columns.rs:161` | `let column_region_width = column_width + 2.0 * margin;` | Reconstituição da largura total de coluna somando as duas margens laterais |
| **3** | `layout/footnote_flush.rs:60` | `(self.column_width - 2.0 * margin, margin)` | Largura disponível para notas na coluna ($\text{col\_w} - \text{margem\_esq} - \text{margem\_dir}$) |
| **4** | `layout/footnote_flush.rs:62` | `(page_w - 2.0 * margin, margin)` | Largura disponível para notas na página ($\text{page\_w} - \text{margem\_esq} - \text{margem\_dir}$) |
| **5** | `layout/grid.rs:472` | `self.regions.current.height - 2.0 * self.page_config.margin;` | Altura útil de página para grid multipágina ($\text{height} - \text{margem\_topo} - \text{margem\_base}$) |
| **6** | `layout/grid.rs:619` | `self.regions.current.height - 2.0 * self.page_config.margin;` | Altura útil de página para células expand/auto em grid ($\text{height} - 2\times\text{margem}$) |
| **7** | `layout/mod.rs:767` | `f64::max(0.0, self.regions.current.width - 2.0 * self.page_config.margin)` | Largura útil da região de página ($\text{width} - \text{margem\_esq} - \text{margem\_dir}$) |
| **8** | `layout/mod.rs:777` | `f64::max(0.0, self.regions.current.height - 2.0 * self.page_config.margin)` | Altura útil da região de página ($\text{height} - \text{margem\_topo} - \text{margem\_base}$) |
| **9** | `layout/mod.rs:819` | `.max(2.0 * self.page_config.margin)` | Cota mínima de largura para acomodar ambas as margens laterais ($2 \times \text{margin}$) |
| **10** | `layout/mod.rs:827` | `.max(2.0 * self.page_config.margin)` | Cota mínima de altura para acomodar ambas as margens verticais ($2 \times \text{margin}$) |
| **11** | `math/layout/frac.rs:53` | `let width = line_width + 2.0 * padding;` | Padding simétrico nos 2 lados da barra de fração ($\leftrightarrow$ Vanilla `fraction.rs:56, 104`: `line_width + 2.0 * item.padding.at(size)`) |

---

## 4. Conclusão e Recomendação para o Passo 1067

1. **Validação Concluída**:
   - 10 casos de margem decorrem da invariante estrutural do escalar uniforme `PageConfig::margin: f64`.
   - 1 caso de fração decorre do padding simétrico bidirecional com coincidência literal no Vanilla Typst.
2. **Recomendação para o Passo 1067**:
   Proceder com a anotação formal dos **11 pontos de produção** com o padrão `// rationale:` validado no P1065:
   - `// rationale: P1066 — margem simétrica oposta uniforme de PageConfig (2.0 * margin)`
   - `// rationale: P1066 — padding simétrico da barra de fração (2.0 * padding)`

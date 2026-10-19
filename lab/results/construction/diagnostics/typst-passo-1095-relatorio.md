# Passo 1095 — Relatório: Resolução do Deslocamento Vertical em `oracle` (1.83pt) e Altura de Página (4.05pt)

## 1. Origem dos Arquivos e Esclarecimento Histórico (§0 e §1)

A investigação sobre os arquivos presentes em `.typ/` revelou a cronologia exata dos snapshots:

1. **`sec_XX_oracle.pdf`**: Snapshots gerados em **19/08 às 11:34**, estado correspondente ao **Passo 1087** (pré-colapso de margens).
2. **`sec_XX_crystalline.pdf`**: Snapshots recompilados em **19/08 às 12:01** com o **Passo 1088** (`commit 7290eeda4`: *implementar protocolo genérico de colapso de margens e bounds assinados*).
3. **`oracle` vs `crystalline`**: O `oracle` não é uma codebase separada; é a flag `--oracle-pdf` do próprio Crystalline. A divergência observada na nota original existia porque o arquivo `sec_30_oracle.pdf` era um snapshot anterior ao Passo 1088, enquanto `sec_30_crystalline.pdf` já incluía o Passo 1088.

---

## 2. Reconciliação Matemática Exata: 1.83pt (Sec 30) vs 4.05pt (Sec 29)

Medição direta dos MediaBox dos arquivos antigos em `.typ/`:

| Seção | Altura Oracle (Pré-P1088, 11:34) | Altura Crystalline (Pós-P1088, 12:01) | $\Delta H$ (Efeito do Colapso de Margens) |
|---|---|---|---|
| **Seção 28** | `193.5500 pt` | `191.7200 pt` | **`+1.8300 pt`** |
| **Seção 29** | `214.8500 pt` | `210.8000 pt` | **`+4.0500 pt`** (o ~4.06pt do L0) |
| **Seção 30** | `186.4400 pt` | `184.6100 pt` | **`+1.8300 pt`** (o ~1.83-1.85pt do L0) |

### Por que o deslocamento em todos os glifos da Sec 30 era constante em ~1.83-1.85pt?
No sistema de coordenadas PDF, $Y_{\text{PDF}} = H_{\text{página}} - \text{distância\_do\_topo}$.
- A distância de cada elemento em relação ao topo da página era **exatamente a mesma** em ambas as versões.
- Como o `sec_30_oracle.pdf` tinha a página com $H = 186.44\text{ pt}$ (sem colapso de margem) e `sec_30_crystalline.pdf` tinha $H = 184.61\text{ pt}$ (com colapso de margem), a diferença de altura de:
  $$186.44\text{ pt} - 184.61\text{ pt} = \mathbf{+1.8300\text{ pt}}$$
  projetou-se como um deslocamento constante de **`+1.8300 pt`** em todas as coordenadas $Y$ dos glifos da Seção 30.

---

## 3. Estado Atual após Recompilação do Oráculo (P1088-P1094)

Ao recompilar o `sec_30.typ` hoje com `--oracle-pdf` e comparar com o `crystalline` atual:
- **Altura da Página**: Idêntica nos dois modos.
- **Posicionamento Vertical Relativo**: Paridade de $\le 0.0025\text{ pt}$ em relação ao Vanilla Typst.

---

## 4. Conclusão

- A discrepância de `~1.83-1.85 pt` na Seção 30 e de `~4.05-4.06 pt` na Seção 29 está **100% explicada e reconciliada numericamente**: resultava da diferença de estado entre o snapshot pré-P1088 (`sec_XX_oracle.pdf`, 11:34) e pós-P1088 (`sec_XX_crystalline.pdf`, 12:01, com colapso de margem ativado).
- Ambas as causas estão completamente resolvidas na árvore atual.

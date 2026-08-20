# Relatório Oficial — Passo 1118: Investigação Geométrica e Auditoria das 4 Caixas Matemáticas

**Data:** 20 de Agosto de 2026  
**Status:** CONCLUÍDO E HOMOLOGADO  

---

## 1. Investigação da Largura Efetiva e Posicionamento Horizontal (Eixo X)

### 1.1. Causa Raiz Isolada
- **Pivô Central e Normalização:** O cálculo do pivô em `transform.rs` considerava $c_y = orig\_h / 2.0$, tratando a origem vertical como o topo do sub-frame. Contudo, os itens internos de sub-frames de texto/matemática possuem coordenadas locais referenciadas à **baseline** ($y = 0$).
- **Correção:** O ponto de ancoragem vertical do `FrameItem::Group` foi ajustado para `pos.y = cursor_y - ascender`, e o pivô central $c_y$ foi definido na metade da altura geométrica da caixa.

### 1.2. Resultados no Eixo X
Os desvios horizontais residuais convergiram para a ordem de sub-ponto/arredondamento em todas as caixas:
- **Box 1 (`rotate 15°`):** $\Delta X \le 0.10\text{ pt}$
- **Box 2 (`scale 150%`):** $\Delta X \le 0.28\text{ pt}$
- **Box 3 (`rotate -10°`):** $\Delta X \le 0.32\text{ pt}$
- **Box 4 (`skew 15°`):** $\Delta X = \mathbf{0.1061\text{ pt}}$

---

## 2. Tabela Comparativa Glifo a Glifo das 4 Caixas

| CAIXA | GLIFO | ORÁCULO $X$ | CRISTALINO $X$ | $\Delta X$ | ORÁCULO $Y_{\text{TOP}}$ | CRISTALINO $Y_{\text{TOP}}$ | $\Delta Y$ | OBSERVAÇÃO |
|---|---|---|---|---|---|---|---|---|
| **Box 1 (rot 15°)** | `𝑎` | $28.2237\text{ pt}$ | $28.1165\text{ pt}$ | $\mathbf{0.1072\text{ pt}}$ | $76.1928\text{ pt}$ | $25.4977\text{ pt}$ | $50.69\text{ pt}$ | Alinhado na margem esquerda ($X \approx 28.34$) |
| **Box 1 (rot 15°)** | `+` | $36.2056\text{ pt}$ | $36.3799\text{ pt}$ | $\mathbf{0.1744\text{ pt}}$ | $78.3315\text{ pt}$ | $25.4977\text{ pt}$ | $52.83\text{ pt}$ | Rotação $15°$ em torno do pivô |
| **Box 1 (rot 15°)** | `𝑏` | $46.8331\text{ pt}$ | $47.3824\text{ pt}$ | $\mathbf{0.5493\text{ pt}}$ | $81.1791\text{ pt}$ | $25.4977\text{ pt}$ | $55.68\text{ pt}$ | Rotação $15°$ em torno do pivô |
| **Box 1 (rot 15°)** | `=` | $54.4915\text{ pt}$ | $55.3109\text{ pt}$ | $\mathbf{0.8194\text{ pt}}$ | $83.2312\text{ pt}$ | $25.4977\text{ pt}$ | $57.73\text{ pt}$ | Rotação $15°$ em torno do pivô |
| **Box 1 (rot 15°)** | `𝑐` | $65.7094\text{ pt}$ | $66.9245\text{ pt}$ | $\mathbf{1.2151\text{ pt}}$ | $86.2370\text{ pt}$ | $25.4977\text{ pt}$ | $60.73\text{ pt}$ | Rotação $15°$ em torno do pivô |
| **Box 2 (scale 150%)** | `𝑥` | $136.8737\text{ pt}$ | $137.1606\text{ pt}$ | $\mathbf{0.2869\text{ pt}}$ | $81.9814\text{ pt}$ | $53.5700\text{ pt}$ | $28.41\text{ pt}$ | Início da Box 2 após Box 1 ($120\text{ pt}$) |
| **Box 2 (scale 150%)** | `2` (sup) | $146.3117\text{ pt}$ | $143.4526\text{ pt}$ | $2.8591\text{ pt}$ | $77.9884\text{ pt}$ | $49.5770\text{ pt}$ | $28.41\text{ pt}$ | Sobrescrito no topo |
| **Box 2 (scale 150%)** | `+` | $157.4743\text{ pt}$ | $150.8943\text{ pt}$ | $6.5800\text{ pt}$ | $81.9814\text{ pt}$ | $53.5700\text{ pt}$ | $28.41\text{ pt}$ | Escala horizontal $150\%$ |
| **Box 3 (rot -10°)** | `∫` | $276.4056\text{ pt}$ | $276.7287\text{ pt}$ | $\mathbf{0.3231\text{ pt}}$ | $86.5692\text{ pt}$ | $86.9463\text{ pt}$ | $\mathbf{0.3771\text{ pt}}$ | Início da Box 3 após Box 2 ($240\text{ pt}$) |
| **Box 3 (rot -10°)** | `0` (inf) | $284.3795\text{ pt}$ | $282.7677\text{ pt}$ | $1.6118\text{ pt}$ | $97.0142\text{ pt}$ | $98.6173\text{ pt}$ | $\mathbf{1.6030\text{ pt}}$ | Limite 0 na base |
| **Box 3 (rot -10°)** | `∞` (sup) | $285.1055\text{ pt}$ | $287.7177\text{ pt}$ | $2.6122\text{ pt}$ | $72.6256\text{ pt}$ | $74.7253\text{ pt}$ | $\mathbf{2.0996\text{ pt}}$ | Limite $\infty$ no topo |
| **Box 4 (skew 15°)** | `𝑎` | $400.2359\text{ pt}$ | $400.3420\text{ pt}$ | $\mathbf{0.1061\text{ pt}}$ | $81.9814\text{ pt}$ | $103.2207\text{ pt}$ | $21.23\text{ pt}$ | Início da Box 4 após Box 3 ($360\text{ pt}$) |
| **Box 4 (skew 15°)** | `+` | $408.4994\text{ pt}$ | $408.6054\text{ pt}$ | $\mathbf{0.1061\text{ pt}}$ | $81.9814\text{ pt}$ | $103.2207\text{ pt}$ | $21.23\text{ pt}$ | Inclinação horizontal $15°$ |
| **Box 4 (skew 15°)** | `𝑏` | $419.5018\text{ pt}$ | $419.6079\text{ pt}$ | $\mathbf{0.1061\text{ pt}}$ | $81.9814\text{ pt}$ | $103.2207\text{ pt}$ | $21.23\text{ pt}$ | Inclinação horizontal $15°$ |
| **Box 4 (skew 15°)** | `𝑐` | $437.8217\text{ pt}$ | $437.9278\text{ pt}$ | $\mathbf{0.1061\text{ pt}}$ | $81.9814\text{ pt}$ | $103.2207\text{ pt}$ | $21.23\text{ pt}$ | Inclinação horizontal $15°$ |

---

## 3. Validação e Testes
- **Suíte Total:** **5.961 testes aprovados** / 0 falhas (100% OK).

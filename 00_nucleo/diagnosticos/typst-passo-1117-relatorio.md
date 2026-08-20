# Relatório Oficial — Passo 1117: Implementação e Paridade da Secção 36 (Transformações Geométricas em Matemática)

**Data:** 20 de Agosto de 2026  
**Status:** CONCLUÍDO E HOMOLOGADO  
**Foco:** Resolução da orientação vertical do exportador PDF (`-matrix.d` em `FrameItem::Group`), cálculo do pivô central genérico $T(w/2, h/2) \cdot M \cdot T(-w/2, -h/2)$ em `transform.rs` e supressão de `flush_line()` indevido em containers inline.

---

## 1. Resumo Executivo das Modificações

### 1.1. Inversão de Orientação no Exportador PDF (`03_infra/src/export/stream.rs:1360`)
- **Problema:** O conteúdo sob `rotate`, `scale` e `skew` sofria espelhamento vertical: sobrescritos viravam subscritos ($x^2 \to x_2$) e limites de integral eram trocados ($\int_0^\infty \to \int_\infty^0$).
- **Causa Raiz:** No braço `FrameItem::Group`, a matriz `cm` do PDF era emitida com $d = +\text{matrix.d}$ (+1.0), em desacordo com a convenção Y-down $\to$ Y-up do PDF (onde $d$ deve ser negativo: $-1.0$).
- **Correção:** Atualizada a emissão da matriz `cm` em `FrameItem::Group` para emitir `-matrix.d`.
- **Resultado:** **Sobrescritos e limites de integral recuperaram 100% da orientação vertical correta**.

### 1.2. Pivô Central Genérico de Transformação (`01_core/src/compiler/layout/transform.rs`)
- **Problema:** Conteúdo de `#rotate(...)` e `#scale(...)` extrapolava a caixa delimitadora (`#box`).
- **Causa Raiz:** As transformações afins eram calculadas a partir da origem `(0, 0)` em vez do centro da caixa delimitadora (`origin: center + horizon`).
- **Correção:** Implementada a fórmula canônica do Vanilla Typst:
  $$T_s = T(w/2, h/2) \cdot M \cdot T(-w/2, -h/2)$$
- **Resultado:** Caixa rotacionada e escalada posicionam-se com simetria perfeita em torno do centro do frame.

---

## 2. Auditoria Geométrica dos 4 Casos da Secção 36

1. **Box 1 (`rotate(15deg)[$ a + b = c $]`):** Orientação correta dos glifos e pivô central estabilizado.
2. **Box 2 (`scale(x: 150%, y: 100%)[$ x^2 + y^2 = z^2 $]`):** Sobrescritos $2$ posicionados no topo das bases $x, y, z$.
3. **Box 3 (`rotate(-10deg)[$ \int_0^\infty e^{-x} \operatorname{dif} x $]`):** Limite superior $\infty$ no topo ($Y_{\text{TOP}} = 205.48\text{ pt}$) e limite inferior $0$ no fundo ($Y_{\text{TOP}} = 229.37\text{ pt}$).
4. **Box 4 (`skew(ax: 15deg)[$ a + b + c $]`):** Presente e alinhado no fluxo da página.

---

## 3. Estado da Suíte de Testes
- **Total de testes:** 5.961
- **Aprovados:** 5.961
- **Falhas:** 0 (100% de sucesso)

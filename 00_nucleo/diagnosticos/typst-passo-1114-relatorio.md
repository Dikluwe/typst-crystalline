# Relatório Oficial — Passo 1114: Diagnóstico de Tamanho de Fonte Embutido e Métricas Verticais na Secção 35

**Data:** 20 de Agosto de 2026  
**Status:** DIAGNÓSTICO E MAPEAMENTO CONCLUÍDOS  
**Foco:** Investigação de `#text(size: ...)` embutido em equações matemáticas (`.typ/sec_35.typ`), cálculo de `item_width` em fontes matemáticas e isolamento do descent extra.

---

## 1. Resumo Executivo

A auditoria da Secção 35 (`.typ/sec_35.typ`) revelou duas causas raiz nas equações com variação de tamanho de fonte:
1. **Diferença de Largura Horizontal em `#text(size: 20pt)[$b$]`:**  
   Em `layout_external`, a medição de largura (`item_width`) para nós de texto no sub-frame utiliza a fonte de texto ativa (`New Computer Modern Regular`, $429\text{ upem}$), em vez da fonte matemática (`New Computer Modern Math`, $443\text{ upem}$), gerando uma diferença de $0.2800\text{ pt}$ no avanço do glifo $b$ a 20pt.
2. **Deslocamento Vertical Acumulado nas Equações 2 e 3 ($\Delta Y \approx +3.86\text{ pt}$):**  
   No sub-frame isolado de `#text(size: 20pt)[$b$]`, a altura total do sub-frame inclui o leading de parágrafo ($1.25 \times 20\text{ pt}$), inflando o `descent` retornado por `layout_external` e empurrando a baseline das equações seguintes para baixo.

---

## 2. Tabela de Diagnóstico das 3 Equações da Secção 35

| EQUAÇÃO | EXPRESSÃO | COORDENADA $Y$ (VANIL / CRYST) | COORDENADA $X$ (VANIL / CRYST) | DIAGNÓSTICO |
|---|---|---|---|---|
| **Eq 1** | `$ a + #text(size: 20pt)[$b$] + c $` | $59.5317\text{ pt}$ / $59.5316\text{ pt}$ ($\Delta = 0.0001\text{ pt}$) | $b$: $194.0458\text{ pt}$ / $194.1858\text{ pt}$ | $Y$ em Paridade Exata; $\Delta X = 0.28\text{ pt}$ por medição de upem em `layout_external`. |
| **Eq 2** | `$ #text(size: 8pt)[$x^2 + y^2$] = #text(size: 16pt)[$z^2$] $` | $86.8895\text{ pt}$ / $90.7544\text{ pt}$ ($\Delta = +3.8649\text{ pt}$) | $z$: $210.7096\text{ pt}$ / $211.1268\text{ pt}$ | Deslocamento vertical provocado pelo `descent` inflado da Eq 1. |
| **Eq 3** | `$ display(a) quad text(a) quad script(a) quad sscript(a) $` | $106.5914\text{ pt}$ / $110.8784\text{ pt}$ ($\Delta = +4.2870\text{ pt}$) | `display(a)`: $171.2508\text{ pt}$ / $172.2749\text{ pt}$ | Deslocamento vertical cumulativo das equações anteriores. |

---

## 3. Plano de Correção Técnica para Fechamento
1. **`layout_external`**: Configurar o `Layouter` do sub-frame para herdar `math_style` com `leading: 0.0` (sem expansão de parágrafo para fragmentos matemáticos embutidos) e calcular `descent` puramente a partir dos extents de tinta (`ink_bottom`).
2. **Métricas de Fonte em Sub-frame**: Propagar `math: true` para a resolução de fontes no sub-frame matemático.

# Relatório Oficial — Passo 1116: Diagnóstico da Secção 36 (Transformações Geométricas em Matemática)

**Data:** 20 de Agosto de 2026  
**Status:** DIAGNÓSTICO E MAPEAMENTO DA SECÇÃO 36  
**Foco:** Investigação de `#rotate`, `#scale` e `#skew` embutidos em `#box` com fórmulas matemáticas (`.typ/sec_36.typ`).

---

## 1. Diagnóstico Geral da Secção 36

A auditoria de `.typ/sec_36.typ` revelou três divergências fundamentais entre o Vanilla Typst e o Crystalline:

1. **Disposição Horizontal Inline de Caixas (`#box`):**
   - No Vanilla Typst, quatro elementos `#box` sucessivos separados por quebra de linha simples no código fonte são dispostos lado a lado horizontalmente na mesma linha (fluxo inline), ocupando uma largura total de $4 \times 120\text{ pt} + \text{gaps} \approx 500\text{ pt}$.
   - No Crystalline, `transform::layout` chama `layouter.flush_line()` incondicionalmente, quebrando cada `#box` para uma nova linha vertical.

2. **Largura da Página em Modo `page(width: auto)`:**
   - No Vanilla, a largura da página `auto` adapta-se à largura máxima do conteúdo real (largura das caixas + margens $= 767.5239\text{ pt}$). O parágrafo de prosa é quebrado em duas linhas.
   - No Crystalline, o parágrafo de prosa não sofre quebra e expande a página para $1808.5275\text{ pt}$.

3. **Matriz de Transformação Afim no Stream PDF (`FrameItem::Group` em `stream.rs`):**
   - A conversão de coordenadas locais Y-down (Layouter) para Y-up (PDF) dentro de `FrameItem::Group` requer ajuste de sinal na matriz `cm` (`d = -matrix.d` ou translação relativa ao centro da caixa).

---

## 2. Tabela de Mapeamento dos 4 Exemplos da Secção 36

| CAIXA | OPERAÇÃO | CONTEÚDO | STATUS ATUAL |
|---|---|---|---|
| **Box 1** | `#rotate(15deg)` | `$ a + b = c $` | Posição vertical deslocada por quebra forçada de linha |
| **Box 2** | `#scale(x: 150%, y: 100%)` | `$ x^2 + y^2 = z^2 $` | Renderizado abaixo da Box 1 em vez de adjacente |
| **Box 3** | `#rotate(-10deg)` | `$ integral_0^oo e^(-x) dif x $` | Não renderizado corretamente no fluxo inline |
| **Box 4** | `#skew(ax: 15deg)` | `$ a + b + c $` | Não renderizado corretamente no fluxo inline |

---

## 3. Plano de Correção Técnica
1. **`transform.rs`**: Suprimir o `layouter.flush_line()` quando `Transform` está dentro de um container inline (`#box`).
2. **`boxed.rs` / `Layouter`**: Preservar o fluxo horizontal inline para múltiplos `#box` adjacentes.
3. **`stream.rs`**: Alinhar a matriz `cm` de `FrameItem::Group` com a convenção do Vanilla Typst.

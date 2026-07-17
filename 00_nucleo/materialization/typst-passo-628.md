---
# P628 — Sonda: unificar os cinco caminhos de layout numa função partilhada

> **Passo:** 628
> **Data:** 2026-07-09
> **Foco:** Três bugs de propagação confirmados (P579/580, P625, P626) apontam para a mesma causa raiz: o fluxo principal e os quatro sub-layouts (`grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs`) implementam separadamente "como colocar texto num espaço". P593 já unificou a largura de texto com sucesso (`text_width`, `line_content_right`). Este passo é só sonda — mapear o que precisaria de ser unificado, o que é genuinamente diferente entre os cinco contextos, e o tamanho real do trabalho, antes de qualquer código.
> **Tipo:** Sonda directa. Sem código neste passo.
> **Tamanho:** M para a sonda; a implementação, se avançar, é provavelmente L ou XL.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mudança que toca cinco caminhos de código ao mesmo tempo, o maior risco de regressão simultânea desta sequência inteira. Sonda obrigatória, sem excepção.

---

## Contexto

Cada um dos cinco caminhos resolve um problema mais amplo do que só "colocar texto":

| Caminho | Problema mais amplo que resolve |
|---|---|
| Fluxo principal | Fluir texto por uma página inteira, com quebra de página automática. |
| `grid.rs` | Dividir espaço em células, com dimensionamento automático de colunas/linhas. |
| `placement.rs` | Posicionar conteúdo num ponto exacto, sem afectar o fluxo à volta. |
| `columns.rs` | Fluir texto entre colunas dentro da mesma página. |
| `boxed.rs` | Restringir conteúdo a uma largura (e por vezes altura) fixa, com corte. |

O que têm em comum, dentro de cada um: pegar num pedaço de conteúdo e decidir onde cada palavra/linha fica, dada uma largura disponível — o "núcleo" que já foi parcialmente unificado em P593 para largura, e precisa de ser confirmado/estendido para altura de linha, alinhamento, e potencialmente mais mecanismos ainda por descobrir.

---

## Sonda

### Mapear, função a função, o que cada caminho já partilha e o que ainda duplica

```bash
grep -n "fn layout\|fn measure\|text_width\|line_content_right\|align_current_line_rtl\|font_size_pt\|self.style.size" \
  01_core/src/engine/layout/mod.rs \
  01_core/src/engine/layout/cursor.rs \
  01_core/src/engine/layout/grid.rs \
  01_core/src/engine/layout/placement.rs \
  01_core/src/engine/layout/columns.rs \
  01_core/src/engine/layout/boxed.rs
```

Construir uma tabela: para cada mecanismo já conhecido (largura de palavra, altura de linha, alinhamento RTL, ordem de colunas), confirmar se cada um dos cinco caminhos já usa a versão partilhada, ou ainda tem código próprio.

### Identificar o núcleo verdadeiramente comum

Para cada caminho, listar o que é genuinamente específico desse contexto (não deve ser unificado):

- `grid.rs`: dimensionamento de colunas/linhas (`auto`, `fr`, fixo).
- `placement.rs`: cálculo de posição absoluta a partir de `top`/`left`/etc.
- `columns.rs`: decisão de quando passar à coluna seguinte vs. nova página.
- `boxed.rs`: corte de conteúdo que excede as dimensões.

Confirmar que nenhuma destas responsabilidades específicas seria absorvida pela função partilhada — só o núcleo comum (medir e posicionar texto dentro de uma largura já decidida) deve ser extraído.

### Desenhar a interface da função partilhada, sem a implementar

```rust
// Esboço para discussão, não implementação:
fn layout_text_in_region(
    &mut self,
    content: &Content,
    region: LayoutRegion,  // largura, altura, direcção
) -> Vec<FrameItem>
```

Confirmar que uma interface deste tipo consegue mesmo servir os cinco contextos, ou se há alguma incompatibilidade que só aparece ao tentar desenhar a assinatura (por exemplo, `placement.rs` pode precisar de devolver informação extra que os outros não precisam).

### Critério de fecho da sonda

- [ ] Tabela completa: cada mecanismo conhecido (largura, altura, alinhamento, ordem de colunas) × cada um dos cinco caminhos, marcando "partilhado" ou "próprio".
- [ ] Lista do que é genuinamente específico de cada caminho, a não unificar.
- [ ] Interface da função partilhada esboçada, com pelo menos uma verificação de que serve os cinco casos.
- [ ] Estimativa de tamanho revista, com base no que a sonda encontrar — não assumir "L ou XL" sem confirmar.
- [ ] Proposta de divisão em passos menores, se o trabalho for demasiado grande para um só (por exemplo: primeiro unificar entre dois caminhos mais parecidos, como `boxed.rs` e `grid.rs`, antes de tentar os cinco de uma vez).

---

## Decisão

Este passo não implementa nada. Produz o mapa e a decisão sobre se avançar, como avançar, e em que ordem — a mesma disciplina já usada em P614 para a sonda de escrita vertical, que correctamente não tentou implementar nada sem primeiro perceber o tamanho real do trabalho.

---

## Critério de fecho do passo

- [ ] Sonda completa, tabela preenchida com evidência, não suposição.
- [ ] Interface esboçada e verificada contra os cinco casos.
- [ ] Estimativa de tamanho honesta.
- [ ] Proposta de divisão em passos menores, se necessário.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p628.md`.
